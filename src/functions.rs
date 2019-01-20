#[derive(Serialize,Debug,Clone)]
pub struct GetUpdates {
	pub offset: i32,
}

#[derive(Serialize,Debug,Clone)]
pub struct SendMessage {
	pub chat_id: String,
	pub text: String,
	pub reply_to_message_id: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parse_mode: Option<&'static str>,	
}

#[derive(Serialize,Debug,Clone)]
pub struct DeleteMessage {
	pub chat_id: String,
	pub message_id: i32,
}

#[derive(Serialize,Debug,Clone)]
pub struct EditMessageText {
	pub chat_id: String,
	pub message_id: i32,
	pub text: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parse_mode: Option<&'static str>,
}

#[derive(Serialize,Debug,Clone)]
pub struct GetChat {
	pub chat_id: String,
}

#[derive(Serialize,Debug,Clone)]
pub struct GetChatMember {
	pub chat_id: String,
	pub user_id: i32,
}

#[derive(Serialize,Debug,Clone)]
pub struct GetChatMembersCount {
	pub chat_id: String,
}

