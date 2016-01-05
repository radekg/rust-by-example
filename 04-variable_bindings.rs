fn intro() {
  let an_integer = 1u32;
  let a_boolean = true;
  let unit = ();

  let copied_integer = an_integer;

  println!("An integer: {:?}", copied_integer);
  println!("A boolean: {:?}", a_boolean);
  println!("Meet the unit value: {:?}", unit);

  let _unused_variable = 3u32;
  let noisy_unused_variable = 2u32;
}

fn mutability() {
  let _immutable_binding = 1;
  let mut mutable_binding = 1;

  println!("Before mutation: {}", mutable_binding);

  mutable_binding += 1;

  println!("After mutation: {}", mutable_binding);

  // _immutable_binding += 1; // error
}

fn scope_and_shadowing() {
  let long_lived_binding = 1;
  {
    let short_lived_binding = 2;
    println!("inner short: {}", short_lived_binding);
    let long_lived_binding = 5_f32; // shadows...
    println!("inner long: {}", long_lived_binding);
  }
  println!("outer long: {}", long_lived_binding);
  let long_lived_binding = 'a';
  println!("outer long: {}", long_lived_binding);
}

fn declare_first() {
  let a_binding;
  {
    let x = 2;
    a_binding = x * x;
  }
  println!("A binding is: {}", a_binding);

  let another_binding;

  // println!("Another binding is: {}", another_binding); // error: use of possibly uninitialized variable: `another_binding`

  another_binding = 1;

  println!("Another binding is: {}", another_binding);
}

fn main() {
  intro();
  mutability();
  scope_and_shadowing();
  declare_first();
}