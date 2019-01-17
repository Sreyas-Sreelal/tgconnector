#[derive(Serialize,Debug,Clone)]
pub struct GetUpdates {
	pub offset: i32,
}

#[derive(Serialize,Debug,Clone)]
pub struct SendMessage {
	pub chat_id: String,
	pub text: String,
	pub reply_to_message_id: Option<i32>,
	pub parse_mode: Option<&'static str>,	
}
