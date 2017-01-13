#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub nickname: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandResponse {
    pub status: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub from: String,
    pub message: String,
    pub time: String
}
