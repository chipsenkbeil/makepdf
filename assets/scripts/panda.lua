-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

print("hello world")

pdf.hooks.on_monthly_page = function(page, date)
    print("PDF", pdf.inspect(pdf))
    print("PAGE", pdf.inspect(page))
    print("DATE", pdf.inspect(date))
    page.push(pdf.object.rect({ { 0, 0 }, { 50, 50 } }))
end
