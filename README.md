# makepdf

Build PDF documents via [Luau](https://luau.org/) scripts, designed to make
creating planners for e-ink devices.

## Usage

```sh
# Make a planner for the current year using the default script
makepdf make

# Make a planner for the current year using the specified script
makepdf make --script /path/to/script.lua

# Make a planner for a different year using the specified script
makepdf make --year 2035 /path/to/script.lua

# Make a planner for a specific device dimensions using the specified script
makepdf make --dimensions 1404x1879px --script /path/to/script.lua
```

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or
[apache-license][apache-license]) MIT license (LICENSE-MIT or
[mit-license][mit-license]) at your option.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
[mit-license]: http://opensource.org/licenses/MIT
