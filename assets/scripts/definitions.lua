---@meta
-------------------------------------------------------------------------------
-- DEFINITIONS
--
-- Primary definition file representing makepdf's Lua library.
-------------------------------------------------------------------------------

---@class pdf
pdf = {}

-------------------------------------------------------------------------------
-- PAGE CONFIGURATION
-------------------------------------------------------------------------------

---@class pdf.page
pdf.page = {
    ---DPI of the page.
    ---@type number
    dpi = 0,
    ---Path to an external font to load as the default font.
    ---If none provided, uses builtin font.
    ---@type string|nil
    font = "",
    ---Width of the page in millimeters.
    ---@type number
    width = 0,
    ---Height of the page in millimeters.
    ---@type number
    height = 0,
    ---Default size of text font in points.
    ---@type number
    font_size = 0,
    ---Used for the interior of rects and shapes, and for text.
    ---@type pdf.common.Color
    fill_color = "",
    ---Used for the exterior of rects and shapes, and for lines.
    ---@type pdf.common.Color
    outline_color = "",
    ---Default thickness of lines.
    ---@type number
    outline_thickness = 0,
    ---Default style of lines.
    ---@type pdf.object.line.Style
    line_style = "solid",
}

---Returns the bounds covering the entire page.
---@return pdf.common.Bounds
function pdf.page:bounds() end

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
---@alias pdf.common.PaintMode "clip"|"fill"|"fill_stroke"|"stroke"
---@alias pdf.common.WindingOrder "even_odd"|"non_zero"
---@alias pdf.common.Align {h:pdf.common.HorizontalAlign, v:pdf.common.VerticalAlign}
---@alias pdf.common.HorizontalAlign "left"|"middle"|"right"
---@alias pdf.common.VerticalAlign "top"|"middle"|"bottom"

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
---| {[1]:{[1]:number, [2]:number}, [2]:{[1]:number, [2]:number}}
---| {[1]:number, [2]:number, [3]:number, [4]:number}


---@class pdf.common.Bounds
---@field ll pdf.common.Point
---@field ur pdf.common.Point
local PdfBounds = {}

---Aligns these bounds to the provided bounds, returning an updated bounds.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.common.Bounds
function PdfBounds:align_to(bounds, align) end

---Moves the bounds to the specified x & y position for the lower-left point,
---returning updated bounds.
---
---Both the `x` and `y` fields are optional, so you can supply just one to
---only affect that axis.
---@param opts? {x?:number, y?:number}
---@return pdf.common.Bounds
function PdfBounds:move_to(opts) end

---Shifts the bounds by the specified x & y offset, returning updated bounds.
---
---Both the `x` and `y` fields are optional, so you can supply just one to
---only affect that axis.
---@param opts? {x?:number, y?:number}
---@return pdf.common.Bounds
function PdfBounds:shift_by(opts) end

---Scales the bounds to the specified width & height, returning updated bounds.
---
---Both the `width` and `height` fields are optional, so you can supply just
---one to only affect that dimension.
---@param opts? {width?:number, height?:number}
---@return pdf.common.Bounds
function PdfBounds:scale_to(opts) end

---Scales the bounds by a factor of width & height, returning updated bounds.
---
---For example, a `width` of 2 will double the width of the bounds and a
---`height` of 0.5 will shrink the height of the bounds to be half the size.
---
---Both the `width` and `height` fields are optional, so you can supply just one
---to only affect that dimension.
---@param opts? {width?:number, height?:number}
---@return pdf.common.Bounds
function PdfBounds:scale_by_factor(opts) end

---Calculates the width of the bounds.
---@return number
function PdfBounds:width() end

---Calculates the height of the bounds.
---@return number
function PdfBounds:height() end

---Returns coordinates in table as {ll.x, ll.y, ur.x, ur.y}.
---@return {[1]:number, [2]:number, [3]:number, [4]:number}
function PdfBounds:to_coords() end

---@class pdf.common.Date
local PdfDate = {}

---@type integer
PdfDate.year = 0

