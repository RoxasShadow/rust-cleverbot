extern crate url;
extern crate hyper;
extern crate crypto;

use std::collections::HashMap;
pub type Params = HashMap<&'static str, String>;

pub mod utils;
pub mod cleverbot;

pub use utils::Utils;
pub use cleverbot::Cleverbot;
