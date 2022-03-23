use ggez::Context;

use crate::message::*;

use super::Engine;

impl Engine {
    pub fn tick_as_client(&mut self, ctx: &mut Context) {
        // Client require a complete sync as first
        if self.local_state.is_first_frame() {
            self.network
                .send(vec![Message::Network(NetworkMessage::RequireCompleteSync)]);
        }

        // Grab and apply messages from server
        self.react(self.sync());
        self.react(self.deal_with_sync_errors_as_client());

        // Collect player activity and react according to
        let mut player_activity_messages = vec![];
        player_activity_messages.extend(self.collect_player_inputs(ctx));
        player_activity_messages.extend(self.ui_events(ctx));
        self.dispatch_as_client(&player_activity_messages);
        self.react(player_activity_messages);
    }
}
