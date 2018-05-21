extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


#[proc_macro_derive(example)]
pub fn example_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_example(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn getContentFN(token: &mut quote::Tokens, field: &syn::Field){
    token.append("asdf");	
}



fn impl_example(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
//    println!("{:?}",ast);
    println!("{:?}",ast.body);
    let body = &ast.body;
//    let VarDat = body;

    let mut txt = quote! {
        impl exampleTrait for #name {
            fn hello_world(&self) {
                println!("Hello, World! My name is {} and i'm {:?}", stringify!(#name), self);
            }
        }
    };

    match body {
	&syn::Body::Struct(ref x) => {println!("Struct! {:?}",x); for y in x.fields(){ getContentFN(&mut txt, y); }},
	&syn::Body::Enum(ref x) => println!("Enum"),
    };
//    println!("{:?}",VarDat);
//    getContentFN(txt, );
    txt

}
