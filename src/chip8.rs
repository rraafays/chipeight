struct CPU {
    opcode: u16,
    memory: [u8; 4096],
    graphics: [u8; 64 * 32],

    register: [u8; 16],
    index_register: u16,
    program_counter: u16,

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    stack_pointer: u16,

    keys: [u8; 16],
}

pub fn init() {
    let cpu = CPU {
        opcode: 0,
        memory: [0; 4096],
        graphics: [0; 2048],

        register: [0; 16],
        index_register: 0,
        program_counter: 0,

        delay_timer: 0,
        sound_timer: 0,

        stack: [0; 16],
        stack_pointer: 0,

        keys: [0; 16],
    };
}
