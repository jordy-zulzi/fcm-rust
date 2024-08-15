use std::borrow::Cow;

use serde::Serialize;
use serde_json::Value;

use crate::notification::Notification;

#[cfg(test)]
mod tests;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Normal,
    High,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Unspecified,
    Private,
    Public,
    Secret
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Proxy {
    Unspecified,
    Allow,
    Deny,
    IfPriorityLowered
}

#[derive(Serialize, Debug, PartialEq)]
pub struct NotificationV2<'a> {

    /// The notification's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,

    /// The notification's body text.
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,

    /// Contains the URL of an image that is going to be downloaded on the device and
    /// displayed in a notification. JPEG, PNG, BMP have full support across platforms.
    /// Animated GIF and video only work on iOS. WebP and HEIF have varying levels of
    /// support across platforms and platform versions. Android has 1MB image size limit.
    /// Quota usage and implications/costs for hosting image on Firebase Storage:
    /// https://firebase.google.com/pricing
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<&'a str>
}

#[derive(Serialize, Debug, PartialEq)]
pub struct LightSettings<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    light_on_duration: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    light_off_duration: Option<&'a str>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct AndroidNotification<'a> { // new
    /// The notification's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,

    /// The notification's body text. If present, it will override
    /// google.firebase.fcm.v1.Notification.body.
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,

    /// The notification's icon. Sets the notification icon to myicon for drawable
    /// resource myicon. If you don't send this key in the request, FCM displays the
    /// launcher icon specified in your app manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a str>,

    /// The notification's icon color, expressed in #rrggbb format.
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<&'a str>,

    /// The sound to play when the device receives the notification. Supports "default" or the
    /// filename of a sound resource bundled in the app. Sound files must reside in /res/raw/.
    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<&'a str>,

    /// Identifier used to replace existing notifications in the notification drawer. If not
    /// specified, each request creates a new notification. If specified and a notification
    /// with the same tag is already being shown, the new notification replaces the existing
    /// one in the notification drawer.
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<&'a str>,

    /// The action associated with a user click on the notification. If specified, an activity
    /// with a matching intent filter is launched when a user clicks on the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    click_action: Option<&'a str>,

    /// The key to the body string in the app's string resources to use to localize the body text
    /// to the user's current localization. See String Resources for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_args: Option<Vec<&'a str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_args: Option<Vec<&'a str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ticker: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sticky: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_time: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification_priority: Option<Priority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_sound: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_vibrate_timings: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_light_settings: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vibrate_timings: Option<Vec<&'a str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Visibility>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification_count: Option<i64>,

    /// Contains the URL of an image that is going to be displayed in a notification.
    /// If present, it will override google.firebase.fcm.v1.Notification.image.
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<&'a str>,

    /// Contains the URL of an image that is going to be displayed in a notification.
    /// If present, it will override google.firebase.fcm.v1.Notification.image.
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_proxy_notification: Option<bool>,

    /// Setting to control when a notification may be proxied.
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<Proxy>
}

#[derive(Serialize, Debug, PartialEq)]
pub struct AndroidConfig<'a> {
    /// An identifier of a group of messages that can be collapsed, so that only the last
    /// message gets sent when delivery can be resumed. A maximum of 4 different collapse
    /// keys is allowed at any given time.
    #[serde(skip_serializing_if = "Option::is_none")]
    collapse_key: Option<&'a str>,

    /// Message priority. Can take "normal" and "high" values.
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<Priority>,

    /// How long (in seconds) the message should be kept in FCM storage if the device is offline.
    /// The maximum time to live supported is 4 weeks, and the default value is 4 weeks if not set.
    /// Set it to 0 if want to send the message immediately. In JSON format, the Duration type is
    /// encoded as a string rather than an object, where the string ends in the suffix "s"
    /// (indicating seconds) and is preceded by the number of seconds, with nanoseconds expressed
    /// as fractional seconds. For example, 3 seconds with 0 nanoseconds should be encoded in JSON
    /// format as "3s", while 3 seconds and 1 nanosecond should be expressed in JSON format as
    /// "3.000000001s". The ttl will be rounded down to the nearest second.
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<&'a str>,

    /// Package name of the application where the registration token must match in order to
    /// receive the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_package_name: Option<&'a str>,

    /// An object containing a list of "key": value pairs
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,


    /// If set to true, messages will be allowed to be delivered to the app while the device
    /// is in direct boot mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_boot_ok: Option<bool>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct ApnsFcmOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_label: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<&'a str>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct ApnsConfig<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<ApnsFcmOptions<'a>>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct MessageBodyV2<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    android: Option<AndroidConfig<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    apns: Option<ApnsConfig<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<&'a str>
}

