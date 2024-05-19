pub mod jsonobject {
    use std::{collections::HashMap, f32, fmt::Error};

    pub struct JsonObject {
        obj: JsonEntity,
    }

    impl JsonObject {
        pub fn new() -> JsonObject {
            let map = HashMap::new();
            let ent = JsonEntity::Object(map);
            JsonObject { obj: ent }
        }

        pub fn add_text(&self, name: String, value: String) -> Result<String, Error> {
            match &self.obj {
                JsonEntity::Object(map) => {}
                _ => panic!("Wrong type of entity!"),
            }
        }
    }

    #[derive(Debug)]
    enum JsonEntity {
        Number(f32),
        Text(String),
        Boolean(bool),
        Array(Vec<JsonEntity>),
        Object(HashMap<String, JsonEntity>),
    }

    impl JsonEntity {
        pub fn get_value_as_string(&self) -> String {
            match self {
                JsonEntity::Text(s) => quote(&s),
                JsonEntity::Number(n) => n.to_string(),
                JsonEntity::Boolean(b) => b.to_string(),
                JsonEntity::Array(arr) => build_from_array(&arr),
                JsonEntity::Object(obj) => build_from_obj(&obj),
            }
        }
        pub fn flush_to_string(self) -> String {
            self.get_value_as_string()
        }
    }

    fn build_from_obj(obj: &HashMap<String, JsonEntity>) -> String {
        let mut st = String::from("{");

        for (key, val) in obj.iter() {
            st.push_str(format!("{}:{},", quote(&key), val.get_value_as_string()).as_str());
        }
        st.pop();
        st.push_str("}");
        st
    }

    fn build_from_array(arr: &Vec<JsonEntity>) -> String {
        let mut st = String::from("[");
        for i in arr {
            st.push_str((i.get_value_as_string() + ",").as_str());
        }
        st.pop();
        st.push_str("]");
        st
    }

    fn quote(str: &String) -> String {
        format!("\"{str}\"")
    }

    #[test]
    fn JsonEntity_boolean_get_value_as_string_test() {
        let x = JsonEntity::Boolean(false);
        assert_eq!(x.get_value_as_string(), String::from("false"))
    }

    #[test]
    fn JsonEntity_text_get_value_as_string_test() {
        let x = JsonEntity::Text(String::from("test"));
        assert_eq!(x.get_value_as_string(), String::from("\"test\""));
        println!("{:?}", x);
    }

    #[test]
    fn JsonEntity_intnumber_get_value_as_string_test() {
        let x = JsonEntity::Number(10.0);
        assert_eq!(x.get_value_as_string(), String::from("10"));
    }

    #[test]
    fn JsonEntity_floatnumber_get_value_as_string_test() {
        let x = JsonEntity::Number(10.1);
        assert_eq!(x.get_value_as_string(), String::from("10.1"));
    }

    #[test]
    fn JsonEntity_array_ofnumbes_get_value_as_string_test() {
        let x = JsonEntity::Array(vec![
            JsonEntity::Number(10.0),
            JsonEntity::Number(3.1),
            JsonEntity::Number(12.0),
        ]);
        assert_eq!(x.get_value_as_string(), String::from("[10,3.1,12]"));
    }

    #[test]
    fn JsonEntity_array_get_value_as_string_test() {
        let x = JsonEntity::Array(vec![
            JsonEntity::Text(String::from("asd")),
            JsonEntity::Text(String::from("asd")),
            JsonEntity::Text(String::from("asd")),
        ]);
        assert_eq!(
            x.get_value_as_string(),
            String::from("[\"asd\",\"asd\",\"asd\"]")
        );
    }
    fn get_hashpmap() -> HashMap<String, JsonEntity> {
        let mut map = HashMap::new();
        map.insert(String::from("BooleanValue"), JsonEntity::Boolean(false));
        map.insert(String::from("NumericValue"), JsonEntity::Number(10.0));
        map
    }
    #[test]
    fn JsonEntity_object_get_value_as_string_test() {
        let map = get_hashpmap();
        let obj = JsonEntity::Object(map);
        assert_eq!(
            obj.get_value_as_string(),
            String::from("{\"BooleanValue\":false,\"NumericValue\":10}")
        );
    }
}
