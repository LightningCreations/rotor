use crate::cog::P2Cog;

pub struct P2Hub {
    _ram: Box<[u8]>,
}

impl P2Hub {
    pub fn execute_cycle(&mut self, _cogs: &mut [P2Cog]) {
        unimplemented!();
    }
}
