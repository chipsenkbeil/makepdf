-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

pdf.page.font_size = 6.5

pdf.hooks.on_monthly_page(function(page)
    -- 25 rows, 8 columns
    local ROW_HEIGHT = pdf.page.height / 25.0
    local COL_WIDTH = pdf.page.width / 8.0

    ---@param row integer
    ---@param col integer
    ---@param args? pdf.object.RectArgs
    ---@param opts? {width?:integer, height?:integer}
    ---@return pdf.object.RectArgs
    local function cell(row, col, args, opts)
        opts = opts or {}
        local width = opts.width or 1
        local height = opts.height or 1

        ---@type pdf.common.Point
        local ll = {
            x = (col - 1) * COL_WIDTH,
            -- NOTE: Need to flip since 0 is bottom instead of top
            y = pdf.page.height - (row * ROW_HEIGHT) - ((height - 1) * ROW_HEIGHT),
        }

        ---@type pdf.common.Point
        local ur = {
            x = ll.x + (COL_WIDTH * width),
            y = ll.y + (ROW_HEIGHT * height)
        }

        args = args or {}
        args.ll = ll
        args.ur = ur

        return args
    end

    -- Build out our top line that includes the month, focus, and habit
    page.push(pdf.object.rect_text({
        rect = cell(1, 1),
        text = { text = "MONTH", color = "#FFFFFF" },
    }))
    page.push(pdf.object.rect_text({
        rect = cell(1, 2, { fill_color = "#FFFFFF" }, { width = 3 }),
        text = { text = page.date.format("%B %Y"), color = "#000000" },
    }))
    page.push(pdf.object.rect_text({
        rect = cell(1, 5),
        text = { text = "FOCUS", color = "#FFFFFF" },
    }))
    page.push(pdf.object.rect_text({
        rect = cell(1, 7),
        text = { text = "HABIT", color = "#FFFFFF" },
    }))

    -- Build our header for the days of the week
    for i, text in ipairs({
        "SUNDAY", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY"
    }) do
        page.push(pdf.object.rect_text({
            rect = cell(2, i),
            text = { text = text, color = "#FFFFFF" },
        }))
    end

    -- Build our 7 x 5 grid of calendar days
    for row = 1, 5 do
        for col = 1, 7 do
            -- Create the container block for the day
            local block = pdf.object.rect(cell((row * 2) + 1, col, {}, { height = 2 }))
            print("BLOCK", pdf.utils.inspect(block))
            page.push(block)

            local day_rect = {
                ll = block:bounds().ll,
                ur = block:bounds().ur,
            }

            day_rect.ur.x = day_rect.ur.x / 4
            day_rect.ur.y = day_rect.ur.y / 4

            -- Place the day as a number in the top-left (1/4 of size)
            local day = pdf.object.rect_text({
                rect = day_rect,
                text = { text = "1", color = "#FFFFFF", }
            }):align_to(block:bounds(), { v = "top", h = "left" })
            print("DAY", pdf.utils.inspect(day))
            page.push(day)
        end
    end

    -- Add the notes box (TODO - it's just a black box)
    page.push(pdf.object.rect_text({
        rect = cell(2, 8),
        text = { text = "NOTES", color = "#FFFFFF" },
    }))
    page.push(pdf.object.rect(cell(3, 8, {
        fill_color = "#000000",
        outline_color = "#000000",
    }, { height = 23 })))
end)
