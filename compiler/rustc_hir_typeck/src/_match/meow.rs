use rustc_errors::codes::*;
use rustc_errors::{
    Applicability, Diag, DiagArgValue, DiagCtxtHandle, Diagnostic, EmissionGuarantee, Level,
    MultiSpan, Subdiagnostic, msg,
};
use rustc_hir::def_id::DefId;
use rustc_macros::{Diagnostic, Subdiagnostic};
use rustc_middle::ty::{self, Ty, TyCtxt, TypingEnv};
use rustc_pattern_analysis::diagnostics::Uncovered;
use rustc_pattern_analysis::rustc::RustcPatCtxt;
use rustc_span::{Ident, Span, Symbol};

pub(crate) struct NonExhaustivePatternsTypeNotEmpty<'tcx> {
    pub(crate) tcx: TyCtxt<'tcx>,
    pub(crate) module: DefId,
    pub(crate) typing_env: TypingEnv<'tcx>,
    pub(crate) scrut_span: Span,
    pub(crate) braces_span: Option<Span>,
    pub(crate) ty: Ty<'tcx>,
}

impl<'a, G: EmissionGuarantee> Diagnostic<'a, G> for NonExhaustivePatternsTypeNotEmpty<'_> {
    fn into_diag(self, dcx: DiagCtxtHandle<'a>, level: Level) -> Diag<'a, G> {
        let mut diag =
            Diag::new(dcx, level, msg!("non-exhaustive patterns: type `{$ty}` is non-empty"));
        diag.span(self.scrut_span);
        diag.code(E0004);
        let peeled_ty = self.ty.peel_refs();
        diag.arg("ty", self.ty);
        diag.arg("peeled_ty", peeled_ty);

        if let ty::Adt(def, _) = peeled_ty.kind() {
            let def_span = self
                .tcx
                .hir_get_if_local(def.did())
                .and_then(|node| node.ident())
                .map(|ident| ident.span)
                .unwrap_or_else(|| self.tcx.def_span(def.did()));

            // workaround to make test pass
            let mut span: MultiSpan = def_span.into();
            span.push_span_label(def_span, "");

            diag.span_note(span, msg!("`{$peeled_ty}` defined here"));
        }

        let is_non_exhaustive = matches!(self.ty.kind(),
            ty::Adt(def, _) if def.variant_list_has_applicable_non_exhaustive());
        if is_non_exhaustive {
            diag.note(msg!(
                "the matched value is of type `{$ty}`, which is marked as non-exhaustive"
            ));
        } else {
            diag.note(msg!("the matched value is of type `{$ty}`"));
        }

        if let ty::Ref(_, sub_ty, _) = self.ty.kind() {
            if !sub_ty.is_inhabited_from(self.tcx, self.module, self.typing_env) {
                diag.note(msg!("references are always considered inhabited"));
            }
        }

        let sm = self.tcx.sess.source_map();
        if let Some(braces_span) = self.braces_span {
            // Get the span for the empty match body `{}`.
            let (indentation, more) = if let Some(snippet) = sm.indentation_before(self.scrut_span)
            {
                (format!("\n{snippet}"), "    ")
            } else {
                (" ".to_string(), "")
            };
            diag.span_suggestion_verbose(
                braces_span,
                msg!("ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown"),
                // ignore-tidy-todo
                format!(" {{{indentation}{more}_ => todo!(),{indentation}}}"),
                Applicability::HasPlaceholders,
            );
        } else {
            diag.help(msg!(
                "ensure that all possible cases are being handled by adding a match arm with a wildcard pattern"
            ));
        }

        diag
    }
}
