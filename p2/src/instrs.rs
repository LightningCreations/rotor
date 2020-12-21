#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InstructionKind {
    NOP,
    ROR,
    ROL,
    SHR,
    SHL,
    RCR,
    RCL,
    SAR,
    SAL,
    ADD,
    ADDX,
    ADDS,
    ADDSX,
    SUB,
    SUBX,
    SUBS,
    SUBSX,
    CMP,
    CMPX,
    CMPS,
    CMPSX,
    CMPR,
    CMPM,
    SUBR,
    CMPSUB,
    FGE,
    FLE,
    FGES,
    FLES,
    SUMC,
    SUMNC,
    SUMZ,
    SUMNZ,
    TESTB,
    TESTBN,
    TESTB_AND,
    TESTBN_AND,
    TESTB_OR,
    TESTBN_OR,
    TESTB_XOR,
    TESTBN_XOR,
    BITL,
    BITH,
    BITC,
    BITNC,
    BITZ,
    BITNZ,
    BITRND,
    BITNOT,
    AND,
    ANDN,
    OR,
    XOR,
    MUXC,
    MUXNC,
    MUXZ,
    MUXNZ,
    MOV,
    NOT,
    ABS,
    NEG,
    NEGC,
    NEGNC,
    NEGZ,
    NEGNZ,
    INCMOD,
    DECMOD,
    ZEROX,
    SIGNX,
    ENCOD,
    ONES,
    TEST,
    TESTN,
    SETNIB,
    GETNIB,
    ROLNIB,
    SETBYTE,
    GETBYTE,
    ROLBYTE,
    SETWORD,
    GETWORD,
    ROLWORD,
    ALTSN,
    ALTGN,
    ALTSB,
    ALTGB,
    ALTSW,
    ALTGW,
    ALTR,
    ALTD,
    ALTS,
    ALTB,
    ALTI,
    SETR,
    SETD,
    SETS,
    DECODE,
    BMASK,
    CRCBIT,
    CRCNIB,
    MUXNITS,
    MUXNIBS,
    MUXQ,
    MOVBYTS,
    MUL,
    MULS,
    SCA,
    SCAS,
    ADDPIX,
    MULPIX,
    BLNPIX,
    MIXPIX,
    ADDCT1,
    ADDCT2,
    ADDCT3,
    WMLONG,
    RQPIN,
    RDPIN,
    RDLUT,
    RDBYTE,
    RDWORD,
    RDLONG,
    CALLD,
    CALLPA,
    CALLPB,
    DJZ,
    DJNZ,
    DJF,
    DJNF,
    IJZ,
    IJNZ,
    TJZ,
    TJNZ,
    TJF,
    TJNF,
    TJS,
    TJNS,
    TJV,
    JEVENT, // JINT through JNQMT. The way they're encoded means the specifics of which is which can be offloaded to the instr's impl.
    // Also dodges around a ton of potential code dupe, as they all do roughly the same thing.
    UNUSED1, // Unused instruction slot 0b1011110_1
    UNUSED2, // Unused instruction slot 0b1011111_0
    SETPAT,
    WRPIN,
    WXPIN,
    WYPIN,
    WRLUT,
    WRBYTE,
    WRWORD,
    WRLONG,
    RDFAST,
    WRFAST,
    FBLOCK,
    XINIT,
    XZERO,
    XCONT,
    REP,
    COGINIT,
    QMUL,
    QDIV,
    QFRAC,
    QSQRT,
    QROTATE,
    QVECTOR,
    HUBSET,
    COGID,
    COGSTOP,
    LOCKNEW,
    LOCKRET,
    LOCKTRY,
    LOCKREL,
    QLOG,
    QEXP,
    RFBYTE,
    RFWORD,
    RFLONG,
    RFVAR,
    RFVARS,
    WFBYTE,
    WFWORD,
    WFLONG,
    GETQX,
    GETQY,
    GETCT,
    GETRND,
    GETDACS,
    SETXFRQ,
    GETXACC,
    WAITX,
    SETSE,     // See JEVENT rationale.
    POLLEVENT, // See JEVENT rationale.
    WAITEVENT, // See JEVENT rationale.
    ALLOWI,
    STALLI,
    TRGINT, // See JEVENT rationale.
    NIXINT, // See JEVENT rationale.
    SETINT, // See JEVENT rationale.
    SETQ,
    SETQ2,
    PUSH,
    POP,
    JMP,
    CALL,
    RET,
    CALLA,
    RETA,
    CALLB,
    RETB,
    JMPREL,
    SKIP,
    SKIPF,
    EXECF,
    GETPTR,
    GETBRK,
    COGBRK,
    BRK,
    SETLUTS,
    SETCOLORSPACEVAR, // See JEVENT rationale.
    COGATN,
    TESTP,
    TESTPN,
    TESTP_AND,
    TESTPN_AND,
    TESTP_OR,
    TESTPN_OR,
    TESTP_XOR,
    TESTPN_XOR,
    PINCTRL, // DIR*/OUT*/FLT*/DRV*, see JEVENT rationale.
    SPLITB,
    MERGEB,
    SPLITW,
    MERGEW,
    SEUSSF,
    SEUSSR,
    RGBSQZ,
    RGBEXP,
    XORO32,
    REV,
    RCZR,
    RCZL,
    WRC,
    WRNC,
    WRZ,
    WRNZ,
    MODCZ,
    SETSCP,
    GETSCP,
    LJMP, // A variant branches. See "JMP     #{\}A" in the spreadsheet.
    LCALL,
    LCALLA,
    LCALLB,
    LCALLD,
    LOC,
    AUGS,
    AUGD,
}

