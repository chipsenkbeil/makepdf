use super::{OnPageFn, RuntimePage};
use anyhow::Context;
use mlua::OwnedFunction;

#[derive(Clone, Debug)]
pub struct OnMonthlyPageFn(OwnedFunction);

impl From<OwnedFunction> for OnMonthlyPageFn {
    fn from(f: OwnedFunction) -> Self {
        Self(f)
    }
}

impl OnPageFn for OnMonthlyPageFn {
    fn call(&self, page: RuntimePage) -> anyhow::Result<()> {
        self.0
            .call::<_, ()>(page)
            .context("Failed invoking hook: on_monthly_page")
    }
}
