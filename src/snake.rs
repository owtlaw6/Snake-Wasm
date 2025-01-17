use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::canvas::Canvas;
use crate::direction::Direction;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Block(u32, u32);

#[derive(Debug)]
pub struct Snake {
    head: Block,
    tail: Vec<Block>,
    food: Block,
    height: u32,
    width: u32,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction,
    score: u32,
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        let head_x: u32 = (js_sys::Math::random() * width as f64).floor() as u32;
        
        let head_y: u32 = (js_sys::Math::random() * height as f64).floor() as u32;

        let head = Block(head_x, head_y);
        
        let food_x: u32 = (js_sys::Math::random() * width as f64).floor() as u32;
        
        let food_y: u32 = (js_sys::Math::random() * height as f64).floor() as u32;

        let food = Block(food_x, food_y);

        let tail = Vec::new();

        Snake {
            head,
            tail, 
            food, 
            height,
            width,
            direction: None, 
            next_direction: None,
            last_direction: Direction::Right,
            score: 0,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if !self.last_direction.opposite_direction_check(direction) && self.direction.is_none() {
            self.direction = Some(direction)
        } else if self.direction.iter().any(|d| !d.opposite_direction_check(direction)) {
            self.next_direction = Some(direction)
        }
    }

    pub fn update(&mut self) {
        let direction = self.direction.unwrap_or(self.last_direction);
        self.last_direction = direction;

        let new_head = match direction {
            Direction::Up => Block(
                (self.head.0) % self.width, 
                (self.head.1.checked_sub(1).unwrap_or(self.height - 1)) % self.height,),
            Direction::Down => Block(
                (self.head.0) % self.width, 
                (self.head.1 + 1) % self.height),
            Direction::Right => Block(
                (self.head.0 + 1) % self.width, 
                (self.head.1) % self.height),
            Direction::Left => Block(
                (self.head.0.checked_sub(1).unwrap_or(self.width - 1)) % self.width,
                (self.head.1) % self.height,),
        };

        self.tail.insert(0, self.head);
        let last_end = self.tail.pop();

        if self.tail.contains(&new_head) {
            *self = Snake::new(self.width, self.height);
        }

        self.head = new_head;

        if self.head == self.food {

            loop {
                let food_x: u32 = (js_sys::Math::random() * self.width as f64).floor() as u32;
                let food_y: u32 = (js_sys::Math::random() * self.height as f64).floor() as u32;
                let new_food = Block(food_x, food_y);
        
                if new_food != self.head && !self.tail.contains(&new_food) {
                    self.food = new_food;
                    break;
                }
            }

            self.score += 1;
            last_end.map(|x| self.tail.push(x));
        }

        self.direction = self.next_direction.take();
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.clear_all();
        
        canvas.draw(self.head.0, self.head.1, "#339966");

        for &Block(x, y) in &self.tail {
            canvas.draw(x, y, "#66cc99");
        }

        canvas.draw(self.food.0, self.food.1, "red");
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }
}