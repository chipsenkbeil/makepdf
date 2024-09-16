-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

local o = pdf.object
local u = pdf.utils

print("hello world")
pdf.hooks.on_monthly_page(function(page)
    print("Processing monthly page", page.date.format("%B"))
    print("DATE", page.date)
    print("DATE INSPECTED", u.inspect(page.date))

    local daily = page.daily("2024-09-01")
    if daily then
        print("--> Daily month is", daily.date.format("%B"))
    end

    local p = page.next_page()
    if p then
        print("--> Next page is month", p.date.format("%B"))
    end

    page.push(o.rect({
        { 0,  0 },
        { 50, 50 },
        fill_color = "#999999",
        link = "https://example.com",
    }))

    page.push(o.text({
        0,
        pdf.page.height - 25,
        text = "Page " .. page.id,
        fill_color = "#999999",
        link = p and p.id,
    }))
end)
