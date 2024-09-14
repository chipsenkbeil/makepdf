use mlua::prelude::*;
use printpdf::path::WindingOrder;

/// Winding order to use with shapes.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfWindingOrder(WindingOrder);

impl From<PdfWindingOrder> for WindingOrder {
    fn from(order: PdfWindingOrder) -> Self {
        order.0
    }
}

impl<'lua> IntoLua<'lua> for PdfWindingOrder {
    #[inline]
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        lua.create_string(match self.0 {
            WindingOrder::EvenOdd => "even_odd",
            WindingOrder::NonZero => "non_zero",
        })
        .map(LuaValue::String)
    }
}

impl<'lua> FromLua<'lua> for PdfWindingOrder {
    #[inline]
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        let from = value.type_name();
        match value {
            LuaValue::String(s) => match s.to_string_lossy().as_ref() {
                "even_odd" => Ok(Self(WindingOrder::EvenOdd)),
                "non_zero" => Ok(Self(WindingOrder::NonZero)),
                ty => Err(LuaError::FromLuaConversionError {
                    from,
                    to: "pdf.common.winding_order",
                    message: Some(format!("unknown type: {ty}")),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from,
                to: "pdf.common.winding_order",
                message: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::PdfUtils;
    use mlua::chunk;

    #[test]
    fn should_be_able_to_convert_from_lua() {
        assert_eq!(
            Lua::new()
                .load(chunk!("even_odd"))
                .eval::<PdfWindingOrder>()
                .unwrap(),
            PdfWindingOrder(WindingOrder::EvenOdd),
        );
        assert_eq!(
            Lua::new()
                .load(chunk!("non_zero"))
                .eval::<PdfWindingOrder>()
                .unwrap(),
            PdfWindingOrder(WindingOrder::NonZero),
        );
    }

    #[test]
    fn should_be_able_to_convert_into_lua() {
        let even_odd_winding_order = PdfWindingOrder(WindingOrder::EvenOdd);
        let non_zero_winding_order = PdfWindingOrder(WindingOrder::NonZero);

        Lua::new()
            .load(chunk! {
                local u = $PdfUtils
                u.assert_deep_equal($even_odd_winding_order, "even_odd")
                u.assert_deep_equal($non_zero_winding_order, "non_zero")
            })
            .exec()
            .expect("Assertion failed");
    }
}
