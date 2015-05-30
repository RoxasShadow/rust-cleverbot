extern crate cleverbot;

use cleverbot::Cleverbot;

use std::env;

fn main() {
  let args : Vec<_> = env::args().collect();
  if args.len() == 1 {
    panic!("think something");
  }

  let mut cleverbot = Cleverbot::new();
  let mut reply;

  while {
    reply = cleverbot.think(args[1].to_string());
    reply.answer.is_empty()
  } {}

  println!("<Cleverbot> {}", reply.answer);
}
