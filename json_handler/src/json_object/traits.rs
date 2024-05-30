use super::{json_entity::JsonEntity, JsonObject};

pub trait JsonValueTrait {
    fn get_as_json_entity(self) -> JsonEntity;
}

impl JsonValueTrait for String {
    fn get_as_json_entity(self) -> JsonEntity {
        JsonEntity::Text(self)
    }
}
impl JsonValueTrait for &str {
    fn get_as_json_entity(self) -> JsonEntity {
        JsonEntity::Text(self.to_string())
    }
}
impl JsonValueTrait for bool {
    fn get_as_json_entity(self) -> JsonEntity {
        JsonEntity::Boolean(self)
    }
}

impl JsonValueTrait for f32 {
    fn get_as_json_entity(self) -> JsonEntity {
        JsonEntity::Number(self)
    }
}

impl JsonValueTrait for JsonObject {
    fn get_as_json_entity(self) -> JsonEntity {
        self.obj
    }
}
