//! Facts for the code-smell rules (task #118), read from one Rust file:
//! each function's inputs, the numbers written in code, method chains,
//! the size of each struct and enum, comments on a line of their own,
//! and text written three or more times in a file's code. Each fact
//! says whether it is in test code (`kind` is `test`) or not (`code`).
//! Comments are read from the raw lines, so a `//` line inside a string
//! counts as a comment.

use std::collections::BTreeMap;

use syn::visit::{self, Visit};

use crate::facts::{Facts, number, text};
use crate::rust_facts::TOP;
use crate::shapes::{in_test_file, is_cfg_test, is_test_attr, line_of};

struct Smells<'a> {
    path: &'a str,
    // How deep in `#[cfg(test)]` modules and test functions.
    test_depth: usize,
    // How deep in places where a number is a size, not a magic value:
    // types, `const` and `static` items, and attributes.
    fixed_depth: usize,
    // Whether inside a method chain already recorded.
    in_chain: bool,
    functions: Vec<String>,
    // The first and last lines of test modules and test functions.
    test_lines: Vec<(usize, usize)>,
    // Each text written in code, and the lines of every copy.
    texts: BTreeMap<String, Vec<usize>>,
    facts: &'a mut Facts,
}

impl Smells<'_> {
    fn kind(&self) -> &'static str {
        if self.test_depth > 0 || in_test_file(self.path) {
            "test"
        } else {
            "code"
        }
    }

    fn here(
        &mut self,
        relation: &'static str,
        rest: &[(&str, serde_json::Value)],
    ) {
        let function = self
            .functions
            .last()
            .cloned()
            .unwrap_or_else(|| TOP.to_owned());
        let mut fields = vec![
            ("path", text(self.path)),
            ("function", text(function)),
            ("kind", text(self.kind())),
        ];
        fields.extend(rest.iter().cloned());
        self.facts.add(relation, &fields);
    }

    fn function(&mut self, attrs: &[syn::Attribute], sig: &syn::Signature) {
        let test = attrs.iter().any(is_test_attr);
        if test {
            self.test_depth = self.test_depth.saturating_add(1);
        }
        let bools = sig
            .inputs
            .iter()
            .filter(|i| match i {
                syn::FnArg::Typed(t) => is_bool(&t.ty),
                syn::FnArg::Receiver(_) => false,
            })
            .count();
        self.functions.push(sig.ident.to_string());
        let at = number(line_of(sig.ident.span()));
        self.here(
            "signature",
            &[
                ("line", at),
                ("inputs", number(sig.inputs.len())),
                ("bools", number(bools)),
            ],
        );
    }

    fn leave(&mut self, attrs: &[syn::Attribute]) {
        let _done = self.functions.pop();
        if attrs.iter().any(is_test_attr) {
            self.test_depth = self.test_depth.saturating_sub(1);
        }
    }

    fn fixed<F: FnOnce(&mut Self)>(&mut self, inner: F) {
        self.fixed_depth = self.fixed_depth.saturating_add(1);
        inner(self);
        self.fixed_depth = self.fixed_depth.saturating_sub(1);
    }

    fn shape(&mut self, name: &syn::Ident, what: &str, size: usize) {
        let at = number(line_of(name.span()));
        let fields = [
            ("name", text(name.to_string())),
            ("what", text(what)),
            ("size", number(size)),
            ("line", at),
        ];
        self.here("shape", &fields);
    }
}

/// Whether a type is `bool`.
fn is_bool(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(p) if p.path.is_ident("bool"))
}

