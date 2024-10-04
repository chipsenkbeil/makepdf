-------------------------------------------------------------------------------
-- PLANNER
--
-- Recreation of a physical planner that I often used, specialized towards
-- fitting the Supernote A6 X2 Nomad.
-------------------------------------------------------------------------------

-- GENERAL CONFIGURATION --

-- NOTE: Font size is in points and not millimeters!
pdf.page.font_size = 6.5

-- CONSTANTS --

---Padding to apply to the page's bounds before adding objects to the page.
---@type pdf.common.PaddingLike
local PAGE_PADDING = 5

---Spacing to apply between items on the same row.
---@type pdf.common.PaddingLike
local SPACING = 0.5

-------------------------------------------------------------------------------
-- UTILITY FUNCTIONS
-------------------------------------------------------------------------------

---Makes a section for the specified bounds and label.
---@param opts {bounds:pdf.common.Bounds, label:string, on_inner?:fun(opts:{bounds:pdf.common.Bounds, group:pdf.object.Group})}
---@return pdf.object.Group
local function make_section(opts)
    return pdf.object.section({
        bounds = opts.bounds,
        header = { text = opts.label, foreground = "#FFFFFF" },
        outline_dash_pattern = "dashed:1",
        outline_color = pdf.page.fill_color,
        outline_thickness = 0,
        padding = 1,
        on_inner = opts.on_inner,
    })
end

---Makes a section containing numbered lines for the specified bounds and label.
---@param opts {bounds:pdf.common.Bounds, label:string, lines:string[]}
---@return pdf.object.Group
local function make_lined_section(opts)
    local lines = opts.lines
    return make_section({
        bounds = opts.bounds,
        label = opts.label,
        on_inner = function(opts)
            local bounds = opts.bounds
            local group = opts.group
            table.insert(group, pdf.object.lined_list({
                bounds = bounds,
                rows = lines,
                line_color = pdf.page.fill_color,
                text_color = pdf.page.fill_color,
            }))
        end,
    })
end

---Makes a line with some optional text filled in the center.
---@param opts {bounds:pdf.common.Bounds, text?:string}
---@return pdf.object.Group
local function make_single_line(opts)
    return pdf.object.lined_list({
        bounds = opts.bounds,
        rows = { opts.text or "" },
        align = { h = "middle" },
        line_color = pdf.page.fill_color,
        text_color = pdf.page.fill_color,
    })
end

---Makes a habit rect that fits in the bottom-right of the bounds.
---@param opts {bounds:pdf.common.Bounds}
---@return pdf.object.Group
local function make_habit_rect(opts)
    local bounds = opts.bounds
    local hb = bounds:scale_by_factor({ width = 0.25, height = 0.25 })
    return pdf.object.rect_text({
        rect = {
            ll = hb.ll,
            ur = hb.ur,
            fill_color = pdf.utils.color(pdf.page.fill_color):lighten(0.5),
        },
        text = { text = "H", color = "#FFFFFF" },
    }):align_to(bounds, {
        v = "bottom",
        h = "right",
    })
end

---Makes a heading for the specified bounds.
---@param opts {bounds:pdf.common.Bounds, label:string, link?:pdf.common.LinkLike}
---@return pdf.object.Group
local function make_heading(opts)
    local bounds = opts.bounds
    local label = opts.label
    local link = opts.link

    return pdf.object.rect_text({
        rect = { ll = bounds.ll, ur = bounds.ur },
        text = { text = label, color = "#FFFFFF" },
        link = link,
    })
end

