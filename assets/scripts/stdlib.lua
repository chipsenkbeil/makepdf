-------------------------------------------------------------------------------
-- STDLIB
--
-- Executed prior to the user script, enabling standard library implementations
-- that are written in Lua. This is designed as faster turnaround than Rust.
-------------------------------------------------------------------------------

---@class pdf.object.RectTextLike
---@field rect? pdf.object.RectLike
---@field text? string|pdf.object.TextLikeBase
---@field align? pdf.common.Align #where to place the text relative to the rect, after padding factored
---@field margin? pdf.common.PaddingLike #padding applied to the rect bounds before the rect is created
---@field padding? pdf.common.PaddingLike #padding applied to the text within the rect before created

---Creates a group containing a rect and text overlayed on top.
---
---Supports configuring the text's alignment within the bounds of the rect.
---@param tbl pdf.object.RectTextLike
---@return pdf.object.Group
function pdf.object.rect_text(tbl)
    local objects = {}

    -- Create a rect from the provided configuration
    local rect = pdf.object.rect(tbl.rect or {})
    rect = rect:with_bounds(rect:bounds():with_padding(tbl.margin))
    table.insert(objects, rect)

    local text_args = tbl.text
    if type(text_args) == "string" then
        text_args = { text = text_args }
    end

    -- Create a text object aligned to the rect above
    if text_args then
        local text = pdf.object.text(text_args):align_to(
            rect:bounds():with_padding(tbl.padding),
            tbl.align or {
                h = "middle",
                v = "middle",
            }
        )
        table.insert(objects, text)
    end

    -- Build a group comprising the two together
    return pdf.object.group(objects)
end

---@class pdf.object.CalendarArgs
---@field bounds pdf.common.Bounds
---@field month pdf.common.Date
---@field fill_color? pdf.common.ColorLike
---@field text_color? pdf.common.ColorLike
---@field date_to_link? fun(date:pdf.common.Date):(pdf.common.LinkLike|nil)

---Creates a calendar-like object for the specified `month` that fits into `bounds`.
---
---Calendar starts with Sunday as first day of the week.
---@param tbl pdf.object.CalendarArgs
---@return pdf.object.Group
function pdf.object.calendar(tbl)
    ---@type pdf.Object[]
    local objects = {}
    local month = tbl.month
    local date_to_link = tbl.date_to_link

    -- Text color for text placed on top of filled rects
    local fill_color = tbl.fill_color or pdf.page.fill_color
    local text_color = tbl.text_color

    -- Determine default text color by lightness
    if not text_color then
        -- Check the fill color to determine what to use for text color
        local color = pdf.utils.color(fill_color)
        if color:is_light() then
            text_color = "#000000"
        else
            text_color = "#FFFFFF"
        end
    end

    -- Create a fill color for an invalid block in the calendar
    local invalid_fill_color = pdf.utils.color(fill_color)
    if invalid_fill_color:is_light() then
        local lum_left = 1.0 - invalid_fill_color:luminance()
        invalid_fill_color = invalid_fill_color:darken(lum_left * 0.5)
    else
        local lum_left = 1.0 - invalid_fill_color:luminance()
        invalid_fill_color = invalid_fill_color:lighten(lum_left * 0.5)
    end

    -- Create a grid of 7 columns (7 days) and 13 rows to fit the header plus
    -- enough rows (6 x 2 height) to handle all month variations
    local grid = pdf.utils.grid({
        bounds = tbl.bounds,
        rows = 13,
        columns = 7,
    })

    ---Creates a new rect text object fitting the cell bounds.
    ---@param opts? pdf.object.RectTextLike
    local cell_rect_text = grid.map_cell(function(bounds, opts)
        opts = opts or {}

        ---@type pdf.object.RectLike
        local rect_args = { ll = bounds.ll, ur = bounds.ur }

        -- Copy over rect-specific properties
        for key, value in pairs(opts.rect or {}) do
            rect_args[key] = value
        end

        return pdf.object.rect_text({
            rect = rect_args,
            text = opts.text or {},
        })
    end)

    -- Build our header for the days of the week
    for i, text in ipairs({
        "SUNDAY",
        "MONDAY",
        "TUESDAY",
        "WEDNESDAY",
        "THURSDAY",
        "FRIDAY",
        "SATURDAY"
    }) do
        table.insert(objects, cell_rect_text({ row = 1, col = i }, {
            rect = { fill_color = fill_color },
            text = { text = text, color = text_color },
        }))
    end

    -- Get beginning and end day of week for the month for a Sunday-based calendar
    -- indexed where Sunday = 1, Monday = 2, ...
    local month_start_day_of_week = month:last_month()
        :end_of_month()
        :tomorrow()
        .weekday
        :number_from_sunday()
    local month_end_day_of_week = month:next_month()
        :beginning_of_month()
        :yesterday()
        .weekday
        :number_from_sunday()
    local weeks_in_month = month:weeks_in_month_sunday()

    -- Build our 7 x 6 grid of calendar days
    for week_of_month = 1, 6 do
        for day_of_week = 1, 7 do
            -- Check if the day on the calendar is within our expected range,
            -- and if so display the date on the block, otherwise show nothing
            --
            -- We start with Sunday in our calendar!
            local is_valid_block =
                (week_of_month == 1 and day_of_week >= month_start_day_of_week)
                or (week_of_month == weeks_in_month and day_of_week <= month_end_day_of_week)
                or (week_of_month > 1 and week_of_month < weeks_in_month)

            -- Calculate the calendar number from 1 to 31 by looking at
            -- the raw number from 1 to 35 and subracting the start of
            -- the month and adding 1 to get the actual start num
            local day_num = ((week_of_month - 1) * 7)
                + day_of_week
                - (month_start_day_of_week - 1)

            -- Calculate date of day this represents, if valid
            local date = is_valid_block and month:beginning_of_month():add_days(day_num - 1)

            -- Create the container block for the day
            local block = cell_rect_text({
                row = week_of_month * 2,
                col = day_of_week,
                height = 2,
            }, {
                rect = {
                    fill_color = is_valid_block and fill_color or invalid_fill_color,
                    outline_color = fill_color,
                    mode = is_valid_block and "stroke" or "fill_stroke",
                    link = date and date_to_link and date_to_link(date) or nil,
                }
            })
            table.insert(objects, block)

            if is_valid_block then
                -- Make a block with text that is a quarter of the size of the day block
                local bounds = block:bounds():scale_by_factor({
                    width = 0.25,
                    height = 0.25,
                })

                -- Place the day as a number in the top-left (1/4 of size)
                local day = pdf.object.rect_text({
                    rect = { ll = bounds.ll, ur = bounds.ur, fill_color = fill_color },
                    text = { text = tostring(day_num), color = text_color, }
                }):align_to(block:bounds(), { v = "top", h = "left" })
                table.insert(objects, day)
            end
        end
    end

    return pdf.object.group(objects)
