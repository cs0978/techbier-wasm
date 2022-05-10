use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  let mut name = String::from("world");
  if args.len() > 1 {
    name = (&args[1]).to_owned();
  }

  println!("hello {}!", name);
}
