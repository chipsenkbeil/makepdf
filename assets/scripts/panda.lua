-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

pdf.page.fill_color = "#999999"

pdf.hooks.on_monthly_page(function(page)
    page.push(pdf.object.rect({
        { 0,  0 },
        { 50, 50 },
    }))

    page.push(pdf.object.text({
        { 0, pdf.page.height - 25 },
        text = "Page " .. page.id,
    }))
end)
