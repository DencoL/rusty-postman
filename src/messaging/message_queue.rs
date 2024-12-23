use queue::Queue;

pub struct MessageQueue<TMessage> {
    messages: Queue<TMessage>,
}

impl<TMessage> MessageQueue<TMessage>
where
    TMessage: Clone,
{
    pub fn new(message: &Option<TMessage>) -> Self {
        let mut queue = MessageQueue {
            messages: Queue::new(),
        };

        match message {
            Some(m) => {
                _ = queue.messages.queue(m.to_owned());
            }
            None => {}
        }

        return queue;
    }

    pub fn get_message(&mut self) -> Option<TMessage> {
        return self.messages.dequeue();
    }

    pub fn add_messages(&mut self, messages: Vec<Option<TMessage>>) {
        messages
            .iter()
            .filter(|m| m.is_some())
            .for_each(|m| _ = self.messages.queue(m.to_owned().unwrap()));
    }
}
