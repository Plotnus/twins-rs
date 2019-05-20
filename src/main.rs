// TODO find way to get rid of image
extern crate cgmath;
extern crate image;
extern crate piston_window; // used for pixel_buffer and Texture

use cgmath::InnerSpace;
use cgmath::Point2;
use piston_window::Loop::*;
use piston_window::*;

type Vec2 = cgmath::Vector2<f32>;

// TODO: eventify window loop using match
// TODO: move step code to own function
// TODO: move render code to own function
// TODO: borders enforce collision
// TODO: input doesn't have nan result
// TODO: bullets dissapear on border
// TODO: fix draw trait with colors
// TODO: bullet effect when explode
// TODO: split into multiple files above 750 lines
////////////////////////////////////////////////////////////////////////
type PistonTransform = [[f64; 3]; 2];
////////////////////////////////////////////////////////////////////////
trait Renderable {
    fn draw(&self, color: [f32; 4], c: &PistonTransform, g: &mut G2d);
}
////////////////////////////////////////////////////////////////////////
const PLAYER_WIDTH: f32 = 5.0;
const PLAYER_HEIGHT: f32 = 9.0;
struct Player {
    color: [f32; 4],
    aabb: AABB,
    move_dir: Vec2,
    shoot_dir: Vec2,
    move_speed: f32,
}
impl Player {
    const MOVE_SPEED: f32 = 64.0; // pixels per second
}
impl Renderable for Player {
    // TODO make player model larger than collider
    fn draw(&self, _color: [f32; 4], t: &PistonTransform, g: &mut G2d) {
        self.aabb.draw(self.color, t, g);

        // draw shoot-dir
        let width = self.aabb.max.x - self.aabb.min.x;
        let height = self.aabb.max.y - self.aabb.min.y;
        let center = Vec2::new(
            self.aabb.min.x + width * 0.5,
            self.aabb.min.y + height * 0.5,
        );
        let length = 8.0;
        let radius = 0.32;
        let end = center + self.shoot_dir * length;
        let turret_color: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
        let line = [center.x as f64, center.y as f64, end.x as f64, end.y as f64];
        piston_window::line(turret_color, radius, line, *t, g);
        //        piston_window::line(turret_color, radius, line, *t.translate(20.0,20.0), g);
    }
}
////////////////////////////////////////////////////////////////////////
struct AABB {
    min: Point2<f32>,
    max: Point2<f32>,
}
impl Renderable for AABB {
    fn draw(&self, color: [f32; 4], t: &PistonTransform, g: &mut G2d) {
        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;
        let rect = [
            (self.min.x) as f64,
            (self.min.y) as f64,
            width as f64,
            height as f64,
        ];
        rectangle(color, rect, *t, g);
    }
}
fn are_colliding_aabb_aabb(a: &AABB, b: &AABB) -> bool {
    if a.max.x < b.min.x || b.max.x < a.min.x {
        return false;
    }
    if a.max.y < b.min.y || b.max.y < a.min.y {
        return false;
    }
    return true;
}
////////////////////////////////////////////////////////////////////////
struct Wall {
    color: [f32; 4],
    aabb: AABB,
}
impl Renderable for Wall {
    fn draw(&self, _color: [f32; 4], t: &PistonTransform, g: &mut G2d) {
        self.aabb.draw(self.color, t, g);
    }
}
////////////////////////////////////////////////////////////////////////
// Camera
////////////////////////////////////////////////////////////////////////
struct Game {
    player: Player,
}

////////////////////////////////////////////////////////////////////////
struct Screen {}
impl Screen {
    const PX_WIDTH: u32 = 320;
    const PX_HEIGHT: u32 = 240;
}
////////////////////////////////////////////////////////////////////////
const WORLD_UP: Vec2 = Vec2::new(0.0, -1.0);
const WORLD_DOWN: Vec2 = Vec2::new(0.0, 1.0);
const WORLD_LEFT: Vec2 = Vec2::new(-1.0, 0.0);
const WORLD_RIGHT: Vec2 = Vec2::new(1.0, 0.0);
const WORLD_WIDTH: f32 = Screen::PX_WIDTH as f32;
const WORLD_HEIGHT: f32 = Screen::PX_HEIGHT as f32;
////////////////////////////////////////////////////////////////////////
// Lets make this an ecs

