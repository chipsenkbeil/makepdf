# makepdf

Build PDF documents via [Luau](https://luau.org/) scripts, designed to make
creating planners for e-ink devices.

## Usage

```sh
# Make a PDF using default script path of `makepdf.lua`
makepdf make

# Make a PDF using a specific script path of `examples/planner.lua`
makepdf make --script examples/planner.lua

# Make a planner for specific device dimensions
makepdf make --dimensions 1404x1879px
```

## Lua definitions

To get types available when writing your Lua script, check out the
[definitions.lua](assets/scripts/definitions.lua) and
[stdlib.lua](assets/scripts/stdlib.lua) files.

You can set up a `.luarc.json` file to point to the `assets/` directory or copy
those files to a different directory local to yourself.

```json
{
  "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
  "workspace.library": ["assets/scripts"],
  "runtime.version": "Lua 5.1",
}
```

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or
[apache-license][apache-license]) MIT license (LICENSE-MIT or
[mit-license][mit-license]) at your option.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
[mit-license]: http://opensource.org/licenses/MIT
