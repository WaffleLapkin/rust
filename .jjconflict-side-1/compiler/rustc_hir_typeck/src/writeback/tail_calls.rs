use rustc_abi::ExternAbi;
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::{ErrorGuaranteed, Span};

pub(super) fn report_builtin_op(
    tcx: TyCtxt<'_>,
    call_expr: &hir::Expr<'_>,
    become_span: rustc_span::Span,
) -> ErrorGuaranteed {
    tcx.dcx()
        .struct_span_err(call_expr.span, "`become` does not support operators")
        .with_note("using `become` on a builtin operator is not useful")
        .with_span_suggestion(
            become_span,
            "try using `return` instead",
            "return ",
            Applicability::MachineApplicable,
        )
        .emit()
}

pub(super) fn report_non_call(
    tcx: TyCtxt<'_>,
    call_expr: &hir::Expr<'_>,
    become_span: rustc_span::Span,
) -> ErrorGuaranteed {
    tcx.dcx()
        .struct_span_err(become_span, "`become` requires a function call")
        .with_span_note(call_expr.span, "not a function call")
        .with_span_suggestion(
            become_span,
            "try using `return` instead",
            "return ",
            Applicability::MaybeIncorrect,
        )
        .emit()
}

pub(super) fn report_calling_closure(
    tcx: TyCtxt<'_>,
    become_expr: &hir::Expr<'_>,
    fun_expr: &hir::Expr<'_>,
    tupled_args: Ty<'_>,
) -> ErrorGuaranteed {
    let underscored_args = match tupled_args.kind() {
        ty::Tuple(tys) if tys.is_empty() => "".to_owned(),
        ty::Tuple(tys) => std::iter::repeat_n("_, ", tys.len() - 1).chain(["_"]).collect(),
        _ => "_".to_owned(),
    };

    tcx.dcx()
        .struct_span_err(become_expr.span, "tail calling closures directly is not allowed")
        .with_multipart_suggestion(
            "try casting the closure to a function pointer type",
            vec![
                (fun_expr.span.shrink_to_lo(), "(".to_owned()),
                (fun_expr.span.shrink_to_hi(), format!(" as fn({underscored_args}) -> _)")),
            ],
            Applicability::MaybeIncorrect,
        )
        .emit()
}

pub(super) fn report_calling_nonfn(
    tcx: TyCtxt<'_>,
    call_sp: Span,
    fun_sp: Span,
    ty: Ty<'_>,
) -> ErrorGuaranteed {
    let mut err = tcx
        .dcx()
        .struct_span_err(
            call_sp,
            "tail calls can only be performed with function definitions or pointers",
        )
        .with_note(format!("callee has type `{ty}`"));

    let mut ty = ty;
    let mut refs = 0;
    while ty.is_box() || ty.is_ref() {
        ty = ty.builtin_deref(false).unwrap();
        refs += 1;
    }

    if refs > 0 && ty.is_fn() {
        let thing = if ty.is_fn_ptr() { "pointer" } else { "definition" };

        let derefs = std::iter::once('(').chain(std::iter::repeat_n('*', refs)).collect::<String>();

        err.multipart_suggestion(
            format!("consider dereferencing the expression to get a function {thing}"),
            vec![(fun_sp.shrink_to_lo(), derefs), (fun_sp.shrink_to_hi(), ")".to_owned())],
            Applicability::MachineApplicable,
        );
    }

    err.emit()
}

pub(super) fn report_calling_intrinsic(tcx: TyCtxt<'_>, become_expr_span: Span) -> ErrorGuaranteed {
    tcx.dcx().struct_span_err(become_expr_span, "tail calling intrinsics is not allowed").emit()
}

pub(super) fn report_unsupported_abi(
    tcx: TyCtxt<'_>,
    sp: Span,
    callee_abi: ExternAbi,
) -> ErrorGuaranteed {
    tcx.dcx()
        .struct_span_err(sp, "ABI does not support guaranteed tail calls")
        .with_note(format!("`become` is not supported for `extern {callee_abi}` functions"))
        .emit()
}

pub(super) fn report_signature_mismatch(
    tcx: TyCtxt<'_>,
    sp: Span,
    caller_sig: ty::Binder<'_, ty::FnSig<'_>>,
    callee_sig: ty::Binder<'_, ty::FnSig<'_>>,
) -> ErrorGuaranteed {
    tcx.dcx()
        .struct_span_err(sp, "mismatched signatures")
        .with_note("`become` requires caller and callee to have matching signatures")
        .with_note(format!("caller signature: `{caller_sig}`"))
        .with_note(format!("callee signature: `{callee_sig}`"))
        .emit()
}

pub(super) fn report_track_caller_caller(tcx: TyCtxt<'_>, span: Span) -> ErrorGuaranteed {
    tcx.dcx()
        .struct_span_err(
            span,
            "a function marked with `#[track_caller]` cannot perform a tail-call",
        )
        .emit()
}

pub(super) fn report_c_variadic_caller(tcx: TyCtxt<'_>, sp: Span) -> ErrorGuaranteed {
    tcx.dcx()
        // FIXME(explicit_tail_calls): highlight the `...`
        .struct_span_err(sp, "tail-calls are not allowed in c-variadic functions")
        .emit()
}

pub(super) fn report_c_variadic_callee(tcx: TyCtxt<'_>, sp: Span) -> ErrorGuaranteed {
    tcx.dcx()
        // FIXME(explicit_tail_calls): highlight the function or something...
        .struct_span_err(sp, "c-variadic functions can't be tail-called")
        .emit()
}
