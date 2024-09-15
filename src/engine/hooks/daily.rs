use super::{EnginePage, OnPageFn};
use anyhow::Context;
use mlua::OwnedFunction;

#[derive(Clone, Debug)]
pub struct OnDailyPageFn(OwnedFunction);

impl From<OwnedFunction> for OnDailyPageFn {
    fn from(f: OwnedFunction) -> Self {
        Self(f)
    }
}

impl OnPageFn for OnDailyPageFn {
    fn call(&self, page: EnginePage) -> anyhow::Result<()> {
        self.0
            .call::<_, ()>(page)
            .context("Failed invoking hook: on_daily_page")
    }
}
