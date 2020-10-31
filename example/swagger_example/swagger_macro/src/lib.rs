use hard_coded_kv::{
    proc_macro2::{
        TokenStream,
        TokenTree,
        Group,
        Delimiter
    },
    Kv,
    Source
};
use std::fs;
use std::str::FromStr;

#[proc_macro]
pub fn swagger(_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let s = fs::read_to_string("./swagger.yaml").unwrap();
    let value: serde_yaml::Value = serde_yaml::from_str(s.as_str()).unwrap();
    let paths = value["paths"].as_mapping().unwrap();

    let mut source = Source::new();
    *source.replace_default() = TokenStream::from_str(
        "None".to_string().as_str()
    ).unwrap().into();

    *source.matching_value() = TokenStream::from_str("(&'static str, &[&str])").unwrap();
    *source.returning_value() = TokenStream::from_str("Option<&'static str>").unwrap();

    for (k, v) in paths.iter() {
        let slice = {
            let keys = k.as_str()
                .unwrap()
                .split("/")
                .map(|i| {
                    let first = i.chars().next();
                    let last = i.chars().next_back();
                    let check = Some(('{', '}')) == first.zip(last);
                    if check {
                        "_".to_string()
                    } else {
                        format!("{:?}", i.to_string())
                    }
                })
                .collect::<Vec<String>>();

            format!("[{}]", keys.join(","))
        };

        for (method, item) in v.as_mapping().unwrap().iter() {
            let summary = {
                format!("Some({:?})", item["summary"].as_str().unwrap())
            };
            let value = TokenStream::from_str(summary.as_str()).unwrap();
            
            let tuple = {
                let tuple = format!("{:?},{}", method.as_str().unwrap(), slice);
                TokenStream::from_str(tuple.as_str()).unwrap()
            };
            let key = TokenTree::Group(Group::new(Delimiter::Parenthesis, tuple));
            let kv = Kv::new(key.into(), None, value);

            source.kv().push(kv);
        }
    }

    
    TokenStream::from_str(source.into_code().to_string().as_str()).unwrap().into()
}