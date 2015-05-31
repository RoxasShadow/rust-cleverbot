extern crate cleverbot;

use cleverbot::Cleverbot;

use std::io;
use std::io::prelude::*;

fn main() {
  let mut cleverbot = Cleverbot::new();

  loop {
    print!("<You> ");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut reader   = io::stdin();
    let mut question = String::new();
    reader.read_line(&mut question).unwrap();

    question = question.trim().to_string();

    if question == "!backlog" {
      println!("{:?}", cleverbot.backlog)
    }
    else {
      let reply = cleverbot.think(question);
      println!("<Cleverbot> {}", reply.answer);
    }
  }
}
