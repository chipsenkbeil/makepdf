use crate::constants::SCRIPTS;
use anyhow::Context;
use mlua::{FromLua, IntoLua, Lua};
use std::ops::{Deref, DerefMut};

/// Represents a script that can be executed to generate a PDF.
pub struct RuntimeScript {
    /// Lua runtime that is used to execute the code
    lua: Lua,

    /// Code loaded as raw bytes
    bytes: Vec<u8>,
}

impl RuntimeScript {
    /// Prefix used for internal script access.
    const PREFIX: &'static str = "builtin:";

    /// Loads a script from a file (or internally) to be executed.
    ///
    /// The act of loading the script does not even parse the code, only loading it into memory.
    pub fn load_from_script(script: impl AsRef<str>) -> anyhow::Result<Self> {
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

        Self::load_from_bytes(bytes)
    }

    /// Loads a script for a series of bytes.
    ///
    /// The act of loading the bytes does not even parse the code, only loading it into memory.
    pub fn load_from_bytes(bytes: impl IntoIterator<Item = u8>) -> anyhow::Result<Self> {
        // Create a new Lua instance in sandbox mode (should not fail with Luau)
        let lua = Lua::new();
        lua.sandbox(true)
            .context("Failed to set sandbox mode on Lua runtime")?;

        Ok(Self {
            lua,
            bytes: bytes.into_iter().collect(),
        })
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

impl Deref for RuntimeScript {
    type Target = Lua;

    fn deref(&self) -> &Self::Target {
        &self.lua
    }
}

impl DerefMut for RuntimeScript {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lua
    }
}
