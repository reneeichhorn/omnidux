pub use cssparser::{
  RuleListParser,
  ParserInput,
  Parser,
  Token,
  QualifiedRuleParser,
  BasicParseError,
  ParseError,
  AtRuleParser,
  CowRcStr,
  SourceLocation,
  DeclarationListParser,
  DeclarationParser,
};
use proc_macro2::{TokenStream, TokenTree, Literal};
use quote::{quote, format_ident, ToTokens};

pub struct StyleParser {}
pub struct CustomParser {}

#[derive(Debug)]
pub struct StyleDeclaration<'i> {
  pub name: String,
  pub token: Token<'i>,
}

#[derive(Debug)]
pub struct Rule<'i> {
  pub key: String,
  pub decls: Vec<StyleDeclaration<'i>>,
}

/// Some type information for our parser.
impl<'i> AtRuleParser<'i> for CustomParser {
  type PreludeBlock = ();
  type PreludeNoBlock = ();
  type AtRule = Rule<'i>;
  type Error = BasicParseError<'i>;
}

impl<'i> QualifiedRuleParser<'i> for CustomParser {
  type Prelude = String;
  type QualifiedRule = Rule<'i>;
  type Error = BasicParseError<'i>;

  /// Parses out the selector.
  fn parse_prelude<'t>(
      &mut self,
      input: &mut Parser<'i, 't>,
  ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
      let location = input.current_source_location();

      let selector = match input.next()? {
        Token::Ident(ref element_name) => element_name.to_string(),
        t => { return Err(location.new_unexpected_token_error(t.clone())); }
      };

      Ok(selector)
  }

  /// Parses the block (`{...}`) into a Rule struct.
  fn parse_block<'t>(
    &mut self,
    key: Self::Prelude,
    _location: SourceLocation,
    input: &mut Parser<'i, 't>,
  ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
    let styles = DeclarationListParser::new(input, StyleParser {}).collect::<Vec<_>>();

    Ok(Rule {
      key: key,
      decls: styles.into_iter().filter_map(|decl| {
        if !decl.is_ok() {
          eprintln!("{:?}", decl);
        }

        decl.ok()
      }).collect()
    })
  }
}

impl<'i> AtRuleParser<'i> for StyleParser {
  type PreludeBlock = ();
  type PreludeNoBlock = ();
  type AtRule = StyleDeclaration<'i>;
  type Error = BasicParseError<'i>;
}

impl<'i> DeclarationParser<'i> for StyleParser {
    type Declaration = StyleDeclaration<'i>;
    type Error = BasicParseError<'i>;

    /// Parses a value (e.g, `background-color: #307ace;`) into a `Styles` value.
    fn parse_value<'t>(
      &mut self,
      name: CowRcStr<'i>,
      input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
      Ok (StyleDeclaration { name: name.to_string(), token: input.next().unwrap().clone() })
    }
}

pub fn map_style_declaration<'i> (decl: &StyleDeclaration<'i>) -> TokenStream {
  fn map_dimension<'a>(token: Token<'a>) -> TokenStream {
    match token {
      Token::Number { value, .. } => TokenStream::from(TokenTree::Literal(Literal::f32_suffixed(value))),
      _ => panic!("Invalid style value")
    }
  }
  fn map_number<'a>(token: Token<'a>) -> TokenStream {
    match token {
      Token::Number { value, .. } => TokenStream::from(TokenTree::Literal(Literal::f32_suffixed(value))),
      _ => panic!("Invalid style value")
    }
  }

  let (name_str, value_appendix) = match (decl.name.as_ref()) {
    "flex-basis" => ("flex_basis", map_dimension(decl.token.clone())),
    "flex-grow" => ("flex_grow", map_number(decl.token.clone())),
    "flex-shrink" => ("flex_shrink", map_number(decl.token.clone())),
    _ => panic!("Unknown styling property")
  };

  let name = format_ident!("{}", name_str);

  quote! { #name: #value_appendix, }
}