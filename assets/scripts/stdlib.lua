-------------------------------------------------------------------------------
-- STDLIB
--
-- Executed prior to the user script, enabling standard library implementations
-- that are written in Lua. This is designed as faster turnaround than Rust.
-------------------------------------------------------------------------------

---@class pdf.object.RectTextArgs
---@field rect pdf.object.RectArgs
---@field text string|pdf.object.TextArgs
---@field align pdf.common.Align|nil

---Creates a group containing a rect and text overlayed on top.
---
---Supports configuring the text's alignment within the bounds of the rect.
---@param tbl pdf.object.RectTextArgs
---@return pdf.object.Group
function pdf.object.rect_text(tbl)
    -- Create a rect from the provided configuration
    local rect = pdf.object.rect(tbl.rect)

    local text_args = tbl.text
    if type(text_args) == "string" then
        text_args = { text = text_args, x = 0, y = 0 }
    end

    -- Create a text object aligned to the rect above
    local text = pdf.object.text(text_args):align_to(
        rect:bounds(),
        tbl.align or {
            h = "middle",
            v = "middle",
        }
    )

    -- Build a group comprising the two together
    return pdf.object.group({ rect, text })
end
