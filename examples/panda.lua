-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

-- NOTE: Font size is in points and not millimeters!
pdf.page.font_size = 6.5

-- NOTE: A thickness of 0 does a 1px outline!
pdf.page.outline_thickness = 0

---Creates a callback to populate a section with a numbered list.
---@return fun(opts:{bounds:pdf.common.Bounds, group:pdf.object.Group})
local function on_inner_make_list()
    return function(opts)
        local bounds = opts.bounds
        local group = opts.group
        local sgrid = pdf.utils.grid({
            bounds = bounds,
            rows = 6,
            columns = 1,
        })

        for i = 1, 6 do
            local cell = sgrid.cell({ row = i, col = 1 })

            -- Create the blank line that extends the bottom
            -- of the cell within the grid
            table.insert(group, pdf.object.line({
                cell.ll,
                cell:lr(),
            }))

            -- For odd numbers, we insert as a numeric text label
            if math.fmod(i, 2) == 1 then
                local text = pdf.object.text({
                    text = tostring((i + 1) / 2),
                }):align_to(cell, { h = "left" })
                table.insert(group, text)
            end
        end
    end
end

pdf.hooks.on_monthly_page(function(page)
    local grid = pdf.utils.grid({
        bounds = pdf.page:bounds():with_padding(3.5),
        rows = 35,
        columns = 7,
    })

    ---Creates a new rect text object fitting the cell bounds.
    ---@param opts? pdf.object.RectTextLike
    local cell_rect_text = grid.map_cell(function(bounds, opts)
        opts = opts or {}

        -- Create a blank rect args if it does not exist
        opts.rect = opts.rect or {}

        -- Explicitly overwrite rect bounds with cell bounds
        opts.rect.ll = bounds.ll
        opts.rect.ur = bounds.ur

        return pdf.object.rect_text(opts)
    end)

    ---Creates a new line object representing a blank line fitting the cell bounds.
    ---@param opts? {line?:pdf.object.LineLike, margin?:pdf.common.PaddingLike}
    local cell_blank_line = grid.map_cell(function(bounds, opts)
        opts = opts or {}

        -- Adjust bounds to factor in margin
        bounds = bounds:with_padding(opts.margin)

        -- Create blank line
        local line = {}

        -- Copy over all non-point fields to the line
        for key, value in pairs(opts.line or {}) do
            line[key] = value
        end

        -- Adjust points of line such that it fits at the bottom of the bounds
        line[1] = bounds.ll
        line[2] = bounds:lr()

        return pdf.object.line(line)
    end)

    -- Build out our top line that includes the month, focus, and habit
    page.push(cell_rect_text({ row = 1, col = 1 }, {
        text = { text = "MONTH", color = "#FFFFFF" },
        margin = 0.5,
    }))
    page.push(cell_rect_text({ row = 1, col = 2, width = 2 }, {
        rect = { fill_color = "#FFFFFF" },
        text = { text = page.date:format("%B %Y"), color = "#000000" },
        margin = 0.5,
    }))
    page.push(cell_blank_line({ row = 1, col = 2, width = 2 }, {
        line = { color = pdf.page.fill_color },
        margin = 0.5,
    }))
    page.push(cell_rect_text({ row = 1, col = 4 }, {
        text = { text = "FOCUS", color = "#FFFFFF" },
        margin = 0.5,
    }))
    page.push(cell_blank_line({ row = 1, col = 5 }, {
        line = { color = pdf.page.fill_color },
        margin = 0.5,
    }))
    page.push(cell_rect_text({ row = 1, col = 6 }, {
        text = { text = "HABIT", color = "#FFFFFF" },
        margin = 0.5,
    }))
    page.push(cell_blank_line({ row = 1, col = 7 }, {
        line = { color = pdf.page.fill_color },
        margin = 0.5,
    }))

    -- Build our calendar
    page.push(pdf.object.calendar({
        bounds = grid.cell({ row = 3, col = 1, width = 7, height = 21 }),
        month = page.date,
        on_day_block = function(opts)
            local date = opts.date
            local group = opts.group
            local bounds = group:bounds()

            -- If we have a date for a block, link to the daily
            -- and inject a habits rect in the bottom-right
            if date then
                local target_page = page.daily(date)
                if target_page then
                    group.link = pdf.utils.link(target_page.id)
                end

                local hb = bounds:scale_by_factor({ width = 0.25, height = 0.25 })
                local habit_rect = pdf.object.rect_text({
                    rect = { ll = hb.ll, ur = hb.ur, fill_color = pdf.utils.color(pdf.page.fill_color):lighten(0.5) },
                    text = { text = "H", color = "#FFFFFF" },
                }):align_to(group:bounds(), {
                    v = "bottom",
                    h = "right",
                })
                table.insert(group, habit_rect)
            end
        end
    }))

    -- Build our planning section
    page.push(cell_rect_text({ row = 25, col = 1, width = 7 }, {
        text = { text = "PLAN", color = "#FFFFFF" },
    }))
    page.push(pdf.object.section({
        bounds = grid.cell({ row = 26.25, col = 1, width = 3.25, height = 4 }),
        header = { text = "THIS MONTH'S GOALS", foreground = "#FFFFFF" },
        outline = { dash_pattern = "dashed:1", color = pdf.page.fill_color },
        padding = 1,
        on_inner = on_inner_make_list(),
    }))
    page.push(pdf.object.section({
        bounds = grid.cell({ row = 26.25, col = 4.75, width = 3.25, height = 4 }),
        header = { text = "DISTRACTIONS TO AVOID", foreground = "#FFFFFF" },
        outline = { dash_pattern = "dashed:1", color = pdf.page.fill_color },
        padding = 1,
        on_inner = on_inner_make_list(),
    }))

    -- Build our review section
    page.push(cell_rect_text({ row = 31, col = 1, width = 7 }, {
        text = { text = "REVIEW", color = "#FFFFFF" },
    }))
    page.push(pdf.object.section({
        bounds = grid.cell({ row = 32.25, col = 1, width = 3.25, height = 4 }),
        header = { text = "THIS MONTH'S WINS", foreground = "#FFFFFF" },
        outline = { dash_pattern = "dashed:1", cap_style = "round", join_style = "round", color = pdf.page.fill_color },
        padding = 1,
        on_inner = on_inner_make_list(),
    }))
    page.push(pdf.object.section({
        bounds = grid.cell({ row = 32.25, col = 4.75, width = 3.25, height = 4 }),
        header = { text = "INSIGHTS GAINED", foreground = "#FFFFFF" },
        outline = { dash_pattern = "dashed:1", color = pdf.page.fill_color },
        padding = 1,
        on_inner = on_inner_make_list(),
    }))
end)

pdf.hooks.on_weekly_page(function(page)
    local bounds = pdf.page:bounds()
    local text = pdf.object.text({
        text = "Week " .. tostring(page.date.week),
    })
    page.push(text:align_to(bounds, { v = "middle", h = "middle" }))
end)

pdf.hooks.on_daily_page(function(page)
    local bounds = pdf.page:bounds()
    local text = pdf.object.text({
        text = "Day " .. page.date:format("%F"),
    })
    page.push(text:align_to(bounds, { v = "middle", h = "middle" }))
end)
