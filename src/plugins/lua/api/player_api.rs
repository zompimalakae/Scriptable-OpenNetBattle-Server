use super::lua_errors::{create_area_error, create_player_error};
use crate::net::Net;
use std::cell::RefCell;

pub fn add_player_api<'a, 'b>(
  api_table: &rlua::Table<'a>,
  scope: &rlua::Scope<'a, 'b>,
  net_ref: &'b RefCell<&mut Net>,
) -> rlua::Result<()> {
  api_table.set(
    "list_players",
    scope.create_function(move |_, area_id: String| {
      let mut net = net_ref.borrow_mut();

      if let Some(area) = net.get_area_mut(&area_id) {
        let connected_players = area.get_connected_players();
        let result: Vec<String> = connected_players.to_vec();

        Ok(result)
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "is_player",
    scope.create_function(move |_, id: String| {
      let net = net_ref.borrow();

      Ok(net.get_player(&id).is_some())
    })?,
  )?;

  api_table.set(
    "get_player_area",
    scope.create_function(move |_, id: String| {
      let net = net_ref.borrow_mut();

      if let Some(player) = net.get_player(&id) {
        Ok(player.area_id.clone())
      } else {
        Err(create_player_error(&id))
      }
    })?,
  )?;

  api_table.set(
    "get_player_name",
    scope.create_function(move |_, id: String| {
      let net = net_ref.borrow_mut();

      if let Some(player) = net.get_player(&id) {
        Ok(player.name.clone())
      } else {
        Err(create_player_error(&id))
      }
    })?,
  )?;

  api_table.set(
    "set_player_name",
    scope.create_function(move |_, (id, name): (String, String)| {
      let mut net = net_ref.borrow_mut();

      net.set_player_name(&id, name);

      Ok(())
    })?,
  )?;

  api_table.set(
    "get_player_position",
    scope.create_function(move |lua_ctx, id: String| {
      let net = net_ref.borrow();

      if let Some(player) = net.get_player(&id) {
        let table = lua_ctx.create_table()?;
        table.set("x", player.x)?;
        table.set("y", player.y)?;
        table.set("z", player.z)?;

        Ok(table)
      } else {
        Err(create_player_error(&id))
      }
    })?,
  )?;

  api_table.set(
    "get_player_avatar",
    scope.create_function(move |_, id: String| {
      let net = net_ref.borrow_mut();

      if let Some(player) = net.get_player(&id) {
        Ok(vec![
          player.texture_path.clone(),
          player.animation_path.clone(),
        ])
      } else {
        Err(create_player_error(&id))
      }
    })?,
  )?;

  api_table.set(
    "set_player_avatar",
    scope.create_function(
      move |_, (id, texture_path, animation_path): (String, String, String)| {
        let mut net = net_ref.borrow_mut();

        net.set_player_avatar(&id, texture_path, animation_path);

        Ok(())
      },
    )?,
  )?;

  api_table.set(
    "lock_player_input",
    scope.create_function(move |_, id: String| {
      let mut net = net_ref.borrow_mut();

      net.lock_player_input(&id);

      Ok(())
    })?,
  )?;

  api_table.set(
    "unlock_player_input",
    scope.create_function(move |_, id: String| {
      let mut net = net_ref.borrow_mut();

      net.unlock_player_input(&id);

      Ok(())
    })?,
  )?;

  api_table.set(
    "move_player",
    scope.create_function(move |_, (id, x, y, z): (String, f32, f32, f32)| {
      let mut net = net_ref.borrow_mut();

      net.move_player(&id, x, y, z);

      Ok(())
    })?,
  )?;

  api_table.set(
    "message_player",
    scope.create_function(
      move |_,
            (id, message, mug_texture_path, mug_animation_path): (
        String,
        String,
        Option<String>,
        Option<String>,
      )| {
        let mut net = net_ref.borrow_mut();

        net.message_player(
          &id,
          &message,
          &mug_texture_path.unwrap_or_default(),
          &mug_animation_path.unwrap_or_default(),
        );

        Ok(())
      },
    )?,
  )?;

  api_table.set(
    "question_player",
    scope.create_function(
      move |_,
            (id, message, mug_texture_path, mug_animation_path): (
        String,
        String,
        Option<String>,
        Option<String>,
      )| {
        let mut net = net_ref.borrow_mut();

        net.question_player(
          &id,
          &message,
          &mug_texture_path.unwrap_or_default(),
          &mug_animation_path.unwrap_or_default(),
        );

        Ok(())
      },
    )?,
  )?;

  api_table.set(
    "transfer_player",
    scope.create_function(
      move |_,
            (id, area_id, warp_in_option, x_option, y_option, z_option): (
        String,
        String,
        Option<bool>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
      )| {
        let mut net = net_ref.borrow_mut();
        let warp_in = warp_in_option.unwrap_or(true);
        let x;
        let y;
        let z;

        if let Some(player) = net.get_player(&id) {
          x = x_option.unwrap_or(player.x);
          y = y_option.unwrap_or(player.y);
          z = z_option.unwrap_or(player.z);
        } else {
          return Err(create_player_error(&id));
        }

        net.transfer_player(&id, &area_id, warp_in, x, y, z);

        Ok(())
      },
    )?,
  )?;

  Ok(())
}
