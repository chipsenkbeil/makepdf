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
