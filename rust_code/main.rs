struct Person {
  name: String
}

impl Person {
  fn new(_name: &str) -> Person {
    Person {name: _name.to_string()}
  }
}

fn main() {
  let eg = Person::new("Egor");
  println!("Hello, {name}", name = eg.name);
}
