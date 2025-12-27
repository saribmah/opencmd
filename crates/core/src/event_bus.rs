//! Pub/sub event bus for component communication.

use opencmd_protocol::AppEvent;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::debug;

/// Channel capacity for the event bus.
const CHANNEL_CAPACITY: usize = 256;

/// Event subscriber that receives events from the bus.
pub struct EventSubscriber {
    receiver: broadcast::Receiver<Arc<AppEvent>>,
}

impl EventSubscriber {
    /// Receive the next event (blocking).
    pub async fn recv(&mut self) -> Option<Arc<AppEvent>> {
        self.receiver.recv().await.ok()
    }

    /// Try to receive an event without blocking.
    pub fn try_recv(&mut self) -> Option<Arc<AppEvent>> {
        self.receiver.try_recv().ok()
    }
}

/// Event bus for pub/sub communication between components.
pub struct EventBus {
    sender: broadcast::Sender<Arc<AppEvent>>,
}

impl EventBus {
    /// Create a new event bus.
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self { sender }
    }

    /// Publish an event to all subscribers.
    pub fn publish(&self, event: AppEvent) {
        debug!("Publishing event: {:?}", event);
        // Ignore send errors (no subscribers)
        let _ = self.sender.send(Arc::new(event));
    }

    /// Subscribe to receive events.
    pub fn subscribe(&self) -> EventSubscriber {
        EventSubscriber {
            receiver: self.sender.subscribe(),
        }
    }

    /// Get the number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_bus() {
        let bus = EventBus::new();
        let mut sub = bus.subscribe();

        bus.publish(AppEvent::ConfigChanged {
            key: "test".to_string(),
        });

        let event = sub.recv().await.unwrap();
        match event.as_ref() {
            AppEvent::ConfigChanged { key } => assert_eq!(key, "test"),
            _ => panic!("Unexpected event type"),
        }
    }
}
