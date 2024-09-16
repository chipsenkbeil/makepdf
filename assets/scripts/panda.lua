-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

local f = pdf.font
local o = pdf.object
local u = pdf.utils

for _, id in ipairs(f.ids()) do
    local is_fallback = f.fallback() == id
    print("Font " .. id .. (is_fallback and " (fallback)" or ""))
end

print("hello world")
pdf.hooks.on_monthly_page(function(page)
    page.push(o.rect({
        { 0,  0 },
        { 50, 50 },
        fill_color = "#999999",
    }))

    page.push(o.text({
        0,
        pdf.page.height - 25,
        text = "Page " .. page.id,
        fill_color = "#999999",
    }))
end)
