use samp_sdk::amx::AmxResult;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use telegram::BOT;

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

    pub fn bot_send_message(&mut self,_amx:&AMX,botid:usize,chatid:String,text:String) -> AmxResult<Cell> {
        if !self.bots.contains_key(&botid) {
            log!("**[TGConnector] Error Invalid bot id {} passed",botid);
            Ok(0)
        }else {
            self.bots[&botid].send_message(chatid,text);
            Ok(1)
        }
    }

    pub fn get_message(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
        let string = match self.telegram_messages.front() {
            Some(string) => string.to_string(),
            None => String::from("")
        };
        
        let string = samp_sdk::cp1251::encode(&string).unwrap();
        set_string!(string,dest,size);
        Ok(1)
    }

    pub fn get_username(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
        let string = match self.telegram_username.front(){
            Some(string) => string.to_string(),
            None => String::from("")
        };
        let string = samp_sdk::cp1251::encode(&string).unwrap();
        set_string!(string,dest,size);
        Ok(1)
    }

    pub fn get_chatid(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
        let string = match self.telegram_chatid.front(){
            Some(string) => string.to_string(),
            None => String::from("")
        };

        let string = samp_sdk::cp1251::encode(&string).unwrap();
        set_string!(string,dest,size);
        Ok(1)
    }

    pub fn get_chatname(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
        let string = match self.telegram_chatname.front(){
            Some(string) => string.to_string(),
            None => String::from("")
        };
        let string = samp_sdk::cp1251::encode(&string).unwrap();
        set_string!(string,dest,size);
        Ok(1)
    }

    pub fn get_chattype(&mut self,_amx:&AMX,dest:&mut Cell,size:usize) -> AmxResult<Cell> {
        let string = match self.telegram_chattype.front(){
            Some(string) => string.to_string(),
            None => String::from("")
        };
        let string = samp_sdk::cp1251::encode(&string).unwrap();
        set_string!(string,dest,size);
        Ok(1)
    }
}