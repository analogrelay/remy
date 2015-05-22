/// Describes cartridge hardware to be emulated
#[derive(Debug)]
pub struct Cartridge {
    /// Indicates the iNES Mapper Number to use
    pub mapper: u16,

    /// Indicates the NES 2.0 Submapper Number to use
    pub submapper: u8,

    /// Indicates if there are bus conflicts on the cartridge
    pub bus_conflicts: bool
}

impl Cartridge {
    /// Creates a new `Cartridge` from the provided values
    pub fn new(mapper: u16, submapper: u8, bus_conflicts: bool) -> Cartridge {
        Cartridge {
            mapper: mapper,
            submapper: submapper,
            bus_conflicts: bus_conflicts
        }
    }
}
