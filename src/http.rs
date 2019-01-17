use minihttp::request::Request;
use types::*;

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
    pub fn make_request(&self) -> Result<APIResponse,String> {      
        match Request::new(&self.url) {
            Ok(mut requests_obj) => {
                let method = match self.method {
                    HttpMethod::Get => {
                        requests_obj.get()
                    },
                    
                    HttpMethod::Post => {
                        let body = &self.body.clone().unwrap();
                        requests_obj.body_str(&body);
                        
                        let mut headers = std::collections::HashMap::new();
                        headers.insert("Content-Type".to_string(),"application/json; charset=utf-8".to_string());
                        
                        requests_obj.headers(headers);
                        
                        requests_obj.post()
                    }
                };

                

                match method.send() {
                    
                    Ok(data) => {
                        let data:Result<APIResponse,serde_json::Error> = serde_json::from_str(&data.text());
                        
                        match data {
                            Ok(data) => {
                                Ok(data)
                            },

                            Err(_err) => {
                                //log!("err is {:?}",err);
                                Ok(
                                    APIResponse {
                                        ok: true,
                                        result: None
                                    }
                                )
                            }
                        }
                        
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
