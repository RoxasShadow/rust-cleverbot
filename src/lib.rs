extern crate url;
extern crate hyper;
extern crate crypto;

use std::io::Read;
use std::collections::HashMap;

use hyper::{Client, Url};
use url::form_urlencoded;

use crypto::md5::Md5;
use crypto::digest::Digest;

pub type Params = HashMap<&'static str, String>;

pub trait Cleverbot {}

impl Cleverbot {
  pub fn request(params: &Params) -> String {
    let url = "http://www.cleverbot.com/webservicemin";

    let mut client  = Client::new();
    let     uri     = Url::parse(url).ok().unwrap();
    let     body    = Self::to_query_string(&params);
    let mut builder = client.post(uri)
      .body(&*body)
      .send()
      .unwrap();

    let mut response = String::new();
    builder.read_to_string(&mut response).unwrap();

    return response;
  }

  fn to_query_string(params: &Params) -> String {
    return form_urlencoded::serialize(params.into_iter())
  }

  fn checksum(query_string: &String) -> String {
    return Self::md5(&query_string[9..35]);
  }

  fn md5(s: &str) -> String {
    let mut h = Md5::new();
    h.input_str(s);
    return h.result_str();
  }

  fn params_for(thought: String) -> Params {
    // TODO: Cache this (except `stimulus`, ofc)
    let mut params = HashMap::new();
    params.insert("start",      "y"    .to_string());
    params.insert("icognoid",   "wsf"  .to_string());
    params.insert("fno",        "0"    .to_string());
    params.insert("sub",        "Say"  .to_string());
    params.insert("islearning", "1"    .to_string());
    params.insert("cleanslate", "false".to_string());
    params.insert("stimulus",   thought);

    let query_string = Self::to_query_string(&params);
    params.insert("icognocheck", Self::checksum(&query_string));

    return params;
  }

  fn prepare_post(query_string: &mut Params) {
    let params = [
      "sessionid", "logurl", "vText8", "vText7",
      "vText6", "vText5", "vText4", "vText3",
      "vText2", "prevref", "emotionalhistory", "ttsLocMP3",
      "ttsLocTXT", "ttsLocTXT3", "ttsText", "lineRef",
      "lineURL", "linePOST", "lineChoices", "lineChoicesAbbrev",
      "typingData", "divert"
    ];

    for i in 1..23 {
      query_string.insert(params[i - 1], i.to_string());
    }
  }

  pub fn think(thought: String) -> String {
    let mut params   = Self::params_for(thought);
    let     response = Self::request(&params);
    Self::prepare_post(&mut params);
    // TODO: Backlog

    let slice : Vec<&str> = response.split("\r").collect();
    let reply = if slice[0].is_empty() { slice[1] } else { slice[0] };
    return reply.to_string();
  }
}
