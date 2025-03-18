use xdg::BaseDirectories;
use mlua::{Lua, FromLua, IntoLua, };
use mlua::prelude::{LuaResult, LuaValue};

pub struct NitteiConfig {
    pub remove_completed: bool,
}

impl<'lua> FromLua<'lua> for NitteiConfig {

    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
        print!("value es tabla? {}", value.type_name());
        let x: &mlua::Table = value.as_table().unwrap();
        let b: bool = x.get("remove_completed").unwrap();
        Ok(NitteiConfig {
            remove_completed:b,
        }  )          
    }
}

impl<'lua> IntoLua<'lua> for NitteiConfig {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let config_table: mlua::Table<'lua> = lua.create_table().unwrap();
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

    pub fn load_config(lua: &Lua) -> NitteiConfig {
        let xdg_dirs = BaseDirectories::with_prefix("nittei").unwrap();
        print!("Dir: {:#?}", xdg_dirs);
        let config_path = xdg_dirs
            .place_config_file("config.lua")
            .expect("Cannot create configuration directory");
        let default_config = NitteiConfig{..Default::default()};
        match lua.globals().set("config", default_config) {
            Ok(_o) => print!("todo bien"),
            Err(e) => print!("error al setear {}",e)

        }
        match lua.load(config_path.clone()).exec() {
            Ok(o) => print!("todo bien \n"),
            Err(e) => print!("error al cargar config {:#?}: {}", config_path, e)
        }
        let config: NitteiConfig = lua.globals().get("config")
            .unwrap();

        return config
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
        let config = NitteiConfig::load_config(&lua);
        let config_opt: bool = lua.globals().get("remove_completed").unwrap();
        assert_eq!(config.remove_completed, true, "Config generating a wrong default value: {} expected {}", config_opt, true);
    }
}
