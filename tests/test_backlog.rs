extern crate cleverbot;

use cleverbot::Cleverbot;

#[test]
pub fn backlog() {
  let mut cleverbot = Cleverbot::new();
  cleverbot.think("How're you today?".to_string());

  assert!(cleverbot.backlog.len() == 1);
}
