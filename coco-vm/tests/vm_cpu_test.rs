use coco_core::opcodes::*;
use coco_core::Cpu;
use coco_vm::SCREEN_HEIGHT;
use coco_vm::VIDEO_BUFFER_LEN;
use coco_vm::{Vm, SCREEN_WIDTH};

#[test]
fn test_deo_system_debug() {
    let rom = [PUSH, 0xff, PUSH, 0x01, PUSH, 0x02, DEO, BRK];
    let mut cpu = Cpu::new(&rom);
    let mut vm = Vm::new();

    let output = vm.on_reset(&mut cpu);

    let expected_sys_output = "WRK: [ff]\nRET: []".to_string();
    assert_eq!(output.sys_stdout, expected_sys_output);
}

#[test]
fn test_deo_video_pixel_put() {
    let rom = [
        PUSH, 0x01, PUSH, 0x12, DEO, PUSH, 0x01, PUSH, 0x13, DEO, PUSH, 0x08, PUSH, 0x14, DEO, BRK,
    ];
    let mut cpu = Cpu::new(&rom);
    let mut vm = Vm::new();

    let _ = vm.on_reset(&mut cpu);
    let buffer = vm.pixels();

    assert_eq!(buffer[0x01 * SCREEN_WIDTH as usize + 0x01], 0x08);
}

#[test]
fn test_deo_video_pixel_fill() {
    let rom = [
        PUSH,
        0x60,
        PUSH,
        0x12,
        DEO, // x = 96
        PUSH,
        0x48,
        PUSH,
        0x13,
        DEO, // y = 72
        PUSH,
        0b0010_0001,
        PUSH,
        0x14,
        DEO, // fill bg with color 0x01
        BRK,
    ];
    let mut cpu = Cpu::new(&rom);
    let mut vm = Vm::new();

    let _ = vm.on_reset(&mut cpu);
    let buffer = vm.pixels();

    assert_eq!(
        buffer[0x00..VIDEO_BUFFER_LEN / 2],
        [0x00; VIDEO_BUFFER_LEN / 2]
    );
    let expected_slice = [
        [0x00; SCREEN_WIDTH as usize / 2],
        [0x01; SCREEN_WIDTH as usize / 2],
    ]
    .concat();
    for y in (SCREEN_HEIGHT / 2)..SCREEN_HEIGHT {
        let i = y as usize * SCREEN_WIDTH as usize;
        assert_eq!(buffer[i..(i + SCREEN_WIDTH as usize)], expected_slice);
    }
}

#[test]
fn test_deo_video_pixel_fill_with_flip() {
    let rom = [
        PUSH,
        0x60,
        PUSH,
        0x12,
        DEO, // x = 96
        PUSH,
        0x48,
        PUSH,
        0x13,
        DEO, // y = 72
        PUSH,
        0b1110_0001,
        PUSH,
        0x14,
        DEO, // fill top left quadrant of bg with color 0x01
        BRK,
    ];
    let mut cpu = Cpu::new(&rom);
    let mut vm = Vm::new();

    let _ = vm.on_reset(&mut cpu);
    let buffer = vm.pixels();

    let expected_slice = [
        vec![0x01_u8; 1 + SCREEN_WIDTH as usize / 2],
        vec![0x00_u8; SCREEN_WIDTH as usize / 2 - 1],
    ]
    .concat();
    for y in 0..=SCREEN_HEIGHT / 2 {
        let i = y as usize * SCREEN_WIDTH as usize;
        assert_eq!(buffer[i..(i + SCREEN_WIDTH as usize)], expected_slice);
    }

    const IDX: usize = (1 + SCREEN_HEIGHT as usize / 2) * SCREEN_WIDTH as usize;
    assert_eq!(
        buffer[IDX..VIDEO_BUFFER_LEN],
        [0x00; VIDEO_BUFFER_LEN - IDX]
    );
}

#[test]
fn test_deo_sprite() {
    let rom = [
        PUSH2, 0x01, 0x0c, PUSH, 0x18, DEO2, PUSH, 0x00, PUSH, 0x1a, DEO, BRK, 0x11, 0x11, 0x11,
        0x11, 0x10, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x01, 0x10, 0x00,
        0x00, 0x01, 0x10, 0x00, 0x00, 0x01, 0x10, 0x00, 0x00, 0x01, 0x11, 0x11, 0x11, 0x11,
    ];
    let mut cpu = Cpu::new(&rom);
    let mut vm = Vm::new();

    let _ = vm.on_reset(&mut cpu);
    let buffer = vm.pixels();
    // println!("{:?}", buffer);

    assert_eq!(buffer[0..8], [0x01; 8]);
}
