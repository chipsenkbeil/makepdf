-------------------------------------------------------------------------------
-- PANDA
--
-- Recreation of the planner that has a panda.
-------------------------------------------------------------------------------

print("hello world")

pdf.hooks.on_monthly_page = function(page)
    print("PDF", pdf.utils.inspect(pdf))
    print("PAGE", pdf.utils.inspect(page))
    page.push(pdf.object.rect({ { 0, 0 }, { 50, 50 } }))
end
