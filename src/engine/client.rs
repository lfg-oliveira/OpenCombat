use ggez::{Context, GameResult};

use crate::message::*;

use super::Engine;

impl Engine {
    pub fn tick_as_client(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Client require a complete sync as first
        if self.local_state.is_first_frame() {
            self.network
                .send(vec![Message::Network(NetworkMessage::RequireCompleteSync)]);
        }

        // Grab and apply messages from server
        self.react(self.sync(), ctx)?;
        self.react(self.deal_with_sync_errors_as_client(), ctx)?;

        // Collect player activity and react according to
        let mut messages = vec![];
        messages.extend(self.collect_player_inputs(ctx));
        messages.extend(self.ui_events(ctx));
        messages.extend(self.tick_interiors());
        messages.extend(self.tick_physics());
        self.dispatch_as_client(&messages);

        let side_effects = self.react(messages, ctx)?;
        self.react_side_effects(side_effects, ctx);

        Ok(())
    }
}
