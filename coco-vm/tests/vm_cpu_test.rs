use coco_core::opcodes::*;
use coco_core::Cpu;
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
    let (bg, _) = vm.pixels();

    assert_eq!(bg[0x01 * SCREEN_WIDTH + 0x01], 0x08);
}
