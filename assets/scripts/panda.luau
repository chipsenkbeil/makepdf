print("Hello world!")
-- pdf.config.page.width = 5000.0

-- Print contents of `tbl`, with indentation.
-- `indent` sets the initial level of indentation.
function tprint (tbl, indent)
  if not indent then indent = 0 end
  for k, v in pairs(tbl) do
    formatting = string.rep("  ", indent) .. k .. ": "
    if type(v) == "table" then
      print(formatting)
      tprint(v, indent+1)
    elseif type(v) == "boolean" then
      print(formatting .. tostring(v))
    elseif type(v) == "function" then
      print(formatting .. "<function>")
    else
      print(formatting .. v)
    end
  end
end

tprint(pdf)
local line = pdf.object.line({
    bounds = {llx=0, lly=0, urx=0, ury=0},
})
tprint(line)
