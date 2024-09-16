---@meta
---Primary definition file representing makepdf's Lua library.

---@class pdf
pdf = {}

-------------------------------------------------------------------------------
-- PAGE CONFIGURATION
-------------------------------------------------------------------------------

---@class pdf.page
pdf.page = {
    ---@type number
    dpi = 0,
    --@type string|nil
    font = "",
    --@type number
    width = 0,
    --@type number
    height = 0,
    --@type number
    font_size = 0,
    --@type pdf.common.Color
    fill_color = "",
    --@type pdf.common.Color
    outline_color = "",
    --@type number
    outline_thickness = 0,
    --@type pdf.object.line.Style
    line_style = "solid",
}

-------------------------------------------------------------------------------
-- PLANNER CONFIGURATION
-------------------------------------------------------------------------------

---@class pdf.planner
pdf.planner = {
    ---@type integer
    year = 0,
    ---@class pdf.planner.monthly
    monthly = {
        ---@type boolean
        enabled = true,
    },
    ---@class pdf.planner.weekly
    weekly = {
        ---@type boolean
        enabled = true,
    },
    ---@class pdf.planner.daily
    daily = {
        ---@type boolean
        enabled = true,
    },
}

-------------------------------------------------------------------------------
-- COMMON TYPES
-------------------------------------------------------------------------------

---@alias pdf.common.Color string
---@alias pdf.common.Point {x:number, y:number}
---@alias pdf.common.Bounds {ll:pdf.common.Point, ur:pdf.common.Point}
---@alias pdf.common.PaintMode "clip"|"fill"|"fill_stroke"|"stroke"
---@alias pdf.common.WindingOrder "even_odd"|"non_zero"

---@alias pdf.common.Link
---| {type:"goto", page:integer}
---| {type:"uri", uri:string}

---@alias pdf.common.LinkArg
---| integer #representing a page's id
---| string #representing a URI
---| pdf.common.Link

---@alias pdf.common.PointArg
---| pdf.common.Point
---| {[1]:number, [2]:number}

---@alias pdf.common.BoundsArg
---| pdf.common.Bounds
---| {llx:number, lly:number, urx:number, ury:number}
---| {[1]:{[1]:number, [2]:number}, [2]:{[1]:number, [2]:number}}
---| {[1]:number, [2]:number, [3]:number, [4]:number}

---@class pdf.common.Date
local Date = {}

---@type integer
Date.year = 0

---Between 1 and 12
---@type integer
Date.month = 0

---Between 1 and 53 (last week of year differs by years)
---@type integer
Date.week = 0

---Between 1 and 31
---@type integer
Date.day = 0

---Between 1 and 366 (last day of year differs by years)
---@type integer
Date.ordinal = 0

---Produces a string based on a formatting syntax from the chrono library.
---
---See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
---@param fmt string
---@return string
function Date.format(fmt) end

---@param days integer
---@return pdf.common.Date|nil
function Date.add_days(days) end

---@param weeks integer
---@return pdf.common.Date|nil
function Date.add_weeks(weeks) end

---@param months integer
---@return pdf.common.Date|nil
function Date.add_months(months) end

---@return pdf.common.Date|nil
function Date.tomorrow() end

---@return pdf.common.Date|nil
function Date.yesterday() end

---@return pdf.common.Date|nil
function Date.next_week() end

---@return pdf.common.Date|nil
function Date.last_week() end

---@return pdf.common.Date|nil
function Date.next_month() end

---@return pdf.common.Date|nil
function Date.last_month() end

-------------------------------------------------------------------------------
-- ENGINE TYPES
-------------------------------------------------------------------------------

---@class pdf.engine.Page
local Page = {}

---@type integer
Page.id = 0

---@type pdf.common.Date
Page.date = {}

---@param date pdf.common.Date|string|nil
---@return pdf.engine.Page|nil
function Page.daily(date) end

---@param date pdf.common.Date|string|nil
---@return pdf.engine.Page|nil
function Page.monthly(date) end

---@param date pdf.common.Date|string|nil
---@return pdf.engine.Page|nil
function Page.weekly(date) end

---@return pdf.engine.Page|nil
function Page.next_page() end

---@return pdf.engine.Page|nil
function Page.prev_page() end

---@param obj pdf.Object
function Page.push(obj) end

-------------------------------------------------------------------------------
-- HOOKS FUNCTIONS
-------------------------------------------------------------------------------

---@class pdf.hooks
pdf.hooks = {}

