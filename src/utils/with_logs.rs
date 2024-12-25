use log::debug;

pub trait WithLogs {
    fn with_logs<T, F>(&mut self, module_name: &str, action_name: &str, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T;
}

impl<T> WithLogs for T {
    fn with_logs<TRes, F>(&mut self, module_name: &str, action_name: &str, f: F) -> TRes
    where
        F: FnOnce(&mut Self) -> TRes,
    {
        debug!("{module_name}: {action_name} started");
        let result = f(self);
        debug!("{module_name}: {action_name} completed");

        result
    }
}
