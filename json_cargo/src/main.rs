#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

fn main() {
    let data = r#" {
     "name": "John Doe", "age": 43,
     "address": {"street": "Main", "city": "Broomfield"},
     "phones":["27726550023"]
    } "#;
    let p: Person = serde_json::from_str(data).expect("Deserialization Error!");
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    println!("{:#?}", p);
}
