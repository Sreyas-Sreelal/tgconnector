use samp_sdk::amx::AmxResult;
use samp_sdk::types::Cell;
use samp_sdk::amx::AMX;
use telegram::BOT;

impl super::TgConnector{
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
        }else{
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
}