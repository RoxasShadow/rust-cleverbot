use url::form_urlencoded;

use crypto::md5::Md5;
use crypto::digest::Digest;

use ::Params;

pub trait Utils {}

impl Utils {
  pub fn to_query_string(params: &Params) -> String {
    return form_urlencoded::serialize(params.into_iter())
  }

  pub fn checksum(query_string: &String) -> String {
    return Self::md5(&query_string[9..35]);
  }

  pub fn md5(s: &str) -> String {
    let mut h = Md5::new();
    h.input_str(s);
    return h.result_str();
  }
}
