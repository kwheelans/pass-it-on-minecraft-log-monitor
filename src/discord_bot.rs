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
    let _results:Vec<_> =  messages.iter().map(|msg| client.send_message(msg)).collect();

/*    let mut ok_count = 0;
    let mut err_count = 0;
    for result in results {
        match result {
            Ok(_) => ok_count += 1,
            _ => err_count += 1,
        }
    }
    println!("OK: {}  Error: {}", ok_count, err_count)*/
}