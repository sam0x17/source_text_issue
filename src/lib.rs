#![cfg(test)]

use syn::spanned::Spanned;

#[test]
fn test_source_text_on_root_file() {
    let parsed = syn::parse_file("struct Something;").unwrap();
    parsed.span().source_text().unwrap();
}

#[test]
fn test_source_text_on_item() {
    let parsed = syn::parse_file("struct Something;").unwrap();
    parsed.items.first().unwrap().span().source_text().unwrap();
}

#[test]
fn test_source_text_on_function() {
    let parsed = syn::parse_file(
        "fn hello_world() {\n\
            println!(\"hello world!\");\n\
        }\n",
    )
    .unwrap();
    parsed.items.first().unwrap().span().source_text().unwrap();
}
