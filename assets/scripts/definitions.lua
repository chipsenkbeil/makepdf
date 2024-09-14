---@meta
---Primary definition file representing makepdf's Lua library.

pdf = {}

---Transforms any Lua value into a human-readable representation.
---@param value any
---@param opts? {pretty:boolean} if pretty, will make string pretty
---@return string
function pdf.inspect(value, opts) end

---@class pdf.config
pdf.config = {}

---@class pdf.object
pdf.object = {}

---Creates a new line object.
---
---@param tbl {x:number, y:number}
---@return pdf.object.Line
function pdf.object.line(tbl) end

---@class pdf.object.Line
---@field x number #x coordinate of line
---@field y number #y coordinate of line