---Between 1 and 12
---@type integer
PdfDate.month = 0

---Between 1 and 53 (last week of year differs by years)
---@type integer
PdfDate.week = 0

---Weekday associated with the date.
---@type pdf.common.DateWeekday
PdfDate.weekday = {}

---Between 1 and 31
---@type integer
PdfDate.day = 0

---Between 1 and 366 (last day of year differs by years)
---@type integer
PdfDate.ordinal = 0

---Produces a string based on a formatting syntax from the chrono library.
---
---See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
---@param fmt string
---@return string
function PdfDate.format(fmt) end

---@param days integer
---@return pdf.common.Date|nil
function PdfDate.add_days(days) end

---@param weeks integer
---@return pdf.common.Date|nil
function PdfDate.add_weeks(weeks) end

---@param months integer
---@return pdf.common.Date|nil
function PdfDate.add_months(months) end

---@return pdf.common.Date|nil
function PdfDate.tomorrow() end

---@return pdf.common.Date|nil
function PdfDate.yesterday() end

---@return pdf.common.Date|nil
function PdfDate.next_week() end

---@return pdf.common.Date|nil
function PdfDate.last_week() end

---@return pdf.common.Date|nil
function PdfDate.next_month() end

---@return pdf.common.Date|nil
function PdfDate.last_month() end

---@return pdf.common.Date|nil
function PdfDate.beginning_of_year() end

---@return pdf.common.Date|nil
function PdfDate.end_of_year() end

---@return pdf.common.Date|nil
function PdfDate.beginning_of_month() end

---@return pdf.common.Date|nil
function PdfDate.end_of_month() end

---Returns current date moved to beginning of week where beginning of week starts on Sunday.
---@return pdf.common.Date|nil
function PdfDate.beginning_of_week_sunday() end

---Returns current date moved to end of week where beginning of week starts on Sunday.
---@return pdf.common.Date|nil
function PdfDate.end_of_week_sunday() end

---Returns current date moved to beginning of week where beginning of week starts on Monday.
---@return pdf.common.Date|nil
function PdfDate.beginning_of_week_monday() end

---Returns current date moved to end of week where beginning of week starts on Monday.
---@return pdf.common.Date|nil
function PdfDate.end_of_week_monday() end

---Returns total calendar weeks the month of the date spans where beginning of week starts on Sunday.
---@return integer
function PdfDate.weeks_in_month_sunday() end

---Returns total calendar weeks the month of the date spans where beginning of week starts on Monday.
---@return integer
function PdfDate.weeks_in_month_monday() end

---@class pdf.common.DateWeekday
local PdfDateWeekday = {}

---Returns the short name of the weekday.
---@return "mon"|"tue"|"wed"|"thu"|"fri"|"sat"|"sun"
function PdfDateWeekday.short_name() end

---Returns the short name of the weekday.
---@return "monday"|"tuesday"|"wednesday"|"thursday"|"friday"|"saturday"|"sunday"
function PdfDateWeekday.long_name() end

---Returns next day of the week.
---@return pdf.common.DateWeekday
function PdfDateWeekday.next_weekday() end

---Returns previous day of the week.
---@return pdf.common.DateWeekday
function PdfDateWeekday.prev_weekday() end

---Returns a day-of-week number starting from Monday = 1.
---(ISO 8601 weekday number)
---@return integer
function PdfDateWeekday.number_from_monday() end

---Returns a day-of-week number starting from Sunday = 1.
---@return integer
function PdfDateWeekday.number_from_sunday() end

---Returns a day-of-week number starting from Monday = 0.
---@return integer
function PdfDateWeekday.num_days_from_monday() end

---Returns a day-of-week number starting from Sunday = 0.
---@return integer
function PdfDateWeekday.num_days_from_sunday() end

---Returns number of days from specified `weekday`.
---
---For example, if this weekday is Monday, and the specified `weekday`
---is Wednesday, this would return 2.
---@param weekday pdf.common.DateWeekday
---@return integer
function PdfDateWeekday.days_since(weekday) end

