use samp_sdk::amx::AmxResult;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use api::BOT;
use encode::encode_replace;
use functions::*;

impl super::TgConnector {
	pub fn bot_connect(&mut self,_amx:&AMX,token:String) -> AmxResult<Cell> {
        let api = BOT::new(token);
        if api.connect() {
            self.bots.insert(self.bot_context_id,api);
            self.bot_context_id += 1;
            Ok(self.bot_context_id as Cell -1)
        }else {
            Ok(-1)
        }
    }

    pub fn bot_connect_from_env(&mut self,_amx:&AMX,variable:String) -> AmxResult<Cell> {
        let token = std::env::var_os(&variable);
        if token == None {
            log!("**[TGConnector] Error environment variable {:?} is not set",variable);
            Ok(-1)
        }else {
            let token = token.unwrap().into_string().unwrap();
            let api = BOT::new(token);

            if api.connect() {
                self.bots.insert(self.bot_context_id,api);
                self.bot_context_id += 1;
                Ok(self.bot_context_id as Cell -1)
            }else {
                Ok(-1)
            }
        }
    }

    pub fn bot_send_message(&mut self,_amx:&AMX,botid:usize,chatid:String,text:String,reply_id:i32,parse_mode:i32) -> AmxResult<Cell> {
        let reply: Option<i32>;
        if reply_id == -1 {
            reply = None;
        }else {
            reply = Some(reply_id);
        }

        let parsemode:Option<&str> = match parse_mode {
            0 => Some("HTML"),
            1 => Some("markdown"),
            _ => None
        };
        
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed",botid);
            Ok(0)
        }else {
            let send_message_obj = SendMessage {
				chat_id: chatid,
				text: text,
				reply_to_message_id: reply,
				parse_mode: parsemode
		    };
            self.bots[&botid].send_message(send_message_obj);
            Ok(1)
        }
    }

    pub fn bot_delete_message(&mut self,_amx:&AMX,botid:usize,chatid:String,messageid:i32) -> AmxResult<Cell> {
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed",botid);
            Ok(0)
        }else {
            let delete_message_obj = DeleteMessage {
				chat_id: chatid,
				message_id: messageid,
		    };
            self.bots[&botid].delete_message(delete_message_obj);
            Ok(1)
        }
    }
    
    pub fn get_message(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
        let string = self.telegram_messages.front();

        if string != None {
            match encode_replace(&string.unwrap()) {
                Ok(encoded) => {
                    set_string!(encoded,dest,size);
                    Ok(1)
                },
                Err(err) => {
                    log!("**[TGConnector][get_message] Failed encoding {:?} \n {:?}",string.unwrap(),err);
                    Ok(0)
                }   
            }
        }else {
             
            Ok(0)
        }
    }

    pub fn get_username(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
        let string = self.telegram_username.front();
        
        if string != None {
            match encode_replace(&string.unwrap()) {
                Ok(encoded) => {
                    set_string!(encoded,dest,size);
                    Ok(1)
                },
                Err(err) => {
                    log!("**[TGConnector][get_username] Failed encoding {:?} \n {:?}",string.unwrap(),err);
                    Ok(0)
                }   
            }
        }else {   
            Ok(0)
        }
    }
    

    pub fn get_chatid(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
        let string = self.telegram_chatid.front();

        if string != None {
            match encode_replace(&string.unwrap()) {
                Ok(encoded) => {
                    set_string!(encoded,dest,size);
                    Ok(1)
                },
                Err(err) => {
                    log!("**[TGConnector][get_chatid] Failed encoding {:?} \n {:?}",string.unwrap(),err);
                    Ok(0)
                }   
            }
        }else {   
            Ok(0)
        }
    }

    pub fn get_chatname(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
       let string =  self.telegram_chatname.front();
       
       if string != None {
            match encode_replace(&string.unwrap()) {
                Ok(encoded) => {
                    set_string!(encoded,dest,size);
                    Ok(1)
                },
                Err(err) => {
                    log!("**[TGConnector][get_chatname] Failed encoding {:?} \n {:?}",string.unwrap(),err);
                    Ok(0)
                }   
            }
        }else {   
            Ok(0)
        }
    }

    pub fn get_chattype(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
        let string = self.telegram_chattype.front();
        
        if string != None {
            match encode_replace(&string.unwrap()) {
                Ok(encoded) => {
                    set_string!(encoded,dest,size);
                    Ok(1)
                },
                Err(err) => {
                    log!("**[TGConnector][get_chattype] Failed encoding {:?} \n {:?}",string.unwrap(),err);
                    Ok(0)
                }   
            }
        }else {   
            Ok(0)
        }
    }
}