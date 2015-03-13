#![feature(no_std)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate "std" as std;




// Example expansion
pub struct TestCpu;
pub struct TestExecError;

pub struct Operand;
#[automatically_derived]
impl ::std::fmt::Debug for Operand {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Operand =>
            __arg_0.write_fmt(::std::fmt::Arguments::new_v1({
                                                                static __STATIC_FMTSTR:
                                                                       &'static [&'static str]
                                                                       =
                                                                    &["Operand"];
                                                                __STATIC_FMTSTR
                                                            },
                                                            &match () {
                                                                 () => [],
                                                             })),
        }
    }
}
#[automatically_derived]
impl ::std::marker::Copy for Operand { }
pub enum Test {
    ADC(


        Operand),
    ADD(Operand, Operand),
    BIT(()),
}
#[automatically_derived]
impl ::std::marker::Copy for Test { }
#[automatically_derived]
impl ::std::fmt::Debug for Test {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Test::ADC(ref __self_0),) =>
            __arg_0.write_fmt(::std::fmt::Arguments::new_v1({
                                                                static __STATIC_FMTSTR:
                                                                       &'static [&'static str]
                                                                       =
                                                                    &["ADC(",
                                                                      ")"];
                                                                __STATIC_FMTSTR
                                                            },
                                                            &match (&(*__self_0),)
                                                                 {
                                                                 (__arg0,) =>
                                                                 [::std::fmt::ArgumentV1::new(__arg0,
                                                                                              ::std::fmt::Debug::fmt)],
                                                             })),
            (&Test::ADD(ref __self_0, ref __self_1),) =>
            __arg_0.write_fmt(::std::fmt::Arguments::new_v1({
                                                                static __STATIC_FMTSTR:
                                                                       &'static [&'static str]
                                                                       =
                                                                    &["ADD(",
                                                                      ", ",
                                                                      ")"];
                                                                __STATIC_FMTSTR
                                                            },
                                                            &match (&(*__self_0),
                                                                    &(*__self_1))
                                                                 {
                                                                 (__arg0,
                                                                  __arg1) =>
                                                                 [::std::fmt::ArgumentV1::new(__arg0,
                                                                                              ::std::fmt::Debug::fmt),
                                                                  ::std::fmt::ArgumentV1::new(__arg1,
                                                                                              ::std::fmt::Debug::fmt)],
                                                             })),
            (&Test::BIT(ref __self_0),) =>
            __arg_0.write_fmt(::std::fmt::Arguments::new_v1({
                                                                static __STATIC_FMTSTR:
                                                                       &'static [&'static str]
                                                                       =
                                                                    &["BIT(",
                                                                      ")"];
                                                                __STATIC_FMTSTR
                                                            },
                                                            &match (&(*__self_0),)
                                                                 {
                                                                 (__arg0,) =>
                                                                 [::std::fmt::ArgumentV1::new(__arg0,
                                                                                              ::std::fmt::Debug::fmt)],
                                                             })),
        }
    }
}
impl ::instruction_set::Instruction for Test {type
    CpuState
    =
    TestCpu;type
    ExecutionError
    =
    TestExecError;
    fn exec(&self, cpu: &mut TestCpu) -> Result<(), TestExecError> {
        match self {
            &ADC(op) => { cpu.print("test") }
            &ADD(l, r) => { cpu.print("test") }
            &BIT(a) => { cpu.print("test") }
        }
    }
    fn mnemonic(&self) -> &'static str {
        match self { ADC(..) => "ADC", ADD(..) => "ADD", BIT(..) => "BIT", }
    }
}
