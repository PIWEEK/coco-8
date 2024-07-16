use std::rc::Rc;
use std::{cell::RefCell, path::Display};
use wasm_bindgen::prelude::*;

use coco_core::Cpu;
use coco_vm::{Vm, SCREEN_HEIGHT, SCREEN_WIDTH};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Output {
    pub pc: u16,
}

pub type Result<T> = core::result::Result<T, JsValue>;

pub type DisplayBuffer = [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4];
pub type DeviceBuffer = [u8; SCREEN_WIDTH * SCREEN_HEIGHT];
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
    let mut memory = [0; 0x10000];
    memory[0x100..(0x100 + rom.len())].copy_from_slice(rom);

    let cpu = Rc::new(RefCell::new(Cpu::new(memory)));
    let vm = Rc::new(RefCell::new(Vm::new()));

    // call reset vector
    let output = vm.borrow_mut().on_reset(&mut cpu.borrow_mut());

    // setup requestAnimationFrame handler
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let ctx = canvas_context();
    let mut canvas_buffer: DisplayBuffer = [0; SCREEN_WIDTH * SCREEN_HEIGHT * 4];
    *g.borrow_mut() = Some(Closure::new(move || {
        vm.borrow_mut().on_video(&mut cpu.borrow_mut());
        render(&vm.borrow(), &ctx, &mut canvas_buffer);
        request_animation_frame(f.borrow().as_ref().unwrap())
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(Output { pc: output.pc })
}

fn render(vm: &Vm, ctx: &web_sys::CanvasRenderingContext2d, buffer: &mut DisplayBuffer) {
    let (bg, fg) = vm.pixels();

    // clear background
    ctx.set_fill_style(&JsValue::from("#000000"));
    ctx.fill_rect(0.0, 0.0, SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);

    // update buffer and copy its pixel to the canvas
    update_display_buffer(buffer, bg, fg);
    let image_data = image_data(buffer.as_slice(), SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    ctx.put_image_data(&image_data, 0_f64, 0_f64)
        .expect("Could not copy pixels to canvas");
}

fn update_display_buffer(buffer: &mut DisplayBuffer, bg: &DeviceBuffer, fg: &DeviceBuffer) {
    for i in 0..(SCREEN_WIDTH * SCREEN_HEIGHT) {
        let raw_color = if fg[i] == 0x00 { bg[i] } else { fg[i] };
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
