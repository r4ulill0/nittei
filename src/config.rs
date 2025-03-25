use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result;
use std::error::Error;
use xdg::BaseDirectories;
use mlua::{ErrorContext, FromLua, IntoLua, Lua, Table};
use mlua::prelude::{LuaResult, LuaValue, LuaError};
use ConfigError::*;

pub struct NitteiConfig {
    pub remove_completed: bool,
}

impl<'lua> FromLua<'lua> for NitteiConfig {

    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let t: &Table = match value.as_table() {
            Some(t) =>  t,
            None =>  return Err(LuaError::FromLuaConversionError{
                from: "Table",
                to: "NitteiConfig",
                message: Some(format!("could not read nittei config as table, it was a {}", value.type_name()))
                })
        };

        Ok(NitteiConfig {
            remove_completed:t.get("remove_completed")?,
        }  )          
    }
}

impl<'lua> IntoLua<'lua> for NitteiConfig {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let config_table: mlua::Table<'lua> = match lua.create_table() {
            Ok(t) => t,
            Err(e) => return Err(e.context(format!("could not create table for NitteiConfig"))),
        };
        match config_table.set("remove_completed", self.remove_completed){
            Err(e) => print!("error {}", e),
            _ => print!("todo ok"),
        };
        return Ok(LuaValue::Table(config_table))
    }
}

impl Default for NitteiConfig {
    fn default() -> NitteiConfig {
        NitteiConfig {
            remove_completed: false,
        }
    }
}

impl NitteiConfig {


    pub fn load_config(lua: &Lua) -> Result<NitteiConfig, ConfigError> {
        let xdg_dirs = match BaseDirectories::with_prefix("nittei") {
            Ok(dirs) => dirs,
            Err(e) => return Err(PathError(e.to_string())),
        };

        let config_path = match xdg_dirs.find_config_file("config.lua") {
            Some(path) => path,
            None => return Err(PathError(format!("did not find any configuration files"))),
        };

        let _ = lua.create_table().and_then(|table| {
            lua.globals().set("config", table)
        }).map_err(|e| LuaWriteError(e.to_string()));


        match lua.load(config_path.clone()).exec() {
            Ok(_) => (),
            Err(e) => return Err(PathError(e.to_string())),
        };
        let config: NitteiConfig = match lua.globals().get("config") {
            Ok(c) => c,
            Err(_) => return Err(LuaReadError(format!("could not read nittei config from lua context"))),
        };

        return Ok(config);
    }
}

#[derive(Debug)]
pub enum ConfigError {
    PathError(String),
    LuaReadError(String),
    LuaWriteError(String),
}

impl Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PathError(e) => write!(f, "path error: {}", e),
            LuaReadError(e) => write!(f, "error reading lua: {}", e),
            LuaWriteError(e) => write!(f, "error writing lua: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn lua_load() {
        let conf_path = format!("{}/resources/tests/", env!("CARGO_MANIFEST_DIR"));
        std::env::set_var("XDG_CONFIG_HOME", conf_path);
        let lua = Lua::new();
        let config = NitteiConfig::load_config(&lua).unwrap();
        let config_opt: bool = lua.globals().get("remove_completed").unwrap();
        assert_eq!(config.remove_completed, true, "Config generating a wrong default value: {} expected {}", config_opt, true);
    }
}
