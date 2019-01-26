use samp_sdk::amx::AMX;

macro_rules! execute {
    ($amx_list:ident,$name:tt,$botid:ident;$($args:tt)*) => {
        let mut executed: bool = false;
        for amx in $amx_list {
            let amx = cast_amx!(amx);
            let botid: usize = *$botid;
            match exec_callback!(amx,$name;botid,$($args)*) {
                Ok(_) => {
                    executed = true;
                }
                Err(_err) => {
                    continue;
                }
            }
        }
        if !executed {
            log!("**[TGConnector] Error executing callback {}",$name);
        }
    };
}

pub fn on_tg_message(amx_list: &Vec<usize>, botid: &usize, fromid: i32, message_id: i32) {
    execute!(amx_list,"OnTGMessage",botid;fromid,message_id);
}

pub fn on_tg_send_message(amx_list: &Vec<usize>, name: String, botid: &usize, message_id: i32) {
    execute!(amx_list,name,botid;message_id);
}

pub fn on_tg_channel_post(amx_list: &Vec<usize>, botid: &usize, message_id: i32) {
    execute!(amx_list,"OnTGChannelPost",botid;message_id);
}

pub fn on_tg_user_joined(amx_list: &Vec<usize>, botid: &usize, userid: i32) {
    execute!(amx_list,"OnTGUserJoined",botid;userid);
}

pub fn on_tg_user_left(amx_list: &Vec<usize>, botid: &usize, userid: i32) {
    execute!(amx_list,"OnTGUserLeft",botid;userid);
}