/// An expression with any `?`, `.await` and brackets around it taken
/// off, so `a()?.b()` is still one chain.
fn bare(mut at: &syn::Expr) -> &syn::Expr {
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
fn chain_length(call: &syn::ExprMethodCall) -> usize {
    let mut length = 1_usize;
    let mut at = bare(&call.receiver);
    while let syn::Expr::MethodCall(inner) = at {
        length = length.saturating_add(1);
        at = bare(&inner.receiver);
    }
    length
}

/// The first and last lines of a braced item.
fn lines_of(
    open: proc_macro2::Span,
    close: proc_macro2::Span,
) -> (usize, usize) {
    (line_of(open), line_of(close))
}

impl<'ast> Visit<'ast> for Smells<'_> {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let test = item.attrs.iter().any(is_cfg_test);
        if test {
            self.test_depth = self.test_depth.saturating_add(1);
            if let Some((brace, _)) = &item.content {
                let open = item.mod_token.span;
                self.test_lines.push(lines_of(open, brace.span.close()));
            }
        }
        visit::visit_item_mod(self, item);
        if test {
            self.test_depth = self.test_depth.saturating_sub(1);
        }
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        if item.attrs.iter().any(is_test_attr) {
            let close = item.block.brace_token.span.close();
            self.test_lines
                .push(lines_of(item.sig.fn_token.span, close));
        }
        self.function(&item.attrs, &item.sig);
        visit::visit_item_fn(self, item);
        self.leave(&item.attrs);
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        self.function(&item.attrs, &item.sig);
        visit::visit_impl_item_fn(self, item);
        self.leave(&item.attrs);
    }

    fn visit_trait_item_fn(&mut self, item: &'ast syn::TraitItemFn) {
        // With a body or without: the signature is the trait's own.
        self.function(&item.attrs, &item.sig);
        visit::visit_trait_item_fn(self, item);
        self.leave(&item.attrs);
    }

    fn visit_lit_str(&mut self, lit: &'ast syn::LitStr) {
        if self.fixed_depth == 0
            && !self.functions.is_empty()
            && self.kind() == "code"
        {
            let at = line_of(lit.span());
            self.texts.entry(lit.value()).or_default().push(at);
        }
    }

    fn visit_item_const(&mut self, item: &'ast syn::ItemConst) {
        self.fixed(|s| visit::visit_item_const(s, item));
    }

    fn visit_item_static(&mut self, item: &'ast syn::ItemStatic) {
        self.fixed(|s| visit::visit_item_static(s, item));
    }

    fn visit_type(&mut self, ty: &'ast syn::Type) {
        self.fixed(|s| visit::visit_type(s, ty));
    }

    fn visit_attribute(&mut self, _attribute: &'ast syn::Attribute) {}

    fn visit_lit_int(&mut self, lit: &'ast syn::LitInt) {
        if self.fixed_depth == 0 && !self.functions.is_empty() {
            let at = number(line_of(lit.span()));
            self.here(
                "number",
                &[("text", text(lit.base10_digits())), ("line", at)],
            );
        }
    }

    fn visit_lit_float(&mut self, lit: &'ast syn::LitFloat) {
        if self.fixed_depth == 0 && !self.functions.is_empty() {
            let at = number(line_of(lit.span()));
            self.here(
                "number",
                &[("text", text(lit.base10_digits())), ("line", at)],
            );
        }
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        let outer = !self.in_chain;
        if outer {
            let at = number(line_of(call.method.span()));
            self.here(
                "chain",
                &[("length", number(chain_length(call))), ("line", at)],
            );
        }
        // Only a chain goes on being this chain; anything else in the
        // receiver, such as `f(x.a())`, holds chains of its own.
        let inner = bare(&call.receiver);
        self.in_chain = matches!(inner, syn::Expr::MethodCall(_));
        self.visit_expr(inner);
        self.in_chain = false;
        for arg in &call.args {
            self.visit_expr(arg);
        }
        self.in_chain = !outer;
    }

    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        self.shape(&item.ident, "struct", item.fields.len());
        visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
        self.shape(&item.ident, "enum", item.variants.len());
        visit::visit_item_enum(self, item);
    }
}

/// Adds the smell facts of one parsed Rust file to `facts`.
pub fn read(path: &str, source: &str, file: &syn::File, facts: &mut Facts) {
    let mut smells = Smells {
        path,
        test_depth: 0,
        fixed_depth: 0,
        in_chain: false,
        functions: Vec::new(),
        test_lines: Vec::new(),
        texts: BTreeMap::new(),
        facts,
    };
    smells.visit_file(file);
    let (test_lines, texts) = (smells.test_lines, smells.texts);
    text_facts::comments(path, source, &test_lines, facts);
    text_facts::repeated(path, &texts, facts);
}

#[path = "smell_text.rs"]
mod text_facts;

#[cfg(test)]
#[path = "smell_facts_tests.rs"]
mod tests;
