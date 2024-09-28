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
    ---@type pdf.common.ColorLike
    fill_color = "",
    ---Used for the exterior of rects and shapes, and for lines.
    ---@type pdf.common.ColorLike
    outline_color = "",
    ---Default thickness of lines.
    ---@type number
    outline_thickness = 0,
    ---Default dash pattern of lines.
    ---@type pdf.common.line.DashPatternLike
    dash_pattern = "solid",
    ---Default cap style of lines.
    ---@type pdf.common.line.CapStyle
    cap_style = "round",
    ---Default join style of lines.
    ---@type pdf.common.line.JoinStyle
    join_style = "round",
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

---@alias pdf.common.PaintMode "clip"|"fill"|"fill_stroke"|"stroke"
---@alias pdf.common.WindingOrder "even_odd"|"non_zero"
---@alias pdf.common.Align {h:pdf.common.HorizontalAlign, v:pdf.common.VerticalAlign}
---@alias pdf.common.HorizontalAlign "left"|"middle"|"right"
---@alias pdf.common.VerticalAlign "top"|"middle"|"bottom"
---@alias pdf.common.Padding {top:number, right:number, bottom:number, left:number}

---@alias pdf.common.line.CapStyle "butt"|"round"|"projecting_square"
---@alias pdf.common.line.JoinStyle "limit"|"miter"|"round"
---@alias pdf.common.line.DashPatternLike
---| string
---| pdf.common.line.DashPattern

---@alias pdf.common.ColorLike
---| string
---| {[1]:integer, [2]:integer, [3]:integer}
---| {r:integer, g:integer, b:integer}
---| pdf.common.Color

---@alias pdf.common.Link
---| {type:"goto", page:integer}
---| {type:"uri", uri:string}

---@alias pdf.common.LinkLike
---| integer #representing a page's id
---| string #representing a URI
---| pdf.common.Link

---@alias pdf.common.PointLike
---| pdf.common.Point
---| {[1]:number, [2]:number}

---@alias pdf.common.BoundsLike
---| pdf.common.Bounds
---| {[1]:{[1]:number, [2]:number}, [2]:{[1]:number, [2]:number}}
---| {[1]:number, [2]:number, [3]:number, [4]:number}

---@alias pdf.common.PaddingLike
---| {top?:number, right?:number, bottom?:number, left?:number}
---| {[1]:number, [2]:number, [3]:number, [4]:number}
---| {[1]:number, [2]:number, [3]:number}
---| {[1]:number, [2]:number}
---| {[1]:number}
---| number

---@class pdf.common.line.DashPattern
---@field offset integer
---@field dash_1 integer|nil
---@field dash_2 integer|nil
---@field dash_3 integer|nil
---@field gap_1 integer|nil
---@field gap_2 integer|nil
---@field gap_3 integer|nil

---@class pdf.common.Bounds
---@field ll pdf.common.Point
---@field ur pdf.common.Point
local PdfBounds = {}

---Calculates and returns the lower-right point of the bounds.
---@return pdf.common.Point
function PdfBounds:lr() end

---Calculates and returns the upper-left point of the bounds.
---@return pdf.common.Point
function PdfBounds:ul() end

---Aligns these bounds to the provided bounds, returning an updated bounds.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.common.Bounds
function PdfBounds:align_to(bounds, align) end

---Returns a copy of bounds with padding applied.
---
---All padding fields are optional and default to 0.
---@param padding? pdf.common.PaddingLike
---@return pdf.common.Bounds
function PdfBounds:with_padding(padding) end

---Returns a copy of bounds with points rounded to precision.
---@param precision integer
---@return pdf.common.Bounds
function PdfBounds:with_precision(precision) end

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

---@class pdf.common.Color
---@field red integer
---@field green integer
---@field blue integer
local PdfColor = {}

---Returns the luminance (brightness of the color) as a value between 0 and 1.
---@return number
function PdfColor:luminance() end

---Returns true if the color is considered light (luminance > 0.5).
---@return boolean
function PdfColor:is_light() end

---Returns a new color variant lightened by `percentage`.
---@param percentage number
---@return pdf.common.Color
function PdfColor:lighten(percentage) end

---Returns a new color variant darkend by `percentage`.
---@param percentage number
---@return pdf.common.Color
function PdfColor:darken(percentage) end

---Converts color into a hex string.
---@return string
function PdfColor:__tostring() end

---@class pdf.common.Date
---@field year integer
---@field month integer # between 1 and 12
---@field week integer # between 1 and 53 (last week of year differs by years)
---@field weekday pdf.common.DateWeekday # weekday associated with the date
---@field day integer # between 1 and 31
---@field ordinal integer # between 1 and 366 (last day of year differs by years)
local PdfDate = {}

