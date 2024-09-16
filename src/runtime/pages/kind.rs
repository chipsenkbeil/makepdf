use mlua::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RuntimePageKind {
    Daily,
    Monthly,
    Weekly,
}

impl<'lua> IntoLua<'lua> for RuntimePageKind {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(match self {
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::Weekly => "weekly",
        })
        .map(LuaValue::String)
    }
}
