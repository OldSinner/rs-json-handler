pub mod jsonobject {
    use std::collections::HashMap;
    pub enum JsonObject {
        Number(f32),
        Text(String),
        Boolean(bool),
        Array(Vec<JsonObject>),
        Object(HashMap<String, JsonObject>),
    }

    impl JsonObject {
        pub fn get_string(self) -> String {
            match self {
                JsonObject::Text(s) => s,
                JsonObject::Number(n) => n.to_string(),
                JsonObject::Boolean(b) => {
                    if b {
                        String::from("true")
                    } else {
                        String::from("false")
                    }
                }
                _ => panic!("Not implemented! Aaaa!"),
            }
        }
    }
}
