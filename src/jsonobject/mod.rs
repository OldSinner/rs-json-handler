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
                JsonObject::Text(s) => {
                    let mut st = String::from("\"");
                    st.push_str(s.as_str());
                    st.push('\"');
                    st
                }
                JsonObject::Number(n) => n.to_string(),
                JsonObject::Boolean(b) => {
                    if b {
                        String::from("true")
                    } else {
                        String::from("false")
                    }
                }
                JsonObject::Array(arr) => {
                    let mut st = String::from("[");
                    for i in arr {
                        st.push_str((i.get_string() + ",").as_str());
                    }
                    st.pop();
                    st.push_str("]");
                    st
                }
                _ => panic!("Not implemented! Aaaa!"),
            }
        }
    }
}

#[cfg(test)]
mod tests;
