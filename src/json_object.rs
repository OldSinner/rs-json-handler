use self::{json_entity::JsonEntity, traits::JsonValueTrait};
mod errors;
mod json_entity;
mod json_parser;
mod traits;
use core::panic;
use std::collections::HashMap;

pub struct JsonObject {
    obj: JsonEntity,
}

impl JsonObject {
    pub fn new() -> JsonObject {
        let map = HashMap::new();
        let ent: JsonEntity = JsonEntity::Object(map);
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

    pub fn add_array<T: JsonValueTrait>(&mut self, key: &str, value: Vec<T>) {
        match &mut self.obj {
            JsonEntity::Object(map) => {
                let mut new_vec: Vec<JsonEntity> = Vec::new();
                for val in value {
                    new_vec.push(val.get_as_json_entity());
                }
                let ent = JsonEntity::Array(new_vec);
                map.insert(key.to_string(), ent);
            }
            _ => panic!("Wront Type! That Shouldn't happend! Panic! Abort Mission!"),
        }
    }

    pub fn flush_to_string(self) -> String {
        self.obj.flush_to_string()
    }
}

// TEST
//-----------------------------------------------------------------------------------------------
#[test]
fn jsonobject_add_string_test() {
    let mut json = JsonObject::new();
    json.add("Test", String::from("abc"));

    assert_eq!(json.flush_to_string(), String::from("{\"Test\":\"abc\"}"));
}
#[test]
fn jsonobject_add_array_test() {
    let mut json = JsonObject::new();
    json.add_array("Test", vec!["ab", "ad"]);
    assert_eq!(
        json.flush_to_string(),
        String::from("{\"Test\":[\"ab\",\"ad\"]}")
    );
}
