///
/// proc_macro是rust自带的crate，是可以操作rust代码的api
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // quote!宏提供了参数替换机制，比如#name会用name变量的值替换
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify!宏将接收到的表达式转换为string literal。即&'static str
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// attribute-like宏，attr对应第一部分，也就是GET,"/"，item对应第二部分，也就是fn index() {}
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    todo!()
}

// function-like宏
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    todo!()
}
