use tokio::runtime::{Builder, Runtime};
use webhook::client::{WebhookClient, WebhookResult};
use webhook::models::Message;

struct DiscordWebhookClient {
    inner: WebhookClient,
    rt: Runtime
}

impl DiscordWebhookClient {
    pub fn new(url: &str) -> Self{
        DiscordWebhookClient { inner: WebhookClient::new(url), rt: Builder::new_current_thread().enable_all().build().unwrap()}
    }

    pub fn send_message(&self, message: &Message) -> WebhookResult<bool> {
        self.rt.block_on(self.inner.send_message(message))
    }
}

pub fn create_message(username: &str, content: &str) -> Message {
    let mut msg = Message::new();
    msg.username(username).content(content);
    msg
}

pub fn send_messages (url: &str, messages: Vec<Message>) {
    let client = DiscordWebhookClient::new(url);
    let results:Vec<_> =  messages.iter().map(|msg| client.send_message(msg)).collect();
    let ok_count = results.iter().filter_map(|result| result.as_ref().ok()).count();
    let err_count = results.iter().filter_map(|result| result.as_ref().err()).count();

    println!("Messages OK: {}  Messages with Error: {}", ok_count, err_count);
}