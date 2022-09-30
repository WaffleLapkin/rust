use crate::{LateContext, LateLintPass, LintContext};
use hir::{
    def::{DefKind, Res},
    ItemKind,
};
use rustc_data_structures::fx::{FxHashMap, FxIndexMap};
use rustc_errors::{Applicability, DiagnosticBuilder, MultiSpan};
use rustc_hir as hir;
use rustc_middle::{ty, hir::Owner};
use rustc_span::{symbol::Ident, Symbol};

declare_lint! {
    /// The `...` lint checks .
    ///
    /// ### Example
    ///
    /// ```
    /// ```
    ///
    /// {{produces}}
    ///
    /// ### Explanation
    ///
    pub MAYBE_MISMATCHED_ARGUMENTS,
    Warn,
    "expl"
}

declare_lint_pass!(MaybeMismatchedArguments => [MAYBE_MISMATCHED_ARGUMENTS]);

impl<'tcx> LateLintPass<'tcx> for MaybeMismatchedArguments {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx hir::Expr<'tcx>) {
        let hir::ExprKind::Call(f, args) = expr.kind else { return };
        let hir::ExprKind::Path(qpath) = &f.kind else { return };
        let Res::Def(def_kind, def_id) = cx.qpath_res(qpath, f.hir_id) else { return };
        // FIXME: support intrinsics somehow (can't `expect_item` them)
        let DefKind::Fn = def_kind else { return };
        let Some(def_id) = def_id.as_local() else { return };
        let item = cx.tcx.hir().expect_item(def_id);
        //let hir::Node { } = cx.tcx.hir().get(def_id) else { return };
        let &ItemKind::Fn(_, gen, body) = &item.kind else { return };
        let body = cx.tcx.hir().body(body);

        let named_params: FxHashMap<_, _> =
            body.params.iter().enumerate().filter_map(param_ident).collect();

        let misplaced = args
            .iter()
            .enumerate()
            .filter_map(|(idx, arg)| expr_ident(cx, arg).zip(Some(idx)))
            .filter_map(|(name, idx)| {
                named_params.get(&name).copied().zip(Some(idx)).filter(|(i, idx)| i != idx)
            })
            .map(|(_, idx)| args[idx].span)
            .collect::<Vec<_>>();

        // FIXME: peek instead
        if misplaced.is_empty() {
            return;
        }

        cx.lint(MAYBE_MISMATCHED_ARGUMENTS, "misplaced argument or smt", |lint| {
            lint.span_labels(misplaced, "misplaced")
        });
    }
}

fn param_ident((idx, param): (usize, &hir::Param<'_>)) -> Option<(Symbol, usize)> {
    // FIXME: allow more complecated patterns like `&a` or `ref a` or `(a,)`, etc
    param.pat.simple_ident().map(|ident| (ident.name, idx))
}

fn expr_ident(cx: &LateContext<'_>, expr: &hir::Expr<'_>) -> Option<Symbol> {
    match &expr.kind {
        hir::ExprKind::Box(_) => todo!(),
        hir::ExprKind::ConstBlock(_) => todo!(),
        hir::ExprKind::Array(_) => todo!(),
        hir::ExprKind::Call(_, _) => todo!(),
        hir::ExprKind::MethodCall(_, _, _, _) => todo!(),
        hir::ExprKind::Tup(_) => todo!(),
        hir::ExprKind::Binary(_, _, _) => todo!(),
        hir::ExprKind::Unary(_, _) => todo!(),
        hir::ExprKind::Lit(_) => todo!(),
        hir::ExprKind::Cast(_, _) => todo!(),
        hir::ExprKind::Type(_, _) => todo!(),
        hir::ExprKind::DropTemps(_) => todo!(),
        hir::ExprKind::Let(_) => todo!(),
        hir::ExprKind::If(_, _, _) => todo!(),
        hir::ExprKind::Loop(_, _, _, _) => todo!(),
        hir::ExprKind::Match(_, _, _) => todo!(),
        hir::ExprKind::Closure(_) => todo!(),
        hir::ExprKind::Block(_, _) => todo!(),
        hir::ExprKind::Assign(_, _, _) => todo!(),
        hir::ExprKind::AssignOp(_, _, _) => todo!(),
        hir::ExprKind::Field(_, _) => todo!(),
        hir::ExprKind::Index(_, _) => todo!(),
        hir::ExprKind::Path(qpath) => match cx.qpath_res(qpath, expr.hir_id) {
            Res::Def(_, _) => todo!(),
            Res::Local(hir_id) => cx.tcx.hir().opt_name(hir_id),

            _ => None,
        },
        hir::ExprKind::AddrOf(_, _, _) => todo!(),
        hir::ExprKind::Break(_, _) => todo!(),
        hir::ExprKind::Continue(_) => todo!(),
        hir::ExprKind::Ret(_) => todo!(),
        hir::ExprKind::InlineAsm(_) => todo!(),
        hir::ExprKind::Struct(_, _, _) => todo!(),
        hir::ExprKind::Repeat(_, _) => todo!(),
        hir::ExprKind::Yield(_, _) => todo!(),
        hir::ExprKind::Err => todo!(),
    }
}

fn build_lint<'a, 'b>(
    lint: &'a mut DiagnosticBuilder<'b, ()>,
    local: &hir::Local<'_>,
    init_span: rustc_span::Span,
) -> &'a mut DiagnosticBuilder<'b, ()> {
    lint.span_suggestion_verbose(
        local.pat.span,
        "consider binding to an unused variable to avoid immediately dropping the value",
        "_unused",
        Applicability::MachineApplicable,
    )
    .multipart_suggestion(
        "consider immediately dropping the value",
        vec![
            (local.span.until(init_span), "drop(".to_string()),
            (init_span.shrink_to_hi(), ")".to_string()),
        ],
        Applicability::MachineApplicable,
    )
}
