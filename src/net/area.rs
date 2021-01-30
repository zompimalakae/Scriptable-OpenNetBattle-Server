use super::map::Map;

pub struct Area {
  id: String,
  map: Map,
  // cache
  connected_players: Vec<String>,
  connected_bots: Vec<String>,
}

impl Area {
  pub fn new(map: Map) -> Area {
    Area {
      id: map.get_name().clone(),
      map,
      connected_players: Vec::new(),
      connected_bots: Vec::new(),
    }
  }

  pub fn get_id(&self) -> &String {
    &self.id
  }

  pub fn get_map(&mut self) -> &mut Map {
    &mut self.map
  }

  pub fn get_connected_players(&self) -> &Vec<String> {
    &self.connected_players
  }

  pub(super) fn add_player(&mut self, player_id: String) {
    self.connected_players.push(player_id);
  }

  pub(super) fn remove_player(&mut self, player_id: &String) {
    self
      .connected_players
      .iter()
      .position(|id| id == player_id)
      .map(|position| self.connected_players.remove(position));
  }

  pub fn get_connected_bots(&self) -> &Vec<String> {
    &self.connected_bots
  }

  pub(super) fn add_bot(&mut self, bot_id: String) {
    self.connected_bots.push(bot_id);
  }

  pub(super) fn remove_bot(&mut self, bot_id: &String) {
    self
      .connected_bots
      .iter()
      .position(|id| id == bot_id)
      .map(|position| self.connected_bots.remove(position));
  }
}
