mod json_object;
use json_object::JsonObject;
fn main() {
    let mut chars = "abc".chars();
    chars.next();
    chars.next();
    let x = chars.clone().take(2);
    println!("{}", String::from_iter(x));
}
