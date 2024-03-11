use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn test(x: TokenStream, _: TokenStream) -> TokenStream {
    // not important for the bootstrap issue
    x
}

#[proc_macro_attribute]
pub fn bench(x: TokenStream, _: TokenStream) -> TokenStream {
    // not important for the bootstrap issue
    x
}
