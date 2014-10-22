#[deriving(Show,PartialEq)]
enum Color {
  Red,
  Yellow,
  Green
}

impl Color {
  fn new() -> Color {
    Red  // safe default
  }

  fn peek_next(self) -> Color {
    match self {
      Red => Green,
      Yellow => Red,
      Green => Yellow
    }
  }
}

fn main() {
  let stoplight = Color::new();
  let new_stoplight = stoplight.peek_next();
  println!("{} will soon become {}.", stoplight, new_stoplight);
}

fn timing_example() {
  let seq: [Color, ..3] = [Green, Yellow, Red];
  let timing: Vec<uint> = vec![20u, 4u, 10u];

  let mut stoplight = seq.iter().zip(timing.iter()).cycle();

  let mut stop = 0u;
  for (color, time) in stoplight {
    println!("{} for {}s", color, time);

    if stop > 3 { break; }
    if *color == Red {
      stop += 1;
    }
  }
}

fn stopped_example() {
  let seq: [Color, ..3] = [Green, Yellow, Red];
  let timing: Vec<uint> = vec![20u, 4u, 10u];

  let mut stoplight = seq.iter().zip(timing.iter());

  let mut no_cars = true;
  for (color, time) in stoplight {
    println!("{} for {}s", color, time);

    if *color == Green && no_cars {
        stoplight = [Green].iter().chain(stoplight);
        no_cars = false;
    }
  }
}

#[test]
fn advances_yellow_to_red() {
  let stoplight = Yellow.peek_next();
  match stoplight {
      Red => true,
      _ => fail!("Yellow stoplights advance to Red!")
  };
}