---Register new callback for when a daily page is created.
---
---This will append an additional callback on the stack, and
---can be used multiple times to register multiple callbacks.
---@param f fun(page:pdf.engine.Page)
function pdf.hooks.on_daily_page(f) end

---Register new callback for when a monthly page is created.
---
---This will append an additional callback on the stack, and
---can be used multiple times to register multiple callbacks.
---@param f fun(page:pdf.engine.Page)
function pdf.hooks.on_monthly_page(f) end

---Register new callback for when a weekly page is created.
---
---This will append an additional callback on the stack, and
---can be used multiple times to register multiple callbacks.
---@param f fun(page:pdf.engine.Page)
function pdf.hooks.on_weekly_page(f) end

-------------------------------------------------------------------------------
-- OBJECT FUNCTIONS
-------------------------------------------------------------------------------

---@class pdf.object
pdf.object = {}

---@alias pdf.Object
---| pdf.object.Group
---| pdf.object.Line
---| pdf.object.Rect
---| pdf.object.Shape
---| pdf.object.Text

---@class pdf.object.Group
---@field [number] pdf.Object
local PdfObjectGroup = {
    ---@type "group"
    type = "group",
    ---@type pdf.common.Link|nil
    link = nil,
}

---@class pdf.object.GroupArgs
---@field [number] pdf.Object
local PdfObjectGroupArgs = {
    ---@type pdf.common.LinkArg|nil
    link = nil,
}

---Creates a new group object.
---
---@param tbl pdf.object.GroupArgs
---@return pdf.object.Group
function pdf.object.group(tbl) end

---@alias pdf.object.line.Style "dashed"|"solid"

---@class pdf.object.Line
---@field [number] pdf.common.Point
local PdfObjectLine = {
    ---@type "line"
    type = "line",
    ---@type integer|nil
    depth = nil,
    ---@type pdf.common.Color|nil
    fill_color = nil,
    ---@type pdf.common.Color|nil
    outline_color = nil,
    ---@type number|nil
    thickness = nil,
    ---@type pdf.object.line.Style|nil
    style = nil,
    ---@type pdf.common.Link|nil
    link = nil,
}

---@class pdf.object.LineArgs
---@field [number] pdf.common.PointArg
local PdfObjectLineArgs = {
    ---@type integer|nil
    depth = nil,
    ---@type pdf.common.Color|nil
    fill_color = nil,
    ---@type pdf.common.Color|nil
    outline_color = nil,
    ---@type number|nil
    thickness = nil,
    ---@type pdf.object.line.Style|nil
    style = nil,
    ---@type pdf.common.LinkArg|nil
    link = nil,
}

---Creates a new line object.
---
---@param tbl pdf.object.LineArgs
---@return pdf.object.Line
function pdf.object.line(tbl) end

---@class pdf.object.Rect
local PdfObjectRect = {
    ---@type "rect"
    type = "rect",
    ---@type pdf.common.Point
    ll = {},
    ---@type pdf.common.Point
    ur = {},
    ---@type integer|nil
    depth = nil,
    ---@type pdf.common.Color|nil
    fill_color = nil,
    ---@type pdf.common.Color|nil
    outline_color = nil,
    ---@type pdf.common.Link|nil
    link = nil,
}

---@class pdf.object.RectArgsBase
local PdfObjectRectArgsBase = {
    ---@type integer|nil
    depth = nil,
    ---@type pdf.common.Color|nil
    fill_color = nil,
    ---@type pdf.common.Color|nil
    outline_color = nil,
    ---@type pdf.common.LinkArg|nil
    link = nil,
}

---@class pdf.object.RectArgs1: pdf.object.RectArgsBase
---@field ll {x:number, y:number}
---@field ur {x:number, y:number}

---@class pdf.object.RectArgs2: pdf.object.RectArgsBase
---@field llx number
---@field lly number
---@field urx number
---@field ury number

---@class pdf.object.RectArgs3: pdf.object.RectArgsBase
---@field [1] {[1]:number, [2]:number}
---@field [2] {[1]:number, [2]:number}

---@class pdf.object.RectArgs4: pdf.object.RectArgsBase
---@field [1] number
---@field [2] number
---@field [3] number
---@field [4] number

---@alias pdf.object.RectArgs
---| pdf.object.RectArgs1
---| pdf.object.RectArgs2
---| pdf.object.RectArgs3
---| pdf.object.RectArgs4

