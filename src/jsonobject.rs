pub mod jsonobject {
    use std::collections::HashMap;
    pub enum JsonObject {
        FloatNumber(f32),
        Text(String),
        Boolean(bool),
        Array(Vec<JsonObject>),
        Object(HashMap<String, JsonObject>),
    }
}
