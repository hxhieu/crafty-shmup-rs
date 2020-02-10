use peacock::graphics::{self, Animation, Image, Rectangle, View};
use peacock::window;
use peacock::{Context, ContextBuilder, Result, State};

// TODO: Constant class
const WINDOW_SCALE: f32 = 4.0;
// TODO: Configurable?
const WINDOW_WIDTH: f32 = 320.0;
const WINDOW_HEIGHT: f32 = 240.0;

struct GameState {
    animation: Animation,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let sprite_sheet = Image::from_file(ctx, "res/images/Player_Unit.png")
            .expect("Could not load sprite sheet!");
        let animation = Animation::new(
            sprite_sheet,
            vec![
                Rectangle::<i32>::new(128, 0, 32, 32),
                Rectangle::<i32>::new(160, 0, 32, 32),
            ],
            8,
        );

        Self { animation }
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        self.animation.tick();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        let mut view = View::new(
            (0.0, 0.0).into(),
            (WINDOW_WIDTH * WINDOW_SCALE, WINDOW_HEIGHT * WINDOW_SCALE).into(),
        );
        view.set_zoom(WINDOW_SCALE);

        window::set_view(ctx, &view);

        graphics::draw(ctx, &self.animation);

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new(
        "crafty-shmup-rs",
        (WINDOW_WIDTH * WINDOW_SCALE) as u32,
        (WINDOW_HEIGHT * WINDOW_SCALE) as u32,
    )
    .build()?
    .run_with(GameState::new)
}
