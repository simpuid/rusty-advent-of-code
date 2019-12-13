extern crate aoc;
use aoc::int_code::IntProgram;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{Color, DrawMode, Mesh, Rect};
use ggez::*;
use ggez::{event, graphics};

const SIZE: f32 = 20f32;

fn block_count(v: &[i64]) -> isize {
    let mut iter = v.iter();
    let mut result: isize = 0;
    while let (Some(_), Some(_), Some(id)) = (iter.next(), iter.next(), iter.next()) {
        if *id == 2 {
            result += 1;
        }
    }
    result
}

struct MainState {
    program: IntProgram,
    left: bool,
    right: bool,
    block_code: Vec<i64>,
    delta: f64,
    paint: bool,
    paddle: i64,
    ball: i64,
}

impl MainState {
    fn new(code: Vec<i64>) -> MainState {
        MainState {
            program: IntProgram::new(code),
            left: false,
            right: false,
            block_code: Vec::new(),
            delta: 0f64,
            paint: false,
            paddle: 0,
            ball: 0,
        }
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.paint {
            self.delta -= 0.002f64;
            self.block_code = self.program.execute(vec![aoc::sign((self.ball - self.paddle) as i32) as i64]);
            self.paint = true;
        }
        ggez::timer::yield_now();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.paint {
            self.paint = false;
            let empty = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0f32, 0f32, SIZE, SIZE), Color::new(0f32, 0f32, 0f32, 1f32)).unwrap();
            let wall = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0f32, 0f32, SIZE, SIZE), Color::new(0.2f32, 0.2f32, 0.2f32, 1f32)).unwrap();
            let block = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0f32, 0f32, SIZE, SIZE), Color::new(1f32, 1f32, 1f32, 1f32)).unwrap();
            let paddle = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0f32, 0f32, SIZE, SIZE), Color::new(0.2f32, 0.2f32, 1f32, 1.2f32)).unwrap();
            let ball = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0f32, 0f32, SIZE, SIZE), Color::new(0.2f32, 0.9f32, 0.2f32, 1.2f32)).unwrap();
            let mut iter = self.block_code.iter();
            while let (Some(x), Some(y), Some(id)) = (iter.next(), iter.next(), iter.next()) {
                if let (-1, 0, score) = (x, y, id) {
                    println!("new score {}", score);
                } else {
                    let rec = match id {
                        1 => &wall,
                        2 => &block,
                        3 => {
                            self.paddle = *x;
                            &paddle
                        }
                        4 => {
                            self.ball = *x;
                            &ball
                        }
                        _ => &empty,
                    };
                    graphics::draw(
                        ctx,
                        rec,
                        (ggez::mint::Point2 {
                            x: *x as f32 * 10f32 + 100f32,
                            y: *y as f32 * 10f32 + 100f32,
                        },),
                    )?;
                }
            }
            graphics::present(ctx)?;
        }
        ggez::timer::yield_now();
        Ok(())
    }
    fn key_down_event(&mut self, _ctx: &mut Context, key_code: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match key_code {
            KeyCode::Left => self.left = true,
            KeyCode::Right => self.right = true,
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, key_code: KeyCode, _keymods: KeyMods) {
        match key_code {
            KeyCode::Left => self.left = false,
            KeyCode::Right => self.right = false,
            _ => (),
        }
    }
}

fn main() {
    let code = aoc::parse_file::<i64>("input.txt", ",");
    let mut comp = IntProgram::new(code.clone());
    let output = comp.execute(Vec::new());
    println!("block count: {}", block_count(&output));
    let cb = ggez::ContextBuilder::new("int code arcade", "aoc author");
    let (ctx, event_loop) = &mut cb.build().unwrap();
    let state = &mut MainState::new(code);
    state.program.set(0, 2);
    event::run(ctx, event_loop, state).unwrap();
}
