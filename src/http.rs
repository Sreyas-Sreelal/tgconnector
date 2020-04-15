use minihttp::request::Request;
use std::collections::HashMap;

pub enum HttpMethod {
    Get,
    Post,
}

pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub body: Option<String>,
    pub proxy_url: Option<String>,
}

impl HttpRequest {
    pub fn make_request(&self) -> Result<String, String> {
        let mut requests_obj = match Request::new(&self.url) {
            Ok(requests_obj) => requests_obj,

            Err(err) => {
                return Err(format!("Error building request to telegram api\n{:?}", err));
            }
        };
        let method = match self.method {
            HttpMethod::Get => {
                if let Some(proxy_url) = &self.proxy_url {
                    match requests_obj.proxy(&proxy_url) {
                        Ok(method) => method.get(),
                        Err(err) => {
                            return Err(format!("Error connecting to proxy server \n{:?}", err));
                        }
                    }
                } else {
                    requests_obj.get()
                }
            }

            HttpMethod::Post => {
                let body = &self.body.clone().unwrap();
                requests_obj.body_str(&body);
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                requests_obj.headers(headers);
                if let Some(proxy_url) = &self.proxy_url {
                    match requests_obj.proxy(&proxy_url) {
                        Ok(method) => method.post(),
                        Err(err) => {
                            return Err(format!("Error connecting to proxy server \n{:?}", err));
                        }
                    }
                } else {
                    requests_obj.post()
                }
            }
        };
        match method.send() {
            Ok(data) => Ok(data.text()),

            Err(err) => Err(format!("Error sending request to telegram api\n{:?}", err)),
        }
    }
}
