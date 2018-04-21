use core::fmt::Write;
use libtww::game::Console;
use libtww::prelude::*;

use controller;
use core::cell::RefCell;
use utils::*;
use Mutex;

lazy_static! {
    static ref ITEMS: Mutex<Vec<&'static str>> = {
        let mut vec = Vec::new();
        vec.push("Hello");
        vec.push("From");
        vec.push("alloc");
        Mutex(RefCell::new(vec))
    };
}

static mut cursor: usize = 0;

pub fn transition_into() {}

pub fn render() {
    let console = Console::get();
    let lines = &mut console.lines;

    let _ = write!(lines[0].begin(), "Memory Menu");
    let _ = write!(lines[1].begin(), "===========");

    let pressed_b = controller::B.is_pressed();

    if pressed_b {
        transition(MenuState::MainMenu);
        return;
    }

    move_cursor(ITEMS.borrow().len(), unsafe { &mut cursor });

    if controller::DPAD_RIGHT.is_pressed() {
        ITEMS.borrow_mut().push("1");
    }

    for (index, (line, &content)) in lines
        .into_iter()
        .skip(3)
        .zip(ITEMS.borrow().iter())
        .enumerate()
    {
        if index == unsafe { cursor } {
            let _ = write!(line.begin(), "> ");
        } else {
            let _ = write!(line.begin(), "  ");
        }

        let _ = write!(line.append(), "{}", content);
    }
}