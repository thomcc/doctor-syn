use syn::{Expr, ExprUnary, ExprBinary, UnOp, BinOp, ExprParen};
use syn::spanned::Spanned;
use proc_macro2::Span;

// Negate an expression.
pub fn negate(expr: &Expr) -> Expr {
    match expr {
        Expr::Unary(ExprUnary { op: UnOp::Neg(_), expr, .. }) => {
            *expr.clone()
        }
        _ => ExprUnary {
            attrs: vec!(),
            op: UnOp::Neg(syn::token::Sub(Span::call_site())),
            expr: Box::new(expr.clone()),
        }.into()
    }
}

// Strip parentheses.
pub fn deparen(expr: &Expr) -> &Expr {
    match expr {
        Expr::Paren(ExprParen { expr, .. }) => {
            &*expr
        }
        _ => {
            expr
        }
    }
}

// Convert a vector of expressions to a sum.
pub fn make_sum(sum: &[Expr]) -> Expr {
    let right = &sum[sum.len()-1];
    if sum.len() == 1 {
        right.clone()
    } else {
        let left = Box::new(make_sum(&sum[0..sum.len()-1]));
        if is_negated(&right) {
            let right = Box::new(negate(&right));
            let op = BinOp::Sub(syn::token::Sub(Span::call_site()));
            Expr::Binary(ExprBinary { attrs: vec![], left, op, right })
        } else {
            let right = Box::new(right.clone());
            let op = BinOp::Add(syn::token::Add(Span::call_site()));
            Expr::Binary(ExprBinary { attrs: vec![], left, op, right })
        }
    }
}

pub fn is_negated(expr: &Expr) -> bool {
    match expr {
        Expr::Unary(ExprUnary { op: UnOp::Neg(_), .. }) => true,
        _ => false
    }
}

pub fn _is_paren(expr: &Expr) -> bool {
    match expr {
        Expr::Paren(_) => true,
        _ => false
    }
}

pub fn make_paren(expr: Expr) -> Expr {
    ExprParen {
        attrs: vec![],
        paren_token: syn::token::Paren(expr.span()),
        expr: Box::new(expr),
    }
    .into()
}

pub fn make_binary(left: Expr, op: BinOp, right: Expr) -> Expr {
    ExprBinary {
        attrs: vec![],
        left: Box::new(left),
        op: op,
        right: Box::new(right),
    }.into()
}

pub fn make_unary(op: UnOp, expr: Expr) -> Expr {
    ExprUnary {
        attrs: vec![],
        op: op,
        expr: Box::new(expr),
    }.into()
}
