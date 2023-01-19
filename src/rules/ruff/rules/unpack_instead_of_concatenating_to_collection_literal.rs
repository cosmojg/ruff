use rustpython_ast::{Expr, ExprContext, ExprKind, Operator};

use crate::ast::helpers::{create_expr, unparse_expr};
use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::fix::Fix;
use crate::registry::Diagnostic;
use crate::violations;

fn make_splat_elts(
    splat_element: &Expr,
    other_elements: &[Expr],
    splat_at_left: bool,
) -> Vec<Expr> {
    let mut new_elts = other_elements.to_owned();
    let splat = create_expr(ExprKind::Starred {
        value: Box::from(splat_element.clone()),
        ctx: ExprContext::Load,
    });
    if splat_at_left {
        new_elts.insert(0, splat);
    } else {
        new_elts.push(splat);
    }
    new_elts
}

/// RUF005
/// This suggestion could be unsafe if the non-literal expression in the
/// expression has overridden the `__add__` (or `__radd__`) magic methods.
pub fn unpack_instead_of_concatenating_to_collection_literal(checker: &mut Checker, expr: &Expr) {
    let ExprKind::BinOp { op, left, right } = &expr.node else {
        return;
    };
    if !matches!(op, Operator::Add) {
        return;
    }
    let new_expr = match (&left.node, &right.node) {
        (ExprKind::List { elts: l_elts, ctx }, _) => create_expr(ExprKind::List {
            elts: make_splat_elts(right, l_elts, false),
            ctx: ctx.clone(),
        }),
        (ExprKind::Tuple { elts: l_elts, ctx }, _) => create_expr(ExprKind::Tuple {
            elts: make_splat_elts(right, l_elts, false),
            ctx: ctx.clone(),
        }),
        (_, ExprKind::List { elts: r_elts, ctx }) => create_expr(ExprKind::List {
            elts: make_splat_elts(left, r_elts, true),
            ctx: ctx.clone(),
        }),
        (_, ExprKind::Tuple { elts: r_elts, ctx }) => create_expr(ExprKind::Tuple {
            elts: make_splat_elts(left, r_elts, true),
            ctx: ctx.clone(),
        }),
        _ => return,
    };

    let new_expr_string = unparse_expr(&new_expr, checker.stylist);

    let mut diagnostic = Diagnostic::new(
        violations::UnpackInsteadOfConcatenatingToCollectionLiteral(new_expr_string.clone()),
        Range::from_located(expr),
    );
    if checker.patch(diagnostic.kind.rule()) {
        diagnostic.amend(Fix::replacement(
            new_expr_string,
            expr.location,
            expr.end_location.unwrap(),
        ));
    }
    checker.diagnostics.push(diagnostic);
}
