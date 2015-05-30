extern crate cleverbot;

use cleverbot::Cleverbot;

use std::io;

fn main() {
  print!("<You> ");
  /*let mut reader = io::stdin();
  let mut question = String::new();
  reader.read_line(&mut question); */

  let mut cleverbot = Cleverbot::new();
  let mut reply;

  while {
    reply = cleverbot.think("Hi".to_string());
    reply.answer.is_empty()
  } {}

  println!("<Cleverbot> {}", reply.answer);
}
