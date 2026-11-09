//! Small helpers for `smell_facts`: reading types, chains, line ranges
//! and the names a macro's input uses.

use crate::shapes::line_of;

/// Whether a type is `bool`.
pub fn is_bool(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.is_ident("bool"))
}

/// An expression with any `?`, `.await` and brackets around it taken
/// off, so `a()?.b()` is still one chain.
pub fn bare(mut at: &syn::Expr) -> &syn::Expr {
    loop {
        at = match at {
            syn::Expr::Try(t) => &t.expr,
            syn::Expr::Await(a) => &a.base,
            syn::Expr::Paren(p) => &p.expr,
            _ => return at,
        };
    }
}

/// How many method calls a chain ends with: `a.b()?.c()` is 2.
pub fn chain_length(call: &syn::ExprMethodCall) -> usize {
    let mut length = 1_usize;
    let mut at = bare(&call.receiver);
    while let syn::Expr::MethodCall(inner) = at {
        length = length.saturating_add(1);
        at = bare(&inner.receiver);
    }
    length
}

/// The first and last lines of a braced item.
pub fn lines_of(
    open: proc_macro2::Span,
    close: proc_macro2::Span,
) -> (usize, usize) {
    (line_of(open), line_of(close))
}

/// Every name in a macro's input, each with its line, nested macros
/// included, whatever shape the input has.
pub fn macro_names(mac: &syn::Macro) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    names_in(mac.tokens.clone(), &mut out);
    out
}

fn names_in(tokens: proc_macro2::TokenStream, out: &mut Vec<(String, usize)>) {
    for tree in tokens {
        match tree {
            proc_macro2::TokenTree::Ident(name) => {
                out.push((name.to_string(), line_of(name.span())));
            }
            proc_macro2::TokenTree::Group(group) => {
                names_in(group.stream(), out);
            }
            proc_macro2::TokenTree::Punct(_)
            | proc_macro2::TokenTree::Literal(_) => {}
        }
    }
}