end

-------------------------------------------------------------------------------
-- UTILS
-------------------------------------------------------------------------------

---Creates a grid of rows x columns for some bounds that can
---be used to create sub-bounds for cells within the grid.
---
---@param tbl {bounds:pdf.common.Bounds, rows:integer, columns:integer}
---@return pdf.utils.Grid
function pdf.utils.grid(tbl)
    local GRID_BOUNDS = tbl.bounds
    local GRID_WIDTH = GRID_BOUNDS:width()
    local GRID_HEIGHT = GRID_BOUNDS:height()
    local NUM_ROWS = tbl.rows
    local NUM_COLS = tbl.columns
    local ROW_HEIGHT = GRID_HEIGHT / NUM_ROWS
    local COL_WIDTH = GRID_WIDTH / NUM_COLS

    ---@class pdf.utils.Grid
    local M = {}

    ---Returns a reference to the bounds of the grid.
    ---@return pdf.common.Bounds
    function M.bounds()
        return GRID_BOUNDS
    end

    ---Returns the total rows of the grid.
    ---@return integer
    function M.rows()
        return NUM_ROWS
    end

    ---Returns the total columns of the grid.
    ---@return integer
    function M.columns()
        return NUM_COLS
    end

    ---Returns the width of the grid.
    ---@return number
    function M.width()
        return GRID_WIDTH
    end

    ---Returns the height of the grid.
    ---@return number
    function M.height()
        return GRID_HEIGHT
    end

    ---Returns the height of each row.
    ---@return number
    function M.row_height()
        return ROW_HEIGHT
    end

    ---Returns the width of each column.
    ---@return number
    function M.column_width()
        return COL_WIDTH
    end

    ---Creates bounds representing a cell within the grid.
    ---
    ---An optional width and height can be provided to specify how many cells
    ---wide and tall the bounds should be, both defaulting to 1.
    ---@param opts {row:integer, col:integer, width?:integer, height?:integer}
    ---@return pdf.common.Bounds
    function M.cell(opts)
        opts = opts or {}
        local row = opts.row
        local col = opts.col
        local width = opts.width or 1
        local height = opts.height or 1

        assert(type(row) == "number", "numeric row required")
        assert(type(col) == "number", "numeric col required")

        ---@type pdf.common.Point
        local ll = {
            x = (col - 1) * COL_WIDTH,
            -- NOTE: Need to flip since 0 is bottom instead of top
            y = GRID_HEIGHT - (row * ROW_HEIGHT) - ((height - 1) * ROW_HEIGHT),
        }

        ---@type pdf.common.Point
        local ur = {
            x = ll.x + (COL_WIDTH * width),
            y = ll.y + (ROW_HEIGHT * height)
        }

        -- Adjust the bounds from 0,0 origin to fit our global bounds
        return pdf.utils.bounds({
            ll = { x = ll.x + GRID_BOUNDS.ll.x, y = ll.y + GRID_BOUNDS.ll.y },
            ur = { x = ur.x + GRID_BOUNDS.ll.x, y = ur.y + GRID_BOUNDS.ll.y },
        })
    end

    ---Creates a function that maps the bounds of a cell into something else.
    ---@generic T, U
    ---@param f fun(bounds:pdf.common.Bounds, opts?:T):U
    ---@return fun(args:{row:integer, col:integer, width?:integer, height?:integer}, opts?:T):U
    function M.map_cell(f)
        return function(args, opts)
            local bounds = M.cell(args)
            return f(bounds, opts)
        end
    end

    return M
end
