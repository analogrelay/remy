# Reggie

DSLs for building CPU components that can be run on Remy

## Instruction Set

An instruction set is defined in a `.ris` file

```
addressing modes for Mos6502 {
    Immediate(value: u8)
    Absolute(addr: *u16)
    Indexed(addr: *u16, r: reg) => (addr+r)
}
```
