#[derive(Deserialize,Debug,Clone)]
pub struct APIResponse {
    pub ok: bool,
    pub result: Option<Vec<APIResult>>,
}

#[derive(Deserialize,Debug,Clone)]
pub struct APIResult {
    pub message: Message,
    pub update_id: u64,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Message {
    pub text: Option<String>
}