---Makes a section with a cross containing four items.
---@param opts {bounds:pdf.common.Bounds, label:string, items:string[]}
---@return pdf.object.Group
local function make_cross_section(opts)
    local items = opts.items
    return make_section({
        bounds = opts.bounds,
        label = opts.label,
        on_inner = function(opts)
            local bounds = opts.bounds
            local group = opts.group

            -- Place lines that go down the center of the grid
            table.insert(group, pdf.object.line({
                { bounds.ll.x + (bounds:width() / 2), bounds.ll.y },
                { bounds.ll.x + (bounds:width() / 2), bounds.ur.y },
                color = pdf.page.fill_color,
            }))
            table.insert(group, pdf.object.line({
                { bounds.ll.x, (bounds.ll.y + bounds:height() / 2) },
                { bounds.ur.x, (bounds.ll.y + bounds:height() / 2) },
                color = pdf.page.fill_color,
            }))

            -- Add our text labels into the top-left corner of each grid cell
            local g = pdf.utils.grid({ bounds = bounds, rows = 2, columns = 2, padding = 1 })
            table.insert(group, pdf.object.text({ text = items[1] })
                :align_to(g.cell({ row = 1, col = 1 }), { v = "top", h = "left" }))
            table.insert(group, pdf.object.text({ text = items[2] })
                :align_to(g.cell({ row = 1, col = 2 }), { v = "top", h = "left" }))
            table.insert(group, pdf.object.text({ text = items[3] })
                :align_to(g.cell({ row = 2, col = 1 }), { v = "top", h = "left" }))
            table.insert(group, pdf.object.text({ text = items[4] })
                :align_to(g.cell({ row = 2, col = 2 }), { v = "top", h = "left" }))
        end,
    })
end

---@class (exact) planner.daily-circles.Days
---@field M pdf.runtime.PageId|nil # id of page to link for Monday
---@field T pdf.runtime.PageId|nil # id of page to link for Tuesday
---@field W pdf.runtime.PageId|nil # id of page to link for Wednesday
---@field R pdf.runtime.PageId|nil # id of page to link for Thursday
---@field F pdf.runtime.PageId|nil # id of page to link for Friday
---@field S pdf.runtime.PageId|nil # id of page to link for Saturday
---@field U pdf.runtime.PageId|nil # id of page to link for Sunday

---Creates a series of circles, each with the letter of the day and a link to the page.
---
---Order is Monday, Tuesday, ..., Sunday.
---@param opts {bounds:pdf.common.Bounds, days: planner.daily-circles.Days, padding?:pdf.common.PaddingLike}
---@return pdf.object.Group
local function make_daily_circles(opts)
    local grid = pdf.utils.grid({
        bounds = opts.bounds,
        rows = 1,
        columns = 7,
        padding = opts.padding,
    })

    local objects = {}
    for i, letter in ipairs({ "M", "T", "W", "R", "F", "S", "U" }) do
        local bounds = grid.cell({ row = 1, col = i })
        local page_id = opts.days[letter]
        if page_id then
            table.insert(objects, pdf.object.group({
                pdf.object.circle({
                    center = {
                        x = bounds.ll.x + (bounds:width() / 2),
                        y = bounds.ll.y + (bounds:height() / 2),
                    },
                    radius = math.min(bounds:width(), bounds:height()) / 2,
                }),
                pdf.object.text({
                    text = letter,
                    color = "#FFFFFF",
                }):align_to(bounds, { v = "middle", h = "middle" }),
                link = pdf.utils.link(page_id) or nil,
            }))
        end
    end
    return pdf.object.group(objects)
end

-------------------------------------------------------------------------------
-- PLANNER SETUP
-------------------------------------------------------------------------------

local planner = pdf.pages.setup_planner()

-------------------------------------------------------------------------------
-- MONTHLY PAGES
-------------------------------------------------------------------------------

