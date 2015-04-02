pub trait Instruction {
    type CpuState;
    type ExecutionError;
    fn mnemonic(&self) -> &'static str;
    fn exec(&self, cpu: &mut Self::CpuState) -> Result<(), Self::ExecutionError>;
}
