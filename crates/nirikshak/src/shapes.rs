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

/// The text of a call's first argument, when it is a string literal,
/// or a whole number as written, such as `0o777`.
#[must_use]
pub fn first_text(first: Option<&syn::Expr>) -> Option<String> {
    match first {
        Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(s),
            ..
        })) => Some(s.value()),
        Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(n),
            ..
        })) => Some(n.to_string()),
        _ => None,
    }
}

/// Every string literal in a macro's tokens, nested groups included,
/// with its line.
#[must_use]
pub fn token_strings(tokens: &TokenStream) -> Vec<(String, usize)> {
    let mut found = Vec::new();
    for tree in tokens.clone() {
        match tree {
            TokenTree::Group(group) => {
                found.extend(token_strings(&group.stream()));
            }
            TokenTree::Literal(literal) => {
                let at = line_of(literal.span());
                let tree = TokenTree::Literal(literal);
                if let Ok(s) = syn::parse2::<syn::LitStr>(tree.into()) {
                    found.push((s.value(), at));
                }
            }
            TokenTree::Ident(_) | TokenTree::Punct(_) => {}
        }
    }
    found
}

/// A short written form of what a method is called on: paths, fields,
/// `&`, `*`, indexing as `[]` and calls as `()`, without their
/// arguments. Other shapes have none.
#[must_use]
pub fn receiver_text(expr: &syn::Expr) -> Option<String> {
    let around = |inner: &syn::Expr, before: &str, after: &str| {
        receiver_text(inner).map(|t| format!("{before}{t}{after}"))
    };
    match expr {
        syn::Expr::Path(path) => Some(
            path.path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
        ),
        syn::Expr::Field(field) => {
            let member = match &field.member {
                syn::Member::Named(name) => name.to_string(),
                syn::Member::Unnamed(index) => index.index.to_string(),
            };
            around(&field.base, "", &format!(".{member}"))
        }
        syn::Expr::MethodCall(call) => {
            around(&call.receiver, "", &format!(".{}()", call.method))
        }
        syn::Expr::Call(call) => around(&call.func, "", "()"),
        syn::Expr::Reference(inner) => around(&inner.expr, "&", ""),
        syn::Expr::Paren(inner) => receiver_text(&inner.expr),
        syn::Expr::Index(index) => around(&index.expr, "", "[]"),
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

/// Whether an expression is a range from the start: `..n`, `..=n`,
/// `0..n` or `0..=n`.
#[must_use]
pub fn from_start(expr: &syn::Expr) -> bool {
    let syn::Expr::Range(range) = expr else {
        return false;
    };
    let zero = |e: &syn::Expr| {
        matches!(e, syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(n), .. })
            if n.base10_digits() == "0")
    };
    range.end.is_some() && range.start.as_deref().is_none_or(zero)
}