-------------------------------------------------------------------------------
-- RUNTIME TYPES
-------------------------------------------------------------------------------

---@class pdf.runtime.Page
local PdfRuntimePage = {}

---Unique id associated with the page.
---@type integer
PdfRuntimePage.id = 0

---Date associated with the page.
---
---For daily, this is the full date.
---For weekly, this is the start of the week.
---For monthly, this is the start of the month.
---@type pdf.common.Date
PdfRuntimePage.date = {}

---Returns the daily page.
---
---If no argument provided, returns the daily page for the current page.
---If a date or string is provided, returns the daily page for that date.
---@param date pdf.common.Date|string|nil
---@return pdf.runtime.Page|nil
function PdfRuntimePage.daily(date) end

---Returns the monthly page.
---
---If no argument provided, returns the monthly page for the current page.
---If a date or string is provided, returns the monthly page for that date.
---@param date pdf.common.Date|string|nil
---@return pdf.runtime.Page|nil
function PdfRuntimePage.monthly(date) end

---Returns the weekly page.
---
---If no argument provided, returns the weekly page for the current page.
---If a date or string is provided, returns the weekly page for that date.
---@param date pdf.common.Date|string|nil
---@return pdf.runtime.Page|nil
function PdfRuntimePage.weekly(date) end

---Returns the next page in sequence.
---
---This is specifically the next page of the same kind (daily, weekly, monthly).
---@return pdf.runtime.Page|nil
function PdfRuntimePage.next_page() end

---Returns the previous page in sequence.
---
---This is specifically the previous page of the same kind (daily, weekly, monthly).
---@return pdf.runtime.Page|nil
function PdfRuntimePage.prev_page() end

---Pushes a new object onto the page to be rendered during PDF generation.
---@param obj pdf.Object
function PdfRuntimePage.push(obj) end

-------------------------------------------------------------------------------
-- HOOKS FUNCTIONS
-------------------------------------------------------------------------------

---@class pdf.hooks
pdf.hooks = {}

---Register new callback for when a daily page is created.
---
---This will append an additional callback on the stack, and
---can be used multiple times to register multiple callbacks.
---@param f fun(page:pdf.runtime.Page)
function pdf.hooks.on_daily_page(f) end

---Register new callback for when a monthly page is created.
---
---This will append an additional callback on the stack, and
---can be used multiple times to register multiple callbacks.
---@param f fun(page:pdf.runtime.Page)
function pdf.hooks.on_monthly_page(f) end

---Register new callback for when a weekly page is created.
---
---This will append an additional callback on the stack, and
---can be used multiple times to register multiple callbacks.
---@param f fun(page:pdf.runtime.Page)
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

---Aligns the group to the provided bounds, returning an updated group.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Group
function PdfObjectGroup:align_to(bounds, align) end

---Calculates the bounds that contains the entire set of objects within the group.
---@return pdf.common.Bounds
function PdfObjectGroup:bounds() end

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
    color = nil,
    ---@type number|nil
    thickness = nil,
    ---@type pdf.object.line.Style|nil
    style = nil,
    ---@type pdf.common.Link|nil
    link = nil,
}

---Aligns the line to the provided bounds, returning an updated line.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Line
function PdfObjectLine:align_to(bounds, align) end

---Calculates the bounds that contain all points within the lines.
---@return pdf.common.Bounds
function PdfObjectLine:bounds() end

---@class pdf.object.LineArgs
---@field [number] pdf.common.PointArg
local PdfObjectLineArgs = {
    ---@type integer|nil
    depth = nil,
    ---@type pdf.common.Color|nil
    color = nil,
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
    ---@type pdf.common.PaintMode|nil
    mode = nil,
    ---@type pdf.common.WindingOrder|nil
    order = nil,
    ---@type pdf.common.Link|nil
    link = nil,
}

---Aligns the rect to the provided bounds, returning an updated rect.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Rect
function PdfObjectRect:align_to(bounds, align) end