struct Component_Position {
    position: Vec2,
}
struct Component_Velocity {
    velocity: Vec2,
}
struct Component_Scale {
    scale: f32,
}
struct Component_Collider {
    width: f32,
    height: f32,
}
struct Component_Color {
    color: [f32;4]
}
struct World {

}



////////////////////////////////////////////////////////////////////////
fn main() {
    // Screen
    // setup PistonWindow
    let scale = 3.0;
    let px_wide = (Screen::PX_WIDTH as f32 * scale) as u32;
    let px_high = (Screen::PX_HEIGHT as f32 * scale) as u32;
    let window_size = (px_wide, px_high);
    let mut window: PistonWindow = WindowSettings::new("twins", window_size)
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .resizable(false)
        .decorated(true)
        .build()
        .unwrap();
    // // // // // // // // // // // // // // // // // // // //
    // INITALIZE
    let player = Player {
        aabb: AABB {
            min: Point2::new(
                (WORLD_WIDTH - PLAYER_WIDTH) * 0.5,
                (WORLD_HEIGHT - PLAYER_WIDTH) * 0.5,
            ),
            max: Point2::new(
                (WORLD_WIDTH + PLAYER_HEIGHT) * 0.5,
                (WORLD_HEIGHT + PLAYER_HEIGHT) * 0.5,
            ),
        },
        move_dir: Vec2::new(0.0, 0.0),
        shoot_dir: Vec2::new(0.0, 0.0),
        move_speed: 10_f32,
        color: [0.0, 1.0, 1.0, 0.5],
    };
    let mut game = Game { player: player };
    // Setup Input for "Twins"
    let mut movement_input = DirectionalKeyboardInput::new();
    let mut movement_input_map = DirectionalInputMap::new();
    movement_input_map.insert(Button::Keyboard(Key::W), Direction::Up);
    movement_input_map.insert(Button::Keyboard(Key::A), Direction::Left);
    movement_input_map.insert(Button::Keyboard(Key::S), Direction::Down);
    movement_input_map.insert(Button::Keyboard(Key::D), Direction::Right);

    let mut shoot_input = DirectionalKeyboardInput::new();
    let mut shoot_input_map = DirectionalInputMap::new();
    shoot_input_map.insert(Button::Keyboard(Key::I), Direction::Up);
    shoot_input_map.insert(Button::Keyboard(Key::J), Direction::Left);
    shoot_input_map.insert(Button::Keyboard(Key::K), Direction::Down);
    shoot_input_map.insert(Button::Keyboard(Key::L), Direction::Right);

    // Setup Gameboard
    let border_size = 8.0;
    let border_color = [1.0, 0.0, 0.0, 1.0];
    let left_border = Wall {
        aabb: AABB {
            min: Point2::new(0.0, 0.0),
            max: Point2::new(border_size, WORLD_HEIGHT),
        },
        color: border_color,
    };
    let right_border = Wall {
        aabb: AABB {
            min: Point2::new(WORLD_WIDTH - border_size, 0.0),
            max: Point2::new(WORLD_WIDTH, WORLD_HEIGHT),
        },
        color: border_color,
    };
    let bottom_border = Wall {
        aabb: AABB {
            min: Point2::new(0.0, WORLD_HEIGHT - border_size),
            max: Point2::new(WORLD_WIDTH, WORLD_HEIGHT),
        },
        color: border_color,
    };
    let top_border = Wall {
        aabb: AABB {
            min: Point2::new(0.0, 0.0),
            max: Point2::new(WORLD_WIDTH, border_size),
        },
        color: border_color,
    };

    // // // // // // // // // // // // // // // // // // // //
    // LOOP
    while let Some(e) = window.next() {
        match e {
            Event::Input(args) => {
                movement_input.update(&args, &movement_input_map);
                shoot_input.update(&args, &shoot_input_map);
            }
            Event::Loop(Update(args)) => {
                let player = &mut game.player;

                player.move_dir = movement_input.get_direction();
                player.shoot_dir = shoot_input.get_direction();

                player.move_speed = Player::MOVE_SPEED;

                // step game
                let delta = player.move_dir * player.move_speed * args.dt as f32;
                player.aabb.min += delta;
                player.aabb.max += delta;

                // collision detection
                if are_colliding_aabb_aabb(&left_border.aabb, &player.aabb) {
                    let pen_depth = left_border.aabb.max.x - player.aabb.min.x;
                    player.aabb.min.x += pen_depth;
                    player.aabb.max.x += pen_depth;
                }
                if are_colliding_aabb_aabb(&right_border.aabb, &player.aabb) {
                    let pen_depth = right_border.aabb.min.x - player.aabb.max.x;
                    player.aabb.min.x += pen_depth;
                    player.aabb.max.x += pen_depth;
                }
                if are_colliding_aabb_aabb(&top_border.aabb, &player.aabb) {
                    let pen_depth = top_border.aabb.max.y - player.aabb.min.y;
                    player.aabb.min.y += pen_depth;
                    player.aabb.max.y += pen_depth;
                }
                if are_colliding_aabb_aabb(&bottom_border.aabb, &player.aabb) {
                    let pen_depth = bottom_border.aabb.min.y - player.aabb.max.y;
                    player.aabb.min.y += pen_depth;
                    player.aabb.max.y += pen_depth;
                }
            }
            Event::Loop(Render(_args)) => {
                // texture.update(&mut window.encoder, &pixel_buffer).unwrap();
                window.draw_2d(&e, |context, mut g| {
                    let clear_color = [0.2, 0.2, 0.2, 1.0];
                    clear(clear_color, g);
                    // draw board
                    // render enemies
                    let world_to_screen = context.transform.scale(scale as f64, scale as f64);

                    let red = [1.0, 0.0, 0.0, 1.0];
                    let yellow = [1.0, 1.0, 0.0, 1.0];
                    let cyan = [0.0, 1.0, 1.0, 1.0];

                    // draw borders
                    left_border.draw(red, &world_to_screen, &mut g);
                    right_border.draw(red, &world_to_screen, &mut g);
                    top_border.draw(red, &world_to_screen, &mut g);
                    bottom_border.draw(red, &world_to_screen, &mut g);

                    // draw player
                    game.player.draw(yellow, &world_to_screen, &mut g);
                });
            }
            Event::Loop(AfterRender(_args)) => {
                // stuff after rendering and swapping buffers
            }
            Event::Loop(Idle(_args)) => {
                // for background tasks that can be done incrementally
            }
            Event::Custom(id, arc) => println!("CustomEvent: {:?},{:?}", id, arc),
        }
    }
}
////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

