-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

-- GENERAL CONFIGURATION --

-- NOTE: Font size is in points and not millimeters!
pdf.page.font_size = 6.5

-- CONSTANTS --

---Spacing to apply between items on the same row.
---@type pdf.common.PaddingLike
local SPACING = 0.5

-------------------------------------------------------------------------------
-- UTILITY FUNCTIONS
-------------------------------------------------------------------------------

---Makes a section containing numbered lines for the specified bounds and label.
---@param opts {bounds:pdf.common.Bounds, label:string}
---@return pdf.object.Group
local function make_lined_section(opts)
    return pdf.object.section({
        bounds = opts.bounds,
        header = { text = opts.label, foreground = "#FFFFFF" },
        outline_dash_pattern = "dashed:1",
        outline_color = pdf.page.fill_color,
        outline_thickness = 0,
        padding = 1,
        on_inner = function(opts)
            local bounds = opts.bounds
            local group = opts.group
            table.insert(group, pdf.object.lined_list({
                bounds = bounds,
                rows = { "1", "2", "3" },
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

-------------------------------------------------------------------------------
-- MONTHLY PAGES
-------------------------------------------------------------------------------

pdf.hooks.on_monthly_page(function(page)
    local grid = pdf.utils.grid({
        bounds = pdf.page:bounds():with_padding(3.5),
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
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 26.25, col = 4.75, width = 3.25, height = 4 }),
        label = "DISTRACTIONS TO AVOID",
    }))

    -- Build our review section
    page.push(make_heading({
        bounds = grid.cell({ row = 31, col = 1, width = 7 }),
        label = "REVIEW",
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 32.25, col = 1, width = 3.25, height = 4 }),
        label = "THIS MONTH'S WINS",
    }))
    page.push(make_lined_section({
        bounds = grid.cell({ row = 32.25, col = 4.75, width = 3.25, height = 4 }),
        label = "INSIGHTS GAINED",
    }))
end)

-------------------------------------------------------------------------------
-- WEEKLY PAGES
-------------------------------------------------------------------------------

pdf.hooks.on_weekly_page(function(page)
    local bounds = pdf.page:bounds()
    local text = pdf.object.text({
        text = "Week " .. tostring(page.date.week),
    })
    page.push(text:align_to(bounds, { v = "middle", h = "middle" }))
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
