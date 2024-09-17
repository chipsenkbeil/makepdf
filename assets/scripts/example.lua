-------------------------------------------------------------------------------
-- EXAMPLE
--
-- Used to highlight how to leverage different aspects of the PDF Lua library.
-------------------------------------------------------------------------------

pdf.hooks.on_monthly_page(function(page)
    page.push(pdf.object.text({
        { pdf.page.width / 3, pdf.page.height / 2 },
        text = page.date.format("%B"),
    }))
end)

pdf.hooks.on_weekly_page(function(page)
    page.push(pdf.object.text({
        { pdf.page.width / 3, pdf.page.height / 2 },
        text = "Week " .. page.date.week,
    }))
end)

pdf.hooks.on_daily_page(function(page)
    page.push(pdf.object.text({
        { pdf.page.width / 3, pdf.page.height / 2 },
        text = "Day " .. tostring(page.date),
    }))
end)
