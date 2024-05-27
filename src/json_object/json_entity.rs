use std::{collections::HashMap, str::Chars};

#[derive(Debug)]
pub enum JsonEntity {
    Number(f32),
    Text(String),
    Boolean(bool),
    Array(Vec<JsonEntity>),
    Object(HashMap<String, JsonEntity>),
}

pub struct JsonTokenValue {
    value_type: JsonTokenType,
    value: String,
}

pub enum JsonTokenType {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    String,
    Number,
    Comma,
    Colon,
    True,
    False,
    Null,
}

impl JsonEntity {
    pub fn get_value_as_string(&self) -> String {
        match self {
            JsonEntity::Text(s) => quote(&s),
            JsonEntity::Number(n) => n.to_string(),
            JsonEntity::Boolean(b) => b.to_string(),
            JsonEntity::Array(arr) => build_from_array(&arr),
            JsonEntity::Object(obj) => build_from_obj(&obj),
        }
    }
    pub fn flush_to_string(self) -> String {
        self.get_value_as_string()
    }

    pub fn from_chars(chars: &mut Chars) -> JsonEntity {
        JsonEntity::Number(1.1)
    }

    fn build_token_vec(chars: &mut Chars) -> Vec<JsonTokenValue> {
        let mut vec = Vec::new();
        while let Some(c) = chars.next() {
            if c == '{' {
                vec.push(JsonTokenValue {
                    value_type: JsonTokenType::BraceOpen,
                    value: c.to_string(),
                })
            } else if c == '}' {
                vec.push(JsonTokenValue {
                    value_type: JsonTokenType::BraceClose,
                    value: c.to_string(),
                })
            } else if c == '[' {
                vec.push(JsonTokenValue {
                    value_type: JsonTokenType::BracketOpen,
                    value: c.to_string(),
                })
            } else if c == ']' {
                vec.push(JsonTokenValue {
                    value_type: JsonTokenType::BracketClose,
                    value: c.to_string(),
                })
            } else if c == ':' {
                vec.push(JsonTokenValue {
                    value_type: JsonTokenType::Colon,
                    value: c.to_string(),
                })
            } else if c == ',' {
                vec.push(JsonTokenValue {
                    value_type: JsonTokenType::Comma,
                    value: c.to_string(),
                })
            } else if c == 'f' {
                check_if_boolean(chars, &mut vec);
            } else if c == '"' {
                let mut value = String::new();
                while let Some(c_n) = chars.next() {
                    if (c_n == '"') {
                        break;
                    } else {
                        value.push(c_n)
                    }
                }
            }
        }
        vec
    }
}

fn check_if_boolean(chars: &mut Chars, vec: &mut Vec<JsonTokenValue>) {
    let values = chars.clone().take(4);
    if String::from_iter(values) == String::from("alse") {
        vec.push(JsonTokenValue {
            value_type: JsonTokenType::False,
            value: String::from("false"),
        })
    } else {
        panic!("Unnown Value, when interpreting \"False\" token")
    }
}
fn build_from_obj(obj: &HashMap<String, JsonEntity>) -> String {
    let mut st = String::from("{");

    for (key, val) in obj.iter() {
        st.push_str(format!("{}:{},", quote(&key), val.get_value_as_string()).as_str());
    }
    st.pop();
    st.push_str("}");
    st
}

fn build_from_array(arr: &Vec<JsonEntity>) -> String {
    let mut st = String::from("[");
    for i in arr {
        st.push_str((i.get_value_as_string() + ",").as_str());
    }
    st.pop();
    st.push_str("]");
    st
}

fn quote(str: &String) -> String {
    format!("\"{str}\"")
}

// TEST
//-----------------------------------------------------------------------------------------------

#[test]
fn jsonentity_boolean_get_value_as_string_test() {
    let x = JsonEntity::Boolean(false);
    assert_eq!(x.get_value_as_string(), String::from("false"))
}

#[test]
fn jsonentity_text_get_value_as_string_test() {
    let x = JsonEntity::Text(String::from("test"));
    assert_eq!(x.get_value_as_string(), String::from("\"test\""));
    println!("{:?}", x);
}

#[test]
fn jsonentity_intnumber_get_value_as_string_test() {
    let x = JsonEntity::Number(10.0);
    assert_eq!(x.get_value_as_string(), String::from("10"));
}

#[test]
fn jsonentity_floatnumber_get_value_as_string_test() {
    let x = JsonEntity::Number(10.1);
    assert_eq!(x.get_value_as_string(), String::from("10.1"));
}

#[test]
fn jsonentity_array_ofnumbes_get_value_as_string_test() {
    let x = JsonEntity::Array(vec![
        JsonEntity::Number(10.0),
        JsonEntity::Number(3.1),
        JsonEntity::Number(12.0),
    ]);
    assert_eq!(x.get_value_as_string(), String::from("[10,3.1,12]"));
}

#[test]
fn jsonentity_array_get_value_as_string_test() {
    let x = JsonEntity::Array(vec![
        JsonEntity::Text(String::from("asd")),
        JsonEntity::Text(String::from("asd")),
        JsonEntity::Text(String::from("asd")),
    ]);
    assert_eq!(
        x.get_value_as_string(),
        String::from("[\"asd\",\"asd\",\"asd\"]")
    );
}
fn get_hashpmap() -> HashMap<String, JsonEntity> {
    let mut map = HashMap::new();
    map.insert(String::from("BooleanValue"), JsonEntity::Boolean(false));
    map.insert(String::from("NumericValue"), JsonEntity::Number(10.0));
    map
}
#[test]
fn jsonentity_object_get_value_as_string_test() {
    let map = get_hashpmap();
    let obj = JsonEntity::Object(map);
    assert_eq!(
        obj.get_value_as_string(),
        String::from("{\"BooleanValue\":false,\"NumericValue\":10}")
    );
}
