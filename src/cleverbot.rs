use std::io::Read;
use std::collections::HashMap;

use hyper::{Client, Url};

use ::Params;
use ::Utils;

pub struct Cleverbot<'a> {
      params:  Params,
  pub backlog: Vec<&'a Response>
}

#[derive(Clone, Debug)]
pub struct Response {
  pub question: String,
  pub answer:   String
}

impl<'a> Cleverbot<'a> {
  pub fn new() -> Cleverbot<'a> {
    let mut params = HashMap::new();
    params.insert("start",      "y"    .to_string());
    params.insert("icognoid",   "wsf"  .to_string());
    params.insert("fno",        "0"    .to_string());
    params.insert("sub",        "Say"  .to_string());
    params.insert("islearning", "1"    .to_string());
    params.insert("cleanslate", "false".to_string());

    return Cleverbot {
      params:  params,
      backlog: Vec::new()
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

  fn get_response(&mut self, response: &Vec<&str>) -> Response {
    let params = [
      "sessionid",  "logurl",     "vText8",           "vText7",
      "vText6",     "vText5",     "vText4",           "vText3",
      "vText2",     "prevref",    "emotionalhistory", "ttsLocMP3",
      "ttsLocTXT",  "ttsLocTXT3", "ttsText",          "lineRef",
      "lineURL",    "linePOST",   "lineChoices",      "lineChoicesAbbrev",
      "typingData", "divert"
    ];

    for i in 1..23 {
      self.params.insert(params[i - 1], response[i].to_string());
    }

    return Response {
      question: response[8].to_string(),
      answer:   response[16].to_string()
    };
  }

  pub fn think(&mut self, thought: String) -> Response {
    self.ask_for(thought);

    let request              = self.request();
    let slice    : Vec<&str> = request.split("\r").collect();
    let response             = self.get_response(&slice);

    // self.backlog.push(&response);

    return response;
  }
}
