use crate::cpu::Cpu;

pub trait BusInterface {
    fn read(&self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, data: u8);
}

pub struct Bus {
    cpu: Cpu,
}

impl Bus {
    pub fn new() -> Self {
        Self { cpu: Cpu::new() }
    }
}

impl BusInterface for Bus {
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }
}
