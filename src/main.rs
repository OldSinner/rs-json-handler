use crate::jsonobject::jsonobject::JsonObject;

pub mod jsonobject;

fn main() {
    let x = JsonObject::Boolean(false);
    println!("{}", x.get_string());
    println!("Hello, world!");
}
