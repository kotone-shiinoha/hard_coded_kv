pub use proc_macro2;
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use std::str::FromStr;
/// this struct will be expanded into 
/// ```rust
/// // when the guard is Some
/// key if guard => {
///     value
/// }
/// // when the guard is None
/// key => {
///    value
/// }
/// ```
pub struct Kv {
    pub key: TokenStream,
    pub guard: Option<TokenStream>,
    pub value: TokenStream,
}

impl Kv {
    pub fn new(key: TokenStream, guard: Option<TokenStream>, value: TokenStream) -> Self {
        Self {
            key, guard, value
        }
    }
}

/// this struct will be expanded into 
/// ```rust
/// match key {
///     // when the kv's guard is Some
///     key if guard => {
///         value
///     }
///     // when the guard is None
///     key => {
///         value
///     }
///     _ => {
///         value
///     }
///}
/// ```
pub struct Source {
    kv: Vec<Kv>,
    default: Option<TokenStream>,
    func_name: String,
    returning_value: TokenStream,
    matching_value: TokenStream,
}

impl Source {
    pub fn new() -> Self {
        Self {
            kv: Vec::with_capacity(1024),
            default: Default::default(),
            func_name: "hard_coded_kv".to_string(),
            returning_value: TokenStream::from_str("()").unwrap(),
            matching_value: TokenStream::from_str("()").unwrap(),
        }
    }

    pub fn replace_default(&mut self) -> &mut Option<TokenStream> {
        &mut self.default
    }

    pub fn kv(&mut self) -> &mut Vec<Kv> {
        &mut self.kv
    }

    pub fn func_name(&mut self) -> &mut String {
        &mut self.func_name
    }

    pub fn returning_value(&mut self) -> &mut TokenStream {
        &mut self.returning_value
    }

    pub fn matching_value(&mut self) -> &mut TokenStream {
        &mut self.matching_value
    }

    pub fn into_code(self) -> TokenStream {
        let func = |stream: TokenStream| {
            let group = Group::new(Delimiter::Brace, stream);
            TokenStream::from(TokenTree::Group(group))
        };

        let mut stream = {
            let mut stream = TokenStream::new();
            let arrow: TokenStream = "=>".parse().unwrap();

            for i in self.kv {
                let pair = {
                    stream.extend(i.key.into_iter());

                    if let Some(i) = i.guard {
                        let if_block = || TokenTree::Ident(Ident::new("if", Span::call_site()));
                        stream.extend(vec![if_block()]);
                        stream.extend(i.into_iter());
                    }

                    stream.extend(arrow.clone().into_iter());

                    func(i.value)
                };

                stream.extend(pair);
            }

            stream
        };

        // add default value
        let group = {
            let code: TokenStream = "_ => ".parse().unwrap();
            stream.extend(code);
            stream.extend(self.default.into_iter());
            func(stream)
        };

        let mut keyword: TokenStream = "match path ".parse().unwrap();
        keyword.extend(group.into_iter());
        let r = {
            let func = format!{
                "fn {}(path: {}) -> {}", 
                self.func_name, 
                self.matching_value, 
                self.returning_value
            };

            let mut s = TokenStream::from_str(func.as_str()).unwrap();
            let group = TokenTree::Group(Group::new(Delimiter::Brace, keyword));
            s.extend(TokenStream::from(group));
            s
        };

        r
    }
}


pub fn from_json(s: ) {

}