use proc_macro2::{Ident, Span, TokenStream};

pub fn render_object_field(field_name: &str, field_type: TokenStream) -> TokenStream {
    if field_name == "type" {
        let name_ident = Ident::new(&format!("{}_", field_name), Span::call_site());
        return quote! {
            #[serde(rename = #field_name)]
            pub #name_ident: #field_type
        };
    }

    let name_ident = Ident::new(field_name, Span::call_site());

    quote!(pub #name_ident: #field_type)
}