---Produces a string based on a formatting syntax from the chrono library.
---
---See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
---@param fmt string
---@return string
function PdfDate:format(fmt) end

---@param days integer
---@return pdf.common.Date|nil
function PdfDate:add_days(days) end

---@param weeks integer
---@return pdf.common.Date|nil
function PdfDate:add_weeks(weeks) end

---@param months integer
---@return pdf.common.Date|nil
function PdfDate:add_months(months) end

---@return pdf.common.Date|nil
function PdfDate:tomorrow() end

---@return pdf.common.Date|nil
function PdfDate:yesterday() end

---@return pdf.common.Date|nil
function PdfDate:next_week() end

---@return pdf.common.Date|nil
function PdfDate:last_week() end

---@return pdf.common.Date|nil
function PdfDate:next_month() end

---@return pdf.common.Date|nil
function PdfDate:last_month() end

---@return pdf.common.Date|nil
function PdfDate:beginning_of_year() end

---@return pdf.common.Date|nil
function PdfDate:end_of_year() end

---@return pdf.common.Date|nil
function PdfDate:beginning_of_month() end

---@return pdf.common.Date|nil
function PdfDate:end_of_month() end

---Returns current date moved to beginning of week where beginning of week starts on Sunday.
---@return pdf.common.Date|nil
function PdfDate:beginning_of_week_sunday() end

---Returns current date moved to end of week where beginning of week starts on Sunday.
---@return pdf.common.Date|nil
function PdfDate:end_of_week_sunday() end

---Returns current date moved to beginning of week where beginning of week starts on Monday.
---@return pdf.common.Date|nil
function PdfDate:beginning_of_week_monday() end

---Returns current date moved to end of week where beginning of week starts on Monday.
---@return pdf.common.Date|nil
function PdfDate:end_of_week_monday() end

---Returns total calendar weeks the month of the date spans where beginning of week starts on Sunday.
---@return integer
function PdfDate:weeks_in_month_sunday() end

---Returns total calendar weeks the month of the date spans where beginning of week starts on Monday.
---@return integer
function PdfDate:weeks_in_month_monday() end

---Converts date into a string in the format "YYYY-MM-DD".
---@return string
function PdfDate:__tostring() end

---@class pdf.common.DateWeekday
local PdfDateWeekday = {}

---Returns the short name of the weekday.
---@return "mon"|"tue"|"wed"|"thu"|"fri"|"sat"|"sun"
function PdfDateWeekday:short_name() end

---Returns the short name of the weekday.
---@return "monday"|"tuesday"|"wednesday"|"thursday"|"friday"|"saturday"|"sunday"
function PdfDateWeekday:long_name() end

---Returns next day of the week.
---@return pdf.common.DateWeekday
function PdfDateWeekday:next_weekday() end

---Returns previous day of the week.
---@return pdf.common.DateWeekday
function PdfDateWeekday:prev_weekday() end

---Returns a day-of-week number starting from Monday = 1.
---(ISO 8601 weekday number)
---@return integer
function PdfDateWeekday:number_from_monday() end

---Returns a day-of-week number starting from Sunday = 1.
---@return integer
function PdfDateWeekday:number_from_sunday() end

---Returns a day-of-week number starting from Monday = 0.
---@return integer
function PdfDateWeekday:num_days_from_monday() end

---Returns a day-of-week number starting from Sunday = 0.
---@return integer
function PdfDateWeekday:num_days_from_sunday() end

---Returns number of days from specified `weekday`.
---
---For example, if this weekday is Monday, and the specified `weekday`
---is Wednesday, this would return 2.
---@param weekday pdf.common.DateWeekday
---@return integer
function PdfDateWeekday:days_since(weekday) end

---Converts weekday into the long string form.
---@return string
function PdfDateWeekday:__tostring() end

---@class pdf.common.Point
---@field x number
---@field y number
local PdfPoint = {}

---Returns a copy of point with x & y rounded to precision.
---@param precision integer
---@return pdf.common.Point
function PdfPoint:with_precision(precision) end

-------------------------------------------------------------------------------
-- RUNTIME TYPES
-------------------------------------------------------------------------------

---@class pdf.runtime.Page
---@field id integer # unique id associated with the page.
---@field date pdf.common.Date # date associated with page (daily is full, weekly is start of week, monthly is start of month)
local PdfRuntimePage = {}

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
---| pdf.object.Circle
---| pdf.object.Group
---| pdf.object.Line
---| pdf.object.Rect
---| pdf.object.Shape
---| pdf.object.Text

