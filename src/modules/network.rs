use std::thread::JoinHandle;

use derive_more::derive::Display;
use futures_util::StreamExt;
use log::debug;
use notify_rust::Notification;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{
    config::NetworkConfig,
    modules::Module,
    utils::{make_notification::MakeNotification, with_logs::WithLogs},
};

pub struct NetworkModule {
    receiver: UnboundedReceiver<u32>,
    _thread: JoinHandle<anyhow::Result<()>>,
}

impl Module for NetworkModule {
    fn poll(&mut self) -> anyhow::Result<Option<notify_rust::Notification>> {
        let mut notification = None;

        if let Ok(state) = self.receiver.try_recv() {
            debug!("WIFI state changed to {}", state);

            match NetworkStateMap::from(state) {
                NetworkStateMap::Activated => {
                    // TODO: signal_strenght
                    let ssid = "todo".to_string();
                    let signal_strength = 0;
                    notification = (ssid, signal_strength).make_notification();
                }
                _ => {}
            }
        }

        Ok(notification)
    }

    fn name(&self) -> &'static str {
        "NetworkModule"
    }
}

#[derive(Display)]
enum NetworkStateMap {
    Unknown,
    Unmanaged,
    Unavailable,
    Disconnected,
    Prepare,
    Config,
    NeedAuth,
    IPConfig,
    IPCheck,
    Secondaries,
    Activated,
    Deactivating,
    Failed,
}

impl From<u32> for NetworkStateMap {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Unknown,
            10 => Self::Unmanaged,
            20 => Self::Unavailable,
            30 => Self::Disconnected,
            40 => Self::Prepare,
            50 => Self::Config,
            60 => Self::NeedAuth,
            70 => Self::IPConfig,
            80 => Self::IPCheck,
            90 => Self::Secondaries,
            100 => Self::Activated,
            110 => Self::Deactivating,
            120 => Self::Failed,
            _ => unreachable!(),
        }
    }
}

impl NetworkModule {
    const NAME: &str = "NetworkModule";

    pub fn create(config: &NetworkConfig) -> anyhow::Result<Self> {
        (|| Self::initialize(config)).with_logs(Self::NAME, "Initialization")
    }

    fn initialize(_config: &NetworkConfig) -> anyhow::Result<Self> {
        let (sender, receiver) = unbounded_channel();

        let thread = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread().build()?;
            rt.block_on(Self::async_loop(sender))
        });

        Ok(Self {
            receiver,
            _thread: thread,
        })
    }

    async fn async_loop(sender: UnboundedSender<u32>) -> anyhow::Result<()> {
        let conn = zbus::Connection::system().await?;
        let device_path = Self::get_wireless_device_path(&conn).await?;

        let device_proxy = DeviceProxy::builder(&conn)
            .path(&*device_path)?
            .build()
            .await?;

        let state = device_proxy.state().await?;
        debug!(
            "NetworkModule: Initial WIFI state: {}",
            NetworkStateMap::from(state)
        );

        let mut stream = device_proxy.receive_state_changed().await;

        while let Some(signal) = stream.next().await {
            let state = signal.get().await?;
            sender.send(state)?;
        }

        Ok(())
    }

    async fn get_wireless_device_path(conn: &zbus::Connection) -> anyhow::Result<String> {
        const WIFI_DEVICE: u32 = 2;

        let proxy = NetworkManagerProxy::new(conn).await?;
        let devices: Vec<zbus::zvariant::OwnedObjectPath> = proxy.get_all_devices().await?;

        for device_path in devices {
            let device_proxy = zbus::Proxy::new(
                conn,
                "org.freedesktop.NetworkManager",
                device_path.as_str(),
                "org.freedesktop.NetworkManager.Device",
            )
            .await?;

            let device_type: u32 = device_proxy.get_property("DeviceType").await?;
            if device_type == WIFI_DEVICE {
                return Ok(device_path.to_string());
            }
        }

        anyhow::bail!("No wireless device found")
    }
}

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;

    fn get_all_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager.Device",
    default_service = "org.freedesktop.NetworkManager"
)]
trait Device {
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;
}

impl MakeNotification for (String, usize) {
    fn make_notification(self) -> Option<notify_rust::Notification> {
        let (ssid, _signal_strength) = self;

        Some(
            Notification::new()
                .appname("network_module")
                .summary("Network")
                .body(format!("Connected to {}", ssid).as_str())
                .finalize(),
        )
    }
}
