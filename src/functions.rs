#[derive(Serialize,Debug,Clone)]
pub struct GetUpdates {
	pub offset: i32,
}

#[derive(Serialize,Debug,Clone)]
pub struct SendMessage {
	pub chat_id: String,
	pub text: String,
}
