use notify_rust::Notification;

use crate::config::EventConfig;

pub enum NotificationData {
    Network { ssid: String, signal_strength: u8 },
    Power { percentage: u8, charging: String },
    Device { name: String },
    Custom { name: String, value: String },
}

pub trait ComposeNotification {
    fn compose_notification(&self, data: NotificationData) -> Option<Notification>;
}

impl ComposeNotification for EventConfig {
    fn compose_notification(&self, data: NotificationData) -> Option<Notification> {
        if !self.enabled {
            return None;
        }

        // TODO: save id to tempfile based on module type (if config.replace == true)
        let _id: u32;

        let mut notification = Notification::new();

        notification.appname(get_app_name(&data));

        if let Some(icon) = &self.icon {
            notification.icon(icon);
        }

        if let Some(summary_config) = &self.summary {
            if let Some(format) = &summary_config.format {
                let summary_text = replace_template_variables(format, &data);
                notification.summary(&summary_text);
            }
        }

        if let Some(body_config) = &self.body {
            if let Some(format) = &body_config.format {
                let body_text = replace_template_variables(format, &data);
                notification.body(&body_text);
            }
        }

        Some(notification.finalize())
    }
}

fn replace_template_variables(template: &str, event_data: &NotificationData) -> String {
    match event_data {
        NotificationData::Network {
            ssid,
            signal_strength,
            ..
        } => template
            .replace("$ssid", ssid)
            .replace("$signal_strength", &signal_strength.to_string()),
        NotificationData::Power {
            percentage,
            charging,
            ..
        } => template
            .replace("$percentage", &percentage.to_string())
            .replace("$charging", &charging.to_string()),
        NotificationData::Device { name, .. } => template.replace("$name", name),
        NotificationData::Custom { value, .. } => template.replace("$value", value),
    }
}

fn get_app_name(event_data: &NotificationData) -> &str {
    match event_data {
        NotificationData::Network { .. } => "network_module",
        NotificationData::Power { .. } => "power_module",
        NotificationData::Device { .. } => "device_module",
        NotificationData::Custom { name, .. } => name,
    }
}
