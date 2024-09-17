# makepdf

Build PDF documents via [Luau](https://luau.org/) scripts, designed to make
creating planners for e-ink devices.

## Usage

```sh
# Make a planner for the current year using the default script (makepdf.lua)
makepdf make

# Make a planner for the current year using the specified script
makepdf make --script /path/to/script.lua

# Make a planner for a different year
makepdf make --year 2035

# Make a planner for a specific device dimensions
makepdf make --dimensions 1404x1879px
```

## Builtin Scripts

The CLI includes a couple of batteries-included scripts that can be used to
generate PDFs without needing to write or even have Lua scripts on your path.

```sh
# Print a list of all available builtin scripts, one per line
makepdf script

# Print out the source code of the script
makepdf script example

# Leverage a builtin script to make a pdf
makepdf make --script builtin:example
```

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or
[apache-license][apache-license]) MIT license (LICENSE-MIT or
[mit-license][mit-license]) at your option.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
[mit-license]: http://opensource.org/licenses/MIT
