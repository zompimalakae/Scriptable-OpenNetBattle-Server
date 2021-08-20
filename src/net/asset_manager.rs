use super::{Asset, AssetDependency};
use std::collections::HashMap;

pub struct AssetManager {
  assets: HashMap<String, Asset>,
  scripted_character_paths: HashMap<String, String>,
}

impl AssetManager {
  pub fn new() -> AssetManager {
    AssetManager {
      assets: HashMap::new(),
      scripted_character_paths: HashMap::new(),
    }
  }

  pub fn load_assets_from_dir(&mut self, dir: &std::path::Path) {
    use std::fs::read_dir;

    if let Ok(entries) = read_dir(dir) {
      for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
          self.load_assets_from_dir(&path);
        } else {
          let mut path_string = String::from("/server/") + path.to_str().unwrap_or_default();

          // adjust windows paths
          path_string = path_string.replace('\\', "/");

          self.set_asset(path_string, Asset::load_from_file(&path));
        }
      }
    }
  }

  pub fn get_asset(&self, path: &str) -> Option<&Asset> {
    self.assets.get(path)
  }

  pub fn set_asset(&mut self, path: String, asset: Asset) {
    for alternate_name in &asset.alternate_names {
      #[allow(clippy::single_match)]
      match alternate_name {
        AssetDependency::ScriptedCharacter(name) => {
          self
            .scripted_character_paths
            .insert(name.clone(), path.clone());
        }
        _ => {}
      }
    }

    self.assets.insert(path, asset);
  }

  pub fn remove_asset(&mut self, path: &str) {
    let asset = if let Some(asset) = self.assets.remove(path) {
      asset
    } else {
      return;
    };

    for alternate_name in asset.alternate_names {
      #[allow(clippy::single_match)]
      match alternate_name {
        AssetDependency::ScriptedCharacter(name) => {
          let optional_path_str = self
            .scripted_character_paths
            .get(&name)
            .map(|path| path.as_str());

          // make sure another asset did not overwrite us as this name
          if Some(path) == optional_path_str {
            self.scripted_character_paths.remove(&name);
          }
        }
        _ => {}
      }
    }
  }

  pub fn get_flattened_dependency_chain<'a>(&'a self, asset_path: &'a str) -> Vec<&'a str> {
    let mut chain = Vec::new();
    self.build_flattened_dependency_chain_with_recursion(asset_path, &mut chain);
    chain
  }

  fn build_flattened_dependency_chain_with_recursion<'a>(
    &'a self,
    asset_path: &'a str,
    chain: &mut Vec<&'a str>,
  ) {
    if let Some(asset) = self.assets.get(asset_path) {
      for dependency in &asset.dependencies {
        let dependency_path = if let Some(path) = self.resolve_dependency_path(dependency) {
          path
        } else {
          continue;
        };

        if chain.contains(&dependency_path) {
          continue;
        }

        self.build_flattened_dependency_chain_with_recursion(dependency_path, chain);
      }

      chain.push(asset_path);
    }
  }

  fn resolve_dependency_path<'a>(&'a self, dependency: &'a AssetDependency) -> Option<&'a str> {
    match dependency {
      AssetDependency::AssetPath(path) => Some(path),
      AssetDependency::ScriptedCharacter(name) => self
        .scripted_character_paths
        .get(name)
        .map(|path| path.as_str()),
    }
  }
}