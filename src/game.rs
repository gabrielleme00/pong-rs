use macroquad::prelude::*;

pub const SCR_W: f32 = 400.0;
pub const SCR_W_HALF: f32 = SCR_W / 2.;
pub const SCR_H: f32 = 300.0;
pub const SCR_H_HALF: f32 = SCR_H / 2.;

const PAD_W: f32 = 5.;
const PAD_H: f32 = 50.;

const PAD_START_SPEED: f32 = 100.;
const BALL_START_SPEED: f32 = 150.;

pub struct State {
    ball: Ball,
    p1: Player,
    p2: Player,
}

struct Ball {
    pos: Vec2,
    dir: Vec2,
    r: f32,
    speed: f32,
    extent: Vec2,
}

struct Player {
    pos: Vec2,
    up: bool,
    down: bool,
    speed: f32,
    score: u32,
    extent: Vec2,
}

impl State {
    pub fn new() -> State {
        State {
            ball: Ball::new(),
            p1: Player::new(PAD_W),
            p2: Player::new(SCR_W - (PAD_W * 2.)),
        }
    }

    pub fn update(&mut self) {
        // Border hit detection
        let hit_top    = self.ball.pos.y <= 0.;
        let hit_bottom = self.ball.pos.y >= SCR_H;
        let hit_left   = self.ball.pos.x <= 0.;
        let hit_right  = self.ball.pos.x >= SCR_W;

        // Border hit logic
        if hit_top    { self.ball.invert_y(); }
        if hit_bottom { self.ball.invert_y(); }
        if hit_left   { self.p2.score += 1; self.next_turn(); }
        if hit_right  { self.p1.score += 1; self.next_turn(); }

        // If a border was hit, increase speed
        if hit_top || hit_bottom || hit_left || hit_right {
            self.ball.speed += 1.;
        }

        // Update position based on input
        self.process_input();
        self.p1.update();
        self.p2.update();
        self.ball.update();

        // Positions and body extents to calculate collisions
        let ball_pos = &self.ball.pos;
        let ball_extent = &self.ball.extent;
        let p1_pos = &self.p1.center();
        let p2_pos = &self.p2.center();
        let p1_extent = &self.p1.extent;
        let p2_extent = &self.p2.extent;

        // Collision detection
        let hit_p1 = collide(ball_pos, ball_extent, p1_pos, p1_extent);
        let hit_p2 = collide(ball_pos, ball_extent, p2_pos, p2_extent);

        // Collision logic
        if hit_p1 || hit_p2 { self.ball.invert_x(); }
    }

    fn process_input(&mut self) {
        self.p1.up   = is_key_down(KeyCode::A);
        self.p1.down = is_key_down(KeyCode::Z);
        self.p2.up   = is_key_down(KeyCode::K);
        self.p2.down = is_key_down(KeyCode::M);
    }

    fn next_turn(&mut self) {
        self.ball.reset();
    }
}

impl Ball {
    fn new() -> Ball {
        let radius = 6.;
        Ball {
            pos: screen_center(),
            dir: random_direction(),
            r: radius,
            speed: BALL_START_SPEED,
            extent: vec2(radius, radius),
        }
    }

    fn reset(&mut self) {
        self.pos = screen_center();
        self.dir = random_direction();
    }

    fn update(&mut self) {
        // Update position
        let delta = get_frame_time();
        self.pos.x += self.dir.x * self.speed * delta;
        self.pos.y += self.dir.y * self.speed * delta;
    }

    fn invert_x(&mut self) {
        self.dir.x *= -1.;
    }

    fn invert_y(&mut self) {
        self.dir.y *= -1.;
    }
}

impl Player {
    fn new(x: f32) -> Player {
        Player {
            pos: vec2(x, SCR_H_HALF),
            up: false,
            down: false,
            speed: PAD_START_SPEED,
            score: 0,
            extent: vec2(PAD_W, PAD_H / 2.),
        }
    }

    fn update(&mut self) {
        let delta = get_frame_time();

        let can_move_up = self.pos.y > 0.;
        let can_move_down = self.pos.y + PAD_H < SCR_H;

        if self.up   && can_move_up   { self.pos.y -= self.speed * delta }
        if self.down && can_move_down { self.pos.y += self.speed * delta }
    }

    fn center(&mut self) -> Vec2 {
        vec2(self.pos.x + PAD_W / 2., self.pos.y + PAD_H / 2.)
    }
}

pub fn screen_center() -> Vec2 {
    vec2(SCR_W_HALF, SCR_H_HALF)
}

pub fn draw_frame(state: &State, font: Option<Font>) {
    clear_background(DARKGRAY);

    // Middle line
    draw_line(SCR_W_HALF, 0., SCR_W_HALF, SCR_H, 1., GRAY);

    // Ball
    let ball = &state.ball;
    draw_circle(ball.pos.x, ball.pos.y, ball.r, WHITE);

    // Player 1
    let p1 = &state.p1;
    draw_rectangle(p1.pos.x, p1.pos.y, PAD_W, PAD_H, WHITE);

    // Player 2
    let p2 = &state.p2;
    draw_rectangle(p2.pos.x, p2.pos.y, PAD_W, PAD_H, WHITE);

    draw_ui(font, state.p1.score, state.p2.score);
}

fn draw_ui(font: Option<Font>, score_p1: u32, score_p2: u32) {
    // Score
    {
        let text: &str = &format!("{}:{}", score_p1, score_p2);
        let font_size = 10;
        let dimensions = measure_text(text, font, font_size, 1.0);
        let x = SCR_W_HALF - dimensions.width + 6.5;
    
        draw_text_ex(text, x, 25., TextParams {
            color: LIGHTGRAY,
            font: font.unwrap_or(Default::default()),
            font_size: font_size,
            ..Default::default()
        });
    }
}

fn random_direction() -> Vec2 {
    let seed = macroquad::miniquad::date::now() as _;
    rand::srand(seed);

    let mut x = 0.;
    let mut y = 0.;

    while x == 0. || y == 0. {
        x = rand::gen_range(-5, 5) as f32;
        y = rand::gen_range(-5, 5) as f32;
    }

    vec2(x, y).normalize()
}

fn collide(p1: &Vec2, e1: &Vec2, p2: &Vec2, e2: &Vec2) -> bool {
    // TODO use abs
    (if p1.x < p2.x { p2.x - p1.x } else { p1.x - p2.x }) < e1.x + e2.x &&
        (if p1.y < p2.y { p2.y - p1.y } else { p1.y - p2.y }) < e1.y + e2.y
}
