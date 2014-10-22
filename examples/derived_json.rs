extern crate serialize;
use serialize::json;

#[deriving(Show,Encodable,Decodable)]
struct User {
  id: u64,
  name: String,
  friends: Vec<u64>
}

fn main() {
  let brenda = User {
    id: 1,
    name: "BrendaTheSuccessfulDieter".to_string(),
    friends: vec![4,47,92]
  };

  let json_brenda = json::encode(&brenda);
  println!("brenda.to_json(): \n{}", json_brenda);

  println!("decoded brenda: \n{}\n", json::decode::<User>(json_brenda.as_slice()));
}
