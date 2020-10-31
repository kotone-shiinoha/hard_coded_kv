use std::fs;

fn main() {
    swagger_macro::swagger!();

    let s = fs::read_to_string("./swagger.yaml").unwrap();
    let value: serde_yaml::Value = serde_yaml::from_str(s.as_str()).unwrap();
    let paths = value["paths"].as_mapping().unwrap();
    for (key, _) in paths.iter() {
        let string = key.as_str().unwrap().to_string();
        let stack = string.as_str().split("/").collect::<Vec<&str>>();
        
        for method in ["get", "post", "delete", "put"].iter() {
            let v = hard_coded_kv((*method, &stack[..]));
            println!("{}\t{}\t{:?}", method, &string, v);
        }
        
    }
    
}