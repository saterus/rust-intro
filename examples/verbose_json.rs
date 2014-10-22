extern crate serialize;
use serialize::json::{Json, ToJson, Object};
use std::collections::TreeMap;

#[deriving(Show)]
struct User {
  id: u64,
  name: String,
  friends: Vec<u64>
}

impl ToJson for User {
  fn to_json(&self) -> Json {
    let mut map = TreeMap::new();
    map.insert("id".to_string(), self.id.to_json());
    map.insert("name".to_string(), self.name.to_json());
    map.insert("friends".to_string(), self.friends.to_json());
    Object(map)
  }
}

fn main() {
  let brenda = User {
    id: 1,
    name: "BrendaTheSuccessfulDieter".to_string(),
    friends: vec![4,47,92]
  };

  println!("brenda.show(): \n{}\n", brenda);
  println!("brenda.to_json(): \n{}", brenda.to_json());
}