---@class pdf.object.Circle
---@field type "circle"
---@field center pdf.common.Point
---@field radius number
---@field depth integer|nil
---@field fill_color pdf.common.Color|nil
---@field outline_color pdf.common.Color|nil
---@field outline_thickness number|nil
---@field mode pdf.common.PaintMode|nil
---@field order pdf.common.WindingOrder|nil
---@field dash_pattern pdf.common.line.DashPattern|nil
---@field cap_style pdf.common.line.CapStyle|nil
---@field join_style pdf.common.line.JoinStyle|nil
---@field link pdf.common.Link|nil
local PdfObjectCircle = {}

---Aligns the circle to the provided bounds, returning an updated circle.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Circle
function PdfObjectCircle:align_to(bounds, align) end

---Calculates the bounds that fully contains the circle.
---@return pdf.common.Bounds
function PdfObjectCircle:bounds() end

---@class pdf.object.CircleLike
---@field center pdf.common.PointLike|nil
---@field radius number|nil
---@field depth integer|nil
---@field fill_color pdf.common.ColorLike|nil
---@field outline_color pdf.common.ColorLike|nil
---@field outline_thickness number|nil
---@field mode pdf.common.PaintMode|nil
---@field order pdf.common.WindingOrder|nil
---@field dash_pattern pdf.common.line.DashPatternLike|nil
---@field cap_style pdf.common.line.CapStyle|nil
---@field join_style pdf.common.line.JoinStyle|nil
---@field link pdf.common.LinkLike|nil

---Creates a new shape object.
---
---@param tbl pdf.object.CircleLike
---@return pdf.object.Circle
function pdf.object.circle(tbl) end

---@class pdf.object.Group
---@field [number] pdf.Object
---@field type "group"
---@field link pdf.common.Link|nil
local PdfObjectGroup = {}

---Aligns the group to the provided bounds, returning an updated group.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Group
function PdfObjectGroup:align_to(bounds, align) end

---Calculates the bounds that contains the entire set of objects within the group.
---@return pdf.common.Bounds
function PdfObjectGroup:bounds() end

---@class pdf.object.GroupLike
---@field [number] pdf.Object
---@field link pdf.common.LinkLike|nil

---Creates a new group object.
---
---@param tbl pdf.object.GroupLike
---@return pdf.object.Group
function pdf.object.group(tbl) end

---@class pdf.object.Line
---@field [number] pdf.common.Point
---@field type "line"
---@field depth integer|nil
---@field color pdf.common.Color|nil
---@field thickness number|nil
---@field dash_pattern pdf.common.line.DashPattern|nil
---@field cap_style pdf.common.line.CapStyle|nil
---@field join_style pdf.common.line.JoinStyle|nil
---@field link pdf.common.Link|nil
local PdfObjectLine = {}

---Aligns the line to the provided bounds, returning an updated line.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Line
function PdfObjectLine:align_to(bounds, align) end

---Calculates the bounds that contain all points within the lines.
---@return pdf.common.Bounds
function PdfObjectLine:bounds() end

---@class pdf.object.LineLike
---@field [number] pdf.common.PointLike
---@field depth integer|nil
---@field color pdf.common.ColorLike|nil
---@field thickness number|nil
---@field dash_pattern pdf.common.line.DashPatternLike|nil
---@field cap_style pdf.common.line.CapStyle|nil
---@field join_style pdf.common.line.JoinStyle|nil
---@field link pdf.common.LinkLike|nil

---Creates a new line object.
---
---@param tbl pdf.object.LineLike
---@return pdf.object.Line
function pdf.object.line(tbl) end

---@class pdf.object.Rect
---@field type "rect"
---@field ll pdf.common.Point
---@field ur pdf.common.Point
---@field depth integer|nil
---@field fill_color pdf.common.Color|nil
---@field outline_color pdf.common.Color|nil
---@field outline_thickness number|nil
---@field mode pdf.common.PaintMode|nil
---@field order pdf.common.WindingOrder|nil
---@field dash_pattern pdf.common.line.DashPattern|nil
---@field cap_style pdf.common.line.CapStyle|nil
---@field join_style pdf.common.line.JoinStyle|nil
---@field link pdf.common.Link|nil
local PdfObjectRect = {}

---Aligns the rect to the provided bounds, returning an updated rect.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Rect
function PdfObjectRect:align_to(bounds, align) end

---Returns a copy of the rect with new bounds.
---@param bounds? pdf.common.BoundsLike
---@return pdf.object.Rect
function PdfObjectRect:with_bounds(bounds) end

---Returns the bounds of the rect.
---@return pdf.common.Bounds
function PdfObjectRect:bounds() end

