use mlua::Lua;
use std::fs;

/// 加载并执行 Lua 脚本
pub fn load_lua_script(path: &str) -> Result<Lua, mlua::Error> {
    let lua = Lua::new();
    let script = fs::read_to_string(path).expect("无法读取Lua脚本");
    lua.load(&script).exec()?;
    Ok(lua)
}
