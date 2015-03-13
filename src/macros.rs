macro_rules! instruction_set {
    (
        $(#[$attr:meta])* instructions $name:ident for $cpu:ident {
            type ExecutionError = $exec_error:ty,
            $(
                $mnemonic:ident($($arg_name:ident : $arg_type:ty),*) => $implementation:block
            ),+
        }
    ) => {
        #[derive(Debug, Copy)]
        $(#[$attr])*
        pub enum $name {
            $(
                $mnemonic ($($arg_type),*),
            )+
        }

        impl $crate::instruction_set::Instruction for $name {
            type CpuState = $cpu;
            type ExecutionError = $exec_error;

            fn exec(&self, cpu: &mut $cpu) -> Result<(), $exec_error> {
                match self {
                    $(
                        &$mnemonic($($arg_name),*) => $implementation
                    ),+
                }
            }

            fn mnemonic(&self) -> &'static str {
                match self {
                    $(
                        $mnemonic(..) => stringify!($mnemonic)
                    ),+
                }
            }
        }
    }
}

// Example expansion
pub struct TestCpu;
pub struct TestExecError;

#[derive(Copy, Debug)]
pub struct Operand;

instruction_set! {
    instructions Test for TestCpu {
        type ExecutionError = TestExecError,

        ADC(op: Operand) => { cpu.print("test") },
        ADD(l: Operand, r: Operand) => { cpu.print("test") },
        BIT(a: ()) => { cpu.print("test") }
    }
}