---Creates a new rect object.
---
---@param tbl pdf.object.RectArgs
---@return pdf.object.Rect
function pdf.object.rect(tbl) end

---@class pdf.object.Shape
---@field [number] pdf.common.Point
local PdfObjectShape = {
    ---@type "shape"
    type = "shape",
    ---@type integer|nil
    depth = nil,
    ---@type pdf.common.Color|nil
    fill_color = nil,
    ---@type pdf.common.Color|nil
    outline_color = nil,
    ---@type pdf.common.PaintMode|nil
    mode = nil,
    ---@type pdf.common.WindingOrder|nil
    order = nil,
    ---@type pdf.common.Link|nil
    link = nil,
}

---@class pdf.object.ShapeArgs
---@field [number] pdf.common.PointArg
local PdfObjectShapeArgs = {
    ---@type integer|nil
    depth = nil,
    ---@type pdf.common.Color|nil
    fill_color = nil,
    ---@type pdf.common.Color|nil
    outline_color = nil,
    ---@type pdf.common.PaintMode|nil
    mode = nil,
    ---@type pdf.common.WindingOrder|nil
    order = nil,
    ---@type pdf.common.LinkArg|nil
    link = nil,
}

---Creates a new shape object.
---
---@param tbl pdf.object.ShapeArgs
---@return pdf.object.Shape
function pdf.object.shape(tbl) end

---@class pdf.object.Text
local PdfObjectText = {
    ---@type "text"
    type = "text",
    ---@type number
    x = 0,
    ---@type number
    y = 0,
    ---@type string
    text = "",
    ---@type integer|nil
    depth = nil,
    ---@type number|nil
    size = nil,
    ---@type pdf.common.Color|nil
    fill_color = nil,
    ---@type pdf.common.Color|nil
    outline_color = nil,
    ---@type pdf.common.Link|nil
    link = nil,
}

---@class pdf.object.TextArgsBase
local PdfObjectTextArgsBase = {
    ---@type string
    text = "",
    ---@type integer|nil
    depth = nil,
    ---@type number|nil
    size = nil,
    ---@type pdf.common.Color|nil
    fill_color = nil,
    ---@type pdf.common.Color|nil
    outline_color = nil,
    ---@type pdf.common.LinkArg|nil
    link = nil,
}

---@class pdf.object.TextArgs1: pdf.object.TextArgsBase
---@field x number
---@field y number

---@class pdf.object.TextArgs2: pdf.object.TextArgsBase
---@field [1] number
---@field [2] number

---@alias pdf.object.TextArgs
---| pdf.object.TextArgs1
---| pdf.object.TextArgs2

---Creates a new text object.
---
---@param tbl pdf.object.TextArgs
---@return pdf.object.Text
function pdf.object.text(tbl) end

-------------------------------------------------------------------------------
-- UTILITY FUNCTIONS
-------------------------------------------------------------------------------

---@class pdf.utils
pdf.utils = {}

---Asserts that two values are deeply equal, which involves recursively
---traversing tables. If not equal, will throw an error.
---
---Accepts an optional table that can be used to disable `__eq` metatable usage
---when tables are being compared.
---@param a any
---@param b any
---@param opts? {ignore_metatable:boolean|nil}
function pdf.utils.assert_deep_equal(a, b, opts) end

---Asserts that two values are not deeply equal, which involves recursively
---traversing tables. If equal, will throw an error.
---
---Accepts an optional table that can be used to disable `__eq` metatable usage
---when tables are being compared.
---@param a any
---@param b any
---@param opts? {ignore_metatable:boolean|nil}
function pdf.utils.assert_not_deep_equal(a, b, opts) end

---Checks if two values are deeply equal, which involves recursively
---traversing tables.
---
---Accepts an optional table that can be used to disable `__eq` metatable usage
---when tables are being compared.
---@param a any
---@param b any
---@param opts? {ignore_metatable:boolean|nil}
---@return boolean
function pdf.utils.deep_equal(a, b, opts) end

---Transforms any Lua value into a human-readable representation.
---@param value any
---@param opts? {pretty:boolean} if pretty, will make string pretty
---@return string
function pdf.utils.inspect(value, opts) end

---Checks if a string starts with a specified prefix.
---@param s string
---@param prefix string
---@return boolean
function pdf.utils.starts_with(s, prefix) end

---Checks if a string ends with a specified prefix.
---@param s string
---@param prefix string
---@return boolean
function pdf.utils.ends_with(s, prefix) end
