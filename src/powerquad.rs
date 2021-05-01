#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Base address register for output region"]
    pub outbase: crate::Reg<outbase::OUTBASE_SPEC>,
    #[doc = "0x04 - Output format"]
    pub outformat: crate::Reg<outformat::OUTFORMAT_SPEC>,
    #[doc = "0x08 - Base address register for temp region"]
    pub tmpbase: crate::Reg<tmpbase::TMPBASE_SPEC>,
    #[doc = "0x0c - Temp format"]
    pub tmpformat: crate::Reg<tmpformat::TMPFORMAT_SPEC>,
    #[doc = "0x10 - Base address register for input A region"]
    pub inabase: crate::Reg<inabase::INABASE_SPEC>,
    #[doc = "0x14 - Input A format"]
    pub inaformat: crate::Reg<inaformat::INAFORMAT_SPEC>,
    #[doc = "0x18 - Base address register for input B region"]
    pub inbbase: crate::Reg<inbbase::INBBASE_SPEC>,
    #[doc = "0x1c - Input B format"]
    pub inbformat: crate::Reg<inbformat::INBFORMAT_SPEC>,
    _reserved8: [u8; 224usize],
    #[doc = "0x100 - PowerQuad Control register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    #[doc = "0x104 - Length register"]
    pub length: crate::Reg<length::LENGTH_SPEC>,
    #[doc = "0x108 - Pre-scale register"]
    pub cppre: crate::Reg<cppre::CPPRE_SPEC>,
    #[doc = "0x10c - Misc register"]
    pub misc: crate::Reg<misc::MISC_SPEC>,
    #[doc = "0x110 - Cursory register"]
    pub cursory: crate::Reg<cursory::CURSORY_SPEC>,
    _reserved13: [u8; 108usize],
    #[doc = "0x180 - Cordic input X register"]
    pub cordic_x: crate::Reg<cordic_x::CORDIC_X_SPEC>,
    #[doc = "0x184 - Cordic input Y register"]
    pub cordic_y: crate::Reg<cordic_y::CORDIC_Y_SPEC>,
    #[doc = "0x188 - Cordic input Z register"]
    pub cordic_z: crate::Reg<cordic_z::CORDIC_Z_SPEC>,
    #[doc = "0x18c - Read/Write register where error statuses are captured (sticky)"]
    pub errstat: crate::Reg<errstat::ERRSTAT_SPEC>,
    #[doc = "0x190 - INTERRUPT enable register"]
    pub intren: crate::Reg<intren::INTREN_SPEC>,
    #[doc = "0x194 - Event Enable register"]
    pub eventen: crate::Reg<eventen::EVENTEN_SPEC>,
    #[doc = "0x198 - INTERRUPT STATUS register"]
    pub intrstat: crate::Reg<intrstat::INTRSTAT_SPEC>,
    _reserved20: [u8; 100usize],
    #[doc = "0x200 - General purpose register bank N."]
    pub gpreg: [crate::Reg<gpreg::GPREG_SPEC>; 16],
    #[doc = "0x240 - Compute register bank"]
    pub compreg: [crate::Reg<compreg::COMPREG_SPEC>; 8],
}
#[doc = "OUTBASE register accessor: an alias for `Reg<OUTBASE_SPEC>`"]
pub type OUTBASE = crate::Reg<outbase::OUTBASE_SPEC>;
#[doc = "Base address register for output region"]
pub mod outbase;
#[doc = "OUTFORMAT register accessor: an alias for `Reg<OUTFORMAT_SPEC>`"]
pub type OUTFORMAT = crate::Reg<outformat::OUTFORMAT_SPEC>;
#[doc = "Output format"]
pub mod outformat;
#[doc = "TMPBASE register accessor: an alias for `Reg<TMPBASE_SPEC>`"]
pub type TMPBASE = crate::Reg<tmpbase::TMPBASE_SPEC>;
#[doc = "Base address register for temp region"]
pub mod tmpbase;
#[doc = "TMPFORMAT register accessor: an alias for `Reg<TMPFORMAT_SPEC>`"]
pub type TMPFORMAT = crate::Reg<tmpformat::TMPFORMAT_SPEC>;
#[doc = "Temp format"]
pub mod tmpformat;
#[doc = "INABASE register accessor: an alias for `Reg<INABASE_SPEC>`"]
pub type INABASE = crate::Reg<inabase::INABASE_SPEC>;
#[doc = "Base address register for input A region"]
pub mod inabase;
#[doc = "INAFORMAT register accessor: an alias for `Reg<INAFORMAT_SPEC>`"]
pub type INAFORMAT = crate::Reg<inaformat::INAFORMAT_SPEC>;
#[doc = "Input A format"]
pub mod inaformat;
#[doc = "INBBASE register accessor: an alias for `Reg<INBBASE_SPEC>`"]
pub type INBBASE = crate::Reg<inbbase::INBBASE_SPEC>;
#[doc = "Base address register for input B region"]
pub mod inbbase;
#[doc = "INBFORMAT register accessor: an alias for `Reg<INBFORMAT_SPEC>`"]
pub type INBFORMAT = crate::Reg<inbformat::INBFORMAT_SPEC>;
#[doc = "Input B format"]
pub mod inbformat;
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "PowerQuad Control register"]
pub mod control;
#[doc = "LENGTH register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "Length register"]
pub mod length;
#[doc = "CPPRE register accessor: an alias for `Reg<CPPRE_SPEC>`"]
pub type CPPRE = crate::Reg<cppre::CPPRE_SPEC>;
#[doc = "Pre-scale register"]
pub mod cppre;
#[doc = "MISC register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "Misc register"]
pub mod misc;
#[doc = "CURSORY register accessor: an alias for `Reg<CURSORY_SPEC>`"]
pub type CURSORY = crate::Reg<cursory::CURSORY_SPEC>;
#[doc = "Cursory register"]
pub mod cursory;
#[doc = "CORDIC_X register accessor: an alias for `Reg<CORDIC_X_SPEC>`"]
pub type CORDIC_X = crate::Reg<cordic_x::CORDIC_X_SPEC>;
#[doc = "Cordic input X register"]
pub mod cordic_x;
#[doc = "CORDIC_Y register accessor: an alias for `Reg<CORDIC_Y_SPEC>`"]
pub type CORDIC_Y = crate::Reg<cordic_y::CORDIC_Y_SPEC>;
#[doc = "Cordic input Y register"]
pub mod cordic_y;
#[doc = "CORDIC_Z register accessor: an alias for `Reg<CORDIC_Z_SPEC>`"]
pub type CORDIC_Z = crate::Reg<cordic_z::CORDIC_Z_SPEC>;
#[doc = "Cordic input Z register"]
pub mod cordic_z;
#[doc = "ERRSTAT register accessor: an alias for `Reg<ERRSTAT_SPEC>`"]
pub type ERRSTAT = crate::Reg<errstat::ERRSTAT_SPEC>;
#[doc = "Read/Write register where error statuses are captured (sticky)"]
pub mod errstat;
#[doc = "INTREN register accessor: an alias for `Reg<INTREN_SPEC>`"]
pub type INTREN = crate::Reg<intren::INTREN_SPEC>;
#[doc = "INTERRUPT enable register"]
pub mod intren;
#[doc = "EVENTEN register accessor: an alias for `Reg<EVENTEN_SPEC>`"]
pub type EVENTEN = crate::Reg<eventen::EVENTEN_SPEC>;
#[doc = "Event Enable register"]
pub mod eventen;
#[doc = "INTRSTAT register accessor: an alias for `Reg<INTRSTAT_SPEC>`"]
pub type INTRSTAT = crate::Reg<intrstat::INTRSTAT_SPEC>;
#[doc = "INTERRUPT STATUS register"]
pub mod intrstat;
#[doc = "gpreg register accessor: an alias for `Reg<GPREG_SPEC>`"]
pub type GPREG = crate::Reg<gpreg::GPREG_SPEC>;
#[doc = "General purpose register bank N."]
pub mod gpreg;
#[doc = "compreg register accessor: an alias for `Reg<COMPREG_SPEC>`"]
pub type COMPREG = crate::Reg<compreg::COMPREG_SPEC>;
#[doc = "Compute register bank"]
pub mod compreg;
