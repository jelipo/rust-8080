use std::borrow::Borrow;

use crate::cpu::register::Register;
use crate::memory::address::Addressing;
use crate::util::U16Util;

/// Abstraction of Intel 8080
struct Cpu {
    register: Register,
    addring: dyn Addressing,
}


impl Cpu {
    fn get_next_byte(&mut self) -> u8 {
        let byte = self.addring.get_mem(self.register.pc);
        self.register.pc += 1;
        byte
    }

    fn get_next_word(&mut self) -> u16 {
        let addr = self.register.pc;
        let word = U16Util::from_le_bytes(self.addring.get_mem(addr), self.addring.get_mem(addr + 1));
        self.register.pc += 2;
        word
    }

    /// OPCODE: INR
    fn inr_add(&mut self, r: u8) -> u8 {
        let new_r = r.wrapping_add(1);
        self.register.flag_z = new_r == 0;
        self.register.flag_s = (new_r & 0b10000000) != 0;
        self.register.flag_p = r.count_ones() & 0x01 == 0x00;
        // TODO  self.register.flag_ac =;
        new_r
    }

    /// OPCODE: DCR
    fn dcr_sub(&mut self, r: u8) -> u8 {
        let new_r = r.wrapping_sub(1);
        self.register.flag_z = new_r == 0;
        self.register.flag_s = (new_r & 0b10000000) != 0;
        self.register.flag_p = r.count_ones() & 0x01 == 0x00;
        // TODO  self.register.flag_ac =;
        new_r
    }


