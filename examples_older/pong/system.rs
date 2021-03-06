use storm::cgmath::InnerSpace;
use storm::cgmath::{Vector2, Vector3};
use storm::input::message::InputFrame::{KeyPressed, KeyReleased};
use storm::input::message::*;
use storm::render::color;
use storm::render::message::*;

use pong::Ball;
use pong::Player;

const BALL_X_SPEED: f32 = 0.1;
const PLAYER_Y_SPEED: f32 = 0.05;

const BALL_VELOCITY: f32 = 0.06;

enum PlayerType {
    A,
    B,
}

pub struct System {
    player_a: Player,
    player_a_direction: f32,
    player_b: Player,
    player_a_scores: Vec<QuadReference>,
    player_b_scores: Vec<QuadReference>,
    ball: Ball,
    ball_velocity: Vector3<f32>,
    count: f32,
    direction: f32,
}

impl System {
    pub fn new(render: &mut RenderMessenger) -> System {
        let player_a_position = Vector3::new(-2.5, -0.5, 0.0);
        let player_a_shape = Vector2::new(0.5, 0.5);
        let player_a_token = render.quad_create(player_a_position, player_a_shape, color::PURPLE, DEFAULT_TEXTURE);
        let player_a = Player::new(player_a_token, player_a_position, player_a_shape, color::PURPLE);

        let player_b_position = Vector3::new(2.0, -0.5, 0.0);
        let player_b_shape = Vector2::new(0.5, 0.5);
        let player_b_token = render.quad_create(player_b_position, player_b_shape, color::ORANGE, DEFAULT_TEXTURE);
        let player_b = Player::new(player_b_token, player_b_position, player_b_shape, color::ORANGE);

        let ball_postion = Vector3::new(0.0, -0.0, 0.0);
        let ball_shape = Vector2::new(0.1, 0.1);
        let ball_token = render.quad_create(ball_postion, ball_shape, color::RED, DEFAULT_TEXTURE);
        let ball = Ball::new(ball_token, ball_postion, ball_shape);

        let mut player_a_scores = vec![];
        //        let mut player_b_scores = vec![];
        let y_height = 2.5;

        for _ in 0..5 {
            let a_score = render.quad_create(
                Vector3::new(0.0, y_height, 0.0),
                Vector2::new(0.2, 0.2),
                color::GREEN,
                DEFAULT_TEXTURE,
            );
            //            let b_score = render.quad_create();
            player_a_scores.push(a_score);
            //            player_b_scores.push(b_score);
        }

        render.send();

        System {
            player_a,
            player_a_direction: 0.0,
            player_b,
            player_a_scores: vec![],
            player_b_scores: vec![],
            ball,
            count: 0.5,
            direction: -1.0,
            ball_velocity: Vector3::new(BALL_VELOCITY * -1.0, 0.0, 0.0),
        }
    }

    pub fn input(&mut self, event: InputFrame) {
        match event {
            KeyPressed(k) => {
                if k == Key::Up {
                    self.player_a_direction = 1.0;
                } else if k == Key::Down {
                    self.player_a_direction = -1.0;
                }
            },
            KeyReleased(k) => {
                if k == Key::Up || k == Key::Down {
                    self.player_a_direction = 0.0;
                }
            },
            _ => {},
        }
    }

    fn the_fucking_worst_ai(&mut self) {
        if self.ball_velocity.x > 0.0 {
            //if it is coming at the second player
            if self.ball.ball_position.y > self.player_b.box_position.y {
                self.player_b.box_position += Vector3::new(0.0, PLAYER_Y_SPEED, 0.0);
            } else if self.ball.ball_position.y < self.player_b.box_position.y {
                self.player_b.box_position += Vector3::new(0.0, -PLAYER_Y_SPEED, 0.0);
            }
        }
    }

    fn is_ball_overlapping(&self) -> Option<PlayerType> {
        if self.player_a.overlaps_box(&self.ball) {
            return Some(PlayerType::A);
        }

        if self.player_b.overlaps_box(&self.ball) {
            return Some(PlayerType::B);
        }
        return None;
    }

    fn find_bounce_angle(player: &Player, ball: &Ball) -> Vector3<f32> {
        let player_center = Vector3::new(
            player.box_position.x + player.box_shape.x / 2.0,
            player.box_position.y + player.box_shape.y / 2.0,
            player.box_position.z,
        );
        let ball_center = Vector3::new(
            ball.ball_position.x + ball.ball_shape.x / 2.0,
            ball.ball_position.y + ball.ball_shape.y / 2.0,
            ball.ball_position.z,
        );
        return (ball_center - player_center).normalize();
    }

    fn check_for_out_of_bounds(&mut self) {
        if self.ball.ball_position.y <= -2.5 || self.ball.ball_position.y >= 2.5 {
            self.ball_velocity.y = self.ball_velocity.y * -1.0;
        }

        if self.ball.ball_position.x <= -2.5 || self.ball.ball_position.x >= 2.5 {
            self.ball.ball_position = Vector3::new(0.0, 0.0, 0.0);
            self.ball_velocity = Vector3::new(BALL_VELOCITY, 0.0, 0.0);
        }
    }

    fn handle_collision_check(&mut self, result: Option<PlayerType>) {
        if result.is_some() {
            let result = result.unwrap();
            self.direction = -1.0 * self.direction;
            self.ball.ball_position += self.ball_velocity; //* -5.0;
            let use_player: &Player;
            match result {
                PlayerType::A => {
                    use_player = &self.player_a;
                },
                PlayerType::B => {
                    use_player = &self.player_b;
                },
            }
            let angle_of_velocity = System::find_bounce_angle(use_player, &self.ball);
            self.ball_velocity = BALL_VELOCITY * angle_of_velocity;
        }
    }

    pub fn tick(&mut self, render: &mut RenderMessenger) {
        let result = self.is_ball_overlapping();
        self.handle_collision_check(result);
        self.check_for_out_of_bounds();
        self.the_fucking_worst_ai();

        self.ball.ball_position += self.ball_velocity;
        self.player_a.box_position += Vector3::new(0.0, self.player_a_direction * PLAYER_Y_SPEED, 0.0);

        render.quad_update(
            self.ball.ball_token,
            self.ball.ball_position,
            self.ball.ball_shape,
            color::RED,
            DEFAULT_TEXTURE,
        );
        self.player_a.render(render);
        self.player_b.render(render);
    }
}
