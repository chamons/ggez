use ggez::audio;
use ggez::audio::SoundSource;
use ggez::event;
use ggez::filesystem;
use ggez::graphics::{self, Color};
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
    a: i32,
    direction: i32,
    image: graphics::Image,
    text: graphics::Text,
    pixel_sized_text: graphics::Text,
    rng: oorandom::Rand32,
}

impl MainState {
    fn draw_crazy_lines(&mut self, ctx: &mut Context) -> GameResult {
        let num_lines = 100;
        let mut colors = Vec::new();
        for _ in 0..num_lines {
            let r = self.rng.rand_u32() as u8;
            let b = self.rng.rand_u32() as u8;
            let g = self.rng.rand_u32() as u8;
            colors.push(Color::from((r, g, b, 255)));
        }

        let mut last_point = glam::Vec2::new(400.0, 300.0);
        let mut mb = graphics::MeshBuilder::new();
        for color in colors {
            let x = (self.rng.rand_i32() % 50) as f32;
            let y = (self.rng.rand_i32() % 50) as f32;
            let point = glam::Vec2::new(last_point.x + x, last_point.y + y);
            mb.line(&[last_point, point], 3.0, color)?;
            last_point = point;
        }
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, (glam::Vec2::new(0.0, 0.0),))?;

        Ok(())
    }

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        filesystem::print_all(ctx);

        let image = graphics::Image::new(ctx, "/dragon1.png")?;

        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        let text = graphics::Text::new(("Hello world!", font, 48.0));
        let mut sound = audio::Source::new(ctx, "/sound.ogg")?;

        let pixel_sized_text = graphics::Text::new(("This text is 32 pixels high", font, 32.0));

        // "detached" sounds keep playing even after they are dropped
        let _ = sound.play_detached(ctx);

        let rng = oorandom::Rand32::new(271828);

        let s = MainState {
            a: 0,
            direction: 1,
            image,
            text,
            rng,
            pixel_sized_text,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.a += self.direction;
            if self.a > 250 || self.a <= 0 {
                self.direction *= -1;

                println!("Delta frame time: {:?} ", timer::delta(ctx));
                println!("Average FPS: {}", timer::fps(ctx));
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let c = self.a as u8;
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let color = Color::from((c, c, c, 255));
        let dest_point = glam::Vec2::new(0.0, 0.0);
        graphics::draw(ctx, &self.image, (dest_point, 0.0, color))?;
        graphics::draw(ctx, &self.text, (dest_point, 0.0, color))?;

        let dest_point2 = glam::Vec2::new(0.0, 256.0);
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 256.0, 500.0, 32.0),
            Color::from((0, 0, 0, 255)),
        )?;
        graphics::draw(ctx, &rectangle, (glam::Vec2::new(0.0, 0.0),))?;
        graphics::draw(
            ctx,
            &self.pixel_sized_text,
            (dest_point2, 0.0, Color::WHITE),
        )?;

        self.draw_crazy_lines(ctx)?;
        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("imageview", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
