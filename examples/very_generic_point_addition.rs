trait Add<RHS, Result> {
  fn add<RHS, Result>(&self, rhs: &RHS) -> Result;
}

#[deriving(Show)]
struct Point<T> {
  x: T,
  y: T
}

impl<T: Add<T,T>> Add<Point<T>,Point<T>> for Point<T>{
  fn add(&self, rhs: &Point<T>) -> Point<T> {
    Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y
    }
  }
}

fn main() {
  let p1 = Point { x: 2.0_f64, y: 2.0 };
  let p2 = Point { x: 10.0_f64, y: 10.0 };

  println!("{} + {} = {}", p1, p2, p1 + p2);
}
