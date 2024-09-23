-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

pdf.page.font_size = 6.5
pdf.page.outline_thickness = pdf.utils.pt_to_mm(1.0)

pdf.hooks.on_monthly_page(function(page)
    local grid = pdf.utils.grid({
        bounds = pdf.page:bounds(),
        rows = 25,
        columns = 7,
    })

    ---Creates a new rect text object fitting the cell bounds.
    ---@param opts? pdf.object.RectTextArgs
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
    ---@param opts? {line?:pdf.object.LineArgs, margin?:pdf.common.PaddingArg}
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
        text = { text = page.date.format("%B %Y"), color = "#000000" },
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
        bounds = grid.cell({ row = 2, col = 1, width = 7, height = 24 }),
        month = page.date,
    }))
end)
