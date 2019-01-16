use std::sync::mpsc::{Sender,Receiver,channel};
use types::*;
use http::make_request;

pub struct BOT {
    pub api_requset_link: String,
    pub update_reciever: Option<Receiver<APIResponse>>,
    pub update_sender: Option<Sender<APIResponse>>
}

impl BOT {
    pub fn new(bot_token:String) -> Self {
        let (update_sender,update_reciever) = channel();
        BOT {
            api_requset_link: String::from("https://api.telegram.org/bot") + &bot_token,
            update_reciever: Some(update_reciever),
            update_sender: Some(update_sender),
        }
    } 

    pub fn connect(&self) -> bool {       
        match make_request(self.api_requset_link.clone(),"getme",None) {
            Ok(response) => {
                if response.ok{
                    self.get_updates();
                    true
                }else {
                    log!("**[TGConnector] Error Invalid token is passed");
                    false
                }
            }
            Err(err) => {
                log!("{:?}",err);
                false
            }
        }
    }
    
    fn get_updates(&self) {
        let mut offset = -2;
        let update_move = self.update_sender.clone();
        let api_link = self.api_requset_link.clone();
        std::thread::spawn(move|| {
            loop {
                let params = Some("offset=".to_string()+&(offset+1).to_string());
                let api_link = api_link.clone();
                match make_request(api_link,"getUpdates",params) {
                    Ok(update) => {
                        let check_result = update.result.clone();
                        //let check_result = check_result.unwrap();
                        let check_result = match check_result{
                            None => {
                                continue;
                            }
                            Some(check_result) => {
                                check_result
                            }
                        };
                        let check_result = check_result.last();
                        match check_result {
                            Some(result) => {
                                offset = result.update_id;
                                update_move.as_ref().unwrap().send(update.clone()).unwrap();
                            }
                            None => {
                                continue;
                            }
                        }
                    }
                    Err(err) => {
                        log!("{:?}",err);
                        continue;                       
                    }
                }
            }
        });
    }

    /*NOTE:TODO
    fn send_message(&self,id:String,text:String) {
        
    }*/
}

