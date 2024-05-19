use crate::jsonobject::jsonobject::JsonObject;

#[test]
fn jsonobject_boolean_getstring() {
    let x = JsonObject::Boolean(false);
    assert_eq!(x.get_string(), String::from("false"))
}

#[test]
fn jsonobject_text_getstring() {
    let x = JsonObject::Text(String::from("test"));
    assert_eq!(x.get_string(), String::from("test"))
}

#[test]
fn jsonobject_intnumber_getstring() {
    let x = JsonObject::Number(10.0);
    assert_eq!(x.get_string(), String::from("10"))
}

#[test]
fn jsonobject_floatnumber_getstring() {
    let x = JsonObject::Number(10.1);
    assert_eq!(x.get_string(), String::from("10.1"))
}
