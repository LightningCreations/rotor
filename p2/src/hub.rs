use crate::cog::P2Cog;

pub struct Hub {
    /// May have 1 to 16 cogs.
    cogs: Box<[P2Cog]>,

    ram: Box<[u8]>,
}