use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInfo {
  pub uuid: String,
  pub usid: String,
}
