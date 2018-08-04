#![no_std]
#![feature(const_fn, trace_macros, log_syntax)]
#![feature(core_intrinsics)]
#![allow(non_upper_case_globals)]

extern crate arrayvec;
#[macro_use]
extern crate libtww;
#[macro_use]
extern crate lazy_static;

use libtww::game::Console;
use libtww::system;
use libtww::system::gx;
use libtww::Link;

pub mod cheat_menu;
pub mod controller;
pub mod flag_menu;
pub mod inventory_menu;
pub mod main_menu;
pub mod memory;
pub mod popups;
pub mod print;
pub mod spawn_menu;
pub mod statics;
pub mod utils;
pub mod warp_menu;

use print::*;

pub static mut visible: bool = false;

#[repr(align(32))]
struct TextureData([u8; 65536]);

struct State {
    tex_obj: gx::TexObj,
}

static mut STATE: Option<State> = None;

unsafe fn get_state() -> &'static mut State {
    static TEXTURE_DATA: TextureData = TextureData(*include_bytes!("../res/out_file.bin"));

    STATE.get_or_insert_with(|| {
        let mut tex_obj = gx::TexObj::default();
        gx::init_tex_obj(
            &mut tex_obj,
            TEXTURE_DATA.0.as_ptr() as *const u8,
            256,
            256,
            gx::TF_I8,
            gx::CLAMP,
            gx::CLAMP,
            gx::FALSE,
        );
        State { tex_obj }
    })
}
#[no_mangle]
pub extern "C" fn init() {
    // Call overriden instruction
    system::cdyl_init_async();

    let console = Console::get();
    console.line_count = 32;
    console.x = 0;
    console.y = 16;
    console.font_scale_x *= 1.2;
    console.font_scale_y *= 1.2;
    console.background_color.a = 150;
    console.clear();
}

#[no_mangle]
pub extern "C" fn game_loop() {
    cheat_menu::apply_cheats();
    let d_down = controller::DPAD_DOWN.is_pressed();
    let rt_down = controller::R.is_down();
    let console = Console::get();

    if unsafe { visible } {
        console.background_color.a = 150;
        utils::render();
    } else if d_down && rt_down && unsafe { !popups::visible } {
        console.visible = true;
        unsafe {
            visible = true;
        }
    } else {
        memory::render_watches();
        // Only check popups if the Debug Menu is not open
        // popups::check_global_flags();
    }
}

fn gu_ortho(mt: &mut gx::Mtx44, t: f32, b: f32, l: f32, r: f32, n: f32, f: f32) {
    let tmp = 1.0 / (r - l);
    mt.cells[0][0] = 2.0 * tmp;
    mt.cells[0][1] = 0.0;
    mt.cells[0][2] = 0.0;
    mt.cells[0][3] = -(r + l) * tmp;

    let tmp = 1.0 / (t - b);
    mt.cells[1][0] = 0.0;
    mt.cells[1][1] = 2.0 * tmp;
    mt.cells[1][2] = 0.0;
    mt.cells[1][3] = -(t + b) * tmp;

    let tmp = 1.0 / (f - n);
    mt.cells[2][0] = 0.0;
    mt.cells[2][1] = 0.0;
    mt.cells[2][2] = -(1.0) * tmp;
    mt.cells[2][3] = -(f) * tmp;

    mt.cells[3][0] = 0.0;
    mt.cells[3][1] = 0.0;
    mt.cells[3][2] = 0.0;
    mt.cells[3][3] = 1.0;
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    gx::set_z_mode(gx::ENABLE, gx::LEQUAL, gx::TRUE);

    //projection
    let mut perspective: gx::Mtx44 = gx::Mtx44 {
        cells: [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ],
    };
    gu_ortho(&mut perspective, 0.0, 480.0, 0.0, 640.0, 0.0, 300.0);
    gx::set_projection(&mut perspective, gx::ORTHOGRAPHIC);

    gx::set_cull_mode(gx::CULL_BACK);

    gx::set_blend_mode(
        gx::BM_BLEND,
        gx::BL_SRCALPHA,
        gx::BL_INVSRCALPHA,
        gx::LO_SET,
    );

    // printf("Hello From Rust, just game you fool!", 20.0, 40.0);
    let link = Link::get();
    printf(&format!("Hearts: {}", link.heart_quarters as f32 / 4.0), 35.0, 65.0,0xF19292FF,0xD5524AFF);
    printf(&format!("Magic: {}", link.magic), 35.0, 105.0, 0x00E664FF, 0x008A3FFF);
}
