use ggez::{
    event::EventHandler,
};

// const SCREEN_HEIGHT: f32 = 600.;
// const SCREEN_WIDTH: f32 = 600.;

// const X_OFFSET: f32 = 20.;
// const PADDLE_SPEED: f32 = 12.;
// const PADDLE_WIDTH: f32 = 12.;
// const PADDLE_HEIGHT: f32 = 75.;

// const BALL_RADIUS: f32 = 10.;
// const MAX_VEL: f32 = 5.;
// const MIN_VEL: f32 = 3.;

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
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Fish").build().unwrap();

    // Make a mutable reference to `MainState`
    let main_state = &mut MainState {};

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}
