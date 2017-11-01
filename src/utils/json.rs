
use serde_json;
use serde_json::Value;

pub fn json_parse() {
    let data = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;
    let v: Value = serde_json::from_str(data).unwrap();
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
}

pub fn parse(json_str : &str)  -> Value{
    let v: Value = serde_json::from_str(json_str).unwrap();
    println!("Please call {}", v);
    return v;
}