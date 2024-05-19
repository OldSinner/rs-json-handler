use self::{errors::JsonObjectError, json_entity::JsonEntity, traits::JsonValueTrait};
mod errors;
mod json_entity;
mod traits;
use core::panic;
use std::{
    collections::HashMap,
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

    pub fn add<T: JsonValueTrait>(&mut self, key: &str, value: T) {
        match &mut self.obj {
            JsonEntity::Object(map) => {
                map.insert(key.to_string(), value.get_as_json_entity());
            }
            _ => panic!("Wront Type! That Shouldn't happend! Panic! Abort Mission!"),
        }
    }

    pub fn flush_to_string(self) -> String {
        self.obj.flush_to_string()
    }
}

#[test]
fn jsonobject_add_string_test() {
    let mut json = JsonObject::new();
    json.add("Test", String::from("abc"));
    assert_eq!(json.flush_to_string(), String::from("{\"Test\":\"abc\"}"));
}
