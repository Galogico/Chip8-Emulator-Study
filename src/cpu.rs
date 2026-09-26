use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Stylize},
    terminal::{self, ClearType},
};
use rand::Rng;
use std::fs;
use std::io::{self, Write};
use std::time::{Duration, Instant};

struct CPU{
    registers: [u8; 16],
    index: u16,
    pc: u16,
    memory: [u8; 4096],
    keyboard: [bool; 16],
    delay: u8,
    sound: u8,
    stack_pointer: u8,
    stack: [u16; 16],
    display: [[bool; 64]; 32],
    tela_modificada: bool
}
impl CPU{
    // functions
    fn ld_vx_byte(&mut self, vx: u8, value: u8) {
        self.registers[vx as usize] = value;
    }
    fn add_vx_by(&mut self, vx: u8, value: u8){
        self.registers[vx as usize] = self.registers[vx as usize].wrapping_add(value);
    }
    fn jmp_addr(&mut self, addr: u16){
        self.pc = addr;
    }
    fn se_vx_byte(&mut self, vx: u8, byte: u8){
        if(self.registers[vx as usize] == byte){
            self.pc += 2;
        }
    }
    fn sne_vx_byte(&mut self, vx: u8, byte: u8){
        if(self.registers[vx as usize] != byte){
            self.pc += 2;
        }
    }
    fn se_vx_vy(&mut self, vx: u8, vy: u8){
        if(self.registers[vx as usize] == self.registers[vy as usize]){
            self.pc += 2;
        }
    }
    fn ld_vx_vy(&mut self, vx: u8, vy: u8){
        self.registers[vx as usize] = self.registers[vy as usize]
    }
    fn or_vx_vy(&mut self, vx: u8, vy: u8){
        self.registers[vx as usize] = self.registers[vx as usize]|self.registers[vy as usize];
    }
    fn and_vx_vy(&mut self, vx: u8, vy: u8){
        self.registers[vx as usize] = self.registers[vx as usize] & self.registers[vy as usize];
    }
    fn xor_vx_vy(&mut self, vx: u8, vy: u8){
        self.registers[vx as usize] ^= self.registers[vy as usize];
    }
    fn jmp_vzero_addr(&mut self, addr: u16){
        self.pc = addr + self.registers[0] as u16;
    }
}