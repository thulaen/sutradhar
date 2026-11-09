//! Reads one Rust file with `syn` and records its functions, calls,
//! macros, `?`, thrown-away results and empty error handlers, each with
//! the function it is in. A file `syn` cannot read becomes a
//! `parse_failure` fact, never silence.

use syn::visit::{self, Visit};

use crate::facts::{Facts, number, text};
use crate::shapes::{
    err_wild, first_text, from_start, in_test_file, is_call, is_cfg_test,
    is_empty, is_test_attr, line_of, macro_text, receiver_text, token_strings,
};
use crate::uses::{Aliases, aliases, expand};

/// The function name for code outside any function.
pub const TOP: &str = "<top>";

struct Walker<'a> {
    path: &'a str,
    aliases: Aliases,
    scopes: Vec<String>,
    test_depth: usize,
    // The last part of the trait the current `impl` block implements.
    implementing: Option<String>,
    facts: &'a mut Facts,
}

impl Walker<'_> {
    fn scope(&self) -> String {
        self.scopes
            .last()
            .cloned()
            .unwrap_or_else(|| TOP.to_owned())
    }

    /// Adds a fact that names this file and the function it is in.
    fn here(
        &mut self,
        relation: &'static str,
        rest: &[(&str, serde_json::Value)],
    ) {
        let mut fields =
            vec![("path", text(self.path)), ("function", text(self.scope()))];
        fields.extend(rest.iter().cloned());
        self.facts.add(relation, &fields);
    }

    fn kind(&self, attrs: &[syn::Attribute]) -> &'static str {
        let test = attrs.iter().any(is_test_attr);
        let panics = attrs.iter().any(|a| a.path().is_ident("should_panic"));
        match (test, panics) {
            (true, true) => "test_panics",
            (true, false) => "test",
            _ if self.test_depth > 0 || in_test_file(self.path) => {
                "test_support"
            }
            _ => "code",
        }
    }

    fn function(
        &mut self,
        attrs: &[syn::Attribute],
        sig: &syn::Signature,
        last: usize,
    ) {
        let name = sig.ident.to_string();
        let fields = [
            ("path", text(self.path)),
            ("name", text(name.clone())),
            ("first", number(line_of(sig.ident.span()))),
            ("last", number(last)),
            ("kind", text(self.kind(attrs))),
        ];
        self.facts.add("function", &fields);
        self.scopes.push(name);
    }

    fn call(&mut self, callee: String, at: usize) {
        self.here("call", &[("callee", text(callee)), ("line", number(at))]);
    }

    fn range_cut(&mut self, on: &syn::Expr, at: usize) {
        if let Some(receiver) = receiver_text(on) {
            let fields = [("receiver", text(receiver)), ("line", number(at))];
            self.here("range_cut", &fields);
        }
    }

    fn literal(&mut self, found: String, at: usize) {
        self.here("literal", &[("text", text(found)), ("line", number(at))]);
    }

    fn discard(&mut self, at: usize, kind: &str) {
        self.here("discard", &[("line", number(at)), ("kind", text(kind))]);
    }
}

