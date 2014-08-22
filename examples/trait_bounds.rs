use std::rand;
use std::rand::{Rng, Rand};

#[deriving(Show)]
enum Color {
  Red,
  Yellow,
  Green
}

impl Rand for Color {
    fn rand<R: Rng>(rng: &mut R) -> Color {
        match rng.gen::<u8>() {
            x if x % 3 == 0 => Red,
            x if x % 3 == 1 => Red,
            _ => Green
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        match (*self, *other) {
            (Red, Red) => true,
            (Yellow, Yellow) => true,
            (Green, Green) => true,
            _ => false
        }
    }
}

impl Eq for Color {}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Color) -> Option<Ordering> {
        match (*self, *other) {
            (x, y) if x == y => Some(Equal),
            (Green, Yellow) => Some(Less),
            (Green, Red) => Some(Less),
            (Yellow, Red) => Some(Less),
            _ => Some(Greater)
        }
    }
}

impl Ord for Color {
    fn cmp(&self, other: &Color) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut lights = vec![Red, Red, Green, Yellow, Red, Green, Yellow, Green, Green];

    println!("{}", lights);

    lights.sort();

    println!("Sorted {}", lights);

    let mut rand_lights: Vec<Color> = Vec::new();
    for _ in range(0u, 10) {
        rand_lights.push(rand::random());
    }
    println!("Rand {}", rand_lights);

    rand_lights.sort();

    println!("Sorted Rand {}", rand_lights);


}
