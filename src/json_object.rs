use self::errors::JsonObjectError;
use self::json_entity::JsonEntity;
mod errors;
mod json_entity;

use core::panic;
use std::{
    collections::HashMap,
    f32,
    fmt::{self},
};

pub struct JsonObject {
    obj: JsonEntity,
}

impl JsonObject {
    pub fn new() -> JsonObject {
        let map = HashMap::new();
        let ent = JsonEntity::Object(map);
        JsonObject { obj: ent }
    }

    pub fn add_text(
        &mut self,
        name: String,
        value: String,
    ) -> Result<&JsonObject, JsonObjectError> {
        match &mut self.obj {
            JsonEntity::Object(map) => {
                map.insert(name, JsonEntity::Text(value))
                    .or(return Err(JsonObjectError));
                return Ok(&self);
            }
            _ => panic!("Wrong Entity Type!"),
        }
    }

    pub fn flush_to_string(self) -> String {
        self.obj.flush_to_string()
    }
}

#[test]
fn JsonEntity_add_string_test() {
    let mut json = JsonObject::new();
    json.add_text(String::from("Test"), String::from("abc"));
    json.add_text(String::from("Test"), String::from("abc"));
    assert_eq!(json.flush_to_string(), String::from("{\"Test\":\"abc\"}"));
}
