use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand, left: bool) -> Result<(), ExecError> where M : Memory {
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
    use cpu::mos6502::instr::rotate;
    use cpu::mos6502::{Mos6502,Operand,Flags,RegisterName};

    #[test]
    pub fn rotate_is_tested() {
        panic!("no it isn't!");
    }
}