---Returns the bounds of the rect.
---@return pdf.common.Bounds
function PdfObjectRect:bounds() end

---@class pdf.object.RectArgsBase
local PdfObjectRectArgsBase = {
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

---@class pdf.object.RectArgs1: pdf.object.RectArgsBase
---@field ll {x:number, y:number}
---@field ur {x:number, y:number}

---@class pdf.object.RectArgs2: pdf.object.RectArgsBase
---@field [1] {[1]:number, [2]:number}
---@field [2] {[1]:number, [2]:number}

---@class pdf.object.RectArgs3: pdf.object.RectArgsBase
---@field [1] number
---@field [2] number
---@field [3] number
---@field [4] number

---@alias pdf.object.RectArgs
---| pdf.object.RectArgs1
---| pdf.object.RectArgs2
---| pdf.object.RectArgs3
---| pdf.object.RectArgsBase

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

---Aligns the shape to the provided bounds, returning an updated shape.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Shape
function PdfObjectShape:align_to(bounds, align) end

---Calculates the bounds that contain all points within the shape.
---@return pdf.common.Bounds
function PdfObjectShape:bounds() end

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
    ---@type integer|nil
    font = nil,
    ---@type number|nil
    size = nil,
    ---@type pdf.common.Color|nil
    color = nil,
    ---@type pdf.common.Link|nil
    link = nil,
}

---Aligns the text to the provided bounds, returning an updated text.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Text
function PdfObjectText:align_to(bounds, align) end

---Calculates the bounds of the text object by using its baseline x & y
---coordinates alongside the associated font.
---
---The returned bounds represent the total size and positioning of the text
---within the PDF accounting for ascenders (e.g. capital letters) and
---descenders (e.g. letters like 'g').
---@return pdf.common.Bounds
function PdfObjectText:bounds() end

---@class pdf.object.TextArgsBase
local PdfObjectTextArgsBase = {
    ---@type string
    text = "",
    ---@type integer|nil
    depth = nil,
    ---@type integer|nil
    font = nil,
    ---@type number|nil
    size = nil,
    ---@type pdf.common.Color|nil
    color = nil,
    ---@type pdf.common.LinkArg|nil
    link = nil,
}

---@class pdf.object.TextArgs1: pdf.object.TextArgsBase
---@field x number
---@field y number

---@class pdf.object.TextArgs2: pdf.object.TextArgsBase
---@field [1] number
---@field [2] number

---@class pdf.object.TextArgs3: pdf.object.TextArgsBase
---@field [1] {[1]:number, [2]:number}

---@alias pdf.object.TextArgs
---| pdf.object.TextArgs1
---| pdf.object.TextArgs2
---| pdf.object.TextArgs3
---| pdf.object.TextArgsBase

---Creates a new text object.
---
---@param tbl pdf.object.TextArgs
---@return pdf.object.Text
function pdf.object.text(tbl) end

-------------------------------------------------------------------------------
-- FONT FUNCTIONS
-------------------------------------------------------------------------------

---@class pdf.font
pdf.font = {}

---Adds a new font into the runtime, returning the id associated with the font.
---
---If the font has already been added, this returns the cached id.
---@param path string
---@return number id
function pdf.font.add(path) end

---Retrieves the id or sets the id of the fallback font.
---@param id number
---@overload fun():number
function pdf.font.fallback(id) end

---Retrieves a list of the ids of all fonts loaded into the runtime.
---@return number[]
function pdf.font.ids() end

---Retrieves the path for the font specified by its id.
---
---The builtin font will never have a path.
---@param id number
---@return string|nil path
function pdf.font.path(id) end

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

---Creates a bounds instance from the provided table, or throws an error if invalid.
---@param tbl table
---@return pdf.common.Bounds
function pdf.utils.bounds(tbl) end

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

---Converts millimeters to points (approximate).
---@param mm number
---@return number
function pdf.utils.mm_to_pt(mm) end

---Converts points to millimeters (approximate).
---@param pt number
---@return number
function pdf.utils.pt_to_mm(pt) end
