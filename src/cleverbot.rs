use std::io::Read;
use std::collections::HashMap;

use hyper::{Client, Url};

use ::Params;
use ::Utils;

pub struct Cleverbot {
  params: Params
}

impl Cleverbot {
  pub fn new() -> Cleverbot {
    let mut params = HashMap::new();
    params.insert("start",      "y"    .to_string());
    params.insert("icognoid",   "wsf"  .to_string());
    params.insert("fno",        "0"    .to_string());
    params.insert("sub",        "Say"  .to_string());
    params.insert("islearning", "1"    .to_string());
    params.insert("cleanslate", "false".to_string());

    return Cleverbot {
      params: params
    };
  }

  pub fn request(&mut self) -> String {
    let url = "http://www.cleverbot.com/webservicemin";

    // TODO: Cache client and/or builder
    let mut client  = Client::new();
    let     uri     = Url::parse(url).ok().unwrap();
    let     body    = Utils::to_query_string(&mut self.params);
    let mut builder = client.post(uri)
      .body(&*body)
      .send().unwrap();

    let mut response = String::new();
    builder.read_to_string(&mut response).unwrap();

    return response;
  }

  fn ask_for(&mut self, thought: String) {
    self.params.insert("stimulus", thought);

    let query_string = Utils::to_query_string(&self.params);
    self.params.insert("icognocheck", Utils::checksum(&query_string));
  }

  // fn prepare_post(query_string: &mut Params) {
  //   let params = [
  //     "sessionid", "logurl", "vText8", "vText7",
  //     "vText6", "vText5", "vText4", "vText3",
  //     "vText2", "prevref", "emotionalhistory", "ttsLocMP3",
  //     "ttsLocTXT", "ttsLocTXT3", "ttsText", "lineRef",
  //     "lineURL", "linePOST", "lineChoices", "lineChoicesAbbrev",
  //     "typingData", "divert"
  //   ];

  //   for i in 1..23 {
  //     query_string.insert(params[i - 1], i.to_string());
  //   }
  // }

  pub fn think(&mut self, thought: String) -> String {
    self.ask_for(thought);

    let response = self.request();
    // Self::prepare_post(&mut params);
    // TODO: Backlog

    let slice : Vec<&str> = response.split("\r").collect();
    let reply = if slice[0].is_empty() { slice[1] } else { slice[0] };
    return reply.to_string();
  }
}
