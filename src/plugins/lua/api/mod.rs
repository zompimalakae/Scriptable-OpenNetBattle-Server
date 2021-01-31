mod area_api;
mod bot_api;
mod lua_errors;
mod player_api;

use crate::net::Net;
use std::cell::RefCell;

pub fn add_net_api<'a, 'b>(
  api_table: &rlua::Table<'a>,
  scope: &rlua::Scope<'a, 'b>,
  net_ref: &'b RefCell<&mut Net>,
) -> rlua::Result<()> {
  area_api::add_area_api(api_table, scope, net_ref)?;
  player_api::add_player_api(api_table, scope, net_ref)?;
  bot_api::add_bot_api(api_table, scope, net_ref)?;

  Ok(())
}