#[derive(Serialize, Debug, PartialEq)]
pub struct MessageV2<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    validate_only: Option<bool>,
    message: MessageBodyV2<'a>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct MessageBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    collapse_key: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content_available: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    delay_while_idle: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<Notification<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<Priority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    registration_ids: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_package_name: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_to_live: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mutable_content: Option<bool>,
}

/// Represents a FCM message. Construct the FCM message
/// using various utility methods and finally send it.
/// # Examples:
/// ```rust
/// use fcm::MessageBuilder;
///
/// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
/// builder.dry_run(true);
/// let message = builder.finalize();
/// ```
#[derive(Debug)]
pub struct Message<'a> {
    pub api_key: &'a str,
    pub body: MessageBody<'a>,
}

///
/// A builder to get a `Message` instance.
///
/// # Examples
///
/// ```rust
/// use fcm::MessageBuilder;
///
/// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
/// builder.dry_run(true);
/// let message = builder.finalize();
/// ```
#[derive(Debug)]
pub struct MessageBuilder<'a> {
    api_key: &'a str,
    /// target
    token: Option<&'a str>,
    topic: Option<&'a str>,
    condition: Option< &'a str>,
    /// end target
    validate_only: Option<bool>,
    name: Option<&'a str>,
    collapse_key: Option<&'a str>,
    content_available: Option<bool>,
    data: Option<Value>,
    delay_while_idle: Option<bool>,
    dry_run: Option<bool>,
    notification: Option<Notification<'a>>,
    priority: Option<Priority>,
    registration_ids: Option<Vec<Cow<'a, str>>>,
    restricted_package_name: Option<&'a str>,
    time_to_live: Option<&'a str>,
    to: Option<&'a str>,
    mutable_content: Option<bool>,
}

impl<'a> MessageBuilder<'a> {
    /// Get a new instance of Message. You need to supply to.
    pub fn new(api_key: &'a str, to: &'a str) -> Self {
        MessageBuilder {
            api_key,
            token: None,
            topic: None,
            condition: None,
            validate_only: None,
            to: Some(to),
            registration_ids: None,
            collapse_key: None,
            priority: None,
            content_available: None,
            delay_while_idle: None,
            time_to_live: None,
            restricted_package_name: None,
            dry_run: None,
            data: None,
            notification: None,
            mutable_content: None,
            name: None,
        }
    }

    /// Get a new instance of Message. You need to supply registration ids.
    pub fn new_multi<S>(api_key: &'a str, ids: &'a [S]) -> Self
    where
        S: Into<Cow<'a, str>> + AsRef<str>,
    {
        let converted = ids.iter().map(|a| a.as_ref().into()).collect();

        MessageBuilder {
            api_key,
            token: None,
            topic: None,
            condition: None,
            validate_only: None,
            to: None,
            registration_ids: Some(converted),
            collapse_key: None,
            priority: None,
            content_available: None,
            delay_while_idle: None,
            time_to_live: None,
            restricted_package_name: None,
            dry_run: None,
            data: None,
            notification: None,
            mutable_content: None,
            name: None,
        }
    }

