use std::cell::RefCell;
use std::rc::Rc;
use std::f64;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use web_sys::{HtmlCanvasElement, MouseEvent, CanvasRenderingContext2d, window,};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

// Static Mut Game Object

static mut GAME: Option<Box<Game>> = None;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue>{

    // Html Element

    let document = window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas:HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

    // Initialize Game Object

    unsafe {
        GAME = Some(Box::new(Game::new(canvas.clone())));
    }

    // callback requestAnimationFrame Loop

    let closure = Rc::new(RefCell::new(None)); // for loop request_animation_frame callback interface function point
    let closure_cloned = Rc::clone(&closure);  // for first request_animation_frame callback interface function point

    closure_cloned.replace(Some(Closure::wrap(Box::new(move |_time: f64| {
        unsafe { GAME.as_mut().unwrap().on_animation_frame();}
        request_animation_frame(closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>)));

    request_animation_frame(closure_cloned.borrow().as_ref().unwrap());

    // callback mouse click

    let c = Closure::wrap(Box::new(move |e:MouseEvent| {
        unsafe { GAME.as_mut().unwrap().on_click(e.into()); }
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback(
        "mousedown",
        c.as_ref().unchecked_ref(),
    ).unwrap();
    c.forget();

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// Game 

const BASE_WIDTH:i32 = 500;
const BASE_HEIGHT:i32 = 600;
const START_RADIUS:i32 = 50;
const MAX_NUMBER:i32 = 99;
const MAX_SELECT_NUMBER:i32 = 10;
const MAX_NUMBERS:usize = 10;
const MAX_NUMBER_RADIUS:i32 = 200;
const MAX_COLOR:i32 = 4;
const INCREASE_NUMBER_RADIUS:i32 = 8;
const TEXT_COLOR: &str = "rgb(255 255 255)";
const EXIT:i32 = 10000;
fn get_color(c: i32) -> &'static str {
    match c {
        0 => "rgb(0 128 0)",     // Default
        1 => "rgb(24 255 0)",    //GREEN
        2 => "rgb(131 245 44)",  // Light Green
        3 => "rgb(255 255 0)",   // Light Yellow
        _ => "rgba(0 128 0)",    // Default
    }
}

#[derive(Clone)]
struct Game{
    context: CanvasRenderingContext2d,
    numbers: Vec<(i32,i32,i32,i32,i32)>, //x, y, radius, number, color
    screen_width: i32,
    screen_height:i32,
    click_x: i32,
    click_y: i32,
    frame: u32,
    gain: i32, // We can clear the game with 99 points
    status: bool, // true: start, false: finish
}
impl Game{

    // init

    fn new(canvas: HtmlCanvasElement) -> Self{
        let _context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
        let _screen_width = canvas.client_width().into();
        let _screen_height = canvas.client_height().into();
        let mut _numbers:Vec<(i32, i32, i32, i32, i32)> = Vec::with_capacity(10);
        _numbers.push((100, 100, 100, 1, 0));
        Game {
            context: _context,
            numbers: _numbers,
            screen_width: _screen_width,
            screen_height: _screen_height,
            click_x: 0,
            click_y: 0,
            frame: 0,
            gain: 0,
            status: true,
        }
    }
    fn set_click_point(&mut self, click_x:i32, click_y:i32){
        self.click_x = click_x;
        self.click_y = click_y;

        // restart

        if !self.status {
            let mut _numbers:Vec<(i32, i32, i32, i32, i32)> = Vec::with_capacity(10);
            _numbers.push((100, 100, START_RADIUS, 1, 0));
            self.numbers = _numbers;
            self.gain = 0;
            self.status = true;
        }
    }

    // callback animation

    fn on_animation_frame(&mut self) {
        self.frame += 1;
        if self.frame % 10 == 0 {
            if self.status { self.update();}
            self.clear();
            self.draw();
        }
    }

    // callback click

    fn on_click(&mut self, _event: MouseEvent) {

        let _click_x = _event.client_x() * BASE_WIDTH / self.screen_width;
        let _click_y = _event.client_y() * BASE_HEIGHT / self.screen_height;
        let _ = self.set_click_point(_click_x, _click_y);
    }

    // game controller

    fn update(&mut self){
        let _numbers = self.numbers.clone();
        let mut _gain:i32 = 0;
        self.numbers = _numbers.into_iter()
            .filter_map(|x| {
                if (x.0 - self.click_x).abs().pow(2) + (x.1 - self.click_y).abs().pow(2) - (x.2).pow(2) > 0 {
                    if  x.2 < MAX_NUMBER_RADIUS {
                        Some((x.0, x.1, x.2 + INCREASE_NUMBER_RADIUS, x.3, x.4))
                    } else {
                        None
                    }
                } else {
                    _gain = x.3;
                    None
                }
            })
            .collect();
        self.click_x = EXIT;
        self.click_y = EXIT;
        self.gain += _gain;

        if self.numbers.len() < MAX_NUMBERS {
            let mut rnd = rand::thread_rng();
            let _x: i32 = rnd.gen_range(1..BASE_WIDTH);
            let _y: i32 = rnd.gen_range(1..BASE_HEIGHT);
            let _n: i32 = rnd.gen_range(1..MAX_SELECT_NUMBER);
            let _c: i32 = rnd.gen_range(0..MAX_COLOR);
            self.numbers.push((_x, _y, START_RADIUS, _n, _c));
        }

        // game clear

        if self.gain ==  MAX_NUMBER {
            self.status = false;
        }

        if self.gain >  MAX_NUMBER {
            self.gain -= MAX_NUMBER;
        }
    }

    // draw

    fn draw(&self){

        for t in &self.numbers {

            // circle

            let _color:&str = get_color(t.4);
            self.context.set_fill_style_str(_color);
            self.context.set_global_alpha(0.5);
            self.context.begin_path();
            self.context.arc(
                t.0.into(),
                t.1.into(),
                t.2.into(),
                0.0,
                f64::consts::PI * 2.0
            ).unwrap();
            self.context.close_path();
            self.context.fill();

            self.context.set_fill_style_str(TEXT_COLOR);
            self.context.set_global_alpha(0.5);
            self.context.set_text_align("center");
            self.context.set_font("60px, Arial");
            self.context.fill_text(
                &t.3.to_string(),
                t.0.into(),
                t.1.into(),
            ).unwrap();
        }

        // message

        let mut _message = "Click Circle to reach 99";
        if !&self.status { _message = "Congratuation!!"}
        self.context.set_fill_style_str(TEXT_COLOR);
        self.context.set_global_alpha(1.0);
        self.context.set_text_align("left");
        self.context.set_font("150px, Arial");
        self.context.fill_text(
            &format!("{} / {}  {}", self.gain.to_string(), MAX_NUMBER, _message),
            50.0,
            50.0,
        ).unwrap();
    }

    // screen clear

    fn clear(&self){
        self.context.clear_rect(
            0.0,
            0.0,
            self.screen_width as f64,
            self.screen_height as f64,
        );
    }
}