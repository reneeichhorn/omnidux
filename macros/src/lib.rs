extern crate cssparser;
extern crate quote;
extern crate proc_macro;

mod style;

use proc_macro_hack::proc_macro_hack;
use proc_macro::{TokenStream, Group};
use quote::{quote, format_ident};
use style::{*};

#[proc_macro_hack]
pub fn component(item: TokenStream) -> TokenStream {
  let mut iter = item.into_iter();
  println!("Token: {:#?}", iter.next());
  TokenStream::new()
}

#[proc_macro_hack]
pub fn stylesheet(item: TokenStream) -> TokenStream {
  let s = item.to_string().replace(" ", "");
  let mut input = ParserInput::new(&s);
  let mut parser = Parser::new(&mut input);
      
  let parsed: Vec<Rule> = RuleListParser::new_for_stylesheet(&mut parser, CustomParser {})
    .collect::<Vec<_>>()
    .into_iter()
    .filter_map(|rule| rule.ok())
    .collect();

  let names = parsed
    .iter()
    .map(|rule| format_ident!("{}", rule.key.clone()))
    .collect::<Vec<_>>();

  let instantiate = parsed
    .iter()
    .map(|rule| {
      let ident = format_ident!("{}", rule.key.clone());
      let mapped = rule.decls
        .iter()
        .map(|decl| map_style_declaration(decl))
        .collect::<Vec<_>>();

      quote! {
        #ident: Styles {
          #(#mapped)*
          ..Default::default()
        },
      }
    })
    .collect::<Vec<_>>();

  let out = TokenStream::from(quote! {
    {
      use omnidux_sys_shadow_renderer::style::Styles;

      #[derive(Debug)]
      struct MyStylesheet {
        #(
          pub #names: Styles,
        )*
      }

      impl MyStylesheet {
        pub fn new () -> MyStylesheet {
          MyStylesheet {
            #(
              #instantiate
            )*
          }
        }
      }

      MyStylesheet::new()
    }
  });

  out
}