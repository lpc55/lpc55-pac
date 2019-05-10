#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Base address register for output region"]
    pub outbase: OUTBASE,
    #[doc = "0x04 - Output format"]
    pub outformat: OUTFORMAT,
    #[doc = "0x08 - Base address register for temp region"]
    pub tmpbase: TMPBASE,
    #[doc = "0x0c - Temp format"]
    pub tmpformat: TMPFORMAT,
    #[doc = "0x10 - Base address register for input A region"]
    pub inabase: INABASE,
    #[doc = "0x14 - Input A format"]
    pub inaformat: INAFORMAT,
    #[doc = "0x18 - Base address register for input B region"]
    pub inbbase: INBBASE,
    #[doc = "0x1c - Input B format"]
    pub inbformat: INBFORMAT,
    _reserved0: [u8; 224usize],
    #[doc = "0x100 - PowerQuad Control register"]
    pub control: CONTROL,
    #[doc = "0x104 - Length register"]
    pub length: LENGTH,
    #[doc = "0x108 - Pre-scale register"]
    pub cppre: CPPRE,
    #[doc = "0x10c - Misc register"]
    pub misc: MISC,
    #[doc = "0x110 - Cursory register"]
    pub cursory: CURSORY,
    _reserved1: [u8; 108usize],
    #[doc = "0x180 - Cordic input X register"]
    pub cordic_x: CORDIC_X,
    #[doc = "0x184 - Cordic input Y register"]
    pub cordic_y: CORDIC_Y,
    #[doc = "0x188 - Cordic input Z register"]
    pub cordic_z: CORDIC_Z,
    #[doc = "0x18c - Read/Write register where error statuses are captured (sticky)"]
    pub errstat: ERRSTAT,
    #[doc = "0x190 - INTERRUPT enable register"]
    pub intren: INTREN,
    #[doc = "0x194 - Event Enable register"]
    pub eventen: EVENTEN,
    #[doc = "0x198 - INTERRUPT STATUS register"]
    pub intrstat: INTRSTAT,
    _reserved2: [u8; 100usize],
    #[doc = "0x200 - General purpose register bank N."]
    pub gpreg: [GPREG; 16],
    #[doc = "0x240 - Compute register bank"]
    pub compreg: [COMPREG; 8],
}
#[doc = "Base address register for output region"]
pub struct OUTBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base address register for output region"]
pub mod outbase;
#[doc = "Output format"]
pub struct OUTFORMAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output format"]
pub mod outformat;
#[doc = "Base address register for temp region"]
pub struct TMPBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base address register for temp region"]
pub mod tmpbase;
#[doc = "Temp format"]
pub struct TMPFORMAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Temp format"]
pub mod tmpformat;
#[doc = "Base address register for input A region"]
pub struct INABASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base address register for input A region"]
pub mod inabase;
#[doc = "Input A format"]
pub struct INAFORMAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input A format"]
pub mod inaformat;
#[doc = "Base address register for input B region"]
pub struct INBBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base address register for input B region"]
pub mod inbbase;
#[doc = "Input B format"]
pub struct INBFORMAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input B format"]
pub mod inbformat;
#[doc = "PowerQuad Control register"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PowerQuad Control register"]
pub mod control;
#[doc = "Length register"]
pub struct LENGTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length register"]
pub mod length;
#[doc = "Pre-scale register"]
pub struct CPPRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre-scale register"]
pub mod cppre;
#[doc = "Misc register"]
pub struct MISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Misc register"]
pub mod misc;
#[doc = "Cursory register"]
pub struct CURSORY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cursory register"]
pub mod cursory;
#[doc = "Cordic input X register"]
pub struct CORDIC_X {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cordic input X register"]
pub mod cordic_x;
#[doc = "Cordic input Y register"]
pub struct CORDIC_Y {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cordic input Y register"]
pub mod cordic_y;
#[doc = "Cordic input Z register"]
pub struct CORDIC_Z {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cordic input Z register"]
pub mod cordic_z;
#[doc = "Read/Write register where error statuses are captured (sticky)"]
pub struct ERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read/Write register where error statuses are captured (sticky)"]
pub mod errstat;
#[doc = "INTERRUPT enable register"]
pub struct INTREN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTERRUPT enable register"]
pub mod intren;
#[doc = "Event Enable register"]
pub struct EVENTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Enable register"]
pub mod eventen;
#[doc = "INTERRUPT STATUS register"]
pub struct INTRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTERRUPT STATUS register"]
pub mod intrstat;
#[doc = "General purpose register bank N."]
pub struct GPREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General purpose register bank N."]
pub mod gpreg;
#[doc = "Compute register bank"]
pub struct COMPREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compute register bank"]
pub mod compreg;
