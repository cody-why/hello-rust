/*** 
 * @Author: plucky
 * @Date: 2022-07-11 14:19:47
 * @LastEditTime: 2022-07-14 11:59:31
 * @Description: 
 */

mod model;
mod tests;

#[allow(unused_imports)]
use proc_macro::TokenStream;
use quote::quote;
// use anyhow::{Result};


use syn::{parse_macro_input, DeriveInput};
// use syn::parse::Parse;


/// 这个过程宏帮助struct实现了 `Hello` trait
#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream)->TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;

    // println!("{:?}", name);
    
    // #name 为上面获取的struct name
    let expanded = quote! {
        impl #name {
            pub fn hello() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn hello_attribute_base(args:TokenStream, input: TokenStream)->TokenStream {
    eprintln!("{:#?}", args);
    // eprintln!("------input------");
    // eprintln!("{:#?}", input);

    let literal = args.into_iter().next().unwrap();
    match literal {
        // proc_macro::TokenTree::Group(_) => todo!(),
        // proc_macro::TokenTree::Ident(_) => todo!(),
        // proc_macro::TokenTree::Punct(_) => todo!(),
        proc_macro::TokenTree::Literal(e) => {
            println!("{:?}", e.to_string());// 输出的是symbol的字符串
            println!("{:?}", e); // 输出literal的全部信息
        }
        
       
        _=> println!("Err(anyhow!(unexpected token)") ,
    }   
            
        
    
    input

}

#[proc_macro_attribute]
pub fn hello_attribute(args:TokenStream, input: TokenStream)->TokenStream {
    // println!("{:?}", args);
    // println!("{:?}", input);

    let args2 = args.clone();

    let input= parse_macro_input!(input as syn::ItemFn);
    // 把input转换成一个结构体
    let catch = model::Attribute::parse(args.into(), input).unwrap();

    let user_fn = &catch.function;
    let user_fn_name = &catch.function.sig.ident;
    let vis = &catch.function.vis;
    let status_code = catch.status;
   
    
    let args= parse_macro_input!(args2 as syn::AttributeArgs);
    let mut attr = "".to_string();
    for m in args {
        let t = match m {
            syn::NestedMeta::Meta(syn::Meta::NameValue(ref _a)) => {
               "Meta".into()
            },
            syn::NestedMeta::Lit(a) =>{let a = match a {
                syn::Lit::Str(ref s) => {

                    s.value()+" str"
                },
                syn::Lit::Int(ref i) => {

                    i.base10_digits().to_string()+" int"
                },
                _ => {
                    "Lit".into()
                }
            };
            a
        },
                
            _ => "Other".into(),
           
        };
        attr.push_str(t.as_str());
    }

        
    let debug_str = format!("{:?} {} ( {:?} ) ={:?}",vis, user_fn_name, status_code.unwrap(),attr);

    let expanded = quote! {
        fn some_print() {
            println!("--------from macro----------");
            println!("{:?}", #debug_str);
            println!("------------------");
        }

        fn #user_fn_name() {//函数名
            #user_fn//函数体
            some_print();
            
        }
    };
    expanded.into()
   
    
}


/// 这个属性宏重新定义了一个dummy函数
#[proc_macro_attribute]
pub fn log_entry_and_exit(args: TokenStream, input: TokenStream) -> TokenStream {
    let x = format!(
        r#"
        fn dummy() {{
            println!("entering");
            println!("args tokens: {{:?}}", {args});
            println!("input tokens: {{}}", {input});
            println!("exiting");

        }}
    "#,
        args = args.into_iter().count(),
        input = input.into_iter().count(),
    );

    
    x.parse().expect("Generated invalid tokens")
}

// 这个演示了输出input的类型:
// sql!(select * from user wher id = 1);//Ident="select",Punct="*",Literal=1
#[proc_macro]
pub fn sql(input:TokenStream) ->TokenStream{
    eprintln!("{:#?}", input);
    "fn hello() {
        println!(\"hello\");
    }".parse().expect("Generated invalid tokens")
}

