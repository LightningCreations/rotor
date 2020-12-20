use crate::cog::P2Cog;

pub struct Hub {
    /// May have 1 to 16 cogs.
    _cogs: Box<[P2Cog]>,

    _ram: Box<[u8]>,
}
