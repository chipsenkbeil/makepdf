use crate::constants::SCRIPTS;
use anyhow::Context;
use mlua::{FromLua, IntoLua, Lua};

/// Represents a script that can be executed to generate a PDF.
pub struct Script {
    /// Lua engine that is used to execute the code
    pub lua: Lua,

    /// Code loaded as raw bytes
    bytes: Vec<u8>,
}

impl Script {
    /// Prefix used for internal script access.
    const PREFIX: &'static str = "makepdf:";

    /// Loads a script from a file (or internally) to be executed. The act of loading the script
    /// does not even parse the code, only loading it into memory.
    pub fn load(script: impl AsRef<str>) -> anyhow::Result<Self> {
        let script = script.as_ref();

        // Load our script either from our internal map or an external file
        let bytes = match script
            .strip_prefix(Self::PREFIX)
            .and_then(|s| SCRIPTS.get(s))
        {
            Some(bytes) => bytes.to_vec(),
            None => std::fs::read(script)
                .with_context(|| format!("Failed to load script '{}'", script))?,
        };

        // Create a new Lua instance in sandbox mode (should not fail with Luau)
        let lua = Lua::new();
        lua.sandbox(true)
            .context("Failed to set sandbox mode on Lua runtime")?;

        Ok(Self { lua, bytes })
    }

    /// Executes the script. This will eagerly parse and execute the code.
    pub fn exec(&self) -> anyhow::Result<()> {
        self.lua
            .load(&self.bytes)
            .exec()
            .context("Failed to execute script")
    }

    /// Sets a global within the script. The global's lifetime is tied to the script itself.
    pub fn set_global<'a, T: IntoLua<'a>>(
        &'a mut self,
        name: impl AsRef<str>,
        value: T,
    ) -> anyhow::Result<()> {
        self.lua
            .globals()
            .raw_set(name.as_ref(), value)
            .with_context(|| format!("Failed to set '{}'", name.as_ref()))
    }

    /// Retrieves a global from the script. The global's lifetime is tied to the script itself.
    pub fn get_global<'a, T: FromLua<'a>>(&'a self, name: impl AsRef<str>) -> anyhow::Result<T> {
        self.lua
            .globals()
            .raw_get(name.as_ref())
            .with_context(|| format!("Failed to retrieve '{}'", name.as_ref()))
    }
}
