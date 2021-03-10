use super::lua_errors::create_area_error;
use crate::net::map::{MapObject, MapObjectData, Tile};
use crate::net::Net;
use std::cell::RefCell;

#[allow(clippy::type_complexity)]
pub fn add_area_api<'a, 'b>(
  api_table: &rlua::Table<'a>,
  scope: &rlua::Scope<'a, 'b>,
  net_ref: &'b RefCell<&mut Net>,
) -> rlua::Result<()> {
  api_table.set(
    "get_width",
    scope.create_function(move |_, area_id: String| {
      let mut net = net_ref.borrow_mut();

      if let Some(area) = net.get_area_mut(&area_id) {
        Ok(area.get_map_mut().get_width())
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_height",
    scope.create_function(move |_, area_id: String| {
      let mut net = net_ref.borrow_mut();

      if let Some(area) = net.get_area_mut(&area_id) {
        Ok(area.get_map_mut().get_height())
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_area_name",
    scope.create_function(move |_, area_id: String| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        Ok(area.get_map().get_name().clone())
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "set_area_name",
    scope.create_function(move |_, (area_id, name): (String, String)| {
      let mut net = net_ref.borrow_mut();

      if let Some(area) = net.get_area_mut(&area_id) {
        let map = area.get_map_mut();

        map.set_name(name);

        Ok(())
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_song",
    scope.create_function(move |_, area_id: String| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        Ok(area.get_map().get_song_path().clone())
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "set_song",
    scope.create_function(move |_, (area_id, path): (String, String)| {
      let mut net = net_ref.borrow_mut();

      if let Some(area) = net.get_area_mut(&area_id) {
        let map = area.get_map_mut();

        map.set_song_path(path);

        Ok(())
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_background_name",
    scope.create_function(move |_, area_id: String| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        Ok(area.get_map().get_background_name().clone())
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "set_background",
    scope.create_function(move |_, (area_id, name): (String, String)| {
      let mut net = net_ref.borrow_mut();

      if let Some(area) = net.get_area_mut(&area_id) {
        let map = area.get_map_mut();

        map.set_background_name(name);

        Ok(())
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_spawn_position",
    scope.create_function(move |lua_ctx, area_id: String| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        let (x, y, z) = area.get_map().get_spawn();

        let table = lua_ctx.create_table()?;
        table.set("x", x)?;
        table.set("y", y)?;
        table.set("z", z)?;

        Ok(table)
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "set_spawn_position",
    scope.create_function(move |_, (area_id, x, y, z): (String, f32, f32, f32)| {
      let mut net = net_ref.borrow_mut();

      if let Some(area) = net.get_area_mut(&area_id) {
        let map = area.get_map_mut();

        map.set_spawn(x, y, z);

        Ok(())
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "list_tilesets",
    scope.create_function(move |_, area_id: String| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        let map = area.get_map();
        let tilesets = map.get_tilesets();
        let tileset_paths: Vec<String> = tilesets
          .iter()
          .map(|tileset| tileset.path.clone())
          .collect();

        Ok(tileset_paths)
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_tileset",
    scope.create_function(move |lua_ctx, (area_id, path): (String, String)| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        let map = area.get_map();
        let tilesets = map.get_tilesets();
        let optional_tileset = tilesets.iter().find(|tileset| tileset.path == path);

        if let Some(tileset) = optional_tileset {
          let table = lua_ctx.create_table()?;
          table.set("path", tileset.path.clone())?;
          table.set("firstGid", tileset.first_gid)?;

          return Ok(Some(table));
        }

        Ok(None)
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_tileset_for_tile",
    scope.create_function(move |lua_ctx, (area_id, tile_gid): (String, u32)| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        let map = area.get_map();
        let tilesets = map.get_tilesets();
        let optional_tileset = tilesets
          .iter()
          .take_while(|tileset| tileset.first_gid <= tile_gid)
          .last();

        if let Some(tileset) = optional_tileset {
          let table = lua_ctx.create_table()?;
          table.set("path", tileset.path.clone())?;
          table.set("firstGid", tileset.first_gid)?;

          return Ok(Some(table));
        }

        Ok(None)
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_tile",
    scope.create_function(
      move |lua_ctx, (area_id, x, y, z): (String, usize, usize, usize)| {
        let net = net_ref.borrow();

        if let Some(area) = net.get_area(&area_id) {
          let tile = area.get_map().get_tile(x, y, z);

          let table = lua_ctx.create_table()?;

          table.set("gid", tile.gid)?;

          if tile.flipped_anti_diagonally {
            table.set("flippedHorizontal", tile.flipped_vertically)?;
            table.set("flippedVertical", !tile.flipped_horizontally)?;
          } else {
            table.set("flippedHorizontal", tile.flipped_horizontally)?;
            table.set("flippedVertical", tile.flipped_vertically)?;
          }

          table.set("rotated", tile.flipped_anti_diagonally)?;

          Ok(table)
        } else {
          Err(create_area_error(&area_id))
        }
      },
    )?,
  )?;

  api_table.set(
    "set_tile",
    scope.create_function(
      move |_,
            (area_id, x, y, z, gid, flip_horizontal, flip_vertical, rotate): (
        String,
        usize,
        usize,
        usize,
        u32,
        Option<bool>,
        Option<bool>,
        Option<bool>,
      )| {
        let mut net = net_ref.borrow_mut();

        if let Some(area) = net.get_area_mut(&area_id) {
          let tile = Tile {
            gid,
            flipped_horizontally: flip_horizontal.unwrap_or(false),
            flipped_vertically: flip_vertical.unwrap_or(false),
            flipped_anti_diagonally: rotate.unwrap_or(false),
          };

          area.get_map_mut().set_tile(x, y, z, tile);
          Ok(())
        } else {
          Err(create_area_error(&area_id))
        }
      },
    )?,
  )?;

  api_table.set(
    "list_objects",
    scope.create_function(move |_, area_id: String| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        let result: Vec<String> = area
          .get_map()
          .get_objects()
          .iter()
          .map(|object| object.name.clone())
          .collect();

        Ok(result)
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_object_by_id",
    scope.create_function(move |lua_ctx, (area_id, id): (String, u32)| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        let optional_object = area.get_map().get_object_by_id(id);

        Ok(map_optional_object_to_table(&lua_ctx, optional_object))
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  api_table.set(
    "get_object_by_name",
    scope.create_function(move |lua_ctx, (area_id, name): (String, String)| {
      let net = net_ref.borrow();

      if let Some(area) = net.get_area(&area_id) {
        let optional_object = area.get_map().get_object_by_name(&name);

        Ok(map_optional_object_to_table(&lua_ctx, optional_object))
      } else {
        Err(create_area_error(&area_id))
      }
    })?,
  )?;

  Ok(())
}

fn map_optional_object_to_table<'a>(
  lua_ctx: &rlua::Context<'a>,
  optional_object: Option<&MapObject>,
) -> Option<rlua::Table<'a>> {
  let table = lua_ctx.create_table().ok()?;

  let object = optional_object?;

  table.set("id", object.id).ok()?;
  table.set("name", object.name.clone()).ok()?;
  table.set("type", object.object_type.clone()).ok()?;
  table.set("visible", object.visible).ok()?;
  table.set("x", object.x).ok()?;
  table.set("y", object.y).ok()?;
  table.set("z", object.z).ok()?;
  table.set("width", object.width).ok()?;
  table.set("height", object.height).ok()?;
  table.set("rotation", object.rotation).ok()?;

  let data_table = lua_ctx.create_table().ok()?;

  match &object.data {
    MapObjectData::Polygon { points } => {
      let points_table = lua_ctx.create_table().ok()?;

      // lua lists start at 1
      let mut i = 1;

      for point in points {
        let point_table = lua_ctx.create_table().ok()?;
        point_table.set("x", point.0).ok()?;
        point_table.set("y", point.1).ok()?;

        points_table.set(i, point_table).ok()?;
        i += 1;
      }

      data_table.set("points", points_table).ok()?;
      Some(())
    }
    MapObjectData::TileObject { tile } => {
      data_table.set("gid", tile.gid).ok()?;
      data_table
        .set("flippedHorizontally", tile.flipped_horizontally)
        .ok()?;
      data_table
        .set("flippedVertically", tile.flipped_vertically)
        .ok()?;
      data_table.set("rotated", false).ok()?;
      Some(())
    }
    _ => Some(()),
  }?;

  table.set("data", data_table).ok()?;

  // todo: properties

  Some(table)
}
