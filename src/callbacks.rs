use log::error;
use samp::amx::AmxIdent;
use samp::exec_public;
pub fn on_tg_message(
    amx_list: &[AmxIdent],
    botid: usize,
    fromid: String,
    message_id: i32,
    message_thread_id: i32,
) {
    execute!(amx_list,"OnTGMessage",botid;&fromid => string,message_id,message_thread_id);
}

pub fn on_tg_send_message(amx_list: &[AmxIdent], name: &str, botid: usize, message_id: i32) {
    execute!(amx_list,name,botid;message_id);
}

pub fn on_tg_channel_post(amx_list: &[AmxIdent], botid: usize, message_id: i32) {
    execute!(amx_list,"OnTGChannelPost",botid;message_id);
}

pub fn on_tg_user_joined(amx_list: &[AmxIdent], botid: usize, userid: String) {
    execute!(amx_list,"OnTGUserJoined",botid;&userid => string);
}

pub fn on_tg_user_left(amx_list: &[AmxIdent], botid: usize, userid: String) {
    execute!(amx_list,"OnTGUserLeft",botid;&userid => string);
}
