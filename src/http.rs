use minihttp::request::Request;
use std::collections::HashMap;

pub enum HttpMethod {
	Get,
	Post,
}

pub struct HttpRequest {
	pub url: String,
	pub method: HttpMethod,
	pub body: Option<String>
}

impl HttpRequest {
	pub fn make_request(&self) -> Result<String,String>{      
		match Request::new(&self.url) {
			Ok(mut requests_obj) => {
				let method = match self.method {
					HttpMethod::Get => {
						requests_obj.get()
					},
					
					HttpMethod::Post => {
						let body = &self.body.clone().unwrap();
						requests_obj.body_str(&body);
						let mut headers = HashMap::new();

						headers.insert(
							"Content-Type".to_string(),"application/json".to_string()
						);
						
						requests_obj.headers(headers);
						requests_obj.post()
					}
				};
				
				match method.send() {
					
					Ok(data) => {
						Ok(data.text())
					},

					Err(err) => {
						Err(format!("**[TGConnector] Error sending request to telegram api\n{:?}",err))
					}
				}
			},

			Err(err) => {
				Err(format!("**[TGConnector] Error building request to telegram api\n{:?}",err))
			}
		}
	}
}
