---@meta
---Primary definition file representing mpdf's Lua library.

pdf = {}

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
