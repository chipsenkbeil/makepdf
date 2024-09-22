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

    local month_start_day_of_week = page.date.last_month().tomorrow().weekday.number_from_sunday()
    local month_end_day_of_week = page.date.next_month().yesterday().weekday.number_from_sunday()
    print("MONTH DATE", tostring(page.date))
    print("START OF MONTH -- NUM FROM SUNDAY (SUN = 1, MON = 2, ...)", month_start_day_of_week)
    print("END OF MONTH -- NUM FROM SUNDAY (SUN = 1, MON = 2, ...)", month_end_day_of_week)

    -- Build our 7 x 5 grid of calendar days
    for week_of_month = 1, 5 do
        for day_of_week = 1, 7 do
            -- Create the container block for the day
            local block = pdf.object.rect(cell(
                (week_of_month * 2) + 1,
                day_of_week,
                {},
                { height = 2 }
            ))
            page.push(block)

            -- Check if the day on the calendar is within our expected range,
            -- and if so display the date on the block, otherwise show nothing
            --
            -- We start with Sunday in our calendar!
            if (week_of_month == 1 and day_of_week >= month_start_day_of_week)
                or (week_of_month == 5 and day_of_week <= month_end_day_of_week)
                or (week_of_month > 1 and week_of_month < 5) then
                local day_rect = {
                    ll = block:bounds().ll,
                    ur = block:bounds().ur,
                }

                day_rect.ur.x = day_rect.ur.x / 4
                day_rect.ur.y = day_rect.ur.y / 4

                -- Calculate the calendar number from 1 to 31 by looking at
                -- the raw number from 1 to 35 and subracting the start of
                -- the month and adding 1 to get the actual start num
                local day_num = ((week_of_month - 1) * day_of_week)
                    + day_of_week
                    - (month_start_day_of_week - 1)

                -- Place the day as a number in the top-left (1/4 of size)
                local day = pdf.object.rect_text({
                    rect = { day_rect, fill_color = "#999900" },
                    text = { text = tostring(day_num), color = "#FFFFFF", }
                }):align_to(block:bounds(), { v = "top", h = "left" })
                page.push(day)
            end
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
