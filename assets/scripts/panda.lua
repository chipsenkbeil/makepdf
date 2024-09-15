-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

local h = pdf.hooks
local o = pdf.object
local u = pdf.utils

h.on_monthly_page = function(page --[[@param page pdf.engine.Page]])
    print("Processing monthly page", page.date.format("%B"))

    page.push(o.rect({
        { 0,  0 },
        { 50, 50 },
        fill_color = "#999999",
    }))

    local daily = page.daily("2024-09-01")
    if daily then
        print("--> Daily month is", daily.date.format("%B"))
    end

    local p = page.next_page()
    if p then
        print("--> Next page is month", p.date.format("%B"))
    end
end
