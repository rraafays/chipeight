#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod fontset {
    pub const ZERO:     (u8, u8, u8, u8, u8) = (0xF0, 0x90, 0x90, 0x90, 0xF0);
    pub const ONE:      (u8, u8, u8, u8, u8) = (0x20, 0x60, 0x20, 0x20, 0x70);
    pub const TWO:      (u8, u8, u8, u8, u8) = (0xF0, 0x10, 0xF0, 0x80, 0xF0);
    pub const THREE:    (u8, u8, u8, u8, u8) = (0xF0, 0x10, 0xF0, 0x10, 0xF0);
    pub const FOUR:     (u8, u8, u8, u8, u8) = (0x90, 0x90, 0xF0, 0x10, 0x10);
    pub const FIVE:     (u8, u8, u8, u8, u8) = (0xF0, 0x80, 0xF0, 0x10, 0xF0);
    pub const SIX:      (u8, u8, u8, u8, u8) = (0xF0, 0x80, 0xF0, 0x90, 0xF0);
    pub const SEVEN:    (u8, u8, u8, u8, u8) = (0xF0, 0x10, 0x20, 0x40, 0x40);
    pub const EIGHT:    (u8, u8, u8, u8, u8) = (0xF0, 0x90, 0xF0, 0x90, 0xF0);
    pub const NINE:     (u8, u8, u8, u8, u8) = (0xF0, 0x90, 0xF0, 0x10, 0xF0);

    pub const A:        (u8, u8, u8, u8, u8) = (0xF0, 0x90, 0xF0, 0x90, 0x90);
    pub const B:        (u8, u8, u8, u8, u8) = (0xE0, 0x90, 0xE0, 0x90, 0xE0);
    pub const C:        (u8, u8, u8, u8, u8) = (0xF0, 0x80, 0x80, 0x80, 0xF0);
    pub const D:        (u8, u8, u8, u8, u8) = (0xE0, 0x90, 0x90, 0x90, 0xE0);
    pub const E:        (u8, u8, u8, u8, u8) = (0xF0, 0x80, 0xF0, 0x80, 0xF0);
    pub const F:        (u8, u8, u8, u8, u8) = (0xF0, 0x80, 0xF0, 0x80, 0x80);
}

pub mod cpu {
    pub struct CPU {
        pub opcode: u16,
        pub memory: [u8; 4096],
        pub graphics: [u8; 64 * 32],

        pub register: [u8; 16],
        pub index_register: u16,
        pub program_counter: u16,

        pub delay_timer: u8,
        pub sound_timer: u8,

        pub stack: [u16; 16],
        pub stack_pointer: u16,

        pub keys: [u8; 16],
    }
}
