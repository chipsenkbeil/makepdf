mod daily;
mod monthly;
mod weekly;

pub use daily::OnDailyPageFn;
pub use monthly::OnMonthlyPageFn;
pub use weekly::OnWeeklyPageFn;

use crate::engine::EnginePage;

/// Abstraction around hook functions that are called when a page is created.
trait OnPageFn {
    /// Invokes the callback, passing in `page` as an argument.
    fn call(&self, page: EnginePage) -> anyhow::Result<()>;
}

pub struct EngineHooks {
    /// Invoked when a daily page is created.
    on_daily_page_fns: Vec<OnDailyPageFn>,
    /// Invoked when a monthly page is created.
    on_monthly_page_fns: Vec<OnMonthlyPageFn>,
    /// Invoked when a weekly page is created.
    on_weekly_page_fns: Vec<OnWeeklyPageFn>,
}

impl Default for EngineHooks {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineHooks {
    /// Creates a new instance of the hooks container with no registered hooks.
    pub const fn new() -> Self {
        Self {
            on_daily_page_fns: Vec::new(),
            on_monthly_page_fns: Vec::new(),
            on_weekly_page_fns: Vec::new(),
        }
    }

    /// Registers a new hook for when a daily page is created.
    pub fn register_on_daily_page(&mut self, f: impl Into<OnDailyPageFn>) {
        self.on_daily_page_fns.push(f.into());
    }

    /// Registers a new hook for when a monthly page is created.
    pub fn register_on_monthly_page(&mut self, f: impl Into<OnMonthlyPageFn>) {
        self.on_monthly_page_fns.push(f.into());
    }

    /// Registers a new hook for when a weekly page is created.
    pub fn register_on_weekly_page(&mut self, f: impl Into<OnWeeklyPageFn>) {
        self.on_weekly_page_fns.push(f.into());
    }

    /// Invoke the hook for when a daily page is created.
    pub fn on_daily_page(&self, page: EnginePage) -> anyhow::Result<()> {
        for f in self.on_daily_page_fns.iter() {
            f.call(page.clone())?;
        }

        Ok(())
    }

    /// Invoke the hook for when a monthly page is created.
    pub fn on_monthly_page(&self, page: EnginePage) -> anyhow::Result<()> {
        for f in self.on_monthly_page_fns.iter() {
            f.call(page.clone())?;
        }

        Ok(())
    }

    /// Invoke the hook for when a weekly page is created.
    pub fn on_weekly_page(&self, page: EnginePage) -> anyhow::Result<()> {
        for f in self.on_weekly_page_fns.iter() {
            f.call(page.clone())?;
        }

        Ok(())
    }
}
