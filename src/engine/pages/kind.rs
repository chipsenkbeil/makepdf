use mlua::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EnginePageKind {
    Daily,
    Monthly,
    Weekly,
}

impl<'lua> IntoLua<'lua> for EnginePageKind {
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
