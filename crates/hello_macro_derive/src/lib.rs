///
/// proc_macro是rust自带的crate，是可以操作rust代码的api
///
/// proc_macro 是 Rust 编译器自带的、仅在过程宏 crate (proc-macro = true 的 Cargo.toml) 中可用的标准库
/// [proc_macro::TokenStream] 是一个不带 Span 信息的 Token Stream。
/// 这意味着一旦你从这个 TokenStream 中读取了 Token，你就失去了它们在源代码中的原始位置信息。这使得错误报告和调试变得困难。
///
/// proc_macro2 是一个独立的第三方 crate，可以在任何 Rust crate 中使用（包括普通的库、二进制文件以及过程宏 crate）。
/// [proc_macro2::TokenStream] 是一个 带有 Span 信息 的 Token Stream。
/// 通常与其他过程宏辅助库一起使用，如 syn (解析 Rust 语法) 和 quote (生成 Rust 代码)。
/// 在过程宏 crate 中，proc_macro::TokenStream 输入会被转换为 proc_macro2::TokenStream 进行处理，处理完后再转换回 proc_macro::TokenStream 返回给编译器。
///
/// 协同工作示意图：
//                   proc_macro::TokenStream (from compiler)
//                                  |
//                                  v
//           proc_macro2::TokenStream (with spans) <-------.
//                                  |                       |
//                                  v                       |
//                              syn::AST                    |  (quote! macro generates this)
//                                  |                       |
//                              (Your Macro Logic)          |
//                                  |                       |
//                                  v                       |
//            proc_macro2::TokenStream (generated code) -----/
//                                  |
//                                  v
//                   proc_macro::TokenStream (to compiler)
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