////////////////////////////////////////////////////////////////////////////////
use std::collections::HashMap;
use std::collections::LinkedList;

type DirectionalInputMap = HashMap<piston_window::Button, Direction>;
struct DirectionalKeyboardInput {
    most_recent: LinkedList<Direction>,
}

// TODO: have someway to manage repeated signal from HW so a press only togles state once until released
impl DirectionalKeyboardInput {
    ///
    fn new() -> DirectionalKeyboardInput {
        DirectionalKeyboardInput {
            most_recent: LinkedList::new(),
        }
    }

    ///
    pub fn update(&mut self, piston_input: &piston_window::Input, input_map: &DirectionalInputMap) {
        if let piston_window::Input::Button(args) = piston_input {
            match input_map.get(&args.button) {
                Some(direction) => match args.state {
                    ButtonState::Press => {
                        self.push_direction(&direction);
                    }
                    ButtonState::Release => self.remove_direction(&direction),
                },
                None => (),
                _ => (),
            }
        };
    }

    ///
    fn push_direction(&mut self, dir: &Direction) {
        self.remove_direction(dir);
        self.most_recent.push_front(dir.clone());
        assert!(self.most_recent.len() <= 4);
    }

    ///
    fn remove_direction(&mut self, dir: &Direction) {
        for (index, x) in self.most_recent.iter().enumerate() {
            if x == dir {
                let mut b = self.most_recent.split_off(index);
                b.pop_front();
                for x in b {
                    self.most_recent.push_back(x);
                }
                break;
            }
        }
    }

    ///
    pub fn get_direction(&self) -> Vec2 {
        let mut dir = Vec2::new(0.0, 0.0);

        // vertical
        for x in self.most_recent.iter() {
            if *x == Direction::Down {
                dir += WORLD_DOWN;
                break;
            } else if *x == Direction::Up {
                dir += WORLD_UP;
                break;
            }
        }

        // horizontal
        for x in self.most_recent.iter() {
            if *x == Direction::Left {
                dir += WORLD_LEFT;
                break;
            } else if *x == Direction::Right {
                dir += WORLD_RIGHT;
                break;
            }
        }

        if dir.magnitude2() > 0.0 {
            dir = dir.normalize();
        }
        dir
    }
}
