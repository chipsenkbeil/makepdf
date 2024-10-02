-------------------------------------------------------------------------------
-- STDLIB
--
-- Executed prior to the user script, enabling standard library implementations
-- that are written in Lua. This is designed as faster turnaround than Rust.
-------------------------------------------------------------------------------

-------------------------------------------------------------------------------
-- OBJECT ENHANCEMENTS
-------------------------------------------------------------------------------

---@class pdf.object.RectTextLike
---@field rect? pdf.object.RectLike #custom rect configuration
---@field text? string|pdf.object.TextLikeBase #custom text configuration
---@field align? pdf.common.Align #where to place the text relative to the rect, after padding factored
---@field margin? pdf.common.PaddingLike #padding applied to the rect bounds before the rect is created
---@field padding? pdf.common.PaddingLike #padding applied to the text within the rect before created
---@field link? pdf.common.LinkLike #link to associate with the object

---Creates a group containing a rect and text overlayed on top.
---@param tbl pdf.object.RectTextLike
---@return pdf.object.Group
function pdf.object.rect_text(tbl)
    ---@type pdf.object.GroupLike
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

    -- Associate a link with the group of objects
    objects.link = tbl.link

    -- Build a group comprising the two together
    return pdf.object.group(objects)
end

---@class pdf.object.SectionArgs
---@field bounds pdf.common.Bounds
---@field header? {text?:string, background?:pdf.common.ColorLike, foreground?:pdf.common.ColorLike, height?:number}
---@field padding? pdf.common.PaddingLike #padding applied to the items within the section
---@field outline_color? pdf.common.ColorLike
---@field outline_thickness? number
---@field outline_dash_pattern? pdf.common.line.DashPatternLike
---@field outline_cap_style? pdf.common.line.CapStyle
---@field outline_join_style? pdf.common.line.JoinStyle
---@field on_inner? fun(opts:{bounds:pdf.common.Bounds, group:pdf.object.Group})

---Creates a section denoted by a header, outline, and items padded within.
---
---@param tbl pdf.object.SectionArgs
---@return pdf.object.Group
function pdf.object.section(tbl)
    local objects = {}

    local bounds = tbl.bounds
    local header = tbl.header or {}
    local padding = tbl.padding or {}
    local on_inner = tbl.on_inner or function() end

    local header_height = header.height
        or pdf.object.text({ text = header.text or "" }):bounds():height()

    -- First we create a header at the top of the bounds; if no height is
    -- specified, we will use the height of the text
    table.insert(objects, pdf.object.rect_text({
        rect = {
            mode = "fill",
            ll = {
                x = bounds.ll.x,
                y = bounds.ur.y - header_height,
            },
            ur = bounds.ur,
            fill_color = header.background,
        },
        text = {
            text = header.text,
            color = header.foreground,
        },
    }))

    -- Secnd, we create a three lines representing the outline
    -- of the section
    table.insert(objects, pdf.object.line({
        { x = bounds.ll.x, y = bounds.ur.y - header_height },
        { x = bounds.ll.x, y = bounds.ll.y },
        { x = bounds.ur.x, y = bounds.ll.y },
        { x = bounds.ur.x, y = bounds.ur.y - header_height },
        color = tbl.outline_color,
        thickness = tbl.outline_thickness,
        dash_pattern = tbl.outline_dash_pattern,
        cap_style = tbl.outline_cap_style,
        join_style = tbl.outline_join_style,
    }))

    -- Third, we populate within the section using the callback
    local group = pdf.object.group({})
    on_inner({
        bounds = bounds
            :scale_to({ height = bounds:height() - header_height })
            :with_padding(padding),
        group = group
    })
    table.insert(objects, group)

    return pdf.object.group(objects)
end

---@class pdf.object.LinedListArgs
---@field bounds pdf.common.Bounds
---@field rows string[]
---@field align? pdf.common.Align
---@field line_color? pdf.common.ColorLike
---@field text_color? pdf.common.ColorLike

