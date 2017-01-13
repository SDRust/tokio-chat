#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub user: String,
    pub msg: String
}