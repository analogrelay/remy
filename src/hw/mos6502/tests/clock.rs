use mem::{self,MemoryExt};
use hw::mos6502::{self,Instruction,Operand,RegisterName,Flags};

use byteorder::LittleEndian;

#[test]
pub fn adc() {
    TestContext::new()
        .test(Instruction::ADC(Operand::Immediate(0xA5)), 2)
        .test(Instruction::ADC(Operand::Absolute(0x0010)), 3)
        .test(Instruction::ADC(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::ADC(Operand::Absolute(0x0110)), 4)
        .test(Instruction::ADC(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::ADC(Operand::Indexed(0x01FF, RegisterName::X)), 5)
        .test(Instruction::ADC(Operand::PreIndexedIndirect(0x0000)), 6)
        .test(Instruction::ADC(Operand::PostIndexedIndirect(0x0000)), 5)
        .test(Instruction::ADC(Operand::PostIndexedIndirect(0x0010)), 6);
}

#[test]
pub fn and() {
    TestContext::new()
        .test(Instruction::AND(Operand::Immediate(0xA5)), 2)
        .test(Instruction::AND(Operand::Absolute(0x0010)), 2)
        .test(Instruction::AND(Operand::Indexed(0x0010, RegisterName::X)), 3)
        .test(Instruction::AND(Operand::Absolute(0x0110)), 4)
        .test(Instruction::AND(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::AND(Operand::Indexed(0x01FF, RegisterName::X)), 5)
        .test(Instruction::AND(Operand::PreIndexedIndirect(0x0000)), 6)
        .test(Instruction::AND(Operand::PostIndexedIndirect(0x0000)), 5)
        .test(Instruction::AND(Operand::PostIndexedIndirect(0x0010)), 6);
}

#[test]
pub fn asl() {
    TestContext::new()
        .test(Instruction::ASL(Operand::Accumulator), 2)
        .test(Instruction::ASL(Operand::Absolute(0x0010)), 5)
        .test(Instruction::ASL(Operand::Indexed(0x0010, RegisterName::X)), 6)
        .test(Instruction::ASL(Operand::Absolute(0x0110)), 6)
        .test(Instruction::ASL(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::ASL(Operand::Indexed(0x01FF, RegisterName::X)), 7);
}

#[test]
pub fn bcc() {
    TestContext::new()
        .test(Instruction::BCC(Operand::Offset(0x01)), 3)
        .test(Instruction::BCC(Operand::Offset(0x11)), 4)
        .with_cpu(|c| c.flags.set(Flags::CARRY()))
        .test(Instruction::BCC(Operand::Offset(0x01)), 2);
}

#[test]
pub fn bcs() {
    TestContext::new()
        .with_cpu(|c| c.flags.set(Flags::CARRY()))
        .test(Instruction::BCS(Operand::Offset(0x01)), 3)
        .test(Instruction::BCS(Operand::Offset(0x11)), 4)
        .with_cpu(|c| c.flags.clear(Flags::CARRY()))
        .test(Instruction::BCS(Operand::Offset(0x01)), 2);
}

#[test]
pub fn beq() {
    TestContext::new()
        .with_cpu(|c| c.flags.set(Flags::ZERO()))
        .test(Instruction::BEQ(Operand::Offset(0x01)), 3)
        .test(Instruction::BEQ(Operand::Offset(0x11)), 4)
        .with_cpu(|c| c.flags.clear(Flags::ZERO()))
        .test(Instruction::BEQ(Operand::Offset(0x01)), 2);
}

#[test]
pub fn bit() {
    TestContext::new()
        .test(Instruction::BIT(Operand::Absolute(0x01)), 3)
        .test(Instruction::BIT(Operand::Absolute(0x0101)), 4);
}

#[test]
pub fn bne() {
    TestContext::new()
        .test(Instruction::BNE(Operand::Offset(0x01)), 3)
        .test(Instruction::BNE(Operand::Offset(0x11)), 4)
        .with_cpu(|c| c.flags.set(Flags::ZERO()))
        .test(Instruction::BNE(Operand::Offset(0x01)), 2);
}

#[test]
pub fn bpl() {
    TestContext::new()
        .test(Instruction::BPL(Operand::Offset(0x01)), 3)
        .test(Instruction::BPL(Operand::Offset(0x11)), 4)
        .with_cpu(|c| c.flags.set(Flags::SIGN()))
        .test(Instruction::BPL(Operand::Offset(0x01)), 2);
}

#[test]
pub fn brk() {
    TestContext::new().test(Instruction::BRK, 7);
}

#[test]
pub fn bvc() {
    TestContext::new()
        .test(Instruction::BVC(Operand::Offset(0x01)), 3)
        .test(Instruction::BVC(Operand::Offset(0x11)), 4)
        .with_cpu(|c| c.flags.set(Flags::OVERFLOW()))
        .test(Instruction::BVC(Operand::Offset(0x01)), 2);
}

#[test]
pub fn bvs() {
    TestContext::new()
        .with_cpu(|c| c.flags.set(Flags::OVERFLOW()))
        .test(Instruction::BVS(Operand::Offset(0x01)), 3)
        .test(Instruction::BVS(Operand::Offset(0x11)), 4)
        .with_cpu(|c| c.flags.clear(Flags::OVERFLOW()))
        .test(Instruction::BVS(Operand::Offset(0x01)), 2);
}

#[test]
pub fn clc() {
    TestContext::new().test(Instruction::CLC, 2);
}

#[test]
pub fn cld() {
    TestContext::new().test(Instruction::CLD, 2);
}

#[test]
pub fn cli() {
    TestContext::new().test(Instruction::CLI, 2);
}

#[test]
pub fn clv() {
    TestContext::new().test(Instruction::CLV, 2);
}

#[test]
pub fn dec() {
    TestContext::new()
        .test(Instruction::DEC(Operand::Absolute(0x01)), 5)
        .test(Instruction::DEC(Operand::Indexed(0x01, RegisterName::X)), 6)
        .test(Instruction::DEC(Operand::Absolute(0x0101)), 6)
        .test(Instruction::DEC(Operand::Indexed(0x0101, RegisterName::X)), 7);
}

#[test]
pub fn dex() {
    TestContext::new().test(Instruction::DEX, 2);
}

#[test]
pub fn dey() {
    TestContext::new().test(Instruction::DEY, 2);
}

#[test]
pub fn eor() {
    TestContext::new()
        .test(Instruction::EOR(Operand::Immediate(0xA5)), 2)
        .test(Instruction::EOR(Operand::Absolute(0x0010)), 3)
        .test(Instruction::EOR(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::EOR(Operand::Absolute(0x0110)), 4)
        .test(Instruction::EOR(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::EOR(Operand::Indexed(0x01FF, RegisterName::X)), 5)
        .test(Instruction::EOR(Operand::PreIndexedIndirect(0x0000)), 6)
        .test(Instruction::EOR(Operand::PostIndexedIndirect(0x0000)), 5)
        .test(Instruction::EOR(Operand::PostIndexedIndirect(0x0010)), 6);
}

#[test]
pub fn inc() {
    TestContext::new()
        .test(Instruction::INC(Operand::Absolute(0x01)), 5)
        .test(Instruction::INC(Operand::Indexed(0x01, RegisterName::X)), 6)
        .test(Instruction::INC(Operand::Absolute(0x0101)), 6)
        .test(Instruction::INC(Operand::Indexed(0x0101, RegisterName::X)), 7);
}

#[test]
pub fn inx() {
    TestContext::new().test(Instruction::INX, 2);
}

#[test]
pub fn iny() {
    TestContext::new().test(Instruction::INY, 2);
}

#[test]
pub fn jmp() {
    TestContext::new()
        .test(Instruction::JMP(Operand::Absolute(0x0101)), 3)
        .test(Instruction::JMP(Operand::Indirect(0x0101)), 5);
}

#[test]
pub fn jsr() {
    TestContext::new().test(Instruction::JSR(Operand::Absolute(0x0101)), 6);
}

#[test]
pub fn lda() {
    TestContext::new()
        .test(Instruction::LDA(Operand::Immediate(0xA5)), 2)
        .test(Instruction::LDA(Operand::Absolute(0x0010)), 3)
        .test(Instruction::LDA(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::LDA(Operand::Absolute(0x0110)), 4)
        .test(Instruction::LDA(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::LDA(Operand::Indexed(0x01FF, RegisterName::X)), 5)
        .test(Instruction::LDA(Operand::PreIndexedIndirect(0x0000)), 6)
        .test(Instruction::LDA(Operand::PostIndexedIndirect(0x0000)), 5)
        .test(Instruction::LDA(Operand::PostIndexedIndirect(0x0010)), 6);
}

#[test]
pub fn ldx() {
    TestContext::new()
        .test(Instruction::LDX(Operand::Immediate(0xA5)), 2)
        .test(Instruction::LDX(Operand::Absolute(0x0010)), 3)
        .test(Instruction::LDX(Operand::Indexed(0x0010, RegisterName::Y)), 4)
        .test(Instruction::LDX(Operand::Absolute(0x0110)), 4)
        .test(Instruction::LDX(Operand::Indexed(0x01E0, RegisterName::Y)), 4)
        .test(Instruction::LDX(Operand::Indexed(0x01FF, RegisterName::Y)), 5);
}

#[test]
pub fn ldy() {
    TestContext::new()
        .test(Instruction::LDY(Operand::Immediate(0xA5)), 2)
        .test(Instruction::LDY(Operand::Absolute(0x0010)), 3)
        .test(Instruction::LDY(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::LDY(Operand::Absolute(0x0110)), 4)
        .test(Instruction::LDY(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::LDY(Operand::Indexed(0x01FF, RegisterName::X)), 5);
}

#[test]
pub fn lsr() {
    TestContext::new()
        .test(Instruction::LSR(Operand::Accumulator), 2)
        .test(Instruction::LSR(Operand::Absolute(0x0010)), 5)
        .test(Instruction::LSR(Operand::Indexed(0x0010, RegisterName::X)), 6)
        .test(Instruction::LSR(Operand::Absolute(0x0110)), 6)
        .test(Instruction::LSR(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::LSR(Operand::Indexed(0x01FF, RegisterName::X)), 7);
}

#[test]
pub fn nop() {
    TestContext::new()
        .test(Instruction::NOP, 2)
        .test(Instruction::NOPX, 2);
}

#[test]
pub fn ora() {
    TestContext::new()
        .test(Instruction::ORA(Operand::Immediate(0xA5)), 2)
        .test(Instruction::ORA(Operand::Absolute(0x0010)), 2)
        .test(Instruction::ORA(Operand::Indexed(0x0010, RegisterName::X)), 3)
        .test(Instruction::ORA(Operand::Absolute(0x0110)), 4)
        .test(Instruction::ORA(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::ORA(Operand::Indexed(0x01FF, RegisterName::X)), 5)
        .test(Instruction::ORA(Operand::PreIndexedIndirect(0x0000)), 6)
        .test(Instruction::ORA(Operand::PostIndexedIndirect(0x0000)), 5)
        .test(Instruction::ORA(Operand::PostIndexedIndirect(0x0010)), 6);
}

#[test]
pub fn pha() {
    TestContext::new().test(Instruction::PHA, 3);
}

#[test]
pub fn php() {
    TestContext::new().test(Instruction::PHP, 3);
}

#[test]
pub fn pla() {
    TestContext::new().test(Instruction::PLA, 4);
}

#[test]
pub fn plp() {
    TestContext::new().test(Instruction::PLP, 4);
}

#[test]
pub fn rol() {
    TestContext::new()
        .test(Instruction::ROL(Operand::Accumulator), 2)
        .test(Instruction::ROL(Operand::Absolute(0x0010)), 5)
        .test(Instruction::ROL(Operand::Indexed(0x0010, RegisterName::X)), 6)
        .test(Instruction::ROL(Operand::Absolute(0x0110)), 6)
        .test(Instruction::ROL(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::ROL(Operand::Indexed(0x01FF, RegisterName::X)), 7);
}

#[test]
pub fn ror() {
    TestContext::new()
        .test(Instruction::ROR(Operand::Accumulator), 2)
        .test(Instruction::ROR(Operand::Absolute(0x0010)), 5)
        .test(Instruction::ROR(Operand::Indexed(0x0010, RegisterName::X)), 6)
        .test(Instruction::ROR(Operand::Absolute(0x0110)), 6)
        .test(Instruction::ROR(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::ROR(Operand::Indexed(0x01FF, RegisterName::X)), 7);
}

#[test]
pub fn rti() {
    TestContext::new().test(Instruction::RTI, 6);
}

#[test]
pub fn rts() {
    TestContext::new().test(Instruction::RTS, 6);
}

#[test]
pub fn sbc() {
    TestContext::new()
        .test(Instruction::SBC(Operand::Immediate(0xA5)), 2)
        .test(Instruction::SBC(Operand::Absolute(0x0010)), 3)
        .test(Instruction::SBC(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::SBC(Operand::Absolute(0x0110)), 4)
        .test(Instruction::SBC(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::SBC(Operand::Indexed(0x01FF, RegisterName::X)), 5)
        .test(Instruction::SBC(Operand::PreIndexedIndirect(0x0000)), 6)
        .test(Instruction::SBC(Operand::PostIndexedIndirect(0x0000)), 5)
        .test(Instruction::SBC(Operand::PostIndexedIndirect(0x0010)), 6)

        .test(Instruction::SBCX(Operand::Immediate(0xA5)), 2)
        .test(Instruction::SBCX(Operand::Absolute(0x0010)), 3)
        .test(Instruction::SBCX(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::SBCX(Operand::Absolute(0x0110)), 4)
        .test(Instruction::SBCX(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::SBCX(Operand::Indexed(0x01FF, RegisterName::X)), 5)
        .test(Instruction::SBCX(Operand::PreIndexedIndirect(0x0000)), 6)
        .test(Instruction::SBCX(Operand::PostIndexedIndirect(0x0000)), 5)
        .test(Instruction::SBCX(Operand::PostIndexedIndirect(0x0010)), 6);
}

#[test]
pub fn sec() {
    TestContext::new().test(Instruction::SEC, 2);
}

#[test]
pub fn sed() {
    TestContext::new().test(Instruction::SED, 2);
}

#[test]
pub fn sei() {
    TestContext::new().test(Instruction::SEI, 2);
}

#[test]
pub fn sta() {
    TestContext::new()
        .test(Instruction::STA(Operand::Absolute(0x0010)), 3)
        .test(Instruction::STA(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::STA(Operand::Absolute(0x0110)), 4)
        .test(Instruction::STA(Operand::Indexed(0x01E0, RegisterName::X)), 5)
        .test(Instruction::STA(Operand::Indexed(0x01FF, RegisterName::X)), 5)
        .test(Instruction::STA(Operand::PreIndexedIndirect(0x0000)), 6)
        .test(Instruction::STA(Operand::PostIndexedIndirect(0x0000)), 6)
        .test(Instruction::STA(Operand::PostIndexedIndirect(0x0010)), 6);
}

#[test]
pub fn stx() {
    TestContext::new()
        .test(Instruction::STX(Operand::Absolute(0x0010)), 3)
        .test(Instruction::STX(Operand::Indexed(0x0010, RegisterName::Y)), 4)
        .test(Instruction::STX(Operand::Absolute(0x0110)), 4);
}

#[test]
pub fn sty() {
    TestContext::new()
        .test(Instruction::STY(Operand::Absolute(0x0010)), 3)
        .test(Instruction::STY(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::STY(Operand::Absolute(0x0110)), 4);
}

#[test]
pub fn tax() {
    TestContext::new().test(Instruction::TAX, 2);
}

#[test]
pub fn tay() {
    TestContext::new().test(Instruction::TAY, 2);
}

#[test]
pub fn tsx() {
    TestContext::new().test(Instruction::TSX, 2);
}

#[test]
pub fn txa() {
    TestContext::new().test(Instruction::TXA, 2);
}

#[test]
pub fn txs() {
    TestContext::new().test(Instruction::TXS, 2);
}

#[test]
pub fn tya() {
    TestContext::new().test(Instruction::TYA, 2);
}

#[test]
pub fn alr() {
    TestContext::new().test(Instruction::ALR(Operand::Immediate(0x01)), 2);
}

#[test]
pub fn anc() {
    TestContext::new().test(Instruction::ANC(Operand::Immediate(0x01)), 2);
}

#[test]
pub fn arr() {
    TestContext::new().test(Instruction::ARR(Operand::Immediate(0x01)), 2);
}

#[test]
pub fn axs() {
    TestContext::new().test(Instruction::AXS(Operand::Immediate(0x00)), 2);
}

#[test]
pub fn lax() {
    TestContext::new()
        .test(Instruction::LAX(Operand::Absolute(0x0010)), 3)
        .test(Instruction::LAX(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::LAX(Operand::Absolute(0x0110)), 4)
        .test(Instruction::LAX(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::LAX(Operand::Indexed(0x01FF, RegisterName::X)), 5)
        .test(Instruction::LAX(Operand::PreIndexedIndirect(0x0000)), 6)
        .test(Instruction::LAX(Operand::PostIndexedIndirect(0x0000)), 5)
        .test(Instruction::LAX(Operand::PostIndexedIndirect(0x0010)), 6);
}

#[test]
pub fn las() {
    TestContext::new()
        .test(Instruction::LAS(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::LAS(Operand::Indexed(0x01FF, RegisterName::X)), 5);
}

#[test]
pub fn sax() {
    TestContext::new()
        .test(Instruction::SAX(Operand::Absolute(0x0010)), 3)
        .test(Instruction::SAX(Operand::Indexed(0x0010, RegisterName::X)), 4)
        .test(Instruction::SAX(Operand::Absolute(0x0110)), 4)
        .test(Instruction::SAX(Operand::PreIndexedIndirect(0x0000)), 6);
}

#[test]
pub fn dcp() {
    TestContext::new()
        .test(Instruction::DCP(Operand::PreIndexedIndirect(0x0000)), 8)
        .test(Instruction::DCP(Operand::Absolute(0x0001)), 5)
        .test(Instruction::DCP(Operand::Absolute(0x0101)), 6)
        .test(Instruction::DCP(Operand::PostIndexedIndirect(0x0000)), 8)
        .test(Instruction::DCP(Operand::PostIndexedIndirect(0x0010)), 8)
        .test(Instruction::DCP(Operand::Indexed(0x0000, RegisterName::X)), 6)
        .test(Instruction::DCP(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::DCP(Operand::Indexed(0x01FF, RegisterName::X)), 7);
}

#[test]
pub fn isb() {
    TestContext::new()
        .test(Instruction::ISB(Operand::PreIndexedIndirect(0x0000)), 8)
        .test(Instruction::ISB(Operand::Absolute(0x0001)), 5)
        .test(Instruction::ISB(Operand::Absolute(0x0101)), 6)
        .test(Instruction::ISB(Operand::PostIndexedIndirect(0x0000)), 8)
        .test(Instruction::ISB(Operand::PostIndexedIndirect(0x0010)), 8)
        .test(Instruction::ISB(Operand::Indexed(0x0000, RegisterName::X)), 6)
        .test(Instruction::ISB(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::ISB(Operand::Indexed(0x01FF, RegisterName::X)), 7);
}

#[test]
pub fn rla() {
    TestContext::new()
        /*.test(Instruction::RLA(Operand::PreIndexedIndirect(0x0000)), 8)
        .test(Instruction::RLA(Operand::Absolute(0x0001)), 5)
        .test(Instruction::RLA(Operand::Absolute(0x0101)), 6)
        .test(Instruction::RLA(Operand::PostIndexedIndirect(0x0000)), 8) */
        .test(Instruction::RLA(Operand::PostIndexedIndirect(0x0010)), 8); /*
        .test(Instruction::RLA(Operand::Indexed(0x0000, RegisterName::X)), 6)
        .test(Instruction::RLA(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::RLA(Operand::Indexed(0x01FF, RegisterName::X)), 7);*/
}

#[test]
pub fn rra() {
    TestContext::new()
        .test(Instruction::RRA(Operand::PreIndexedIndirect(0x0000)), 8)
        .test(Instruction::RRA(Operand::Absolute(0x0001)), 5)
        .test(Instruction::RRA(Operand::Absolute(0x0101)), 6)
        .test(Instruction::RRA(Operand::PostIndexedIndirect(0x0000)), 8)
        .test(Instruction::RRA(Operand::PostIndexedIndirect(0x0010)), 8)
        .test(Instruction::RRA(Operand::Indexed(0x0000, RegisterName::X)), 6)
        .test(Instruction::RRA(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::RRA(Operand::Indexed(0x01FF, RegisterName::X)), 7);
}

#[test]
pub fn slo() {
    TestContext::new()
        .test(Instruction::SLO(Operand::PreIndexedIndirect(0x0000)), 8)
        .test(Instruction::SLO(Operand::Absolute(0x0001)), 5)
        .test(Instruction::SLO(Operand::Absolute(0x0101)), 6)
        .test(Instruction::SLO(Operand::PostIndexedIndirect(0x0000)), 8)
        .test(Instruction::SLO(Operand::PostIndexedIndirect(0x0010)), 8)
        .test(Instruction::SLO(Operand::Indexed(0x0000, RegisterName::X)), 6)
        .test(Instruction::SLO(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::SLO(Operand::Indexed(0x01FF, RegisterName::X)), 7);
}

#[test]
pub fn sre() {
    TestContext::new()
        .test(Instruction::SRE(Operand::PreIndexedIndirect(0x0000)), 8)
        .test(Instruction::SRE(Operand::Absolute(0x0001)), 5)
        .test(Instruction::SRE(Operand::Absolute(0x0101)), 6)
        .test(Instruction::SRE(Operand::PostIndexedIndirect(0x0000)), 8)
        .test(Instruction::SRE(Operand::PostIndexedIndirect(0x0010)), 8)
        .test(Instruction::SRE(Operand::Indexed(0x0000, RegisterName::X)), 6)
        .test(Instruction::SRE(Operand::Indexed(0x01E0, RegisterName::X)), 7)
        .test(Instruction::SRE(Operand::Indexed(0x01FF, RegisterName::X)), 7);
}

#[test]
pub fn skb() {
    TestContext::new()
        .test(Instruction::SKB(Operand::Immediate(0x10)), 2)
        .test(Instruction::SKB(Operand::Absolute(0x0001)), 3)
        .test(Instruction::SKB(Operand::Indexed(0x0001, RegisterName::X)), 4);
}

#[test]
pub fn ign() {
    TestContext::new()
        .test(Instruction::IGN(Operand::Absolute(0x00)), 4)
        .test(Instruction::IGN(Operand::Indexed(0x01E0, RegisterName::X)), 4)
        .test(Instruction::IGN(Operand::Indexed(0x01FF, RegisterName::X)), 5);
}

struct TestContext<'a> {
    cpu: mos6502::Mos6502,
    mem: mem::Virtual<'a>,
    errors: Vec<String>
}

impl<'a> TestContext<'a> {
    pub fn test(mut self, instr: Instruction, cycle_diff: u64) -> Self {
        let before = self.cpu.clock.get();
        if let Err(e) = mos6502::dispatch(instr.clone(), &mut self.cpu, &mut self.mem) {
            panic!("Error dispatching {}: {}", instr, e)
        }
        let actual_diff = self.cpu.clock.get() - before;
        if cycle_diff != actual_diff {
            self.errors.push(format!("{:?} cycles were {}; expected: {}", instr, actual_diff, cycle_diff));
        }

        // Reset CPU state
        self.cpu.registers.sp = 0x80;
        self.cpu.registers.x = 10;
        self.cpu.registers.y = 10;

        self
    }

    pub fn with_cpu<F>(mut self, config: F) -> Self where F: FnOnce(&mut mos6502::Mos6502) {
        config(&mut self.cpu);
        self
    }

    pub fn new() -> TestContext<'a> {
        // 2KB internal ram mirrored through 0x1FFF
        let ram = Box::new(mem::Mirrored::new(mem::Fixed::new(0x0800), 0x2000));

        // Load the ROM into memory
        let prg_rom = Box::new(mem::read_only(mem::Mirrored::new(mem::Fixed::from_contents(vec![0x00]), 0x8000)));

        // Create a black hole for APU/IO registers
        let apu_io = Box::new(mem::Mirrored::new(mem::Fixed::from_contents(vec![0x00]), 0x20));

        // Set up the virtual memory
        let mut memory = mem::Virtual::new();
        memory.attach(0x0000, ram).unwrap();
        memory.attach(0x4000, apu_io).unwrap();
        memory.attach(0x8000, prg_rom).unwrap();

        // Set up the CPU
        let mut cpu = mos6502::Mos6502::without_bcd();
        cpu.flags.replace(mos6502::Flags::new(0x24));
        cpu.pc.set(0xC0F0);
        cpu.registers.sp = 0x80;
        cpu.registers.x = 10;
        cpu.registers.y = 10;

        memory.set_u16::<LittleEndian>(0x0000, 0x0100).unwrap();
        memory.set_u16::<LittleEndian>(0x0010, 0x01FF).unwrap();

        TestContext {
            cpu: cpu,
            mem: memory,
            errors: Vec::new()
        }
    }
}

impl<'a> Drop for TestContext<'a> {
    fn drop(&mut self) {
        if self.errors.len() > 0 {
            println!("");
            println!("Errors:");
            for err in self.errors.iter() {
                println!(" * {}", err);
            }
            panic!("Errors occurred");
        }
    }
}
