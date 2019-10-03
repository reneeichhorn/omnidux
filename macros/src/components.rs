use std::iter::Peekable;
use proc_macro::token_stream::IntoIter;
use proc_macro::{TokenStream, TokenTree};

type InputStream = Peekable<IntoIter>;

#[derive(Debug)]
pub struct Options {
  name: String,
}

#[derive(Debug)]
pub struct Property {
  name: String,
  type_name: String,
  assignment: Option<TokenStream>,
}

#[derive(Debug)]
pub struct State {
  name: String,
  type_name: String,
  assignment: Option<TokenStream>,
}

#[derive(Debug)]
pub struct Binding {
  name: String,
  type_name: String,
  key: Option<TokenStream>,
}

pub struct RenderContent {
  pre_calculation: TokenStream,
  render_tree: TokenStream,
}

#[derive(Debug)]
enum GrammarVariable {
  String(String),
  TokenStream(TokenStream),
}

impl GrammarVariable {
  pub fn to_string(&self) -> String {
    if let GrammarVariable::String(stri) = self {
      return stri.clone();
    }
    panic!("Variable is not a string.")
  }
  pub fn to_stream(&self) -> TokenStream {
    if let GrammarVariable::TokenStream(stream) = self {
      return stream.clone();
    }
    panic!("Variable is not a stream.")
  }
}

