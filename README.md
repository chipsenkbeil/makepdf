# makepdf

Build PDF documents via [Luau](https://luau.org/) scripts, designed to make
creating planners for e-ink devices.

## Installation

```sh
cargo install makepdf
```

## Usage

```sh
# Make a PDF using default script path of `makepdf.lua`
makepdf make

# Make a PDF using a specific script path of `examples/planner.lua`
makepdf make --script examples/planner.lua

# Make a planner for specific device dimensions
makepdf make --dimensions 1404x1879px
```

## Quickstart Guide

Using `makepdf` involves writing a short [Luau](https://luau.org/) script. If
you're used to using [Lua 5.1](https://www.lua.org/manual/5.1/), this is
backwards compatible, but restricts file access and usage of external libraries.

### Lua definitions

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

### The PDF global variable

When you execute a script, a singular global variable is provided named `pdf`
which provides access to configuration details, utility functions, and the
core ability to create and add pages to your generated PDF.

In particular, you'll find:

1. `pdf.page`: page-specific configuration details like the width & height of
   generated pages alongside which font to use by default.
2. `pdf.object` : has functions to use to create objects within a PDF like a
   circle, a rectangle, or text.
3. `pdf.font`: supports adding external fonts to the PDF.
4. `pdf.log`: provides an interface to logging to stdout and the log file used
   by the `makepdf` process.
5. `pdf.pages`: exposes the pages to be generated and provides an
   interface to add new pages.
6. `pdf.utils`: contains an assortment of additional functions to aid with
   building out the script itself including creating common data types and
   converting between different units of measurement.

### Creating a page

Pages are created lazily, which means that in Lua you are defining pages
and what will be displayed within them, but the pages themselves are generated
after the script is finished.

Creating a new page in sequence is really easy:

```lua
-- A unique id for a page is returned so you can reference it later
local id = pdf.pages.create("My new page")
```

Once you've made a page, you can retrieve it by its id:

```lua
-- This technically returns either the page or nil, but since we just created it
-- we can safely assert that it is a non-nil value
local page = assert(pdf.pages.get(id))
```

### Adding objects to a page

With a page created, objects can be added to it easily:

```lua
-- Create a new rectangle whose lower-left coordinate is 3,5 and upper-right
-- coordinate is 10,20.
--
-- Keep in mind that PDFs have 0,0 positioned in the bottom-left of the page,
-- not the top-left!
page.push(pdf.object.rect({
  ll = { x = 3, y = 5 },
  ur = { x = 10, y = 20 },
}))
```

Objects are drawn on the page in the order they are pushed; however, you can
specify a depth field on any object to change when it is drawn. The lower the
depth, the earlier it is drawn:

```lua
-- Draw some text at a depth of 1
page.push(pdf.object.text({
  x = 5
  y = 7,
  depth = 1,
  text = "Hello world!",
}))

-- Draw some text at the default depth of 0, meaning before the text!
page.push(pdf.object.rect({
  ll = { x = 3, y = 5 },
  ur = { x = 10, y = 20 },
}))
```

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or
[apache-license][apache-license]) MIT license (LICENSE-MIT or
[mit-license][mit-license]) at your option.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
[mit-license]: http://opensource.org/licenses/MIT
