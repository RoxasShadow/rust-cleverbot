extern crate cleverbot;

use cleverbot::Cleverbot;

use std::env;

fn main() {
  let args : Vec<_> = env::args().collect();
  if args.len() == 1 {
    panic!("think something");
  }

  let reply = Cleverbot::new().think(args[1].clone());
  println!("<Cleverbot> {}", reply);
}
