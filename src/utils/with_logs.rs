use log::debug;

pub trait WithLogs {
    type Output;
    fn with_logs(self, module_name: &str, action_name: &str) -> Self::Output;
}

impl<F, O> WithLogs for F
where
    Self: FnOnce() -> anyhow::Result<O>,
{
    type Output = anyhow::Result<O>;

    fn with_logs(self, module_name: &str, action_name: &str) -> Self::Output {
        debug!("{module_name}: {action_name} started");
        let result = self()?;
        debug!("{module_name}: {action_name} completed");

        Ok(result)
    }
}
