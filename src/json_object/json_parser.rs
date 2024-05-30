use std::{char, str::Chars, vec};

use super::json_entity::JsonEntity;

static NumericChar: [char; 11] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'];
#[derive(Debug)]
pub struct JsonTokenValue {
    value_type: JsonTokenType,
    value: String,
}
#[derive(Debug)]
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
pub fn build_token_vec(chars: &mut Chars) -> Vec<JsonTokenValue> {
    let mut vec = Vec::new();
    while let Some(c) = chars.next() {
        determinate_token(c, &mut vec, chars);
    }
    vec
}

fn determinate_token(c: char, vec: &mut Vec<JsonTokenValue>, chars: &mut Chars) {
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
        check_if_false(chars, vec);
    } else if c == 't' {
        check_if_true(chars, vec)
    } else if c == 'n' {
        check_if_null(chars, vec)
    } else if c == '"' {
        get_string_value(chars, vec);
    } else if NumericChar.contains(&c) {
        get_number_value(chars, vec, c);
    }
}

fn get_number_value(chars: &mut Chars, vec: &mut Vec<JsonTokenValue>, c: char) {
    let mut value = String::new();
    value.push(c);
    while let Some(c_n) = chars.next() {
        if NumericChar.contains(&c_n) {
            value.push(c_n);
            continue;
        } else {
            determinate_token(c_n, vec, chars);
            break;
        }
    }
    vec.push(JsonTokenValue {
        value_type: JsonTokenType::Number,
        value: value,
    });
}

fn get_string_value(chars: &mut Chars, vec: &mut Vec<JsonTokenValue>) {
    let mut value = String::new();
    while let Some(c_n) = chars.next() {
        if (c_n == '"') {
            break;
        } else {
            value.push(c_n)
        }
    }
    vec.push(JsonTokenValue {
        value_type: JsonTokenType::String,
        value: value,
    })
}

fn check_if_false(chars: &mut Chars, vec: &mut Vec<JsonTokenValue>) {
    let values = chars.take(4);
    if String::from_iter(values) == String::from("alse") {
        vec.push(JsonTokenValue {
            value_type: JsonTokenType::False,
            value: String::from("false"),
        })
    } else {
        panic!("Unnown Value, when interpreting \"False\" token")
    }
}

fn check_if_true(chars: &mut Chars, vec: &mut Vec<JsonTokenValue>) {
    let values = chars.take(3);
    if String::from_iter(values) == String::from("rue") {
        vec.push(JsonTokenValue {
            value_type: JsonTokenType::True,
            value: String::from("true"),
        })
    } else {
        panic!("Unnown Value, when interpreting \"True\" token")
    }
}

fn check_if_null(chars: &mut Chars, vec: &mut Vec<JsonTokenValue>) {
    let values = chars.take(3);
    if String::from_iter(values) == String::from("ull") {
        vec.push(JsonTokenValue {
            value_type: JsonTokenType::Null,
            value: String::from("null"),
        })
    } else {
        panic!("Unnown Value, when interpreting \"null\" token")
    }
}

// TEST
//-----------------------------------------------------------------------------------------------
#[test]
fn check_if_false_test() {
    let mut str = String::from("false");
    let mut chars = str.chars();
    let char = chars.next().unwrap();
    assert_eq!(char, 'f');
    let mut vec = Vec::new();
    check_if_false(&mut chars, &mut vec);
    assert!(vec.len() > 0);
    assert!(chars.count() == 0);
}

#[test]
fn check_if_true_test() {
    let mut str = String::from("true");
    let mut chars = str.chars();
    let char = chars.next().unwrap();
    assert_eq!(char, 't');
    let mut vec = Vec::new();
    check_if_true(&mut chars, &mut vec);
    assert!(vec.len() > 0);
    assert!(chars.count() == 0);
}

#[test]
fn check_if_null_test() {
    let mut str = String::from("null");
    let mut chars = str.chars();
    let char = chars.next().unwrap();
    assert_eq!(char, 'n');
    let mut vec = Vec::new();
    check_if_null(&mut chars, &mut vec);
    assert!(vec.len() > 0);
    assert!(chars.count() == 0);
}

#[test]
fn get_string_value_test() {
    let mut str = String::from("\"Abdul\"");
    let mut chars = str.chars();
    let char = chars.next().unwrap();
    assert_eq!(char, '\"');
    let mut vec = Vec::new();
    get_string_value(&mut chars, &mut vec);
    assert!(vec.len() > 0);
    let value = vec.first();
    assert!(value.unwrap().value == "Abdul");
    assert!(chars.count() == 0);
}

#[test]
fn get_number_value_test() {
    let mut str = String::from("322.1");
    let mut chars = str.chars();
    let mut vec = Vec::new();
    get_number_value(&mut chars, &mut vec, '3');
    assert!(vec.len() > 0);
    let value = vec.first();
    assert!(value.unwrap().value == "3322.1");
    assert!(chars.count() == 0);
}

#[test]
fn build_token_array_test() {
    let mut str =
        String::from("{\"String\":\"John123\",\"Number\":1231 , \"Object\":{},\"TrueValue\":true}");
    let mut chars = str.chars();
    let vec = build_token_vec(&mut chars);
    assert!(vec.len() == 18)
}
