use minihttp::request::Request;
use types::*;

pub fn make_request(api_link:String,endpoint:&str,params:Option<String>) -> Result<APIResponse,String>{
    let mut method = api_link;
    method.push('/');
    method.push_str(endpoint);

    if params != None {
        method.push('?');
        method.push_str(&params.unwrap());
    }

    match Request::new(&method){
        Ok(mut requests_obj) => {
            match requests_obj.get().send() {
                Ok(data) => {
                    let data:Result<APIResponse,serde_json::Error> = serde_json::from_str(&data.text());
                    match data {
                        Ok(data) => {
                            Ok(data)
                        },
                        Err(_) => {
                            Ok(
                                APIResponse {
                                    ok:true,
                                    result:None
                                }
                            )
                        }
                    }
                    
                },
                Err(_) => {
                    Err("**[TGConnector] Error sending request to telegram api".to_string())
                }
            }
        },
        Err(_) => {
            Err("**[TGConnector] Error building request to telegram api".to_string())
        }
    }
}
