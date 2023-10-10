use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser as _,};

type AttributeArgs = syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>;

#[proc_macro_attribute]
pub fn human(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = match syn::parse::<syn::ItemFn>(item.clone()) {
        Ok(input) => input,
        // on parse err, make IDEs happy; see fn docs
        Err(err) => return input_and_compile_error(item, err),
    };

    let parser = AttributeArgs::parse_terminated;
    let args = match parser.parse(args.clone()) {
        Ok(args) => args,
        Err(err) => return input_and_compile_error(args, err),
    };

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &mut input.sig;
    let body = &input.block;
    let mut has_test_attr = false;

    for attr in attrs {
        if attr.path().is_ident("test") {
            has_test_attr = true;
        }
    }

    // if let St

    // println!("the content of the body |||| \n\n {:#?} \n\n", body.stmts[0]);


    // if sig.asyncness.is_none() {
    //     return syn::Error::new_spanned(
    //         input.sig.fn_token,
    //         "the async keyword is missing from the function declaration",
    //     )
    //     .to_compile_error()
    //     .into();
    // }

    // sig.asyncness = None;


    let missing_test_attr = if has_test_attr {
        quote! {}
    } else {
        quote! { #[::core::prelude::v1::test] }
    };

    let mut system = syn::parse_str::<syn::Path>("::actix_rt::System").unwrap();


    // let inner_fn =  body.clone().stmts.iter_mut().map(|s| s).collect::<Vec<_>>();
    let mut modified_body = body.clone();

    let mut stmt = &mut modified_body.stmts[0];

    // let body_in_localset = quote! {
    //     <#system>::new().block_on(async { #stmt })
    // };

    if let syn::Stmt::Item(item) = &mut stmt {
        // if let syn::ItemFn(data) = item {}
        // println!("stmt \n\n {item:#?}");
        if let syn::Item::Fn( data ) = item {
            let block = data.block.clone();
            let ts = quote! {
                {
                    <#system>::new().block_on(async { #block })
                }
            };

            data.block = Box::new(syn::parse2(ts).unwrap());

        }
    }


    // let new_stmt = quote! {
    //     <#system>::new().block_on(async { #stmt })
    // };

    // let vvvv = proc_macro::TokenStream::from(new_stmt);
    // let abc = parse_macro_input!(vvvv as syn::Stmt);

    // modified_body.stmts[0] = abc;




    // let mut target = None::<usize>;


    // modified_body.stmts.iter().enumerate().for_each(|(index, stmt)| {
    //     if let syn::Stmt::Item(item)= stmt {
    //         if let syn::Item::Fn(item_fn) = item {
    //             let name = item_fn.sig.ident.to_string();
    //             // return name == "inner".to_string();
    //             if name == "inner".to_string() {
    //                 println!("|||||||||||||||||||||||||||||-----|||||||||||||||||||||||||||||-----|||||||||||||||||||||||||||||-----|||||||||||||||||||||||||||||-----");
    //                 target = Some(index);
    //             }
    //         }
    //     }
    //     // false
    // });

    // if let Some(index) = target {
    //     let stmt = &modified_body.stmts[index];

    //     let body_in_localset = quote! {
    //         <#system>::new().block_on(async { #stmt })
    //     };

    //     let new_stmt = syn::Stmt::Item(syn::Item::Verbatim(body_in_localset));

    //     modified_body.stmts[index] = new_stmt;
    // }



    for arg in &args {
        match arg {
            syn::Meta::NameValue(syn::MetaNameValue {
                path,
                value:
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(lit),
                        ..
                    }),
                ..
            }) => match path
                .get_ident()
                .map(|i| i.to_string().to_lowercase())
                .as_deref()
            {
                Some("system") => match lit.parse() {
                    Ok(path) => system = path,
                    Err(_) => {
                        return syn::Error::new_spanned(lit, "Expected path")
                            .to_compile_error()
                            .into();
                    }
                },
                _ => {
                    return syn::Error::new_spanned(arg, "Unknown attribute specified")
                        .to_compile_error()
                        .into();
                }
            },
            _ => {
                return syn::Error::new_spanned(arg, "Unknown attribute specified")
                    .to_compile_error()
                    .into();
            }
        }
    }

    // <#system>::new().block_on(async { #body })
    (quote! {
        // #missing_test_attr
        #(#attrs)*
        #vis #sig {
            #modified_body
        }
    })
    .into()
}

/// Converts the error to a token stream and appends it to the original input.
///
/// Returning the original input in addition to the error is good for IDEs which can gracefully
/// recover and show more precise errors within the macro body.
///
/// See <https://github.com/rust-analyzer/rust-analyzer/issues/10468> for more info.
fn input_and_compile_error(mut item: TokenStream, err: syn::Error) -> TokenStream {
    let compile_err = TokenStream::from(err.to_compile_error());
    item.extend(compile_err);
    item
}
