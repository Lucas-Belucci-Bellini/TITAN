use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::broadcast;

/// Evento de aplicação. `topic` permite que consumidores filtrem a categoria.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppEvent {
    pub topic: String,
    pub payload: Value,
}

/// Barramento simples de eventos em memória.
/// Eventos não persistem: caso não haja consumidores ativos, a publicação pode retornar erro.
#[derive(Clone, Debug)]
pub struct EventBus {
    sender: broadcast::Sender<AppEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity.max(1));
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: AppEvent) -> Result<usize, broadcast::error::SendError<AppEvent>> {
        self.sender.send(event)
    }
}

#[cfg(test)]
mod tests {
    use super::{AppEvent, EventBus};
    use serde_json::json;

    #[tokio::test]
    async fn subscribers_receive_published_events() {
        let bus = EventBus::new(8);
        let mut receiver = bus.subscribe();
        bus.publish(AppEvent {
            topic: "user.created".to_string(),
            payload: json!({"id": 7}),
        })
        .expect("receiver is active");

        let event = receiver.recv().await.expect("event should arrive");
        assert_eq!(event.topic, "user.created");
        assert_eq!(event.payload["id"], 7);
    }
}