impl<'ast> Visit<'ast> for Walker<'_> {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        self.function(
            &item.attrs,
            &item.sig,
            line_of(item.block.brace_token.span.close()),
        );
        visit::visit_item_fn(self, item);
        let _done = self.scopes.pop();
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        self.function(
            &item.attrs,
            &item.sig,
            line_of(item.block.brace_token.span.close()),
        );
        if let Some(name) = self.implementing.clone() {
            let at = number(line_of(item.sig.ident.span()));
            self.here("impl_fn", &[("trait", text(name)), ("line", at)]);
        }
        visit::visit_impl_item_fn(self, item);
        let _done = self.scopes.pop();
    }

    fn visit_trait_item_fn(&mut self, item: &'ast syn::TraitItemFn) {
        // A method with no body is left out: as a `function` fact it would
        // make a same-name test fake in the file count as non-test code.
        // Its signature is still read, by `smell_facts`.
        if let Some(block) = &item.default {
            let last = line_of(block.brace_token.span.close());
            self.function(&item.attrs, &item.sig, last);
            visit::visit_trait_item_fn(self, item);
            let _done = self.scopes.pop();
        }
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        let name = item
            .trait_
            .as_ref()
            .and_then(|(path, _)| path.segments.last())
            .map(|s| s.ident.to_string());
        let outer = std::mem::replace(&mut self.implementing, name);
        visit::visit_item_impl(self, item);
        self.implementing = outer;
    }

    fn visit_expr_index(&mut self, index: &'ast syn::ExprIndex) {
        if from_start(&index.index) {
            let at = line_of(index.bracket_token.span.open());
            self.range_cut(&index.expr, at);
        }
        visit::visit_expr_index(self, index);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let test = usize::from(item.attrs.iter().any(is_cfg_test));
        self.test_depth = self.test_depth.saturating_add(test);
        visit::visit_item_mod(self, item);
        self.test_depth = self.test_depth.saturating_sub(test);
    }

    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(callee) = &*call.func {
            let at = callee
                .path
                .segments
                .first()
                .map_or(0, |s| line_of(s.ident.span()));
            let name = expand(&callee.path, &self.aliases);
            if let Some(first) = first_text(call.args.first()) {
                let fields = [
                    ("callee", text(name.clone())),
                    ("text", text(first)),
                    ("line", number(at)),
                ];
                self.here("call_text", &fields);
            }
            self.call(name, at);
        }
        visit::visit_expr_call(self, call);
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        let (name, at) =
            (format!(".{}", call.method), line_of(call.method.span()));
        if let Some(on) = receiver_text(&call.receiver) {
            let fields = [
                ("method", text(name.clone())),
                ("receiver", text(on)),
                ("line", number(at)),
            ];
            self.here("method_receiver", &fields);
        }
        if let Some(first) = first_text(call.args.first()) {
            let fields = [
                ("callee", text(name.clone())),
                ("text", text(first)),
                ("line", number(at)),
            ];
            self.here("call_text", &fields);
        }
        if call.method == "get" && call.args.first().is_some_and(from_start) {
            self.range_cut(&call.receiver, at);
        }
        self.call(name, at);
        visit::visit_expr_method_call(self, call);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if let Some(last) = mac.path.segments.last() {
            let (name, at) =
                (last.ident.to_string(), line_of(last.ident.span()));
            if let Some(first) = macro_text(&mac.tokens) {
                let fields = [
                    ("name", text(name.clone())),
                    ("text", text(first)),
                    ("line", number(at)),
                ];
                self.here("macro_text", &fields);
            }
            self.here("macro", &[("name", text(name)), ("line", number(at))]);
        }
        for (found, at) in token_strings(&mac.tokens) {
            self.literal(found, at);
        }
        visit::visit_macro(self, mac);
    }

    fn visit_lit_str(&mut self, s: &'ast syn::LitStr) {
        self.literal(s.value(), line_of(s.span()));
    }

    // Attributes hold doc comments and settings, not program text.
    fn visit_attribute(&mut self, _attribute: &'ast syn::Attribute) {}

    fn visit_expr_try(&mut self, expr: &'ast syn::ExprTry) {
        let at = number(line_of(expr.question_token.span));
        self.here("uses_try", &[("line", at)]);
        visit::visit_expr_try(self, expr);
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let wild = matches!(local.pat, syn::Pat::Wild(_));
        if wild && local.init.as_ref().is_some_and(|i| is_call(&i.expr)) {
            self.discard(line_of(local.let_token.span), "let _ =");
        }
        visit::visit_local(self, local);
    }

    fn visit_stmt(&mut self, stmt: &'ast syn::Stmt) {
        if let syn::Stmt::Expr(syn::Expr::MethodCall(call), Some(_)) = stmt
            && call.method == "ok"
        {
            self.discard(line_of(call.method.span()), ".ok();");
        }
        visit::visit_stmt(self, stmt);
    }

    fn visit_arm(&mut self, arm: &'ast syn::Arm) {
        if let Some(span) = err_wild(&arm.pat)
            && is_empty(&arm.body)
        {
            self.here("empty_handler", &[("line", number(line_of(span)))]);
        }
        visit::visit_arm(self, arm);
    }
}

/// Adds every fact about one Rust file at `path` to `facts`.
pub fn read(path: &str, source: &str, facts: &mut Facts) {
    facts.add("file", &[("path", text(path)), ("language", text("rust"))]);
    match syn::parse_file(source) {
        Ok(file) => {
            let mut walker = Walker {
                path,
                aliases: aliases(&file),
                scopes: Vec::new(),
                test_depth: 0,
                implementing: None,
                facts,
            };
            walker.visit_file(&file);
            crate::smell_facts::read(path, source, &file, facts);
        }
        Err(e) => facts.add(
            "parse_failure",
            &[
                ("path", text(path)),
                ("line", number(line_of(e.span()))),
                ("reason", text(e.to_string())),
            ],
        ),
    }
}

#[cfg(test)]
#[path = "rust_facts_tests.rs"]
mod tests;