    /// String value to replace format specifiers in the body string.
    pub fn registration_ids<S>(&mut self, ids: &'a [S]) -> &mut Self
    where
        S: Into<Cow<'a, str>> + AsRef<str>,
    {
        let converted = ids.iter().map(|a| a.as_ref().into()).collect();

        self.registration_ids = Some(converted);
        self
    }

    /// Set this parameter to identify groups of messages that can be collapsed.
    pub fn collapse_key(&mut self, collapse_key: &'a str) -> &mut Self {
        self.collapse_key = Some(collapse_key);
        self
    }

    /// Set the priority of the message. You can set Normal or High priorities.
    /// # Examples:
    /// ```rust
    /// use fcm::{MessageBuilder, Priority};
    ///
    /// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
    /// builder.priority(Priority::High);
    /// let message = builder.finalize();
    /// ```
    pub fn priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = Some(priority);
        self
    }

    /// To set the `content-available` field on iOS
    pub fn content_available(&mut self, content_available: bool) -> &mut Self {
        self.content_available = Some(content_available);
        self
    }

    /// When set to `true`, sends the message only when the device is active.
    pub fn delay_while_idle(&mut self, delay_while_idle: bool) -> &mut Self {
        self.delay_while_idle = Some(delay_while_idle);
        self
    }

    /// How long (in seconds) to keep the message on FCM servers in case the device
    /// is offline. The maximum and default is 4 weeks.
    pub fn time_to_live(&mut self, time_to_live: &'a str) -> &mut Self {
        self.time_to_live = Some(time_to_live);
        self
    }

    /// Package name of the application where the registration tokens must match.
    pub fn restricted_package_name(&mut self, restricted_package_name: &'a str) -> &mut Self {
        self.restricted_package_name = Some(restricted_package_name);
        self
    }

    /// When set to `true`, allows you to test FCM without actually sending the message.
    pub fn dry_run(&mut self, dry_run: bool) -> &mut Self {
        self.dry_run = Some(dry_run);
        self
    }

    /// Use this to add custom key-value pairs to the message. This data
    /// must be handled appropriately on the client end. The data can be
    /// anything that Serde can serialize to JSON.
    ///
    /// # Examples:
    /// ```rust
    /// use fcm::MessageBuilder;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("message", "Howdy!");
    ///
    /// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
    /// builder.data(&map);
    /// let message = builder.finalize();
    /// ```
    pub fn data(&mut self, data: &dyn erased_serde::Serialize) -> Result<&mut Self, serde_json::Error> {
        self.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    /// Use this to set a `Notification` for the message.
    /// # Examples:
    /// ```rust
    /// use fcm::{MessageBuilder, NotificationBuilder};
    ///
    /// let mut builder = NotificationBuilder::new();
    /// builder.title("Hey!");
    /// builder.body("Do you want to catch up later?");
    /// let notification = builder.finalize();
    ///
    /// let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
    /// builder.notification(notification);
    /// let message = builder.finalize();
    /// ```
    pub fn notification(&mut self, notification: Notification<'a>) -> &mut Self {
        self.notification = Some(notification);
        self
    }

    /// To set the `mutable_content` field on iOS
    pub fn mutable_content(&mut self, mutable_content: bool) -> &mut Self {
        self.mutable_content = Some(mutable_content);
        self
    }

    pub fn topic(&mut self, topic: &'a str) -> &mut Self {
        self.topic = Some(topic);
        self
    }

    pub fn token(&mut self, token: &'a str) -> &mut Self {
        self.token = Some(token);
        self
    }

    pub fn condition(&mut self, condition: &'a str) -> &mut Self {
        self.condition = Some(condition);
        self
    }
    
    pub fn finalize(self) -> MessageV2<'a> {
        MessageV2 {
            validate_only: self.validate_only,
            message: MessageBodyV2 {
                name: self.name,
                android: Some(AndroidConfig{
                    priority: self.priority,
                    collapse_key: self.collapse_key,
                    data: self.data.clone(),
                    ttl: self.time_to_live,
                    restricted_package_name: self.restricted_package_name,
                    direct_boot_ok: Some(false),
                }),
                apns: None,
                topic: self.topic,
                token: self.to,
                condition: self.condition,
            },
        }
    }
}
