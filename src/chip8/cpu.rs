use rand::Rng;

use crate::CHIP8_MEMORY_SIZE;
use crate::CHIP8_DEFAULT_WIDTH;
use crate::CHIP8_DEFAULT_HEIGHT;

const FONTSET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5 
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

struct ProgramCounter {
    pub counter: usize, // actually u16
}

impl ProgramCounter {
    pub fn new(counter: usize) -> Self {
        Self {
            counter
        }
    }

    fn skip(&mut self) {
        self.counter += 4;
    }

    fn increment(&mut self) {
        self.counter += 2;
    }

    fn skip_if(&mut self, condition: bool) {
        if condition {
            self.skip();
        } else {
            self.increment();
        }
    }
    fn jump(&mut self, addr: usize) {
        self.counter = addr;
    }
}

pub struct Cpu {
    program_counter: ProgramCounter,
    index: usize, // actually u16
    sp: usize, // actually u16
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub graphics: [[u8; CHIP8_DEFAULT_WIDTH]; CHIP8_DEFAULT_HEIGHT],
    memory: [u8; CHIP8_MEMORY_SIZE],
    registers: [u8; 16],
    stack: [usize; 16], // actually u8
    keys: [bool; 16],
    keypad_waiting: bool,
    keypad_register: usize,
    pub graphics_changed: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let program_counter = ProgramCounter::new(0x200);
        let index = 0;
        let sp = 0;
        let delay_timer = 0;
        let sound_timer = 0;
        let graphics = [[0; CHIP8_DEFAULT_WIDTH]; CHIP8_DEFAULT_HEIGHT];
        let mut memory = [0; CHIP8_MEMORY_SIZE];
        let registers = [0;16];
        let stack = [0;16];
        let keys = [false;16];
        for (idx, c) in FONTSET.into_iter().enumerate() {
            memory[idx] = c;
        }
        let keypad_waiting = false;
        let keypad_register = 0;
        let graphics_changed = false;
        Self { program_counter, index , sp , delay_timer , sound_timer , graphics , memory , registers , stack, keys, keypad_waiting, keypad_register, graphics_changed }
    }
    fn get_opcode(&self) -> u16 {
        (self.memory[self.program_counter.counter] as u16) << 8 | (self.memory[self.program_counter.counter + 1] as u16)
    }

    pub fn mainloop(&mut self, keys: [bool; 16]) -> bool {
        self.keys = keys;
        self.graphics_changed = false;

        if self.keypad_waiting {
            for i in 0..keys.len() {
                if keys[i] {
                    self.keypad_waiting = false;
                    self.registers[self.keypad_register] = i as u8;
                    break;
                }
            }
        } else {
            if self.delay_timer > 0 {
                self.delay_timer -= 1
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1
            }
            let opcode = self.get_opcode();
            self.run_opcode(opcode);
        }
        self.graphics_changed
    }

    fn run_opcode(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        match nibbles { 
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8x06(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8x0e(x),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x0c, _, _, _) => self.op_cxkk(x, kk),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            _ => {
                println!("error on opcode: {}", opcode);
                self.program_counter.increment();
            },
        };
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }

    // 00E0 - CLS
    fn op_00e0(&mut self) {
        for y in self.graphics.iter_mut() {
            for x in y.iter_mut() {
                *x = 0;
            }
        }
        self.graphics_changed = true;
        self.program_counter.increment();
    }

    // 00EE - RET
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.program_counter.jump(self.stack[self.sp]);
    }

    // 1nnn - JP addr
    fn op_1nnn(&mut self, addr: usize) {
        self.program_counter.jump(addr);
    }

    // 2nnn - CALL addr
    fn op_2nnn(&mut self, addr: usize) {
        self.stack[self.sp] = self.program_counter.counter + 2;
        self.sp += 1;
        self.program_counter.jump(addr);
    }

    // 3xkk - SE Vx, byte
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        self.program_counter.skip_if(self.registers[x] == kk);
    }

    // 4xkk - SNE Vx, byte
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        self.program_counter.skip_if(self.registers[x] != kk);
    }

    // 5xy0 - SE Vx, Vy
    fn op_5xy0(&mut self, x: usize, y: usize) {
        self.program_counter.skip_if(self.registers[x] == self.registers[y]);
    }

    // 6xkk - LD Vx, byte
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.registers[x] = kk;
        self.program_counter.increment();
    }
    
    // 7xkk - ADD Vx, byte  
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        let vx = self.registers[x] as u16;
        let val = kk as u16;
        let result = vx + val;
        self.registers[x] = result as u8;
        self.program_counter.increment();
    }

    // 8xy0 - LD Vx, Vy
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[y];
        self.program_counter.increment();
    }

    // 8xy1 - OR Vx, Vy
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.registers[x] |= self.registers[y];
        self.program_counter.increment();
    }

    // 8xy2 - AND Vx, Vy
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.registers[x] &= self.registers[y];
        self.program_counter.increment();
    }

    // 8xy3 - XOR Vx, Vy
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.registers[x] ^= self.registers[y];
        self.program_counter.increment();
    }

    // 8xy4 - ADD Vx, Vy
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let vx = self.registers[x] as u16;
        let vy = self.registers[y] as u16;
        let result = vx + vy;
        self.registers[x] = result as u8;
        self.registers[0x0f] = if result > 0xFF { 1 } else { 0 };
        self.program_counter.increment();
    }

    // 8xy5 - SUB Vx, Vy
    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.registers[0x0f] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
        self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
        self.program_counter.increment();
    }

    // 8x06 - SHR Vx {, Vy}
    fn op_8x06(&mut self, x: usize) {
        self.registers[0x0f] = self.registers[x] & 1;
        self.registers[x] >>= 1;
        self.program_counter.increment();
    }

    // 8xy7 - SUBN Vx, Vy
    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.registers[0x0f] = if self.registers[y] > self.registers[x] { 1 } else { 0 };
        self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
        self.program_counter.increment();
    }

    // 8x0e - SHL Vx {, Vy}
    fn op_8x0e(&mut self, x: usize) {
        self.registers[0x0f] = (self.registers[x] & 0b10000000) >> 7;
        self.registers[x] <<= 1;
        self.program_counter.increment();
    }

    // 9xy0 - SNE Vx, Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {
        self.program_counter.skip_if(self.registers[x] != self.registers[y]);
    }

    // annn - LD I, addr
    fn op_annn(&mut self, nnn: usize) {
        self.index = nnn;
        self.program_counter.increment();
    }

    // bnnn - JP V0, addr    
    fn op_bnnn(&mut self, nnn: usize) {
        self.program_counter.jump((self.registers[0] as usize) + nnn);
    }

    // cxkk - RND Vx, byte
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        let mut rng = rand::thread_rng();
        self.registers[x] = rng.gen::<u8>() & kk;
        self.program_counter.increment();
    }

    // dxyn - DRW Vx, Vy, nibble
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.registers[0x0f] = 0;
        for byte in 0..n {
            let y = (self.registers[y] as usize + byte) % CHIP8_DEFAULT_HEIGHT;
            for bit in 0..8 {
                let x = (self.registers[x] as usize + bit) % CHIP8_DEFAULT_WIDTH;
                let color = (self.memory[self.index as usize + byte] >> (7 - bit)) & 1;
                self.registers[0x0f] |= color & self.graphics[y][x];
                self.graphics[y][x] ^= color;
            }
        }
        self.graphics_changed = true;
        self.program_counter.increment();
    }
    // ex9e - SKP Vx
    fn op_ex9e(&mut self, x: usize) {
        self.program_counter.skip_if(self.keys[self.registers[x] as usize]);
    }

    // exa1 - SKNP Vx
    fn op_exa1(&mut self, x: usize) {
        self.program_counter.skip_if(!self.keys[self.registers[x] as usize]);
    }

    // fx07 - LD Vx, DT
    fn op_fx07(&mut self, x: usize) {
        self.registers[x] = self.delay_timer;
        self.program_counter.increment();
    }

    // fx0a - LD Vx, K
    fn op_fx0a(&mut self, x: usize) {
        self.keypad_waiting = true;
        self.keypad_register = x;
        self.program_counter.increment();
    }

    // fx15 - LD DT, Vx
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.registers[x];
        self.program_counter.increment();
    }

    // fx18 - LD ST, Vx
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.registers[x];
        self.program_counter.increment();
    }

    // fx1e - ADD I, Vx
    fn op_fx1e(&mut self, x: usize) {
        self.registers[0x0f] = if self.index > 0x0F00 { 1 } else { 0 };
        self.index += self.registers[x] as usize;
        self.program_counter.increment();
    }

    // fx29 - LD F, Vx
    fn op_fx29(&mut self, x: usize) {
        self.index = (self.registers[x] as usize) * 5;
        self.program_counter.increment();
    }

    // fx33 - LD B, Vx
    fn op_fx33(&mut self, x: usize) {
        self.memory[self.index] = self.registers[x] / 100;
        self.memory[self.index + 1] = (self.registers[x] / 10) % 10;
        self.memory[self.index + 2] = self.registers[x] % 10;
        self.program_counter.increment();
    }
    
    // fx55 - LD [I], Vx
    fn op_fx55(&mut self, x: usize) {
        let mut i: usize = 0;
        while i <= x {
            self.memory[self.index + i] = self.registers[i];
            i+=1;
        }
        self.program_counter.increment();
    }

    // fx65 - LD Vx, [I]
    fn op_fx65(&mut self, x: usize) {
        let mut i: usize = 0;
        while i <= x {
            self.registers[i] = self.memory[self.index + i];
            i+=1;
        }       
        self.program_counter.increment();
    }
}

