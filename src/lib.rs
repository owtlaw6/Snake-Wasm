use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Event, KeyboardEvent};

use std::cell::RefCell;
use std::rc::Rc;

extern crate wasm_bindgen;
extern crate web_sys;

mod canvas;
mod direction;
mod snake;

use canvas::Canvas;
use direction::Direction;
use snake::Snake;

#[wasm_bindgen(start)]
pub fn run() {
    let canvas = Canvas::new("#canvas", 20, 20);
    let snake = Rc::new(RefCell::new(Snake::new(20, 20)));
    snake.borrow().draw(&canvas);

    let document = window().unwrap().document().unwrap();

    let score_element = document.create_element("div").unwrap();
    score_element.set_attribute("id", "score").unwrap();
    score_element.set_inner_html("Score: 0");
    document.body().unwrap().append_child(&score_element).unwrap();

    let started = Rc::new(RefCell::new(false));
    let snake_for_event = snake.clone();
    let canvas_for_event = Rc::new(canvas);
    let started_for_event = started.clone();

    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        match event.key().as_str() {
            "ArrowLeft" => snake_for_event.borrow_mut().change_direction(Direction::Left),
            "ArrowRight" => snake_for_event.borrow_mut().change_direction(Direction::Right),
            "ArrowDown" => snake_for_event.borrow_mut().change_direction(Direction::Down),
            "ArrowUp" => snake_for_event.borrow_mut().change_direction(Direction::Up),
            _ => {}
        }

        if !*started_for_event.borrow() {
            *started_for_event.borrow_mut() = true;
            game_loop(snake_for_event.clone(), canvas_for_event.clone(), 200)
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);

    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
    closure.forget();

    fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<Canvas>, time: u32) {
        let snake_clone = snake.clone();
        let canvas_clone = canvas.clone();

        let closure = Closure::wrap(Box::new(move || {
            game_loop(snake_clone.clone(), canvas_clone.clone(), time);
            snake_clone.borrow_mut().update();
            snake_clone.borrow().draw(&canvas_clone);

            let score = snake_clone.borrow().get_score();
            let document = window().unwrap().document().unwrap();
            let score_element = document.get_element_by_id("score").unwrap();
            score_element.set_inner_html(&format!("Score: {}", score));
        }) as Box<dyn FnMut()>);

        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                time as i32,
            )
            .unwrap();

        closure.forget();
    }
}


    // canvas.draw(5, 5, "red");
    // canvas.draw(10, 10, "orange");
    // canvas.draw(15, 10, "blue");
    // canvas.draw(15, 15, "pink");