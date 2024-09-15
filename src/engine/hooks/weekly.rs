use super::{EnginePage, OnPageFn};
use anyhow::Context;
use mlua::OwnedFunction;

#[derive(Clone, Debug)]
pub struct OnWeeklyPageFn(OwnedFunction);

impl From<OwnedFunction> for OnWeeklyPageFn {
    fn from(f: OwnedFunction) -> Self {
        Self(f)
    }
}

impl OnPageFn for OnWeeklyPageFn {
    fn call(&self, page: EnginePage) -> anyhow::Result<()> {
        self.0
            .call::<_, ()>(page)
            .context("Failed invoking hook: on_weekly_page")
    }
}
