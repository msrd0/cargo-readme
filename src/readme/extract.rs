//! Extract raw doc comments from rust source code

use std::io::{BufReader, Read};
use syn::{Attribute, Ident, Item, Lit, LitStr, Meta};

/// Read the given `Read`er and return a `Vec` of the rustdoc lines found
pub fn extract_docs<R: Read>(reader: R, crate_name: &str) -> anyhow::Result<Vec<String>> {
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let file = syn::parse_file(&buf)?;

    let mut doc = Vec::new();
    for attr in file.attrs {
        if attr.path.is_ident("doc") {
            if let Some(str) = parse_doc_attr(&attr)? {
                doc.push(str.value());
            }
        }
    }

    doc.push(String::new());
    doc.push("<!-- auto-detected links -->".to_owned());
    for item in file.items {
        if let Some((link_file, ident)) = link_file_ident(item) {
            let link = format!(
                "https://docs.rs/{}/*/{}/{}",
                crate_name,
                crate_name.replace('-', "_"),
                link_file
            );
            doc.push(format!(" [{}]: {}", ident, link));
            doc.push(format!(" [`{}`]: {}", ident, link));
        }
    }

    Ok(doc)
}

fn parse_doc_attr(input: &Attribute) -> syn::Result<Option<LitStr>> {
    input.parse_meta().and_then(|meta| {
        Ok(match meta {
            Meta::NameValue(kv) => Some(match kv.lit {
                Lit::Str(str) => str,
                lit => return Err(syn::Error::new(lit.span(), "Expected string literal")),
            }),
            _ => None,
        })
    })
}

/// Return the file name of the docs.rs link for this item
fn link_file_ident(item: Item) -> Option<(String, Ident)> {
    Some(match item {
        Item::Const(i) => (format!("constant.{}.html", i.ident), i.ident),
        Item::Enum(i) => (format!("enum.{}.html", i.ident), i.ident),
        Item::Fn(i) => (format!("fn.{}.html", i.sig.ident), i.sig.ident),
        Item::Macro(i) => {
            return i
                .ident
                .map(|ident| (format!("macro.{}.html", ident), ident))
        }
        Item::Macro2(i) => (format!("macro.{}.html", i.ident), i.ident),
        Item::Mod(i) => (format!("{}/index.html", i.ident), i.ident),
        Item::Struct(i) => (format!("struct.{}.html", i.ident), i.ident),
        Item::Trait(i) => (format!("trait.{}.html", i.ident), i.ident),
        Item::Type(i) => (format!("type.{}.html", i.ident), i.ident),
        _ => return None,
    })
}
