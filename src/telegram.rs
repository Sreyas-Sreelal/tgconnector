use minihttp::request::Request;
use std::sync::mpsc::{Sender,Receiver,channel};
use serde_json::{Value};

#[derive(Deserialize,Debug,Clone)]
pub struct Update {
    pub result: Vec<Result>,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Result {
    pub message: Message,
    pub update_id: u64,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Message {
    pub text: Option<String>
}

pub struct API {
    end_point: String,
    pub update_reciever: Option<Receiver<Update>>,
    pub update_sender: Option<Sender<Update>>
}

impl API {
    pub fn new(bot_token:String) -> Self {
        let (update_sender,update_reciever) = channel();

        API{
            end_point: String::from("https://api.telegram.org/bot") + &bot_token,
            update_reciever: Some(update_reciever),
            update_sender: Some(update_sender),
        }
    } 

    pub fn connect(&self) -> bool {
        let mut method = self.end_point.clone();
        method.push_str("/getme");       
        match Request::new(&method){
            Ok(mut requests_obj) => {
                match requests_obj.get().send() {
                    Ok(data) =>{
                        let data:Value = serde_json::from_str(&data.text()).unwrap();
                        if data["ok"] == true {
                            self.get_updates();
                            true
                        } else{
                            log!("**[TGConnector] Error Invalid token is passed");
                            false
                        }
                    },
                    Err(err) =>{
                        log!("**[TGConnector] Error sending request to telegram api \n {:?}",err);
                        false
                    }
                }
            },
            Err(err) =>{
                log!("**[TGConnector] Error building request to telegram api \n {:?}",err);
                false
            }
        }
    }
    
    fn get_updates(&self) {
        let mut method = self.end_point.clone();
        let mut offset = 0;
        let update_move = self.update_sender.clone();

        method.push_str("/getUpdates?offset=");
        
        std::thread::spawn(move|| {
            loop {
                let mut url = method.clone();
                url.push_str(&(offset+1).to_string());
                
                match Request::new(&url) {
                    Ok(mut requests_obj) => {
                        match requests_obj.get().send() {
                            Ok(data) => {
                                match serde_json::from_str(&data.text()){
                                    Ok(update) =>{
                                        let update:Update = update;
                                        let check_result = update.result.last();

                                        match check_result{
                                            Some(result) => {
                                                offset = result.update_id;
                                                update_move.as_ref().unwrap().send(update.clone()).unwrap();
                                            }

                                            None =>{
                                            }
                                        }
                                        
                                    },

                                    Err(_) =>{
                                        continue;
                                    }
                                };
                            },

                            Err(_) => {
                                //NOTE: todo
                                continue;
                            }
                        }
                    },
                    
                    Err(_) => {
                        //NOTE: todo
                        continue;
                    }
                }
            }
        });
    }
   
}

