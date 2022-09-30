use crate::{LateContext, LateLintPass, LintContext};
use hir::def::{DefKind, Res};
use rustc_data_structures::fx::FxHashMap;
use rustc_hir as hir;
use rustc_span::Symbol;

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
        let DefKind::Fn = def_kind else { return };

        // FIXME: support intrinsics somehow (can't `expect_item` them in `fn_arg_names`)
        if cx.tcx.is_intrinsic(def_id) {
            return;
        }

        let named_params: FxHashMap<Symbol, usize> = cx
            .tcx
            .fn_arg_names(def_id)
            .iter()
            .enumerate()
            .map(|(idx, &ident)| (ident.name, idx))
            .collect();

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
            lint.set_span(f.span).span_labels(misplaced, "misplaced")
        });
    }
}

fn expr_ident(cx: &LateContext<'_>, expr: &hir::Expr<'_>) -> Option<Symbol> {
    match &expr.kind {
        hir::ExprKind::Path(qpath) => match cx.qpath_res(qpath, expr.hir_id) {
            Res::Local(hir_id) => cx.tcx.hir().opt_name(hir_id),
            Res::Def(_, _) => None, /* FIXME? */

            _ => None,
        },
        hir::ExprKind::Field(_, ident) => Some(ident.name),
        hir::ExprKind::AddrOf(_, _, expr) => expr_ident(cx, expr),
        hir::ExprKind::Box(_)
        | hir::ExprKind::ConstBlock(_)
        | hir::ExprKind::Array(_)
        | hir::ExprKind::Call(_, _)
        | hir::ExprKind::MethodCall(_, _, _, _)
        | hir::ExprKind::Tup(_)
        | hir::ExprKind::Binary(_, _, _)
        | hir::ExprKind::Unary(_, _)
        | hir::ExprKind::Lit(_)
        | hir::ExprKind::Cast(_, _)
        | hir::ExprKind::Type(_, _)
        | hir::ExprKind::DropTemps(_)
        | hir::ExprKind::Let(_)
        | hir::ExprKind::If(_, _, _)
        | hir::ExprKind::Loop(_, _, _, _)
        | hir::ExprKind::Match(_, _, _)
        | hir::ExprKind::Closure(_)
        | hir::ExprKind::Block(_, _)
        | hir::ExprKind::Assign(_, _, _)
        | hir::ExprKind::AssignOp(_, _, _)
        | hir::ExprKind::Index(_, _)
        | hir::ExprKind::Break(_, _)
        | hir::ExprKind::Continue(_)
        | hir::ExprKind::Ret(_)
        | hir::ExprKind::InlineAsm(_)
        | hir::ExprKind::Struct(_, _, _)
        | hir::ExprKind::Repeat(_, _)
        | hir::ExprKind::Yield(_, _)
        | hir::ExprKind::Err => None,
    }
}