impl InstructionKind {
    pub fn decode(inp: u32) -> InstructionKind {
        if inp == 0 {
            return InstructionKind::NOP;
        }

        use InstructionKind as IK;

        match (inp & 0b0000_1111111_000_000000000_000000000) >> 21 {
            0b0000000 => IK::ROR,
            0b0000001 => IK::ROL,
            0b0000010 => IK::SHR,
            0b0000011 => IK::SHL,
            0b0000100 => IK::RCR,
            0b0000101 => IK::RCL,
            0b0000110 => IK::SAR,
            0b0000111 => IK::SAL,
            0b0001000 => IK::ADD,
            0b0001001 => IK::ADDX,
            0b0001010 => IK::ADDS,
            0b0001011 => IK::ADDSX,
            0b0001100 => IK::SUB,
            0b0001101 => IK::SUBX,
            0b0001110 => IK::SUBS,
            0b0001111 => IK::SUBSX,
            0b0010000 => IK::CMP,
            0b0010001 => IK::CMPX,
            0b0010010 => IK::CMPS,
            0b0010011 => IK::CMPSX,
            0b0010100 => IK::CMPR,
            0b0010101 => IK::CMPM,
            0b0010110 => IK::SUBR,
            0b0010111 => IK::CMPSUB,
            0b0011000 => IK::FGE,
            0b0011001 => IK::FLE,
            0b0011010 => IK::FGES,
            0b0011011 => IK::FLES,
            0b0011100 => IK::SUMC,
            0b0011101 => IK::SUMNC,
            0b0011110 => IK::SUMZ,
            0b0011111 => IK::SUMNZ,
            0b0100000 if get_wc_flag(inp) ^ get_wz_flag(inp) => IK::TESTB,
            0b0100000 => IK::BITL,
            0b0100001 if get_wc_flag(inp) ^ get_wz_flag(inp) => IK::TESTBN,
            0b0100001 => IK::BITH,
            0b0100010 if get_wc_flag(inp) ^ get_wz_flag(inp) => IK::TESTB_AND,
            0b0100010 => IK::BITC,
            0b0100011 if get_wc_flag(inp) ^ get_wz_flag(inp) => IK::TESTBN_AND,
            0b0100011 => IK::BITNC,
            0b0100100 if get_wc_flag(inp) ^ get_wz_flag(inp) => IK::TESTB_OR,
            0b0100100 => IK::BITZ,
            0b0100101 if get_wc_flag(inp) ^ get_wz_flag(inp) => IK::TESTBN_OR,
            0b0100101 => IK::BITNZ,
            0b0100110 if get_wc_flag(inp) ^ get_wz_flag(inp) => IK::TESTB_XOR,
            0b0100110 => IK::BITRND,
            0b0100111 if get_wc_flag(inp) ^ get_wz_flag(inp) => IK::TESTBN_XOR,
            0b0100111 => IK::BITNOT,
            0b0101000 => IK::AND,
            0b0101001 => IK::ANDN,
            0b0101010 => IK::OR,
            0b0101011 => IK::XOR,
            0b0101100 => IK::MUXC,
            0b0101101 => IK::MUXNC,
            0b0101110 => IK::MUXZ,
            0b0101111 => IK::MUXNZ,
            0b0110000 => IK::MOV,
            0b0110001 => IK::NOT,
            0b0110010 => IK::ABS,
            0b0110011 => IK::NEG,
            0b0110100 => IK::NEGC,
            0b0110101 => IK::NEGNC,
            0b0110110 => IK::NEGZ,
            0b0110111 => IK::NEGNZ,
            0b0111000 => IK::INCMOD,
            0b0111001 => IK::DECMOD,
            0b0111010 => IK::ZEROX,
            0b0111011 => IK::SIGNX,
            0b0111100 => IK::ENCOD,
            0b0111101 => IK::ONES,
            0b0111110 => IK::TEST,
            0b0111111 => IK::TESTN,
            0b1000000 | 0b1000001 => IK::SETNIB,
            0b1000010 | 0b1000011 => IK::GETNIB,
            0b1000100 | 0b1000101 => IK::ROLNIB,
            0b1000110 => IK::SETBYTE,
            0b1000111 => IK::GETBYTE,
            0b1001000 => IK::ROLBYTE,
            0b1001001 => IK::SETWORD,
            0b1001010 => IK::GETWORD,
            0b1001011 => IK::ROLWORD,
            _ => todo!(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InstructionPrefix {
    RET = 0b0000,
    IF_NC_AND_NZ = 0b0001,
    IF_NC_AND_Z = 0b0010,
    IF_NC = 0b0011,
    IF_C_AND_NZ = 0b0100,
    IF_NZ = 0b0101,
    IF_C_NE_Z = 0b0110,
    IF_NC_OR_NZ = 0b0111,
    IF_C_AND_Z = 0b1000,
    IF_C_EQ_Z = 0b1001,
    IF_Z = 0b1010,
    IF_NC_OR_Z = 0b1011,
    IF_C = 0b1100,
    IF_C_OR_NZ = 0b1101,
    IF_C_OR_Z = 0b1110,
    NONE = 0b1111,
}

impl InstructionPrefix {
    pub fn decode(inp: u32) -> InstructionPrefix {
        // ew
        match (inp & 0xF000_0000) >> 28 {
            0 => InstructionPrefix::RET,
            1 => InstructionPrefix::IF_NC_AND_NZ,
            2 => InstructionPrefix::IF_NC_AND_Z,
            3 => InstructionPrefix::IF_NC,
            4 => InstructionPrefix::IF_C_AND_NZ,
            5 => InstructionPrefix::IF_NZ,
            6 => InstructionPrefix::IF_C_NE_Z,
            7 => InstructionPrefix::IF_NC_OR_NZ,
            8 => InstructionPrefix::IF_C_AND_Z,
            9 => InstructionPrefix::IF_C_EQ_Z,
            10 => InstructionPrefix::IF_Z,
            11 => InstructionPrefix::IF_NC_OR_Z,
            12 => InstructionPrefix::IF_C,
            13 => InstructionPrefix::IF_C_OR_NZ,
            14 => InstructionPrefix::IF_C_OR_Z,
            15 => InstructionPrefix::NONE,
            _ => unreachable!(),
        }
    }
}

pub fn get_s_field(inp: u32) -> u16 {
    (inp & 0b0000_0000000_000_000000000_111111111) as u16
}

pub fn get_d_field(inp: u32) -> u16 {
    ((inp & 0b0000_0000000_000_111111111_000000000) >> 9) as u16
}

pub fn get_imm_flag(inp: u32) -> bool {
    ((inp & 0b0000_0000000_001_000000000_000000000) >> 18) != 0
}

pub fn get_wz_flag(inp: u32) -> bool {
    ((inp & 0b0000_0000000_010_000000000_000000000) >> 19) != 0
}

pub fn get_wc_flag(inp: u32) -> bool {
    ((inp & 0b0000_0000000_100_000000000_000000000) >> 20) != 0
}

#[cfg(test)]
mod tests {
    use super::{get_s_field, InstructionKind, InstructionPrefix};
    #[test]
    fn decode_nop() {
        assert_eq!(
            InstructionKind::decode(0b0000_0000000_000_000000000_000000000),
            InstructionKind::NOP
        );
    }

    #[test]
    fn decode_form_1() {
        assert_eq!(
            InstructionKind::decode(0b1111_0000000_000_000000000_000000000),
            InstructionKind::ROR
        );
    }

    #[test]
    fn decode_prefix() {
        assert_eq!(
            InstructionPrefix::decode(0b1010_0000000_000_000000000_000000000),
            InstructionPrefix::IF_Z,
        );
    }

    #[test]
    fn decode_s() {
        assert_eq!(get_s_field(0b1010_0000000_000_000000000_000000001), 1);
    }
}