---Creates a group representing a series of lines pre-filled
---with the text provided within `rows`.
---
---Defaults to aligning text vertically-centered and horizontally-left.
---@param tbl pdf.object.LinedListArgs
---@return pdf.object.Group
function pdf.object.lined_list(tbl)
    local objects = {}
    local align = tbl.align or {}
    local bounds = tbl.bounds
    local rows = tbl.rows
    local grid = pdf.utils.grid({
        bounds = bounds,
        rows = #rows,
        columns = 1,
    })

    for i = 1, #rows do
        local cell = grid.cell({ row = i, col = 1 })

        -- Create the blank line that extends the bottom
        -- of the cell within the grid
        table.insert(objects, pdf.object.line({
            cell.ll,
            cell:lr(),
            color = tbl.line_color,
        }))

        -- For our input rows, add them as text on top of the lines
        -- if they are not empty
        if type(rows[i]) == "string" and rows[i] ~= "" then
            local text = pdf.object.text({
                text = rows[i],
                color = tbl.text_color,
            }):align_to(cell, { h = align.h or "left", v = align.v })
            table.insert(objects, text)
        end
    end

    return pdf.object.group(objects)
end

---@class pdf.object.CalendarArgs
---@field bounds pdf.common.Bounds
---@field month pdf.common.Date
---@field fill_color? pdf.common.ColorLike
---@field text_color? pdf.common.ColorLike
---@field outline_thickness? number
---@field on_day_block? fun(opts:{date?:pdf.common.Date, group:pdf.object.Group})

---Creates a calendar-like group of objects for the specified `month` that fits into `bounds`.
---
---Calendar starts with Sunday as first day of the week.
---@param tbl pdf.object.CalendarArgs
---@return pdf.object.Group
function pdf.object.calendar(tbl)
    ---@type pdf.Object[]
    local objects = {}
    local month = tbl.month
    local on_day_block = tbl.on_day_block

    -- Text color for text placed on top of filled rects
    local fill_color = tbl.fill_color or pdf.page.fill_color
    local text_color = tbl.text_color

    -- Default outline thickness of blocks to 1px (via 0)
    local outline_thickness = tbl.outline_thickness or 0

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
            local date = is_valid_block
                and month:beginning_of_month():add_days(day_num - 1)
                or nil

            -- Create a new, empty group to add date's objects
            local group = pdf.object.group({})

            -- Create the container block for the day
            local block = cell_rect_text({
                row = week_of_month * 2,
                col = day_of_week,
                height = 2,
            }, {
                rect = {
                    fill_color = is_valid_block and fill_color or invalid_fill_color,
                    outline_color = fill_color,
                    outline_thickness = outline_thickness,
                    mode = is_valid_block and "stroke" or "fill_stroke",
                }
            })
            table.insert(group, block)

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
                table.insert(group, day)
            end

            if on_day_block then
                on_day_block({
                    date = date,
                    group = group,
                })
            end

            table.insert(objects, group)
        end
    end

    return pdf.object.group(objects)
end

-------------------------------------------------------------------------------
-- PAGES ENHANCEMENTS
-------------------------------------------------------------------------------

