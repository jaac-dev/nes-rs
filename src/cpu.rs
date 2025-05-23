use crate::bus::{Bus, BusInterface};

#[derive(Debug)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn tick(&mut self, bus: &mut dyn BusInterface) {
        let op = bus.read(self.registers.pc);

        match op {
            _ => {}
        }
    }

    fn read_operand(&mut self, bus: &mut dyn BusInterface, operand: Operand) -> u8 {
        match operand {
            Operand::Accumulator => self.registers.a,
            Operand::Memory(addr) => bus.read(addr),
        }
    }

    fn write_operand(&mut self, bus: &mut dyn BusInterface, operand: Operand, data: u8) {
        match operand {
            Operand::Accumulator => self.registers.a = data,
            Operand::Memory(addr) => bus.write(addr, data),
        }
    }

    fn adc(&mut self, bus: &mut dyn BusInterface, operand: Operand) -> u8 {
        let memory = self.read_operand(bus, operand);
        let result =
            self.registers.a as u16 + memory as u16 + self.registers.get_flag(Flag::Carry) as u16;

        self.registers.set_flag(Flag::Carry, result > 0xFF);

        let result = result as u8;

        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(
            Flag::Overflow,
            (self.registers.a ^ result) & (memory & result) & 0x80 != 0,
        );

        0
    }

    fn and(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn asl(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn bcc(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn bcs(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn beq(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn bit(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn bmi(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn bpl(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn brk(&mut self, bus: &mut dyn BusInterface) -> u8 {
        0
    }
    fn bvc(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn bvs(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn clc(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn cld(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn cli(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn clv(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn cmp(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn cpy(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn dec(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn dex(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn dey(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn eor(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn inc(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn inx(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn jmp(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn jsr(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn lda(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn ldx(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn lsr(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn nop(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn ora(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn pha(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn php(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn pla(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn plp(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn rol(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn ror(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn rti(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn rts(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn sbc(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn sec(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn sed(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn sei(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn sta(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn stx(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn sty(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn tax(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn tay(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn tsx(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn txa(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn txs(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
    fn tya(&mut self, bus: &mut dyn BusInterface, data: Operand) -> u8 {
        0
    }
}

#[derive(Debug)]
struct Registers {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    ps: u8,
    pc: u16,
}

impl Registers {
    fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            ps: 0,
            pc: 0,
        }
    }

    fn get_flag(&self, flag: Flag) -> bool {
        self.ps & flag.as_mask() != 0
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.ps |= flag.as_mask();
        } else {
            self.ps &= !flag.as_mask();
        }
    }
}

#[derive(Debug)]
enum Flag {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    Break,
    Overflow,
    Negative,
}

impl Flag {
    fn as_mask(&self) -> u8 {
        match self {
            Flag::Carry => 0,
            Flag::Zero => 1,
            Flag::InterruptDisable => 2,
            Flag::Decimal => 4,
            Flag::Break => 8,
            Flag::Overflow => 32,
            Flag::Negative => 64,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Accumulator,
    Memory(u16),
}
