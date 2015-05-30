extern crate cleverbot;

use cleverbot::Cleverbot;

#[test]
pub fn backlog() {
  let mut cleverbot = Cleverbot::new();
  let mut reply;

  while {
    reply = cleverbot.think("How're you today?".to_string());
    reply.answer.is_empty()
  } {}

  assert!(cleverbot.backlog.len() == 1);
}
