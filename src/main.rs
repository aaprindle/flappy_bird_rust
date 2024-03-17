use ggez::conf::WindowMode;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawParam, Rect, Text};
use ggez::input::keyboard;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use std::collections::VecDeque;

const SCREEN_WIDTH: f32 = 400.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PLAYER_SIZE: f32 = 20.0;
const PIPE_WIDTH: f32 = 60.0;
const PIPE_GAP: f32 = 150.0;
const PIPE_SPEED: f32 = 2.0;
const GRAVITY: f32 = 0.4;
const JUMP_VELOCITY: f32 = -8.0;

enum GameState {
    Playing,
    GameOver,
}

struct MainState {
    game_state: GameState,
    player_pos: f32,
    player_vel: f32,
    pipes: VecDeque<Rect>,
    score: i32,
    passed_pipe: bool,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            game_state: GameState::Playing,
            player_pos: SCREEN_HEIGHT / 2.0,
            player_vel: 0.0,
            pipes: VecDeque::new(),
            score: 0,
            passed_pipe: false,
        }
    }

    fn reset(&mut self) {
        self.game_state = GameState::Playing;
        self.player_pos = SCREEN_HEIGHT / 2.0;
        self.player_vel = 0.0;
        self.pipes.clear();
        self.score = 0;
        self.passed_pipe = false;
    }

    fn update(&mut self, ctx: &mut Context) {
        while timer::check_update_time(ctx, 60) {
            match self.game_state {
                GameState::Playing => {
                    self.player_vel += GRAVITY;
                    self.player_pos += self.player_vel;

                    if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                        self.player_vel = JUMP_VELOCITY;
                    }

                    if self.player_pos < 0.0 || self.player_pos > SCREEN_HEIGHT {
                        self.game_state = GameState::GameOver;
                    }

                    let player_rect = Rect::new(
                        SCREEN_WIDTH / 2.0 - PLAYER_SIZE / 2.0,
                        self.player_pos - PLAYER_SIZE / 2.0,
                        PLAYER_SIZE,
                        PLAYER_SIZE,
                    );

                    {
                        if let Some(pipe) = self.pipes.front_mut() {
                            pipe.x -= PIPE_SPEED;

                            if pipe.x < SCREEN_WIDTH / 2.0 && !self.passed_pipe {
                                self.score += 1;
                                self.passed_pipe = true;
                            }

                            if pipe.x < -PIPE_WIDTH {
                                self.pipes.pop_front();
                                self.passed_pipe = false;
                            }
                        }
                    }

                    if let Some(pipe) = self.pipes.front() {
                        if player_rect.overlaps(pipe) {
                            self.game_state = GameState::GameOver;
                        }
                    }

                    if self.pipes.is_empty() || self.pipes.back().unwrap().x < SCREEN_WIDTH - 200.0 {
                        let mut rng = rand::thread_rng();
                        let pipe_y = rng.gen_range(100.0..SCREEN_HEIGHT - 100.0);

                        self.pipes.push_back(Rect::new(
                            SCREEN_WIDTH,
                            0.0,
                            PIPE_WIDTH,
                            pipe_y - PIPE_GAP / 2.0,
                        ));
                        self.pipes.push_back(Rect::new(
                            SCREEN_WIDTH,
                            pipe_y + PIPE_GAP / 2.0,
                            PIPE_WIDTH,
                            SCREEN_HEIGHT - pipe_y - PIPE_GAP / 2.0,
                        ));
                    }
                }
                GameState::GameOver => {
                    if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                        self.reset();
                    }
                }
            }
        }
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        match self.game_state {
            GameState::Playing => {
                let player_rect = Rect::new(
                    SCREEN_WIDTH / 2.0 - PLAYER_SIZE / 2.0,
                    self.player_pos - PLAYER_SIZE / 2.0,
                    PLAYER_SIZE,
                    PLAYER_SIZE,
                );

                let player_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    player_rect,
                    Color::RED,
                )?;

                graphics::draw(ctx, &player_mesh, DrawParam::default())?;

                for pipe in &self.pipes {
                    let pipe_mesh = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        *pipe,
                        Color::GREEN,
                    )?;

                    graphics::draw(ctx, &pipe_mesh, DrawParam::default())?;
                }

                let score_text = Text::new(format!("Score: {}", self.score));
                let score_pos = [10.0, 10.0];
                graphics::draw(
                    ctx,
                    &score_text,
                    DrawParam::default()
                        .dest(score_pos)
                        .color(Color::BLACK)
                        .scale([1.5, 1.5]),
                )?;
            }
            GameState::GameOver => {
                let game_over_text = Text::new("Game Over!");
                let game_over_pos = [SCREEN_WIDTH / 2.0 - 50.0, SCREEN_HEIGHT / 2.0 - 50.0];
                graphics::draw(
                    ctx,
                    &game_over_text,
                    DrawParam::default()
                        .dest(game_over_pos)
                        .color(Color::BLACK)
                        .scale([2.0, 2.0]),
                )?;

                let score_text = Text::new(format!("Final Score: {}", self.score));
                let score_pos = [SCREEN_WIDTH / 2.0 - 70.0, SCREEN_HEIGHT / 2.0];
                graphics::draw(
                    ctx,
                    &score_text,
                    DrawParam::default()
                        .dest(score_pos)
                        .color(Color::BLACK)
                        .scale([1.5, 1.5]),
                )?;

                let restart_text = Text::new("Press Space to Restart");
                let restart_pos = [SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0 + 50.0];
                graphics::draw(
                    ctx,
                    &restart_text,
                    DrawParam::default()
                        .dest(restart_pos)
                        .color(Color::BLACK)
                        .scale([1.5, 1.5]),
                )?;
            }
        }

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Space {
            match self.game_state {
                GameState::Playing => {
                    self.player_vel = JUMP_VELOCITY;
                }
                GameState::GameOver => {
                    self.reset();
                }
            }
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("flappy_bird", "ggez")
        .window_mode(WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let main_state = MainState::new();
    ggez::event::run(ctx, event_loop, main_state)
}