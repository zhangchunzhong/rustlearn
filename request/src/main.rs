use reqwest::header::{USER_AGENT, HeaderMap, HeaderValue};
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let mut headers = HeaderMap::new();
	headers.insert(USER_AGENT, HeaderValue::from_static("User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.121 Safari/537.36"));
	let client = Client::new();
	let req = client.get("http://www.baidu.com").headers(headers).build()?;
	let rep = client.execute(req)?;
	println!("{:?}", rep.text());
	Ok(())
}