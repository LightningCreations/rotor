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
    ADDCT, // See JEVENT rationale.
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
    DECOD, // somehow got missed, cba to insert it sorted.
    SETDACS,
    SETPIV,
    SETPIX,
}

impl InstructionKind {
    pub fn decode(inp: u32) -> InstructionKind {
        if inp == 0 {
            return InstructionKind::NOP;
        }

        let wc = get_wc_flag(inp);
        let wz = get_wz_flag(inp);
        let wcz = get_wcz_field(inp);
        let s = get_s_field(inp);
        let d = get_d_field(inp);
        let i = get_imm_flag(inp);
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
            0b0100000 if wc ^ wz => IK::TESTB,
            0b0100000 => IK::BITL,
            0b0100001 if wc ^ wz => IK::TESTBN,
            0b0100001 => IK::BITH,
            0b0100010 if wc ^ wz => IK::TESTB_AND,
            0b0100010 => IK::BITC,
            0b0100011 if wc ^ wz => IK::TESTBN_AND,
            0b0100011 => IK::BITNC,
            0b0100100 if wc ^ wz => IK::TESTB_OR,
            0b0100100 => IK::BITZ,
            0b0100101 if wc ^ wz => IK::TESTBN_OR,
            0b0100101 => IK::BITNZ,
            0b0100110 if wc ^ wz => IK::TESTB_XOR,
            0b0100110 => IK::BITRND,
            0b0100111 if wc ^ wz => IK::TESTBN_XOR,
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
            0b1001001 if !wc => IK::SETWORD,
            0b1001001 => IK::GETWORD,
            0b1001010 if !wc => IK::ROLWORD,
            0b1001010 if wcz == 0b10 => IK::ALTSN,
            0b1001010 if wcz == 0b11 => IK::ALTGN,
            0b1001011 if wcz == 0b00 => IK::ALTSB,
            0b1001011 if wcz == 0b01 => IK::ALTGB,
            0b1001011 if wcz == 0b10 => IK::ALTSW,
            0b1001011 if wcz == 0b11 => IK::ALTGW,
            0b1001100 if wcz == 0b00 => IK::ALTR,
            0b1001100 if wcz == 0b01 => IK::ALTD,
            0b1001100 if wcz == 0b10 => IK::ALTS,
            0b1001100 if wcz == 0b11 => IK::ALTB,
            0b1001101 if wcz == 0b00 => IK::ALTI,
            0b1001101 if wcz == 0b01 => IK::SETR,
            0b1001101 if wcz == 0b10 => IK::SETD,
            0b1001101 if wcz == 0b11 => IK::SETS,
            0b1001110 if wcz == 0b00 => IK::DECOD,
            0b1001110 if wcz == 0b01 => IK::BMASK,
            0b1001110 if wcz == 0b10 => IK::CRCBIT,
            0b1001110 if wcz == 0b11 => IK::CRCNIB,
            0b1001111 if wcz == 0b00 => IK::MUXNITS,
            0b1001111 if wcz == 0b01 => IK::MUXNIBS,
            0b1001111 if wcz == 0b10 => IK::MUXQ,
            0b1001111 if wcz == 0b11 => IK::MOVBYTS,
            0b1010000 if !wc => IK::MUL,
            0b1010000 if wc => IK::MULS,
            0b1010001 if !wc => IK::SCA,
            0b1010001 if wc => IK::SCAS,
            0b1010010 if wcz == 0b00 => IK::ADDPIX,
            0b1010010 if wcz == 0b01 => IK::MULPIX,
            0b1010010 if wcz == 0b10 => IK::BLNPIX,
            0b1010010 if wcz == 0b11 => IK::MIXPIX,
            0b1010011 if (0b00..=0b10).contains(&wcz) => IK::ADDCT,
            0b1010011 => IK::WMLONG,
            0b1010100 if wz => IK::RQPIN,
            0b1010100 if !wz => IK::RDPIN,
            0b1010101 => IK::RDLUT,
            0b1010110 => IK::RDBYTE,
            0b1010111 => IK::RDWORD,
            0b1011000 => IK::RDLONG,
            0b1011001 => IK::CALLD,
            0b1011010 if !wc => IK::CALLPA,
            0b1011010 if wc => IK::CALLPB,
            0b1011011 if wcz == 0b00 => IK::DJZ,
            0b1011011 if wcz == 0b01 => IK::DJNZ,
            0b1011011 if wcz == 0b10 => IK::DJF,
            0b1011011 if wcz == 0b11 => IK::DJNF,
            0b1011100 if wcz == 0b00 => IK::IJZ,
            0b1011100 if wcz == 0b01 => IK::IJNZ,
            0b1011100 if wcz == 0b10 => IK::TJZ,
            0b1011100 if wcz == 0b11 => IK::TJNZ,
            0b1011101 if wcz == 0b00 => IK::TJF,
            0b1011101 if wcz == 0b01 => IK::TJNF,
            0b1011101 if wcz == 0b10 => IK::TJS,
            0b1011101 if wcz == 0b11 => IK::TJNS,
            0b1011110 if wcz == 0b00 => IK::TJV,
            0b1011110 if wcz == 0b01 => IK::JEVENT,
            0b1011110 if wc => IK::UNUSED1,
            0b1011111 if !wc => IK::UNUSED2,
            0b1011111 if wc => IK::SETPAT,
            0b1100000 if !wc => IK::WRPIN,
            0b1100000 if wc => IK::WXPIN,
            0b1100001 if !wc => IK::WYPIN,
            0b1100001 if wc => IK::WRLUT,
            0b1100010 if !wc => IK::WRBYTE,
            0b1100010 if wc => IK::WRWORD,
            0b1100011 if !wc => IK::WRLONG,
            0b1100011 if wc => IK::RDFAST,
            0b1100100 if !wc => IK::WRFAST,
            0b1100100 if wc => IK::FBLOCK,
            0b1100101 if !wc => IK::XINIT,
            0b1100101 if wc => IK::XZERO,
            0b1100110 if !wc => IK::XCONT,
            0b1100110 if wc => IK::REP,
            0b1100111 => IK::COGINIT,
            0b1101000 if !wc => IK::QMUL,
            0b1101000 if wc => IK::QDIV,
            0b1101001 if !wc => IK::QFRAC,
            0b1101001 if wc => IK::QSQRT,
            0b1101010 if !wc => IK::QROTATE,
            0b1101010 if wc => IK::QVECTOR,
            // WELCOME TO DECODE HELL. ENJOY YOUR STAY.
            0b1101011 => {
                match s & 0x75 {
                    0b0000000 => IK::HUBSET,
                    0b0000001 => IK::COGID,
                    0b0000010 => IK::NOP, // Unused
                    0b0000011 => IK::COGSTOP,
                    0b0000100 => IK::LOCKNEW,
                    0b0000101 => IK::LOCKRET,
                    0b0000110 => IK::LOCKTRY,
                    0b0000111 => IK::LOCKREL,
                    0b0001000..=0b0001101 => IK::NOP, // Unused
                    0b0001110 => IK::QLOG,
                    0b0001111 => IK::QEXP,
                    0b0010000 => IK::RFBYTE,
                    0b0010001 => IK::RFWORD,
                    0b0010010 => IK::RFLONG,
                    0b0010011 => IK::RFVAR,
                    0b0010100 => IK::RFVARS,
                    0b0010101 => IK::WFBYTE,
                    0b0010110 => IK::WFWORD,
                    0b0010111 => IK::WFLONG,
                    0b0011000 => IK::GETQX,
                    0b0011001 => IK::GETQY,
                    0b0011010 => IK::GETCT,
                    0b0011011 => IK::GETRND,
                    0b0011100 => IK::SETDACS,
                    0b0011101 => IK::SETXFRQ,
                    0b0011110 => IK::GETXACC,
                    0b0011111 => IK::WAITX,
                    0b0100000..=0b0100011 => IK::SETSE,
                    0b0100100 => {
                        // three matches deep and no end in sight
                        match d & 0b000111111 {
                            0b000000..=0b001111 => IK::POLLEVENT,
                            0b010000..=0b011110 => IK::WAITEVENT,
                            0b011111 => IK::NOP,
                            0b100000 => IK::ALLOWI,
                            0b100001 => IK::STALLI,
                            0b100010..=0b100100 => IK::TRGINT,
                            0b100101..=0b100111 => IK::NIXINT,
                            _ => IK::NOP,
                        }
                    }
                    0b0100101..=0b0100111 => IK::SETINT,
                    0b0101000 => IK::SETQ,
                    0b0101001 => IK::SETQ2,
                    0b0101010 => IK::PUSH,
                    0b0101011 => IK::POP,
                    0b0101100 => IK::JMP,
                    0b0101101 if !i => IK::CALL,
                    0b0101101 if i => IK::RET,
                    0b0101110 if !i => IK::CALLA,
                    0b0101110 if i => IK::RETA,
                    0b0101111 if !i => IK::CALLB,
                    0b0101111 if i => IK::RETB,
                    0b0110000 => IK::JMPREL,
                    0b0110001 => IK::SKIP,
                    0b0110010 => IK::SKIPF,
                    0b0110011 => IK::EXECF,
                    0b0110100 => IK::GETPTR,
                    0b0110101 => IK::GETBRK,
                    0b0110110 => IK::BRK,
                    0b0110111 => IK::SETLUTS,
                    0b0111000..=0b0111100 => IK::SETCOLORSPACEVAR,
                    0b0111101 => IK::SETPIV,
                    0b0111110 => IK::SETPIX,
                    0b0111111 => IK::COGATN,
                    0b1000000 if wc ^ wz => IK::TESTP,
                    0b1000001 if wc ^ wz => IK::TESTPN,
                    0b1000010 if wc ^ wz => IK::TESTP_AND,
                    0b1000011 if wc ^ wz => IK::TESTPN_AND,
                    0b1000100 if wc ^ wz => IK::TESTP_OR,
                    0b1000101 if wc ^ wz => IK::TESTPN_OR,
                    0b1000110 if wc ^ wz => IK::TESTP_XOR,
                    0b1000111 if wc ^ wz => IK::TESTPN_XOR,
                    0b1000000..=0b1011111 if !(wc ^ wz) => IK::PINCTRL,
                    0b1100000 => IK::SPLITB,
                    0b1100001 => IK::MERGEB,
                    0b1100010 => IK::SPLITW,
                    0b1100011 => IK::MERGEW,
                    0b1100100 => IK::SEUSSF,
                    0b1100101 => IK::SEUSSR,
                    0b1100110 => IK::RGBSQZ,
                    0b1100111 => IK::RGBEXP,
                    0b1101000 => IK::XORO32,
                    0b1101001 => IK::REV,
                    0b1101010 => IK::RCZR,
                    0b1101011 => IK::RCZL,
                    0b1101100 => IK::WRC,
                    0b1101101 => IK::WRNC,
                    0b1101110 => IK::WRZ,
                    0b1101111 if !i => IK::WRNZ,
                    0b1101111 if i => IK::MODCZ,
                    0b1110000 => IK::SETSCP,
                    0b1110001 => IK::GETSCP,
                    _ => IK::NOP,
                }
            }
            0b1101100 => IK::LJMP,
            0b1101101 => IK::CALL,
            0b1101110 => IK::CALLA,
            0b1101111 => IK::CALLB,
            0b1110000..=0b1110011 => IK::CALLD,
            0b1110100..=0b1110111 => IK::LOC,
            0b1111000..=0b1111011 => IK::AUGS,
            0b1111100..=0b1111111 => IK::AUGD,
            _ => IK::NOP, // Whatever the fuck this instruction is, we don't care it's not valid.
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

/// Retrieves the S field from an instruction.
/// `EEEE_OOOOOOO_CZI_DDDDDDDDD_SSSSSSSSS`
pub const fn get_s_field(inp: u32) -> u16 {
    (inp & 0b0000_0000000_000_000000000_111111111) as u16
}

/// Retrieves the D field from an instruction.
/// `EEEE_OOOOOOO_CZI_DDDDDDDDD_SSSSSSSSS`
pub const fn get_d_field(inp: u32) -> u16 {
    ((inp & 0b0000_0000000_000_111111111_000000000) >> 9) as u16
}

/// Retrieves the I flag from an instruction.
/// `EEEE_OOOOOOO_CZI_DDDDDDDDD_SSSSSSSSS`
pub const fn get_imm_flag(inp: u32) -> bool {
    ((inp & 0b0000_0000000_001_000000000_000000000) >> 18) != 0
}

/// Retrieves the Z flag from an instruction.
/// `EEEE_OOOOOOO_CZI_DDDDDDDDD_SSSSSSSSS`
pub const fn get_wz_flag(inp: u32) -> bool {
    ((inp & 0b0000_0000000_010_000000000_000000000) >> 19) != 0
}

/// Retrieves the C flag from an instruction.
/// `EEEE_OOOOOOO_CZI_DDDDDDDDD_SSSSSSSSS`
pub const fn get_wc_flag(inp: u32) -> bool {
    ((inp & 0b0000_0000000_100_000000000_000000000) >> 20) != 0
}

/// Retrieves the CZ field from an instruction.
/// `EEEE_OOOOOOO_CZI_DDDDDDDDD_SSSSSSSSS`
pub const fn get_wcz_field(inp: u32) -> u8 {
    ((inp & 0b0000_0000000_110_000000000_000000000) >> 19) as u8
}

/// Retrieves the CZI field from an instruction.
/// `EEEE_OOOOOOO_CZI_DDDDDDDDD_SSSSSSSSS`
pub const fn get_wczi_field(inp: u32) -> u8 {
    ((inp & 0b0000_0000000_111_000000000_000000000) >> 18) as u8
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
    fn decode_altsn() {
        assert_eq!(
            InstructionKind::decode(0b1111_1001010_100_000000000_000000000),
            InstructionKind::ALTSN,
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