---Creates a series of pages representing a common planner, returning a mapping
---of dates to pages.
---@param opts? {year?:integer}
---@return pdf.pages.Planner
function pdf.pages.setup_planner(opts)
    opts = opts or {}

    -- Get the configured year
    local year = opts.year or pdf.planner.year

    -- Set our starting date for pages to beginning of the year
    local start_date = pdf.utils.date({ year = year, month = 1, day = 1 })

    ---@class pdf.pages.Planner
    ---@field months {date:pdf.common.Date, id:pdf.runtime.PageId}[] # ids of pages for months (from 1 to 12)
    ---@field weeks {date:pdf.common.Date, id:pdf.runtime.PageId}[] # ids of pages for weeks (from 1 up to 53)
    ---@field days {date:pdf.common.Date, id:pdf.runtime.PageId}[] # ids of pages for days (from 1 up to 366)
    local M = {
        months = {},
        weeks = {},
        days = {},
    }

    ---Retrieves the monthly page for the given `date` or `id`.
    ---@param date_or_id pdf.common.DateLike|pdf.runtime.PageId
    ---@return pdf.runtime.Page|nil
    function M:get_monthly_page(date_or_id)
        if type(date_or_id) == "number" then
            return pdf.pages.get(date_or_id)
        else
            ---@cast date_or_id -integer
            local date = pdf.utils.date(date_or_id)
            local page = self.months[date.month]
            if page then
                return pdf.pages.get(page.id)
            end
        end
    end

    ---Retrieves the weekly page for the given `date` or `id`.
    ---@param date_or_id pdf.common.DateLike|pdf.runtime.PageId
    ---@return pdf.runtime.Page|nil
    function M:get_weekly_page(date_or_id)
        if type(date_or_id) == "number" then
            return pdf.pages.get(date_or_id)
        else
            ---@cast date_or_id -integer
            local date = pdf.utils.date(date_or_id)
            local page = self.weeks[date:calendar_week_monday()]
            if page then
                return pdf.pages.get(page.id)
            end
        end
    end

    ---Retrieves the daily page for the given `date` or `id`.
    ---@param date_or_id pdf.common.DateLike|pdf.runtime.PageId
    ---@return pdf.runtime.Page|nil
    function M:get_daily_page(date_or_id)
        if type(date_or_id) == "number" then
            return pdf.pages.get(date_or_id)
        else
            ---@cast date_or_id -integer
            local date = pdf.utils.date(date_or_id)
            local page = self.days[date.ordinal]
            if page then
                return pdf.pages.get(page.id)
            end
        end
    end

    ---Executes some function for each monthly page created.
    ---@param f fun(page:pdf.runtime.Page, date:pdf.common.Date)
    function M:for_monthly_page(f)
        for i = 1, 12 do
            local x = self.months[i]
            if x then
                local page = pdf.pages.get(x.id)
                if page then
                    f(page, x.date)
                end
            end
        end
    end

    ---Executes some function for each weekly page created.
    ---@param f fun(page:pdf.runtime.Page, date:pdf.common.Date)
    function M:for_weekly_page(f)
        for i = 1, 53 do
            local x = self.weeks[i]
            if x then
                local page = pdf.pages.get(x.id)
                if page then
                    f(page, x.date)
                end
            end
        end
    end

    ---Executes some function for each daily page created.
    ---@param f fun(page:pdf.runtime.Page, date:pdf.common.Date)
    function M:for_daily_page(f)
        for i = 1, 366 do
            local x = self.days[i]
            if x then
                local page = pdf.pages.get(x.id)
                if page then
                    f(page, x.date)
                end
            end
        end
    end

    -- Create monthly pages
    if pdf.planner.monthly.enabled then
        ---@type pdf.common.Date|nil
        local date = start_date
        while date and date.year == year do
            -- e.g. January 2024
            local id = pdf.pages.create(date:format("%B %Y"))
            table.insert(M.months, { date = date, id = id })

            date = date:next_month()
        end
    end

    -- Create weekly pages
    if pdf.planner.weekly.enabled then
        ---@type pdf.common.Date|nil
        local date = start_date

        while date and date.year == year do
            -- e.g. Week 1 2024
            local id = pdf.pages.create(date:format("Week %V %Y"))
            table.insert(M.weeks, { date = date, id = id })

            date = date:next_week()
        end
    end

    -- Create daily pages
    if pdf.planner.daily.enabled then
        ---@type pdf.common.Date|nil
        local date = start_date

        while date and date.year == year do
            -- e.g. 2024-09-15 (Sunday)
            local id = pdf.pages.create(date:format("%v (%A)"))
            table.insert(M.days, { date = date, id = id })

            date = date:tomorrow()
        end
    end

    return M
end

-------------------------------------------------------------------------------
-- UTILS ENHANCEMENTS
-------------------------------------------------------------------------------

---Creates a grid of rows x columns for some bounds that can
---be used to create sub-bounds for cells within the grid.
---
---Takes optional padding to apply to each cell when retrieving bounds.
---@param tbl {bounds:pdf.common.Bounds, rows:integer, columns:integer, padding?:pdf.common.PaddingLike}
---@return pdf.utils.Grid
function pdf.utils.grid(tbl)
    local GRID_BOUNDS = tbl.bounds
    local GRID_WIDTH = GRID_BOUNDS:width()
    local GRID_HEIGHT = GRID_BOUNDS:height()
    local NUM_ROWS = tbl.rows
    local NUM_COLS = tbl.columns
    local ROW_HEIGHT = GRID_HEIGHT / NUM_ROWS
    local COL_WIDTH = GRID_WIDTH / NUM_COLS
    local CELL_PADDING = tbl.padding

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
        }):with_padding(CELL_PADDING)
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

---For a given date, returns the start and end of the week that is within
---the same calendar year.
---@param date pdf.common.DateLike
---@return pdf.common.Date start_of_week, pdf.common.Date end_of_week
function pdf.utils.start_end_week(date)
    date = pdf.utils.date(date)

    -- Figure out the start & end of the week that are within the calendar year
    local start_of_week = assert(date:beginning_of_week_monday())
    while start_of_week.year < date.year do
        start_of_week = assert(start_of_week:tomorrow())
    end
    local end_of_week = assert(date:end_of_week_monday())
    while end_of_week.year > date.year do
        end_of_week = assert(end_of_week:yesterday())
    end

    return start_of_week, end_of_week
end
