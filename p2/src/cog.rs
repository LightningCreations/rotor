use crate::hub::Hub;

pub struct P2Cog {
    /// Cog RAM, aka the general purpose registers.
    /// IMPORTANT: The last 16 dwords of cog ram are also special purpose registers. They need special behavior, don't write to cog RAM directly, use the correct functions.
    regs: [u32; 512],
    /// LUTRAM.
    lutram: [u32; 512],
    /// Address of instruction being fetched. Pipeline stage 1
    fetching_instr: u32,
    /// Instruction currently being decoded. Pipeline stage 2.
    decoding_instr: u32,
    /// Instruction currently being executed. Pipeline stage 3.
    // CONSIDER: Maybe using decoding_instr to do decode in advance? This is probably slower than normal execution with how simple P2 encoding is though.
    executing_instr: u32,

    /// Altered D register, if set by the last executed instr (ALT* class)
    alt_d: Option<u32>,
    /// Altered S register, if set by the last executed instr (ALT* class)
    alt_s: Option<u32>,
    /// Altered R register, if set by the last executed instr (ALT* class)
    alt_r: Option<u32>,

    /// ALTI things
    alt_i_d: Option<u32>,
    /// ALTI things
    alt_i_s: Option<u32>,

    // need to include stuff some of the other ALT class instrs use, but idk what they need as of now. --moony

    skip_mask: u32,
    skip_address: u32,
    skip_type: SkipType,

    /// The `Q` register.
    q: u32,

    /// Interrupt enable.
    interrupt_en: bool,

    /// Repetition counter, used by REP instruction.
    /// Should be zeroed alongside rep_end if the P2 ever takes a branch.
    rep_loop_counter: u32,
    /// Address of instruction REP will decrement the loop counter on.
    rep_end: u32,

    /// Set by the SETLUTS instruction. If set, any writes to LUTRAM should also write to LUTRAM on adjacent cog, this is done through the Hub.
    lut_sharing_enabled: bool,
}

impl P2Cog {
    pub fn execute_2cycle(&mut self, hub: &mut Hub) {
        unimplemented!();
    }
}

pub enum SkipType {
    Normal,
    Fast,
}