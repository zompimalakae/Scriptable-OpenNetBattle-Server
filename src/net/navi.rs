use super::Direction;

pub struct Navi {
  pub id: String,
  pub name: String,
  pub area_id: String,
  pub texture_path: String,
  pub animation_path: String,
  pub direction: Direction,
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub solid: bool,
}
