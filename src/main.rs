mod json_object;
use json_object::JsonObject;
fn main() {
    let mut json = JsonObject::new();
    json.add("test1", 10.0);
    json.add("test2", 10.4);
    json.add("User", "Tomasz");
    json.add("Second", "Password");
    json.add("Boolean", false);

    print!("{}", json.flush_to_string());
}
