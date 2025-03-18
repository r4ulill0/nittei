use pest::Parser;
use pest_derive::Parser;
use xdg::BaseDirectories;
use mlua::{Lua, FromLua, IntoLua, };
use mlua::prelude::{LuaResult, LuaValue};

struct NitteiConfig {
    remove_completed: bool,
}

impl<'lua> FromLua<'lua> for NitteiConfig {

    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
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

    fn load_config(lua: &Lua) -> NitteiConfig {
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

#[derive(Parser)]
#[grammar="calendar.pest"]
pub struct CalendarParser;
fn main() {
    let lua = Lua::new();
    let config = NitteiConfig::load_config(&lua);
    let example_entry = "*-7-2 #extend 1w | Bob's birthday";
    let parse_tree = CalendarParser::parse(Rule::calendar, example_entry);
    println!("{:#?}", parse_tree);
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use mlua::Lua;

    use crate::{CalendarParser, NitteiConfig, Rule};
    use lazy_static::lazy_static;
    use pest_test::{PestTester, TestError};


    lazy_static! {
        static ref COLORIZE: bool = {
                    option_env!("CARGO_TERM_COLOR").unwrap_or("always") != "never"
                };
        static ref TESTER: PestTester<Rule, CalendarParser> = 
        // TODO bug in pest_test 0.1.6
        PestTester::new("tests/pest","txt", Rule::calendar, HashSet::new());
        //PestTester::from_defaults(Rule::calendar, HashSet::new());
    }

    macro_rules! pest_tests {
        ($($name: ident), *) => {
            $(
                #[test]
                fn $name() -> Result<(), TestError<Rule>> {
                    let res = (*TESTER).evaluate_strict(stringify!($name));
                    if let Err(pest_test::TestError::Diff {ref diff}) = res {
                        diff.print_test_result(*COLORIZE).unwrap();
                    }
                    res
                }

            )*
        }
    }

    pest_tests! {
        every_day_task,
        event_early_notification,
        small_calendar
    }

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
