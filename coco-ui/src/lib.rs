use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

use coco_core::Cpu;
use coco_vm::{VideoBuffer, Vm, SCREEN_HEIGHT, SCREEN_WIDTH, VIDEO_BUFFER_LEN};

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug)]
pub struct Output {
    pub debug: String,
}

pub type Result<T> = core::result::Result<T, JsValue>;

pub type DisplayBuffer = [u8; VIDEO_BUFFER_LEN * 4];
pub type DeviceBuffer = VideoBuffer;
pub type RGB = (u8, u8, u8);

const THEME: [RGB; 0x10] = [
    (0x00, 0x00, 0x00),
    (0x1D, 0x2B, 0x53),
    (0x7E, 0x25, 0x53),
    (0x00, 0x87, 0x51),
    (0xAB, 0x52, 0x36),
    (0x5F, 0x57, 0x4F),
    (0xC2, 0xC3, 0xC7),
    (0xFF, 0xF1, 0xE8),
    (0xFF, 0x00, 0x4D),
    (0xFF, 0xA3, 0x00),
    (0xFF, 0xEC, 0x27),
    (0x00, 0xE4, 0x36),
    (0x29, 0xAD, 0xFF),
    (0x83, 0x76, 0x9C),
    (0xFF, 0x77, 0xA8),
    (0xFF, 0xCC, 0xAA),
];

#[wasm_bindgen(js_name=runRom)]
pub fn run_rom(rom: &[u8]) -> Result<Output> {
    let cpu = Rc::new(RefCell::new(Cpu::new(&rom)));
    let vm = Rc::new(RefCell::new(Vm::new()));

    // call reset vector
    let output = vm.borrow_mut().on_reset(&mut cpu.borrow_mut());

    // setup requestAnimationFrame handler
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let ctx = canvas_context();
    let mut canvas_buffer: DisplayBuffer = [0; VIDEO_BUFFER_LEN * 4];
    render(&mut vm.borrow_mut(), &ctx, &mut canvas_buffer);

    *g.borrow_mut() = Some(Closure::new(move || {
        let on_video_output = vm.borrow_mut().on_video(&mut cpu.borrow_mut());
        if on_video_output.sys_stdout.len() > 0 {
            web_sys::console::log_1(&JsValue::from(on_video_output.sys_stdout));
        }
        render(&mut vm.borrow_mut(), &ctx, &mut canvas_buffer);
        request_animation_frame(f.borrow().as_ref().unwrap())
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(Output {
        debug: output.sys_stdout,
    })
}

fn render(vm: &mut Vm, ctx: &web_sys::CanvasRenderingContext2d, buffer: &mut DisplayBuffer) {
    let pixels = vm.pixels();

    // update buffer and copy its pixel to the canvas
    update_display_buffer(buffer, pixels);
    let image_data = image_data(buffer.as_slice(), SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    ctx.put_image_data(&image_data, 0_f64, 0_f64)
        .expect("Could not copy pixels to canvas");
}

fn update_display_buffer(buffer: &mut DisplayBuffer, pixels: &DeviceBuffer) {
    for i in 0..VIDEO_BUFFER_LEN {
        let raw_color = pixels[i];
        let color = THEME[raw_color as usize];

        let j = i * 4;
        buffer[j + 0] = color.0; // r
        buffer[j + 1] = color.1; // g
        buffer[j + 2] = color.2; // b
        buffer[j + 3] = 0xFF; // alpha
    }
}

#[inline]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Cannot register `requestAnimationFrame`");
}

#[inline]
fn window() -> web_sys::Window {
    web_sys::window().expect("No global `window` exists")
}

#[inline]
fn document() -> web_sys::Document {
    window().document().expect("No `document` in window")
}

#[inline]
fn canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("coco-video")
        .expect("No element with ID `coco-video` exists")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Cannot cast element into <canvas>")
}

#[inline]
fn canvas_context() -> web_sys::CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .expect("Could not get 2D context from <canvas>")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

#[inline]
fn image_data(buffer: &[u8], width: u32, height: u32) -> web_sys::ImageData {
    web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(buffer),
        width,
        height,
    )
    .expect("Could not create image data")
}