    /// 下一步指令
    pub fn next(&mut self) {
        let op_code = self.get_next_byte();
        match op_code {
            // NOP          1
            0x00 => {}
            // LXI B,D16    3                      B <- byte 3, C <- byte 2
            0x01 => {
                let word = self.get_next_word();
                self.register.set_bc(word);
            }
            // STAX B       1                      (BC) <- A
            0x02 => self.addring.set_mem(self.register.get_bc(), self.register.a),
            // INX B        1                      BC <- BC+1
            0x03 => self.register.set_bc(self.register.get_bc().wrapping_add(1)),
            // INR B        1    Z, S, P, AC       B <- B+1
            0x04 => self.register.b = self.inr_add(self.register.b),
            // DCR B        1    Z, S, P, AC       B <- B-1
            0x05 => self.register.b = self.dcr_sub(self.register.b),
            // MVI B, D8    2                      B <- byte 2
            0x06 => self.register.b = self.get_next_byte(),
            // RLC          1    CY                A = A << 1; bit 0 = prev bit 7; CY = prev bit 7
            0x07 => {

            }
            // -
            0x08 => {}
            // DAD B        1    CY                HL = HL + BC
            0x09 => {}
            // LDAX B       1                      A <- (BC)
            0x0a => {}
            // DCX B        1                      BC = BC-1
            0x0b => {}
            // INR C        1    Z, S, P, AC       C <- C+1
            0x0c => self.register.c = self.inr_add(self.register.c),
            // DCR C        1    Z, S, P, AC       C <-C-1
            0x0d => self.register.c = self.dcr_sub(self.register.c),
            // MVI C,D8     2                      C <- byte 2
            0x0e => self.register.c = self.get_next_byte(),
            // RRC          1    CY                A = A >> 1; bit 7 = prev bit 0; CY = prev bit 0
            0x0f => {}
            // -
            0x10 => {}
            // LXI D,D16    3                      D <- byte 3, E <- byte 2
            0x11 => {}
            // STAX D       1                      (DE) <- A
            0x12 => {}
            // INX D        1                      DE <- DE + 1
            0x13 => {}
            // INR D        1    Z, S, P, AC       D <- D+1
            0x14 => self.register.d = self.inr_add(self.register.d),
            // DCR D        1    Z, S, P, AC       D <- D-1
            0x15 => self.register.d = self.dcr_sub(self.register.d),
            // MVI D, D8    2                      D <- byte 2
            0x16 => self.register.d = self.get_next_byte(),
            // RAL          1    CY                A = A << 1; bit 0 = prev CY; CY = prev bit 7
            0x17 => {}
            // -
            0x18 => {}
            // DAD D        1    CY                HL = HL + DE
            0x19 => {}
            // LDAX D       1                      A <- (DE)
            0x1a => {}
            // DCX D        1                      DE = DE-1
            0x1b => {}
            // INR E        1    Z, S, P, AC       E <-E+1
            0x1c => self.register.e = self.inr_add(self.register.e),
            // DCR E        1    Z, S, P, AC       E <- E-1
            0x1d => self.register.e = self.dcr_sub(self.register.e),
            // MVI E,D8     2                      E <- byte 2
            0x1e => self.register.e = self.get_next_byte(),
            // RAR          1    CY                A = A >> 1; bit 7 = prev bit 7; CY = prev bit 0
            0x1f => {}
            // RIM          1                      special
            0x20 => {}
            // LXI H,D16    3                      H <- byte 3, L <- byte 2
            0x21 => {}
            // SHLD adr     3                      (adr) <-L; (adr+1)<-H
            0x22 => {}
            // INX H        1                      HL <- HL + 1
            0x23 => {}
            // INR H        1    Z, S, P, AC       H <- H+1
            0x24 => self.register.h = self.inr_add(self.register.h),
            // DCR H        1    Z, S, P, AC       H <- H-1
            0x25 => self.register.h = self.dcr_sub(self.register.h),
            // MVI H,D8     2                      H <- byte 2
            0x26 => self.register.h = self.get_next_byte(),
            // DAA          1                      special
            0x27 => {}
            // -
            0x28 => {}
            // DAD H        1    CY                HL = HL + HI
            0x29 => {}
            // LHLD adr     3                      L <- (adr); H<-(adr+1)
            0x2a => {}
            // DCX H        1                      HL = HL-1
            0x2b => {}
            // INR L        1    Z, S, P, AC       L <- L+1
            0x2c => self.register.l = self.inr_add(self.register.l),
            // DCR L        1    Z, S, P, AC       L <- L-1
            0x2d => self.register.l = self.dcr_sub(self.register.l),
            // MVI L, D8    2                      L <- byte 2
            0x2e => self.register.l = self.get_next_byte(),
            // CMA          1                      A <- !A
            0x2f => {}
            // SIM          1                      special
            0x30 => {}
            // LXI SP, D16  3                      SP.hi <- byte 3, SP.lo <- byte 2
            0x31 => {}
            // STA adr      3                      (adr) <- A
            0x32 => {}
            // INX SP       1                      SP = SP + 1
            0x33 => {}
            // INR M        1    Z, S, P, AC       (HL) <- (HL)+1
            0x34 => {
                let hl = self.register.get_hl();
                // TODO self.register.b = self.inr_add(self.register.b)
            }
            // DCR M        1    Z, S, P, AC       (HL) <- (HL)-1
            0x35 => {}
            // MVI M,D8     2                      (HL) <- byte 2
            0x36 => {}
            // STC          1    CY                CY = 1
            0x37 => {}
            // -
            0x38 => {}
            // DAD SP       1    CY                 HL = HL + SP
            0x39 => {}
            // LDA adr      3                       A <- (adr)
            0x3a => {}
            // DCX SP       1                       SP = SP-1
            0x3b => {}
            // INR A        1    Z, S, P, AC        A <- A+1
            0x3c => self.register.a = self.inr_add(self.register.a),
            // DCR A        1    Z, S, P, AC        A <- A-1
            0x3d => self.register.a = self.dcr_sub(self.register.a),
            // MVI A,D8     2                       A <- byte 2
            0x3e => self.register.a = self.get_next_byte(),
            // CMC          1    CY                 CY=!CY
            0x3f => {}
            // MOV B,B      1                       B <- B
            0x40 => {
                // TODO ???
            }
            // MOV B,C      1                       B <- C
            0x41 => {}
            // MOV B,D      1                       B <- D
            0x42 => {}
            // MOV B,E      1                       B <- E
            0x43 => {}
            // MOV B,H      1                       B <- H
            0x44 => {}
            // MOV B,L      1                       B <- L
            0x45 => {}
            // MOV B,M      1                       B <- (HL)
            0x46 => {}
            // MOV B,A      1                       B <- A
            0x47 => {}
            // MOV C,B      1                       C <- B
            0x48 => {}
            // MOV C,C      1                       C <- C
            0x49 => {}
            // MOV C,D      1                       C <- D
            0x4a => {}
            // MOV C,E      1                       C <- E
            0x4b => {}
            // MOV C,H      1                       C <- H
            0x4c => {}
            // MOV C,L      1                       C <- L
            0x4d => {}
            // MOV C,M      1                       C <- (HL)
            0x4e => {}
            // MOV C,A      1                       C <- A
            0x4f => {}
            // MOV D,B      1                       D <- B
            0x50 => {}
            // MOV D,C      1                       D <- C
            0x51 => {}
            // MOV D,D      1                       D <- D
            0x52 => {}
            // MOV D,E      1                       D <- E
            0x53 => {}
            // MOV D,H      1                       D <- H
            0x54 => {}
            // MOV D,L      1                       D <- L
            0x55 => {}
            // MOV D,M      1                       D <- (HL)
            0x56 => {}
            // MOV D,A      1                       D <- A
            0x57 => {}
            // MOV E,B      1                       E <- B
            0x58 => {}
            // MOV E,C      1                       E <- C
            0x59 => {}
            // MOV E,D      1                       E <- D
            0x5a => {}
            // MOV E,E      1                       E <- E
            0x5b => {}
            // MOV E,H      1                       E <- H
            0x5c => {}
            // MOV E,L      1                       E <- L
            0x5d => {}
            // MOV E,M      1                       E <- (HL)
            0x5e => {}
            // MOV E,A      1                       E <- A
            0x5f => {}
            // MOV H,B      1                       H <- B
            0x60 => {}
            // MOV H,C      1                       H <- C
            0x61 => {}
            // MOV H,D      1                       H <- D
            0x62 => {}
            // MOV H,E      1                       H <- E
            0x63 => {}
            // MOV H,H      1                       H <- H
            0x64 => {}
            // MOV H,L      1                       H <- L
            0x65 => {}
            // MOV H,M      1                       H <- (HL)
            0x66 => {}
            // MOV H,A      1                       H <- A
            0x67 => {}
            // MOV L,B      1                       L <- B
            0x68 => {}
            // MOV L,C      1                       L <- C
            0x69 => {}
            // MOV L,D      1                       L <- D
            0x6a => {}
            // MOV L,E      1                       L <- E
            0x6b => {}
            // MOV L,H      1                       L <- H
            0x6c => {}
            // MOV L,L      1                       L <- L
            0x6d => {}
            // MOV L,M      1                       L <- (HL)
            0x6e => {}
            // MOV L,A      1                       L <- A
            0x6f => {}
            // MOV M,B      1                       (HL) <- B
            0x70 => {}
            // MOV M,C      1                       (HL) <- C
            0x71 => {}
            // MOV M,D      1                       (HL) <- D
            0x72 => {}
            // MOV M,E      1                       (HL) <- E
            0x73 => {}
            // MOV M,H      1                       (HL) <- H
            0x74 => {}
            // MOV M,L      1                       (HL) <- L
            0x75 => {}
            // HLT          1                       special
            0x76 => {}
            // MOV M,A      1                       (HL) <- C
            0x77 => {}
            // MOV A,B      1                       A <- B
            0x78 => {}
            // MOV A,C      1                       A <- C
            0x79 => {}
            // MOV A,D      1                       A <- D
            0x7a => {}
            // MOV A,E      1                       A <- E
            0x7b => {}
            // MOV A,H      1                       A <- H
            0x7c => {}
            // MOV A,L      1                       A <- L
            0x7d => {}
            // MOV A,M      1                       A <- (HL)
            0x7e => {}
            // MOV A,A      1                       A <- A
            0x7f => {}
            // ADD B        1    Z, S, P, CY, AC    A <- A + B
            0x80 => {}
            // ADD C        1    Z, S, P, CY, AC    A <- A + C
            0x81 => {}
            // ADD D        1    Z, S, P, CY, AC    A <- A + D
            0x82 => {}
            // ADD E        1    Z, S, P, CY, AC    A <- A + E
            0x83 => {}
            // ADD H        1    Z, S, P, CY, AC    A <- A + H
            0x84 => {}
            // ADD L        1    Z, S, P, CY, AC    A <- A + L
            0x85 => {}
            // ADD M        1    Z, S, P, CY, AC    A <- A + (HL)
            0x86 => {}
            // ADD A        1    Z, S, P, CY, AC    A <- A + A
            0x87 => {}
            // ADC B        1    Z, S, P, CY, AC    A <- A + B + CY
            0x88 => {}
            // ADC C        1    Z, S, P, CY, AC    A <- A + C + CY
            0x89 => {}
            // ADC D        1    Z, S, P, CY, AC    A <- A + D + CY
            0x8a => {}
            // ADC E        1    Z, S, P, CY, AC    A <- A + E + CY
            0x8b => {}
            // ADC H        1    Z, S, P, CY, AC    A <- A + H + CY
            0x8c => {}
            // ADC L        1    Z, S, P, CY, AC    A <- A + L + CY
            0x8d => {}
            // ADC M        1    Z, S, P, CY, AC    A <- A + (HL) + CY
            0x8e => {}
            // ADC A        1    Z, S, P, CY, AC    A <- A + A + CY
            0x8f => {}
            // SUB B        1    Z, S, P, CY, AC    A <- A - B
            0x90 => {}
            // SUB C        1    Z, S, P, CY, AC    A <- A - C
            0x91 => {}
            // SUB D        1    Z, S, P, CY, AC    A <- A + D
            0x92 => {}
            // SUB E        1    Z, S, P, CY, AC    A <- A - E
            0x93 => {}
            // SUB H        1    Z, S, P, CY, AC    A <- A + H
            0x94 => {}
            // SUB L        1    Z, S, P, CY, AC    A <- A - L
            0x95 => {}
            // SUB M        1    Z, S, P, CY, AC    A <- A + (HL)
            0x96 => {}
            // SUB A        1    Z, S, P, CY, AC    A <- A - A
            0x97 => {}
            // SBB B        1    Z, S, P, CY, AC    A <- A - B - CY
            0x98 => {}
            // SBB C        1    Z, S, P, CY, AC    A <- A - C - CY
            0x99 => {}
            // SBB D        1    Z, S, P, CY, AC    A <- A - D - CY
            0x9a => {}
            // SBB E        1    Z, S, P, CY, AC    A <- A - E - CY
            0x9b => {}
            // SBB H        1    Z, S, P, CY, AC    A <- A - H - CY
            0x9c => {}
            // SBB L        1    Z, S, P, CY, AC    A <- A - L - CY
            0x9d => {}
            // SBB M        1    Z, S, P, CY, AC    A <- A - (HL) - CY
            0x9e => {}
            // SBB A        1    Z, S, P, CY, AC    A <- A - A - CY
            0x9f => {}
            // ANA B        1    Z, S, P, CY, AC    A <- A & B
            0xa0 => {}
            // ANA C        1    Z, S, P, CY, AC    A <- A & C
            0xa1 => {}
            // ANA D        1    Z, S, P, CY, AC    A <- A & D
            0xa2 => {}
            // ANA E        1    Z, S, P, CY, AC    A <- A & E
            0xa3 => {}
            // ANA H        1    Z, S, P, CY, AC    A <- A & H
            0xa4 => {}
            // ANA L        1    Z, S, P, CY, AC    A <- A & L
            0xa5 => {}
            // ANA M        1    Z, S, P, CY, AC    A <- A & (HL)
            0xa6 => {}
            // ANA A        1    Z, S, P, CY, AC    A <- A & A
            0xa7 => {}
            // XRA B        1    Z, S, P, CY, AC    A <- A ^ B
            0xa8 => {}
            // XRA C        1    Z, S, P, CY, AC    A <- A ^ C
            0xa9 => {}
            // XRA D        1    Z, S, P, CY, AC    A <- A ^ D
            0xaa => {}
            // XRA E        1    Z, S, P, CY, AC    A <- A ^ E
            0xab => {}
            // XRA H        1    Z, S, P, CY, AC    A <- A ^ H
            0xac => {}
            // XRA L        1    Z, S, P, CY, AC    A <- A ^ L
            0xad => {}
            // XRA M        1    Z, S, P, CY, AC    A <- A ^ (HL)
            0xae => {}
            // XRA A        1    Z, S, P, CY, AC    A <- A ^ A
            0xaf => {}
            // ORA B        1    Z, S, P, CY, AC    A <- A | B
            0xb0 => {}
            // ORA C        1    Z, S, P, CY, AC    A <- A | C
            0xb1 => {}
            // ORA D        1    Z, S, P, CY, AC    A <- A | D
            0xb2 => {}
            // ORA E        1    Z, S, P, CY, AC    A <- A | E
            0xb3 => {}
            // ORA H        1    Z, S, P, CY, AC    A <- A | H
            0xb4 => {}
            // ORA L        1    Z, S, P, CY, AC    A <- A | L
            0xb5 => {}
            // ORA M        1    Z, S, P, CY, AC    A <- A | (HL)
            0xb6 => {}
            // ORA A        1    Z, S, P, CY, AC    A <- A | A
            0xb7 => {}
            // CMP B        1    Z, S, P, CY, AC    A - B
            0xb8 => {}
            // CMP C        1    Z, S, P, CY, AC    A - C
            0xb9 => {}
            // CMP D        1    Z, S, P, CY, AC    A - D
            0xba => {}
            // CMP E        1    Z, S, P, CY, AC    A - E
            0xbb => {}
            // CMP H        1    Z, S, P, CY, AC    A - H
            0xbc => {}
            // CMP L        1    Z, S, P, CY, AC    A - L
            0xbd => {}
            // CMP M        1    Z, S, P, CY, AC    A - (HL)
            0xbe => {}
            // CMP A        1    Z, S, P, CY, AC    A - A
            0xbf => {}
            // RNZ          1                       if NZ, RET
            0xc0 => {}
            // POP B        1                       C <- (sp); B <- (sp+1); sp <- sp+2
            0xc1 => {}
            // JNZ adr      3                       if NZ, PC <- adr
            0xc2 => {}
            // JMP adr      3                       PC <= adr
            0xc3 => {}
            // CNZ adr      3                       if NZ, CALL adr
            0xc4 => {}
            // PUSH B       1                       (sp-2)<-C; (sp-1)<-B; sp <- sp - 2
            0xc5 => {}
            // ADI D8       2    Z, S, P, CY, AC    A <- A + byte
            0xc6 => {}
            // RST 0        1                       CALL $0
            0xc7 => {}
            // RZ           1                       if Z, RET
            0xc8 => {}
            // RET          1                       PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2
            0xc9 => {}
            // JZ adr       3                       if Z, PC <- adr
            0xca => {}
            // -
            0xcb => {}
            // CZ adr       3                       if Z, CALL adr
            0xcc => {}
            // CALL adr     3                       (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP+2;PC=adr
            0xcd => {}
            // ACI D8       2    Z, S, P, CY, AC    A <- A + data + CY
            0xce => {}
            // RST 1        1                       CALL $8
            0xcf => {}
            // RNC          1                       if NCY, RET
            0xd0 => {}
            // POP D        1                       E <- (sp); D <- (sp+1); sp <- sp+2
            0xd1 => {}
            // JNC adr      3                       if NCY, PC<-adr
            0xd2 => {}
            // OUT D8       2                       special
            0xd3 => {}
            // CNC adr      3                       if NCY, CALL adr
            0xd4 => {}
            // PUSH D       1                       (sp-2)<-E; (sp-1)<-D; sp <- sp - 2
            0xd5 => {}
            // SUI D8       2    Z, S, P, CY, AC    A <- A - data
            0xd6 => {}
            // RST 2        1                       CALL $10
            0xd7 => {}
            // RC           1                       if CY, RET
            0xd8 => {}
            // -
            0xd9 => {}
            // JC adr       3                       if CY, PC<-adr
            0xda => {}
            // IN D8        2                       special
            0xdb => {}
            // CC adr       3                       if CY, CALL adr
            0xdc => {}
            // -
            0xdd => {}
            // SBI D8       2    Z, S, P, CY, AC    A <- A - data - CY
            0xde => {}
            // RST 3        1                       CALL $18
            0xdf => {}
            // RPO          1                       if PO, RET
            0xe0 => {}
            // POP H        1                       L <- (sp); H <- (sp+1); sp <- sp+2
            0xe1 => {}
            // JPO adr      3                       if PO, PC <- adr
            0xe2 => {}
            // XTHL         1                       L <-> (SP); H <-> (SP+1)
            0xe3 => {}
            // CPO adr      3                       if PO, CALL adr
            0xe4 => {}
            // PUSH H       1                       (sp-2)<-L; (sp-1)<-H; sp <- sp - 2
            0xe5 => {}
            // ANI D8       2    Z, S, P, CY, AC    A <- A & data
            0xe6 => {}
            // RST 4        1                       CALL $20
            0xe7 => {}
            // RPE          1                       if PE, RET
            0xe8 => {}
            // PCHL         1                       PC.hi <- H; PC.lo <- L
            0xe9 => {}
            // JPE adr      3                       if PE, PC <- adr
            0xea => {}
            // XCHG         1                       H <-> D; L <-> E
            0xeb => {}
            // CPE adr      3                       if PE, CALL adr
            0xec => {}
            // -
            0xed => {}
            // XRI D8       2    Z, S, P, CY, AC    A <- A ^ data
            0xee => {}
            // RST 5        1                       CALL $28
            0xef => {}
            // RP           1                       if P, RET
            0xf0 => {}
            // POP PSW      1                       flags <- (sp); A <- (sp+1); sp <- sp+2
            0xf1 => {}
            // JP adr       3                       if P=1 PC <- adr
            0xf2 => {}
            // DI           1                       special
            0xf3 => {}
            // CP adr       3                       if P, PC <- adr
            0xf4 => {}
            // PUSH PSW     1                       (sp-2)<-flags; (sp-1)<-A; sp <- sp - 2
            0xf5 => {}
            // ORI D8       2    Z, S, P, CY, AC    A <- A | data
            0xf6 => {}
            // RST 6        1                       CALL $30
            0xf7 => {}
            // RM           1                       if M, RET
            0xf8 => {}
            // SPHL         1                       SP=HL
            0xf9 => {}
            // JM adr       3                       if M, PC <- adr
            0xfa => {}
            // EI           1                       special
            0xfb => {}
            // CM adr       3                       if M, CALL adr
            0xfc => {}
            // -
            0xfd => {}
            // CPI D8       2    Z, S, P, CY, AC    A - data
            0xfe => {}
            // RST 7        1                       CALL $38
            0xff => {}
            //
            _ => {}
            //
        };
        ()
    }
}
