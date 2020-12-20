use crate::{cog::P2Cog, hub::P2Hub};

pub struct Propeller2 {
    hub: P2Hub,
    cogs: Box<[P2Cog]>,
}

impl Propeller2 {
    pub fn execute_cycle(&mut self) {
        for i in self.cogs.iter_mut() {
            i.execute_cycle(&mut self.hub);
        }

        self.hub.execute_cycle(&mut self.cogs);
    }
}
