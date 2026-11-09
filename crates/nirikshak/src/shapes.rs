//! Small tests on the shapes of Rust syntax, used by the fact reader.

use proc_macro2::{Span, TokenStream, TokenTree};

/// The line a piece of source starts on, from 1.
#[must_use]
pub fn line_of(span: Span) -> usize {
    span.start().line
}

/// Whether an attribute marks a test, such as `#[test]` or
/// `#[tokio::test]`.
#[must_use]
pub fn is_test_attr(attr: &syn::Attribute) -> bool {
    attr.path()
        .segments
        .last()
        .is_some_and(|s| s.ident == "test")
}

/// Whether an attribute is `#[cfg(test)]` or another `cfg` naming test.
#[must_use]
pub fn is_cfg_test(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("cfg")
        && attr.meta.require_list().is_ok_and(|list| {
            list.tokens
                .to_string()
                .split(|c: char| !c.is_alphanumeric())
                .any(|word| word == "test")
        })
}

/// Whether a path is in a test folder, or is a test file of a module.
#[must_use]
pub fn in_test_file(path: &str) -> bool {
    path.ends_with("_tests.rs")
        || ["tests/", "benches/"].iter().any(|folder| {
            path.starts_with(folder) || path.contains(&format!("/{folder}"))
        })
}

/// Whether an expression makes a call, so its value may be a result.
#[must_use]
pub fn is_call(expr: &syn::Expr) -> bool {
    matches!(
        expr,
        syn::Expr::Call(_)
            | syn::Expr::MethodCall(_)
            | syn::Expr::Macro(_)
            | syn::Expr::Await(_)
            | syn::Expr::Try(_)
    )
}

/// Whether an expression is `{}` or `()`.
#[must_use]
pub fn is_empty(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Block(block) => block.block.stmts.is_empty(),
        syn::Expr::Tuple(tuple) => tuple.elems.is_empty(),
        _ => false,
    }
}

/// The span of `Err(_)` in a pattern, when it is one.
#[must_use]
pub fn err_wild(pat: &syn::Pat) -> Option<Span> {
    let syn::Pat::TupleStruct(tuple) = pat else {
        return None;
    };
    let last = tuple.path.segments.last()?;
    let wild = tuple.elems.iter().all(|p| matches!(p, syn::Pat::Wild(_)));
    (last.ident == "Err" && wild).then(|| last.ident.span())
}

/// The text of a call's first argument, when it is a string literal.
#[must_use]
pub fn first_text(first: Option<&syn::Expr>) -> Option<String> {
    match first {
        Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        })) => Some(s.value()),
        _ => None,
    }
}

/// The text of a macro's first argument, when it is a string literal.
#[must_use]
pub fn macro_text(tokens: &TokenStream) -> Option<String> {
    let first = tokens.clone().into_iter().next()?;
    let literal = matches!(first, TokenTree::Literal(_)).then_some(first)?;
    syn::parse2::<syn::LitStr>(literal.into())
        .ok()
        .map(|s| s.value())
}
