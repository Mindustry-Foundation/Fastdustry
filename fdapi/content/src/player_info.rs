use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInfo {
  uuid: String,
  usid: String,
}
