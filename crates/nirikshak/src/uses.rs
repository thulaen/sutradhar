//! The names a file brings in with `use`, so `mpsc::channel` reads as
//! `std::sync::mpsc::channel`. Glob imports add nothing.

use std::collections::HashMap;

use syn::visit::Visit;

/// Short name to full path, for one file.
pub type Aliases = HashMap<String, String>;

fn join(prefix: &[String], last: &str) -> String {
    let mut parts = prefix.to_vec();
    parts.push(last.to_owned());
    parts.join("::")
}

fn add(tree: &syn::UseTree, prefix: &mut Vec<String>, aliases: &mut Aliases) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            add(&path.tree, prefix, aliases);
            let _last = prefix.pop();
        }
        syn::UseTree::Name(name) if name.ident == "self" => {
            if let Some(last) = prefix.last() {
                let _old = aliases.insert(last.clone(), prefix.join("::"));
            }
        }
        syn::UseTree::Name(name) => {
            let short = name.ident.to_string();
            let _old = aliases.insert(short.clone(), join(prefix, &short));
        }
        syn::UseTree::Rename(rename) => {
            let full = join(prefix, &rename.ident.to_string());
            let _old = aliases.insert(rename.rename.to_string(), full);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                add(item, prefix, aliases);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}

struct Collector {
    aliases: Aliases,
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        add(&item.tree, &mut Vec::new(), &mut self.aliases);
    }
}

/// Every `use` in the file, wherever it appears.
#[must_use]
pub fn aliases(file: &syn::File) -> Aliases {
    let mut collector = Collector {
        aliases: Aliases::new(),
    };
    collector.visit_file(file);
    collector.aliases
}

/// A path as written, with its first part expanded through the imports.
#[must_use]
pub fn expand(path: &syn::Path, aliases: &Aliases) -> String {
    let parts: Vec<String> =
        path.segments.iter().map(|s| s.ident.to_string()).collect();
    let expanded = match parts.split_first() {
        Some((first, rest)) if path.leading_colon.is_none() => {
            aliases.get(first).map(|full| {
                std::iter::once(full.clone())
                    .chain(rest.iter().cloned())
                    .collect::<Vec<_>>()
            })
        }
        _ => None,
    };
    expanded.unwrap_or(parts).join("::")
}