macro_rules! grammar {
  // Base definition.
  ($name:ident, $grammar:tt) => {
    fn $name (iter: &mut InputStream) -> Result<std::collections::HashMap<String, GrammarVariable>, String> {
      let mut out = std::collections::HashMap::new();
      grammar! (#root, iter, out, $grammar);
      Ok(out)
    }
  };

  // Empty layer.
  (#empty, $inp:tt) => {};

  // Root layer.
  (#root, $iter:ident, $out:ident, { $($name:ident $opts:tt),* }) => {
    $(
      grammar! (#node, $iter, $out, $name, $opts);
    )*
  };

  // Advancing layer.
  (#advance, $iter:ident, { $($name:ident $opts:tt),* }) => {
    $(
      grammar!(#empty, ($name));
      $iter.next().unwrap();
    )*
  };

  // Ident unwrap rule.
  (#node, $iter:ident, $out:ident, Ident, ( #$val:ident )) => {
    let token = $iter.next();
    if token.is_none() {
      return Err("EOF".to_string());
    }

    match token.unwrap() {
      TokenTree::Ident(ident) => {
        let val = ident.to_string();
        $out.insert(stringify!($val).to_string(), GrammarVariable::String(val));
      },
      _ => {
        return Err("Expected variable identifier.".to_string());
      },
    };
  };

  // Ident rule.
  (#node, $iter:ident, $out:ident, Ident, ( $val:expr )) => {
    let token = $iter.next();
    if token.is_none() {
      return Err("EOF".to_string());
    }

    match token.unwrap() {
      TokenTree::Ident(ident) => {
        let val = ident.to_string();
        let matching = $val.to_string();
        if val != matching {
          return Err("Expected named identifier but found other identifier.".to_string());
        }
      },
      _ => {
        return Err("Expected named identifier.".to_string());
      },
    };
  };

  // Punct rule.
  (#node, $iter:ident, $out:ident, Punct, ( $val:expr )) => {
    let token = $iter.next();
    if token.is_none() {
      return Err("EOF".to_string());
    }

    match token.clone().unwrap() {
      TokenTree::Punct(punct) => {
        let val = punct.to_string();
        let matching = $val.to_string();
        if val != matching {
          return Err("Expected named punct but found other punct.".to_string());
        }
      },
      _ => {
        return Err(format!("Expected named punct. {:?} {:?}", stringify!($val), token).to_string());
      },
    };
  };

  // Expr rule.
  (#node, $iter:ident, $out:ident, Expr, ( #$val:ident)) => {
    let tokens: Vec<TokenTree> = $iter
      .clone()
      .take_while(|token| {
        if let TokenTree::Punct(punct) = token {
          if punct.to_string() == ";" {
            return false;
          }
        }
        return true;
      })
      .collect();

    let mut token_stream = TokenStream::new();
    for _ in 0..tokens.len() {
      $iter.next();
    }

    token_stream.extend(tokens);

    $out.insert(stringify!($val).to_string(), GrammarVariable::TokenStream(token_stream));
  };

  // Optional rule.
  (#node, $iter:ident, $out:ident, Optional, ($($opt:tt)*)) => {
    fn try_optional(in_iter: &mut InputStream) -> Result<std::collections::HashMap<String, GrammarVariable>, String> {
      let mut temp_iter = in_iter.clone();
      let mut out_opt = std::collections::HashMap::new();
      grammar! (#root, temp_iter, out_opt, { $($opt)* });
      grammar! (#advance, in_iter, { $($opt)* });
      Ok(out_opt)
    }

    if let Ok(hashmap) = try_optional($iter) {
      $out.extend(hashmap);
    }
  };
}

grammar! (parse_name, {
  Ident("name"), Punct("="), Ident(#name), Punct(";")
});

grammar! (parse_property, {
  Punct("@"), Ident("prop"),
    Ident("let"), Ident(#name),
    Punct(":"), Ident(#type), 
    Optional(
      Punct("="), Expr(#value)
    ),
  Punct(";")
});

grammar! (parse_state , {
  Punct("@"), Ident("state"),
    Ident("let"), Ident("mut"), Ident(#name),
    Punct(":"), Ident(#type), 
    Punct("="), Expr(#value),
  Punct(";")
});

grammar! (parse_binding, {
  Punct("@"), Ident("bind"),
    Ident("let"), Ident(#name),
    Punct(":"), Ident(#type), 
    Punct("="), Expr(#key),
  Punct(";")
});

pub fn take_options(iter: &mut InputStream) -> Options {
  let output = parse_name(iter).unwrap();
  Options {
    name: output.get("name").unwrap().to_string(),
  }
}

pub fn take_properties(iter: &mut InputStream) -> Vec<Property> {
  let mut out = Vec::new();
  let mut cur_iter = iter.clone();

  while let Ok(prop) = parse_property(&mut cur_iter) {
    // Parse output.
    let mut assiginment = None;
    if let Some(asgn) = prop.get("val") {
      assiginment = Some(asgn.to_stream());
    }

    // Insert property.
    out.push(Property {
      name: prop.get("name").unwrap().to_string(),
      type_name: prop.get("type").unwrap().to_string(),
      assignment: assiginment,
    });
    
    // Advance.
    let _ = parse_property(iter);
  }

  out
}

pub fn take_state(iter: &mut InputStream) -> Vec<State> {
  let mut out = Vec::new();
  let mut cur_iter = iter.clone();

  while let Ok(prop) = parse_state(&mut cur_iter) {
    // Parse output.
    let mut assiginment = None;
    if let Some(asgn) = prop.get("val") {
      assiginment = Some(asgn.to_stream());
    }

    // Insert property.
    out.push(State {
      name: prop.get("name").unwrap().to_string(),
      type_name: prop.get("type").unwrap().to_string(),
      assignment: assiginment,
    });
    
    // Advance.
    let _ = parse_state(iter);
  }

  out
}

pub fn take_binds(iter: &mut InputStream) -> Vec<Binding> {
  let mut out = Vec::new();
  let mut cur_iter = iter.clone();

  while let Ok(prop) = parse_binding(&mut cur_iter) {
    // Parse output.
    let mut key = None;
    if let Some(asgn) = prop.get("key") {
      key = Some(asgn.to_stream());
    }

    // Insert property.
    out.push(Binding {
      name: prop.get("name").unwrap().to_string(),
      type_name: prop.get("type").unwrap().to_string(),
      key: key,
    });
    
    // Advance.
    let _ = parse_binding(iter);
  }

  out
}

pub fn take_render_content(iter: &mut InputStream) {
  let mut pre_calculation = Vec::new();
  let mut after_calculation = Vec::new();

  while let Some(token) = iter.next() {
    // Is last?
    if iter.peek().is_none() {
      match token {
        TokenTree::Group(group) => {
          let root_name  = pre_calculation.pop().unwrap();

          after_calculation.push(root_name);
          after_calculation.push(TokenTree::Group(group));
        },
        _ => panic!("Last token must be a component block"),
      }
      break;
    }

    // 
    pre_calculation.push(token);
  }
}