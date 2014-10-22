use std::collections::HashMap;
use std::collections::PriorityQueue;

fn main() {
  let mut map: HashMap<String, u8> = HashMap::new();
  let mut queue: PriorityQueue<u8> = PriorityQueue::new();

  for n in range(0u, 10) {
    map.insert(n.to_string(), std::rand::random());
    queue.push(std::rand::random());
  }

  for (k,v) in map.iter() { println!("Map Item: {} => {}", k, v) }
  for i in queue.iter() { println!("Queue Item: {}", i) }
}
