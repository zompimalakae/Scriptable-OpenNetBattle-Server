use crate::net::Net;

pub trait PluginInterface {
  fn init(&mut self, net: &mut Net);
  fn tick(&mut self, net: &mut Net, delta_time: f32);
  fn handle_player_connect(&mut self, net: &mut Net, player_id: &String);
  fn handle_player_disconnect(&mut self, net: &mut Net, player_id: &String);
  fn handle_player_move(&mut self, net: &mut Net, player_id: &String, x: f32, y: f32, z: f32); // todo: add a bool return value to prevent default?
  fn handle_player_avatar_change(&mut self, net: &mut Net, player_id: &String, avatar_id: u16);
  fn handle_player_emote(&mut self, net: &mut Net, player_id: &String, emote_id: u8);
}