---@class pdf.object.RectLikeBase
---@field depth integer|nil
---@field fill_color pdf.common.ColorLike|nil
---@field outline_color pdf.common.ColorLike|nil
---@field outline_thickness number|nil
---@field mode pdf.common.PaintMode|nil
---@field order pdf.common.WindingOrder|nil
---@field dash_pattern pdf.common.line.DashPatternLike|nil
---@field cap_style pdf.common.line.CapStyle|nil
---@field join_style pdf.common.line.JoinStyle|nil
---@field link pdf.common.LinkLike|nil

---@class pdf.object.RectLike1: pdf.object.RectLikeBase
---@field ll {x:number, y:number}
---@field ur {x:number, y:number}

---@class pdf.object.RectLike2: pdf.object.RectLikeBase
---@field [1] {[1]:number, [2]:number}
---@field [2] {[1]:number, [2]:number}

---@class pdf.object.RectLike3: pdf.object.RectLikeBase
---@field [1] number
---@field [2] number
---@field [3] number
---@field [4] number

---@alias pdf.object.RectLike
---| pdf.object.RectLike1
---| pdf.object.RectLike2
---| pdf.object.RectLike3
---| pdf.object.RectLikeBase

---Creates a new rect object.
---
---@param tbl pdf.object.RectLike
---@return pdf.object.Rect
function pdf.object.rect(tbl) end

---@class pdf.object.Shape
---@field [number] pdf.common.Point
---@field type "shape"
---@field depth integer|nil
---@field fill_color pdf.common.Color|nil
---@field outline_color pdf.common.Color|nil
---@field outline_thickness number|nil
---@field mode pdf.common.PaintMode|nil
---@field order pdf.common.WindingOrder|nil
---@field dash_pattern pdf.common.line.DashPattern|nil
---@field cap_style pdf.common.line.CapStyle|nil
---@field join_style pdf.common.line.JoinStyle|nil
---@field link pdf.common.Link|nil
local PdfObjectShape = {}

---Aligns the shape to the provided bounds, returning an updated shape.
---@param bounds pdf.common.Bounds
---@param align pdf.common.Align
---@return pdf.object.Shape
function PdfObjectShape:align_to(bounds, align) end

---Calculates the bounds that contain all points within the shape.
---@return pdf.common.Bounds
function PdfObjectShape:bounds() end

---@class pdf.object.ShapeLike
---@field [number] pdf.common.PointLike
---@field depth integer|nil
---@field fill_color pdf.common.ColorLike|nil
---@field outline_color pdf.common.ColorLike|nil
---@field outline_thickness number|nil
---@field mode pdf.common.PaintMode|nil
---@field order pdf.common.WindingOrder|nil
---@field dash_pattern pdf.common.line.DashPatternLike|nil
---@field cap_style pdf.common.line.CapStyle|nil
---@field join_style pdf.common.line.JoinStyle|nil
---@field link pdf.common.LinkLike|nil

---Creates a new shape object.
---
---@param tbl pdf.object.ShapeLike
---@return pdf.object.Shape
function pdf.object.shape(tbl) end

---@class pdf.object.Text
---@field type "text"
---@field x number
---@field y number
---@field text string
---@field depth integer|nil
---@field font integer|nil
---@field size number|nil
---@field color pdf.common.Color|nil
---@field link pdf.common.Link|nil
local PdfObjectText = {}

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

---@class pdf.object.TextLikeBase
---@field text string
---@field depth integer|nil
---@field font integer|nil
---@field size number|nil
---@field color pdf.common.ColorLike|nil
---@field link pdf.common.LinkLike|nil

---@class pdf.object.TextLike1: pdf.object.TextLikeBase
---@field x number
---@field y number

---@class pdf.object.TextLike2: pdf.object.TextLikeBase
---@field [1] number
---@field [2] number

---@class pdf.object.TextLike3: pdf.object.TextLikeBase
---@field [1] {[1]:number, [2]:number}

---@alias pdf.object.TextLike
---| pdf.object.TextLike1
---| pdf.object.TextLike2
---| pdf.object.TextLike3
---| pdf.object.TextLikeBase

---Creates a new text object.
---
---@param tbl pdf.object.TextLike
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

---Creates a bounds instance, or throws an error if invalid.
---@param tbl pdf.common.BoundsLike
---@return pdf.common.Bounds
function pdf.utils.bounds(tbl) end

---Creates a color instance, or throws an error if invalid.
---@param tbl pdf.common.ColorLike
---@return pdf.common.Color
function pdf.utils.color(tbl) end

---Creates a link instance, or throws an error if invalid.
---@param tbl pdf.common.LinkLike
---@return pdf.common.Link
function pdf.utils.link(tbl) end

---Creates a point instance, or throws an error if invalid.
---@param tbl pdf.common.PointLike
---@return pdf.common.Point
function pdf.utils.point(tbl) end

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
