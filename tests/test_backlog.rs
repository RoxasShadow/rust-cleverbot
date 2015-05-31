extern crate cleverbot;

use cleverbot::Cleverbot;

#[test]
pub fn backlog() {
  let mut cleverbot = Cleverbot::new();
  cleverbot.think_about("the meaning of life".to_string());

  assert!(cleverbot.backlog.len() == 1);
}
