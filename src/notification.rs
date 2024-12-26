use notify_rust::Notification;

pub trait NotificationBuilder {
    fn build(&self) -> anyhow::Result<Option<Notification>>;
    fn show(&self) -> anyhow::Result<()> {
        if let Some(noti) = self.build()? {
            noti.show()?;
        }

        Ok(())
    }
}
