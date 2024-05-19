use std::collections::HashMap;

use crate::jsonobject::jsonobject::JsonObject;

#[test]
fn jsonobject_boolean_get_value_as_string_test() {
    let x = JsonObject::Boolean(false);
    assert_eq!(x.get_value_as_string(), String::from("false"))
}

#[test]
fn jsonobject_text_get_value_as_string_test() {
    let x = JsonObject::Text(String::from("test"));
    assert_eq!(x.get_value_as_string(), String::from("\"test\""));
    println!("{:?}", x);
}

#[test]
fn jsonobject_intnumber_get_value_as_string_test() {
    let x = JsonObject::Number(10.0);
    assert_eq!(x.get_value_as_string(), String::from("10"));
    println!("{:?}", x);
}

#[test]
fn jsonobject_floatnumber_get_value_as_string_test() {
    let x = JsonObject::Number(10.1);
    assert_eq!(x.get_value_as_string(), String::from("10.1"));
    println!("{:?}", x);
}

#[test]
fn jsonobject_array_ofnumbes_get_value_as_string_test() {
    let x = JsonObject::Array(vec![
        JsonObject::Number(10.0),
        JsonObject::Number(3.1),
        JsonObject::Number(12.0),
    ]);
    assert_eq!(x.get_value_as_string(), String::from("[10,3.1,12]"));
    println!("{:?}", x);
}

#[test]
fn jsonobject_array_get_value_as_string_test() {
    let x = JsonObject::Array(vec![
        JsonObject::Text(String::from("asd")),
        JsonObject::Text(String::from("asd")),
        JsonObject::Text(String::from("asd")),
    ]);
    assert_eq!(
        x.get_value_as_string(),
        String::from("[\"asd\",\"asd\",\"asd\"]")
    );
    println!("{:?}", x);
}

fn get_hashpmap() -> HashMap<String, JsonObject> {
    let mut map = HashMap::new();
    map.insert(String::from("BooleanValue"), JsonObject::Boolean(false));
    map.insert(String::from("NumericValue"), JsonObject::Number(10.0));
    map
}
#[test]
fn jsonobject_object_get_value_as_string_test() {
    let map = get_hashpmap();
    let obj = JsonObject::Object(map);
    assert_eq!(
        obj.get_value_as_string(),
        String::from("{\"BooleanValue\":false,\"NumericValue\":10}")
    );
    println!("{:?}", obj);
}
