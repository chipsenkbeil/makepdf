-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
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
---@param opts {bounds:pdf.common.Bounds, label:string}
---@return pdf.object.Group
local function make_heading(opts)
    local bounds = opts.bounds
    local label = opts.label

    return pdf.object.rect_text({
        rect = { ll = bounds.ll, ur = bounds.ur },
        text = { text = label, color = "#FFFFFF" },
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

---Creates a series of circles, each with the letter of the day and a link to the page.
---
---Order is Monday, Tuesday, ..., Sunday.
---@param opts {bounds:pdf.common.Bounds, day_pages: pdf.runtime.Page[], padding?:pdf.common.PaddingLike}
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
        local page = opts.day_pages[i]
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
            link = page and pdf.utils.link(page.id) or nil,
        }))
    end
    return pdf.object.group(objects)
end

-------------------------------------------------------------------------------
-- MONTHLY PAGES
-------------------------------------------------------------------------------

pdf.hooks.on_monthly_page(function(page)
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
        text = page.date:format("%B %Y"),
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
        month = page.date,

        -- If we have a date for a block, link to the daily page
        -- and inject a habits box in the bottom-right
        on_day_block = function(opts)
            if opts.date then
                local target_page = page.daily(opts.date)
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

pdf.hooks.on_weekly_page(function(page)
    local grid = pdf.utils.grid({
        bounds = pdf.page:bounds():with_padding(PAGE_PADDING),
        rows = 35,
        columns = 7,
    })

    -- Build out our top line that includes the week with links to month and days
    page.push(make_heading({
        bounds = grid.cell({ row = 1, col = 1 }):with_padding(SPACING),
        label = "WEEK",
    }))
    page.push(pdf.object.group({
        make_single_line({
            bounds = grid.cell({ row = 1, col = 2, width = 2 }):with_padding(SPACING),
            text = page.date:format("%B %d, %Y (%a)"),
        }),
        link = pdf.utils.link(page.monthly().id),
    }))
    page.push(make_daily_circles({
        bounds = grid.cell({ row = 1, col = 4, width = 4 }):with_padding(SPACING),
        day_pages = {
            page.daily(page.date:add_days(0)),
            page.daily(page.date:add_days(1)),
            page.daily(page.date:add_days(2)),
            page.daily(page.date:add_days(3)),
            page.daily(page.date:add_days(4)),
            page.daily(page.date:add_days(5)),
            page.daily(page.date:add_days(6)),
        },
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

pdf.hooks.on_daily_page(function(page)
    local bounds = pdf.page:bounds()
    local text = pdf.object.text({
        text = "Day " .. page.date:format("%F"),
    })
    page.push(text:align_to(bounds, { v = "middle", h = "middle" }))
end)
