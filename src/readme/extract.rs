//! Extract raw doc comments from rust source code

use std::io::{BufReader, Read};
use syn::{Attribute, Lit, LitStr, Meta};

/// Read the given `Read`er and return a `Vec` of the rustdoc lines found
pub fn extract_docs<R: Read>(reader: R) -> anyhow::Result<Vec<String>> {
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
