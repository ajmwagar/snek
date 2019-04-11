#![feature(rustc_private)]
//! Draw some multi-colored geometry to the screen
extern crate quicksilver;
extern crate rand;

use rand::prelude::*;

const GRID_PIXELS: i32 = 25;
const GRID_SIZE: i32 = 32;
const DEBUG: bool = true;
const UPDATE_RATE: f64 = 100.0;

use quicksilver::{
    Future, Result,
    geom::{Shape, Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Img, Background::Col, Color, Font, FontStyle, Image},
    input::{Key},
    lifecycle::{Asset, Settings, State, Window, run},
    combinators::result,
};

#[derive(Debug,PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug)]
struct Snake {
    direction: Direction,
    body: Vec<(i32, i32)>
}

struct Snek {
    snake: Snake,
    game_over: bool,
    food: (i32, i32)
}

impl State for Snek {
    fn new() -> Result<Snek> {
        let mut snake = Snake {
            direction: Direction::Right,
            body: vec!((8,8), (8,7), (8,6)),
        };

        let food = rand_food(&mut snake);

        Ok(Snek {
            snake,
            food,
            game_over: false
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        window.set_update_rate(UPDATE_RATE);

        if !self.game_over {


        if window.keyboard()[Key::Left].is_down() && self.snake.direction != Direction::Right {
            self.snake.direction = Direction::Left
        }
        if window.keyboard()[Key::Right].is_down() && self.snake.direction != Direction::Left {
            self.snake.direction = Direction::Right
        }
        if window.keyboard()[Key::Down].is_down() && self.snake.direction != Direction::Up {
            self.snake.direction = Direction::Down
        }
        if window.keyboard()[Key::Up].is_down() && self.snake.direction != Direction::Down {
            self.snake.direction = Direction::Up
        }

        if DEBUG {
            println!("Body Len: {}", self.snake.body.len());
            println!("Snake: {:?}", self.snake);
            println!("Food: {:?}", self.food);
        }

        let current_head = self.snake.body.get(0).unwrap_or(&(0,0));

        let next_head = match self.snake.direction {
            Direction::Up => (current_head.0, current_head.1 - 1),
            Direction::Down => (current_head.0, current_head.1 + 1),
            Direction::Left => (current_head.0 - 1, current_head.1),
            Direction::Right => (current_head.0 + 1, current_head.1),
        };

            self.game_over = is_dead(&next_head, &self.snake);
        if current_head != &self.food {
            self.snake.body.pop();
        }
        else {
            self.food = rand_food(&mut self.snake);
        }
        self.snake.body.insert(0, next_head);

        }

        Ok(())

    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {


        // window.draw(&Rectangle::new((100, 100), (50, 50)), Col(Color::RED));

        // Draw the snake
        if !self.game_over {
            window.clear(Color::GREEN)?;

            window.draw(&Rectangle::new((self.food.0 * GRID_PIXELS, self.food.1 * GRID_PIXELS), (GRID_PIXELS, GRID_PIXELS)), Col(Color::WHITE));

            self.snake.body.iter().for_each(|segment| {
                window.draw(&Rectangle::new((segment.0 * GRID_PIXELS, segment.1 * GRID_PIXELS), (GRID_PIXELS, GRID_PIXELS)), Col(Color::RED));

            });
        }
        else {
            window.clear(Color::BLACK)?;

            let msg = format!("GAME OVER\nScore: {}", self.snake.body.len() - 3);

            let mut asset = Asset::new(Font::load("font.ttf")
                                   .and_then(move |font| {
                                       let style = FontStyle::new(72.0, Color::RED);
                                       result(font.render(&msg, &style))
                                   }));
            asset.execute(|image| {
                window.draw(&image.area().with_center((400, 300)), Img(&image));
                Ok(())
            })?;
        }

        Ok(())
    }
}

fn main() {
    let size = GRID_PIXELS * GRID_SIZE;
    run::<Snek>("Snek AI", Vector::new(size, size), Settings::default());
}

fn is_dead(next_head: &(i32, i32), snake: &Snake) -> bool {
    let mut res: bool = false;


    if (next_head.0 >= GRID_SIZE || next_head.0 < 0)  || (next_head.1 >= GRID_SIZE || next_head.1 < 0){
        res = true;
    }
    else {
        snake.body.iter().for_each(|segment| {
            if next_head == segment {
                res = true;
            }
        });
    }

    res
}

fn rand_food(snake: &mut Snake) -> (i32, i32) {
    let mut res: bool = true;
    let mut rng = rand::thread_rng();

    let next_food: (i32, i32) = (rng.gen_range(0, 17), rng.gen_range(0,17));

    snake.body.iter().for_each(|segment| {
        if &next_food == segment {
            res = false;
        }
    });

    if res {
        next_food
    }
    else {
        rand_food(snake)
    }
}
