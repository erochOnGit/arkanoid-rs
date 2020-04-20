use super::console;
use crate::event::{EventHandler, EventType};
use std::sync::Once;

static mut APP: Option<Application> = None;
static START_APP: Once = Once::new();

pub struct Application {
    stage: Option<Box<dyn EventHandler>>,
}

impl Application {
    pub unsafe fn init<T, F>(handler: F) -> &'static mut Self
    where
        T: EventHandler + 'static,
        F: FnOnce() -> T,
    {
        let app = Application::get();
        let stage = Box::new(handler());
        app.stage = Some(stage);
        app
    }

    pub fn get() -> &'static mut Self {
        START_APP.call_once(|| unsafe {
            APP = Some(Self { stage: None });
            console::log("Application initialized 🥰");
        });

        unsafe { APP.as_mut().expect("Application is not initialized.") }
    }

    fn get_stage(&mut self) -> &mut Box<dyn EventHandler> {
        self.stage.as_mut().expect("No stage found")
    }

    fn event(&mut self, e: Event) {
        let stage = Application::get().get_stage();

        match e.event {
            EventType::PointerUp => stage.pointer_up(e.values[0], e.values[1]),
            EventType::PointerDown => stage.pointer_down(e.values[0], e.values[1]),
            EventType::PointerMove => stage.pointer_move(e.values[0], e.values[1]),
            EventType::KeyUp => stage.key_up(e.keycode.into()),
            EventType::KeyDown => stage.key_down(e.keycode.into()),
            EventType::KeyPressed => stage.key_pressed(e.keycode.into()),
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub event: crate::event::EventType,
    pub values: [i32; 4],
    pub keycode: super::key::KeyCode,
}

#[no_mangle]
extern "C" fn resize(width: i32, height: i32) {
    let stage = Application::get().get_stage();
    stage.resize(width, height);
}

#[no_mangle]
extern "C" fn frame() {
    let stage = Application::get().get_stage();
    stage.frame();
}

#[no_mangle]
extern "C" fn focus() {
    let stage = Application::get().get_stage();
    stage.focus();
}

#[no_mangle]
extern "C" fn blur() {
    let stage = Application::get().get_stage();
    stage.blur();
}

#[no_mangle]
extern "C" fn pointer(event: EventType, x: i32, y: i32) {
    let mut e: Event = unsafe { std::mem::zeroed() };
    e.event = event;
    e.values[0] = x;
    e.values[1] = y;
    Application::get().event(e);
}

#[no_mangle]
extern "C" fn keyboard(event: EventType, keycode: super::key::KeyCode) {
    let mut e: Event = unsafe { std::mem::zeroed() };
    e.event = event;
    e.keycode = keycode;
    Application::get().event(e);
}