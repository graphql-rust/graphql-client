use failure;
use syn;

/// Extract an configuration parameter specified in the `graphql` attribute.
pub fn extract_attr(ast: &syn::DeriveInput, attr: &str) -> Result<String, failure::Error> {
    let attributes = &ast.attrs;
    let attribute = attributes
        .iter()
        .find(|attr| {
            let path = &attr.path;
            quote!(#path).to_string() == "graphql"
        }).ok_or_else(|| format_err!("The graphql attribute is missing"))?;
    if let syn::Meta::List(items) = &attribute
        .interpret_meta()
        .expect("Attribute is well formatted")
    {
        for item in items.nested.iter() {
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = item {
                let syn::MetaNameValue { ident, lit, .. } = name_value;
                if ident == attr {
                    if let syn::Lit::Str(lit) = lit {
                        return Ok(lit.value());
                    }
                }
            }
        }
    }

    Err(format_err!("attribute not found"))?
}
