extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;




//Einstiegspunkt für das Makro
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


//Für jedes Feld die Dumpfunktion aufrufen
fn OutputDumpField(token: &mut quote::Tokens, field: &syn::Field){
    token.append("stream.sendByte(1);");	
    
}

fn OutputDumpRelation(token: &mut quote::Tokens, field: &syn::Field){
    //token.append("asdf");	
    
}


//Funktionsrumpf
fn impl_example(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    //println!("{:?}",ast);
//    println!("{:?}",ast.body);
    let body = &ast.body;
//  
	let VarDat = body;
	
	let mut txt = quote! {};
	
	match body {
		&syn::Body::Struct(ref x) => {
			//println!("\n\nStruct! {:?}\n",x); 
			
				
			txt.append("impl Dumpable ");
			//txt.append(name);
			txt.append("for");
			//txt.append(" for ");
			txt.append(name);
			//txt.append(">");
			txt.append(" {
			fn DumpObj(&self");
			//txt.append(name);
			txt.append(", stream: &outputStream){");

			for y in x.fields()
			{ 
				OutputDumpField(&mut txt, y); 
			}
			
			txt.append("}}")
		},
		&syn::Body::Enum(ref x) => println!("Enum"),
	};
	
    println!("{}",txt);
    txt

}
