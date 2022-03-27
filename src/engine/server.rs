use ggez::Context;

use super::Engine;

impl Engine {
    pub fn tick_as_server(&mut self, ctx: &mut Context) {
        // Grab and apply messages from clients
        self.react(self.sync());
        self.react(self.deal_with_sync_errors_as_server());

        let mut messages = vec![];
        messages.extend(self.tick_entities());
        messages.extend(self.collect_player_inputs(ctx));
        messages.extend(self.ui_events(ctx));
        self.dispatch_as_server(&messages);
        self.react(messages);

        self.graphics.tick(ctx);
    }
}
