use reqwest::Client as HttpClient;
use songbird::typemap::TypeMapKey;

pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}
