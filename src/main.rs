mod json_object;
use json_object::JsonObject;
fn main() {
    let mut json = JsonObject::new();
    let mut json2 = JsonObject::new();
    json2.add("Test", 1.0);
    json2.add("Test2", 3.0);
    json2.add_array("Test2", vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    json.add("NestedObject", json2);
    json.add("test1", 10.0);
    json.add("test2", 10.4);
    json.add("User", "Tomasz");
    json.add("Second", "Password");
    json.add("Boolean", false);

    print!("{}", json.flush_to_string());
}
