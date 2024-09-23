-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

pdf.page.font_size = 6.5

pdf.hooks.on_monthly_page(function(page)
    local grid = pdf.utils.grid({
        bounds = pdf.page:bounds(),
        rows = 25,
        columns = 7,
    })

    local cell_rect_args = grid.map_cell(function(bounds)
        ---@type pdf.object.RectArgs
        return bounds
    end)

    local white_cell_rect_args = grid.map_cell(function(bounds)
        ---@type pdf.object.RectArgs
        return { ll = bounds.ll, ur = bounds.ur, fill_color = "#FFFFFF" }
    end)

    -- Build out our top line that includes the month, focus, and habit
    page.push(pdf.object.rect_text({
        rect = cell_rect_args({ row = 1, col = 1 }),
        text = { text = "MONTH", color = "#FFFFFF" },
    }))
    page.push(pdf.object.rect_text({
        rect = white_cell_rect_args({ row = 1, col = 2, width = 2 }),
        text = { text = page.date.format("%B %Y"), color = "#000000" },
    }))
    page.push(pdf.object.rect_text({
        rect = cell_rect_args({ row = 1, col = 4 }),
        text = { text = "FOCUS", color = "#FFFFFF" },
    }))
    page.push(pdf.object.rect_text({
        rect = cell_rect_args({ row = 1, col = 6 }),
        text = { text = "HABIT", color = "#FFFFFF" },
    }))

    -- Build our calendar
    page.push(pdf.object.calendar({
        bounds = grid.cell({ row = 2, col = 1, width = 7, height = 24 }),
        month = page.date,
    }))

    -- Add the notes box (TODO - it's just a black box)
    -- page.push(pdf.object.rect_text({
    --     rect = cell_rect_args({ row = 2, col = 8 }),
    --     text = { text = "NOTES", color = "#FFFFFF" },
    -- }))
    -- page.push(pdf.object.rect(cell_rect_args({
    --     row = 3,
    --     col = 8,
    --     height = 23,
    -- })))
end)