planner:for_monthly_page(function(page, date)
    pdf.log.debug("Populating monthly page", date:format("%B %Y"))

    local grid = pdf.utils.grid({
        bounds = pdf.page:bounds():with_padding(PAGE_PADDING),
        rows = 35,
        columns = 7,
    })

    -- Build out our top line that includes the month, focus, and habit
    page.push(make_heading({
        bounds = grid.cell({ row = 1, col = 1 }):with_padding(SPACING),
        label = "MONTH",
    }))
    page.push(make_single_line({
        bounds = grid.cell({ row = 1, col = 2, width = 2 }):with_padding(SPACING),
        text = date:format("%B %Y"),
    }))
    page.push(make_heading({
        bounds = grid.cell({ row = 1, col = 4 }):with_padding(SPACING),
        label = "FOCUS",
    }))
    page.push(make_single_line({
        bounds = grid.cell({ row = 1, col = 5 }):with_padding(SPACING),
    }))
    page.push(make_heading({
        bounds = grid.cell({ row = 1, col = 6 }):with_padding(SPACING),
        label = "HABIT",
    }))
    page.push(make_single_line({
        bounds = grid.cell({ row = 1, col = 7 }):with_padding(SPACING),
    }))

    -- Build our calendar
    page.push(pdf.object.calendar({
        bounds = grid.cell({ row = 3, col = 1, width = 7, height = 21 }),
        month = date,

        -- If we have a date for a block, link to the daily page
        -- and inject a habits box in the bottom-right
        on_day_block = function(opts)
            if opts.date then
                local target_page = planner:get_daily_page(opts.date)
                if target_page then
                    opts.group.link = pdf.utils.link(target_page.id)
                end
                table.insert(opts.group, make_habit_rect({
                    bounds = opts.group:bounds(),
                }))
            end
        end
    }))

    -- Build our planning section
    page.push(make_heading({
        bounds = grid.cell({ row = 25, col = 1, width = 7 }),
        label = "PLAN",
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 26.25, col = 1, width = 3.25, height = 4 }),
        label = "THIS MONTH'S GOALS",
        lines = { "1", "2", "3" },
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 26.25, col = 4.75, width = 3.25, height = 4 }),
        label = "DISTRACTIONS TO AVOID",
        lines = { "1", "2", "3" },
    }))

    -- Build our review section
    page.push(make_heading({
        bounds = grid.cell({ row = 31, col = 1, width = 7 }),
        label = "REVIEW",
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 32.25, col = 1, width = 3.25, height = 4 }),
        label = "THIS MONTH'S WINS",
        lines = { "1", "2", "3" },
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 32.25, col = 4.75, width = 3.25, height = 4 }),
        label = "INSIGHTS GAINED",
        lines = { "1", "2", "3" },
    }))
end)

-------------------------------------------------------------------------------
-- WEEKLY PAGES
-------------------------------------------------------------------------------

planner:for_weekly_page(function(page, date)
    local start_of_week, end_of_week = pdf.utils.start_end_week(date)
    local week_str = date:format("%b %d") .. " to " .. end_of_week:format("%b %d")
    pdf.log.debug("Populating weekly page", week_str .. date:format(" %Y"))

    local grid = pdf.utils.grid({
        bounds = pdf.page:bounds():with_padding(PAGE_PADDING),
        rows = 35,
        columns = 7,
    })

    -- Build out the days based on the weekday
    ---@type planner.daily-circles.Days
    local days = {}
    for i = 0, end_of_week.ordinal - start_of_week.ordinal do
        local day_date = assert(start_of_week:add_days(i))
        local day_page = planner:get_daily_page(day_date)
        if day_page then
            local weekday = day_date.weekday:long_name()
            if weekday == "monday" then
                days.M = day_page.id
            elseif weekday == "tuesday" then
                days.T = day_page.id
            elseif weekday == "wednesday" then
                days.W = day_page.id
            elseif weekday == "thursday" then
                days.R = day_page.id
            elseif weekday == "friday" then
                days.F = day_page.id
            elseif weekday == "saturday" then
                days.S = day_page.id
            elseif weekday == "sunday" then
                days.U = day_page.id
            end
        end
    end

    -- Build out our top line that includes the week with links to month and days
    page.push(make_heading({
        bounds = grid.cell({ row = 1, col = 1 }):with_padding(SPACING),
        label = "WEEK",
    }))
    page.push(pdf.object.group({
        make_single_line({
            bounds = grid.cell({ row = 1, col = 2, width = 2 }):with_padding(SPACING),
            text = week_str,
        }),
        link = pdf.utils.link(planner:get_monthly_page(date).id),
    }))
    page.push(make_daily_circles({
        bounds = grid.cell({ row = 1, col = 4, width = 4 }):with_padding(SPACING),
        days = days,
    }))

    -- Build our reflection section
    page.push(make_heading({
        bounds = grid.cell({ row = 3, col = 1, width = 7 }),
        label = "REFLECTION FROM LAST WEEK",
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 4.25, col = 1, width = 3.25, height = 4 }),
        label = "BIG WINS",
        lines = { "1", "2", "3" },
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 4.25, col = 4.75, width = 3.25, height = 4 }),
        label = "HOW I'LL IMPROVE",
        lines = { "1", "2", "3" },
    }))

    -- Build our planning section
    page.push(make_heading({
        bounds = grid.cell({ row = 9, col = 1, width = 7 }),
        label = "PLANNING FOR THIS WEEK",
    }))
    page.push(make_cross_section({
        bounds = grid.cell({ row = 10.25, col = 1, width = 7, height = 16 }),
        label = "THINGS I WILL DO TO MAKE THIS WEEK GREAT",
        items = { "PERSONAL", "WORK", "FAMILY / FRIENDS", "RELATIONSHIP" },
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 27, col = 1, width = 3.25, height = 4 }),
        label = "I'M LOOKING FORWARD TO",
        lines = { "1", "2", "3" },
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 27, col = 4.75, width = 3.25, height = 4 }),
        label = "HABITS I'M FOCUSING ON DEVELOPING",
        lines = { "1", "2", "3" },
    }))
    page.push(make_section({
        bounds = grid.cell({ row = 32, col = 1, width = 3.25, height = 4 }),
        label = "LEARN SOMETHING NEW",
    }))
    page.push(make_section({
        bounds = grid.cell({ row = 32, col = 4.75, width = 3.25, height = 4 }),
        label = "PASSION PROJECT",
    }))
