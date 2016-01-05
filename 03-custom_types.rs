#![allow(dead_code)]

struct Nil;

struct Pair(i32, f64);

struct Point {
  x: f64,
  y: f64,
}

#[allow(dead_code)]
struct Rectangle {
  p1: Point,
  p2: Point,
}

fn structures() {
  let point = Point { x: 0.3, y: 0.6 };
  println!("Point coordinates: ({} {})", point.x, point.y);

  let Point { x: my_x, y: my_y } = point;

  let rectangle = Rectangle {
    p1: Point { x: my_y, y: my_x },
    p2: point,
  };

  let nil = Nil;

  let pair = Pair(10, 1.45);

  let Pair ( int, dec ) = pair;

  println!("Pair contains {} and {}", int, dec);
}

enum Person {
  Skinny,
  Fat,
  Height(i32),
  Weight(i32),
  Info { name: String, height: i32 },
}

fn inspect(p: Person) {
  match p {
    Person::Skinny                => println!("Is skinny!"),
    Person::Fat                   => println!("Is fat!"),
    Person::Height(i)             => println!("Has a height of {}", i),
    Person::Weight(i)             => println!("Has a weight of {}", i),
    Person::Info { name, height } => println!("{} is {} tall!", name, height),
  }
}

fn enums() {
  let person = Person::Height(18);
  let danny  = Person::Weight(10);
  // `to_owned()` creates an owned `String` from a string slice.
  let dave   = Person::Info { name: "Dave".to_owned(), height: 72 };
  let john   = Person::Fat;
  let larry  = Person::Skinny;
  inspect(person);
  inspect(danny);
  inspect(dave);
  inspect(john);
  inspect(larry);
}

enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

fn using_use() {
  // Explicitly `use` each name so they are available without
  // manual scoping.
  use Status::{Poor, Rich};
  // Automatically `use` each name inside `Work`.
  use Work::*;

  // Equivalent to `Status::Poor`.
  let status = Poor;
  // Equivalent to `Work::Civilian`.
  let work = Civilian;

  match status {
    Rich => println!("The rich have lots of money!"),
    Poor => println!("The poor have no money..."),
  }

  match work {
    Civilian => println!("Civilians work!"),
    Soldier  => println!("Soldiers fight!"),
  }
}

enum Number {
  Zero,
  One,
  Two,
}

// enum with explicit discriminator
enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

fn c_style() {
  // `enums` can be cast as integers.
  println!("zero is {}", Number::Zero as i32);
  println!("one is {}", Number::One as i32);

  println!("roses are #{:06x}", Color::Red as i32);
  println!("violets are #{:06x}", Color::Blue as i32);
}

use List::*;

enum List {
  Cons(u32, Box<List>),
  LNil, // applying different name to avoid conflicts ...
}

impl List {
  fn new() -> List {
    LNil
  }
  fn prepend(self, elem: u32) -> List {
    Cons(elem, Box::new(self))
  }
  fn len(&self) -> u32 {
    match *self {
      LNil => 0,
      Cons(_, ref tail) => 1 + tail.len(),
    }
  }
  fn stringify(&self) -> String {
    match *self {
      Cons(head, ref tail) => format!("({}:{})", head, tail.stringify()),
      LNil => format!("Nil"),
    }
  }
}

fn linked_list() {
  let mut list = List::new();
  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(3);

  println!("The length of the list is: {}", list.len());
  println!("List is: {}", list.stringify());
}

static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_bigger(n: i32) -> bool {
  n > THRESHOLD
}

fn constants() {
  println!("This is {}", LANGUAGE);
  println!("The threshold is {}", THRESHOLD);
  let n = 16;
  println!("{} is {} than {}", n, if is_bigger(n) { "bigger" } else {"smaller"}, THRESHOLD);
}

fn main() {

  structures();
  enums();
  using_use();
  c_style();

  println!("Linked list: -----------------------------");

  linked_list();

  println!("Constants: -------------------------------");

  constants();

}