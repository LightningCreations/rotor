use crate::hub::Hub;
use std::num::NonZeroU32;

pub struct P2Cog {
    /// Cog RAM, aka the general purpose registers.
    /// IMPORTANT: The last 16 dwords of cog ram are also special purpose registers. They need special behavior, don't write to cog RAM directly, use the correct functions.
    _regs: [u32; 512],
    /// LUTRAM.
    _lutram: [u32; 512],
    /// Address of instruction being fetched. Pipeline stage 1
    _fetching_instr: u32,

    /// Instruction (and possibly immediate) currently being decoded. Pipeline stage 2.
    _decoding_instr: (u32, Option<u32>),
    /// Instruction (and possibly immediate) currently being executed. Pipeline stage 3.
    // CONSIDER: Maybe using decoding_instr to do decode in advance? This is probably slower than normal execution with how simple P2 encoding is though.
    _executing_instr: (u32, Option<u32>),

    /// Program counter
    _pc: u32,

    /// Zero flag
    _z: bool,
    /// Carry flag
    _c: bool,

    /// Altered D register, if set by the last executed instr (ALT* class)
    _alt_d: Option<u32>,
    _aug_d: Option<u32>,
    /// Altered S register, if set by the last executed instr (ALT* class)
    _alt_s: Option<u32>,
    _aug_s: Option<u32>,
    /// Altered R register, if set by the last executed instr (ALT* class)
    _alt_r: Option<u32>,

    /// Used by some ALT* class instructions with NIB/BYTE/WORD instructions.
    _alt_n: Option<u8>,

    /// ALTI data and mask, respectively.
    _alt_i: Option<(u32, u32)>,

    // need to include stuff some of the other ALT class instrs use, but idk what they need as of now. --moony
    /// Controls SKIP, SKIPF, and EXECF behavior. (Mask, Address, Type)
    _skip_ctrl: Option<(u32, u32, SkipType)>,

    /// The `Q` register.
    _q: (u32, LastQSetter),

    /// Interrupt enable.
    _interrupt_en: bool,

    /// Repetition counter and end address, used by REP instruction. (Counter, End Address)
    _rep_ctrl: Option<(NonZeroU32, u32)>,

    /// Set by the SETLUTS instruction. If set, any writes to LUTRAM should also write to LUTRAM on adjacent cog, this is done through the Hub.
    _lut_sharing_enabled: bool,
}

impl P2Cog {
    pub fn execute_2cycle(&mut self, _hub: &mut Hub) {
        unimplemented!();
    }
}

pub enum SkipType {
    Normal,
    Fast,
}

pub enum LastQSetter {
    SETQ, 
    SETQ2,
    Other,
}