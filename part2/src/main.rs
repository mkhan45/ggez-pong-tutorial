use ggez::{event::EventHandler, graphics::Rect};

type Vector = ggez::mint::Vector2<f32>;

const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 600.;

const X_OFFSET: f32 = 20.;
const PADDLE_WIDTH: f32 = 12.;
const PADDLE_HEIGHT: f32 = 75.;

const BALL_RADIUS: f32 = 10.;

struct Ball {
    rect: Rect,
    vel: Vector,
}

struct MainState {
    l_paddle: Rect,
    r_paddle: Rect,
    ball: Ball,
    l_score: u16,
    r_score: u16,
}

impl EventHandler for MainState {
    fn update(&mut self, _: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use ggez::graphics;
        use ggez::graphics::Color;

        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0)); // Color::new(0.0, 0.0, 0.0, 1.0) is black

        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball.rect,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .expect("error creating ball mesh");
        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())
            .expect("error drawing ball mesh");

        let l_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.l_paddle,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .expect("error creating ball mesh");
        graphics::draw(ctx, &l_paddle_mesh, graphics::DrawParam::default())
            .expect("error drawing ball mesh");

        let r_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.r_paddle,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .expect("error creating ball mesh");
        graphics::draw(ctx, &r_paddle_mesh, graphics::DrawParam::default())
            .expect("error drawing ball mesh");

        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())
            .expect("error drawing ball mesh");

        graphics::present(ctx).expect("error presenting");
        Ok(())
    }
}

fn main() -> ggez::GameResult {
    // create a mutable reference to a `Context` and `EventsLoop`
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Mikail Khan")
        .build()
        .unwrap();

    // Make a mutable reference to `MainState`
    let main_state = &mut MainState {
        l_paddle: Rect::new(
            X_OFFSET,
            SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ),
        r_paddle: Rect::new(
            SCREEN_WIDTH - X_OFFSET,
            SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ),
        ball: Ball {
            rect: Rect::new(
                SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                BALL_RADIUS,
                BALL_RADIUS,
            ),
            vel: Vector { x: 0.0, y: 0.0 },
        },
        l_score: 0,
        r_score: 0,
    };

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}
