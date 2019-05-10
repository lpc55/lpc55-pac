#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Power Enable register"]
    pub pwren: PWREN,
    #[doc = "0x08 - Clock Divider register"]
    pub clkdiv: CLKDIV,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Clock Enable register"]
    pub clkena: CLKENA,
    #[doc = "0x14 - Time-out register"]
    pub tmout: TMOUT,
    #[doc = "0x18 - Card Type register"]
    pub ctype: CTYPE,
    #[doc = "0x1c - Block Size register"]
    pub blksiz: BLKSIZ,
    #[doc = "0x20 - Byte Count register"]
    pub bytcnt: BYTCNT,
    #[doc = "0x24 - Interrupt Mask register"]
    pub intmask: INTMASK,
    #[doc = "0x28 - Command Argument register"]
    pub cmdarg: CMDARG,
    #[doc = "0x2c - Command register"]
    pub cmd: CMD,
    #[doc = "0x30 - Response register"]
    pub resp: [RESP; 4],
    #[doc = "0x40 - Masked Interrupt Status register"]
    pub mintsts: MINTSTS,
    #[doc = "0x44 - Raw Interrupt Status register"]
    pub rintsts: RINTSTS,
    #[doc = "0x48 - Status register"]
    pub status: STATUS,
    #[doc = "0x4c - FIFO Threshold Watermark register"]
    pub fifoth: FIFOTH,
    #[doc = "0x50 - Card Detect register"]
    pub cdetect: CDETECT,
    #[doc = "0x54 - Write Protect register"]
    pub wrtprt: WRTPRT,
    _reserved1: [u8; 4usize],
    #[doc = "0x5c - Transferred CIU Card Byte Count register"]
    pub tcbcnt: TCBCNT,
    #[doc = "0x60 - Transferred Host to BIU-FIFO Byte Count register"]
    pub tbbcnt: TBBCNT,
    #[doc = "0x64 - Debounce Count register"]
    pub debnce: DEBNCE,
    _reserved2: [u8; 16usize],
    #[doc = "0x78 - Hardware Reset"]
    pub rst_n: RST_N,
    _reserved3: [u8; 4usize],
    #[doc = "0x80 - Bus Mode register"]
    pub bmod: BMOD,
    #[doc = "0x84 - Poll Demand register"]
    pub pldmnd: PLDMND,
    #[doc = "0x88 - Descriptor List Base Address register"]
    pub dbaddr: DBADDR,
    #[doc = "0x8c - Internal DMAC Status register"]
    pub idsts: IDSTS,
    #[doc = "0x90 - Internal DMAC Interrupt Enable register"]
    pub idinten: IDINTEN,
    #[doc = "0x94 - Current Host Descriptor Address register"]
    pub dscaddr: DSCADDR,
    #[doc = "0x98 - Current Buffer Descriptor Address register"]
    pub bufaddr: BUFADDR,
    _reserved4: [u8; 100usize],
    #[doc = "0x100 - Card Threshold Control"]
    pub cardthrctl: CARDTHRCTL,
    #[doc = "0x104 - Power control"]
    pub backendpwr: BACKENDPWR,
    _reserved5: [u8; 248usize],
    #[doc = "0x200 - SDIF FIFO"]
    pub fifo: [FIFO; 64],
}
#[doc = "Control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctrl;
#[doc = "Power Enable register"]
pub struct PWREN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Enable register"]
pub mod pwren;
#[doc = "Clock Divider register"]
pub struct CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Divider register"]
pub mod clkdiv;
#[doc = "Clock Enable register"]
pub struct CLKENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Enable register"]
pub mod clkena;
#[doc = "Time-out register"]
pub struct TMOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time-out register"]
pub mod tmout;
#[doc = "Card Type register"]
pub struct CTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Card Type register"]
pub mod ctype;
#[doc = "Block Size register"]
pub struct BLKSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Size register"]
pub mod blksiz;
#[doc = "Byte Count register"]
pub struct BYTCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Byte Count register"]
pub mod bytcnt;
#[doc = "Interrupt Mask register"]
pub struct INTMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask register"]
pub mod intmask;
#[doc = "Command Argument register"]
pub struct CMDARG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Argument register"]
pub mod cmdarg;
#[doc = "Command register"]
pub struct CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command register"]
pub mod cmd;
#[doc = "Response register"]
pub struct RESP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response register"]
pub mod resp;
#[doc = "Masked Interrupt Status register"]
pub struct MINTSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked Interrupt Status register"]
pub mod mintsts;
#[doc = "Raw Interrupt Status register"]
pub struct RINTSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status register"]
pub mod rintsts;
#[doc = "Status register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod status;
#[doc = "FIFO Threshold Watermark register"]
pub struct FIFOTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Threshold Watermark register"]
pub mod fifoth;
#[doc = "Card Detect register"]
pub struct CDETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Card Detect register"]
pub mod cdetect;
#[doc = "Write Protect register"]
pub struct WRTPRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect register"]
pub mod wrtprt;
#[doc = "Transferred CIU Card Byte Count register"]
pub struct TCBCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transferred CIU Card Byte Count register"]
pub mod tcbcnt;
#[doc = "Transferred Host to BIU-FIFO Byte Count register"]
pub struct TBBCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transferred Host to BIU-FIFO Byte Count register"]
pub mod tbbcnt;
#[doc = "Debounce Count register"]
pub struct DEBNCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debounce Count register"]
pub mod debnce;
#[doc = "Hardware Reset"]
pub struct RST_N {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware Reset"]
pub mod rst_n;
#[doc = "Bus Mode register"]
pub struct BMOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Mode register"]
pub mod bmod;
#[doc = "Poll Demand register"]
pub struct PLDMND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Poll Demand register"]
pub mod pldmnd;
#[doc = "Descriptor List Base Address register"]
pub struct DBADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Descriptor List Base Address register"]
pub mod dbaddr;
#[doc = "Internal DMAC Status register"]
pub struct IDSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal DMAC Status register"]
pub mod idsts;
#[doc = "Internal DMAC Interrupt Enable register"]
pub struct IDINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal DMAC Interrupt Enable register"]
pub mod idinten;
#[doc = "Current Host Descriptor Address register"]
pub struct DSCADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Host Descriptor Address register"]
pub mod dscaddr;
#[doc = "Current Buffer Descriptor Address register"]
pub struct BUFADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Buffer Descriptor Address register"]
pub mod bufaddr;
#[doc = "Card Threshold Control"]
pub struct CARDTHRCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Card Threshold Control"]
pub mod cardthrctl;
#[doc = "Power control"]
pub struct BACKENDPWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control"]
pub mod backendpwr;
#[doc = "SDIF FIFO"]
pub struct FIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDIF FIFO"]
pub mod fifo;
