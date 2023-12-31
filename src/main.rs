mod TD4;
use self::TD4::CPU::*;

fn main() {
    // let mem = vec![
    //     0b10110011,
    //     0b10110110,
    //     0b10111100,
    //     0b10111000,
    //     0b10111000,
    //     0b10111100,
    //     0b10110110,
    //     0b10110011,
    //     0b10110001,
    //     0b11110000
    // ];

    let mem = vec![
        0b10110111,
        0b00000001,
        0b11100001,
        0b00000001,
        0b11100011,
        0b10110110,
        0b00000001,
        0b11100110,
        0b00000001,
        0b11101000,
        0b10110000,
        0b10110100,
        0b00000001,
        0b11101010,
        0b10111000,
        0b11111111,
    ];

    let mut cpu = CPU::new(mem);
    for _ in 0..200 {
        cpu.tick();
        std::thread::sleep(std::time::Duration::new(1, 0));
    }

}