end)

-------------------------------------------------------------------------------
-- DAILY PAGES
-------------------------------------------------------------------------------

planner:for_daily_page(function(page, date)
    pdf.log.debug("Populating daily page", date:format("%B %d, %Y (%a)"))

    local grid = pdf.utils.grid({
        bounds = pdf.page:bounds():with_padding(PAGE_PADDING),
        rows = 35,
        columns = 7,
    })

    -- Build out our top line that includes the day with links to month & week
    page.push(make_heading({
        bounds = grid.cell({ row = 1, col = 1 }):with_padding(SPACING),
        label = "DAY",
    }))
    page.push(pdf.object.group({
        make_single_line({
            bounds = grid.cell({ row = 1, col = 2, width = 2 }):with_padding(SPACING),
            text = date:format("%B %d (%a)"),
        }),
    }))
    page.push(make_heading({
        bounds = grid.cell({ row = 1, col = 4, width = 2 }):with_padding(SPACING),
        label = "GO TO MONTH",
        link = planner:get_monthly_page(date).id,
    }))
    page.push(make_heading({
        bounds = grid.cell({ row = 1, col = 6, width = 2 }):with_padding(SPACING),
        label = "GO TO WEEK",
        link = planner:get_weekly_page(date).id,
    }))

    -- Build our morning review section
    page.push(make_heading({
        bounds = grid.cell({ row = 3, col = 1, width = 7 }),
        label = "MORNING REVIEW",
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 4.25, col = 1, width = 3.25, height = 4 }),
        label = "I'M GRATEFUL FOR",
        lines = { "1", "2", "3" },
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 4.25, col = 4.75, width = 3.25, height = 4 }),
        label = "I'M EXCITED ABOUT",
        lines = { "1", "2", "3" },
    }))
    page.push(make_section({
        bounds = grid.cell({ row = 9, col = 1, width = 2.2, height = 3 }),
        label = "AFFIRMATION",
    }))
    page.push(make_section({
        bounds = grid.cell({ row = 9, col = 3.4, width = 2.2, height = 3 }),
        label = "FOCUS",
    }))
    page.push(make_section({
        bounds = grid.cell({ row = 9, col = 5.8, width = 2.2, height = 3 }),
        label = "EXERCISE",
    }))

    -- Build today's priorities section
    page.push(make_heading({
        bounds = grid.cell({ row = 13, col = 1, width = 7 }),
        label = "TODAY'S PRIORITIES",
    }))
    page.push(make_section({
        bounds = grid.cell({ row = 14.25, col = 1, width = 3.25, height = 6 }),
        label = "PRIORITY 1",
    }))
    page.push(make_section({
        bounds = grid.cell({ row = 14.25, col = 4.75, width = 3.25, height = 6 }),
        label = "PRIORITY 2",
    }))
    page.push(make_section({
        bounds = grid.cell({ row = 20.5, col = 1, width = 3.25, height = 6 }),
        label = "PRIORITY 3",
    }))
    page.push(make_section({
        bounds = grid.cell({ row = 20.5, col = 4.75, width = 3.25, height = 6 }),
        label = "PRIORITY 4",
    }))

    -- Build end of day review section
    page.push(make_heading({
        bounds = grid.cell({ row = 27, col = 1, width = 7 }),
        label = "END OF DAY REVIEW",
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 28.25, col = 1, width = 7, height = 4 }),
        label = "TODAY'S WINS",
        lines = { "1", "2", "3" },
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 33, col = 1, width = 7, height = 3 }),
        label = "HOW I'LL IMPROVE",
        lines = { "1", "2" },
    }))
end)
