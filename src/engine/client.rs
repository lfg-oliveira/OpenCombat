use ggez::Context;

use crate::message::*;

use super::Engine;

impl Engine {
    pub fn tick_as_client(&mut self, ctx: &mut Context) {
        // Client require a complete sync as first
        if self.frame_i == 0 {
            self.network
                .send(vec![Message::Network(NetworkMessage::RequireCompleteSync)]);
        }

        // Will collect all tick messages
        let mut messages = vec![];

        // Retrieve server messages
        messages.extend(self.sync());

        // Check any network errors
        messages.extend(self.deal_with_errors_as_client());

        // Retrieve messages from user inputs
        messages.extend(self.collect_player_inputs(ctx));

        // Apply messages
        self.react(messages);
    }
}
