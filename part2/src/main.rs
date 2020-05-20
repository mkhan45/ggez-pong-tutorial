use ggez::{
    event::EventHandler,
};

struct MainState {
}

impl EventHandler for MainState {
    fn update(&mut self, _: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, _: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}

fn main() -> ggez::GameResult {
    // create a mutable reference to a `Context` and `EventsLoop`
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Mikail Khan").build().unwrap();

    // Make a mutable reference to `MainState`
    let main_state = &mut MainState {};

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}
