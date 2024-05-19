use crate::jsonobject::jsonobject::JsonObject;

#[test]
fn jsonobject_boolean_getstring() {
    let x = JsonObject::Boolean(false);
    assert_eq!(x.get_value_as_string(), String::from("false"))
}

#[test]
fn jsonobject_text_getstring() {
    let x = JsonObject::Text(String::from("test"));
    assert_eq!(x.get_value_as_string(), String::from("\"test\""));
    println!("{:?}", x);
}

#[test]
fn jsonobject_intnumber_getstring() {
    let x = JsonObject::Number(10.0);
    assert_eq!(x.get_value_as_string(), String::from("10"));
    println!("{:?}", x);
}

#[test]
fn jsonobject_floatnumber_getstring() {
    let x = JsonObject::Number(10.1);
    assert_eq!(x.get_value_as_string(), String::from("10.1"));
    println!("{:?}", x);
}

#[test]
fn jsonobject_array_ofnumbes_getstring() {
    let x = JsonObject::Array(vec![
        JsonObject::Number(10.0),
        JsonObject::Number(3.1),
        JsonObject::Number(12.0),
    ]);
    assert_eq!(x.get_value_as_string(), String::from("[10,3.1,12]"));
    println!("{:?}", x);
}

#[test]
fn jsonobject_array_ofstring_getstring() {
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
