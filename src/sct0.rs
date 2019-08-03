#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - SCT control register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SCT limit event select register"]
    pub limit: LIMIT,
    #[doc = "0x0c - SCT halt event select register"]
    pub halt: HALT,
    #[doc = "0x10 - SCT stop event select register"]
    pub stop: STOP,
    #[doc = "0x14 - SCT start event select register"]
    pub start: START,
    _reserved0: [u8; 40usize],
    #[doc = "0x40 - SCT counter register"]
    pub count: COUNT,
    #[doc = "0x44 - SCT state register"]
    pub state: STATE,
    #[doc = "0x48 - SCT input register"]
    pub input: INPUT,
    #[doc = "0x4c - SCT match/capture mode register"]
    pub regmode: REGMODE,
    #[doc = "0x50 - SCT output register"]
    pub output: OUTPUT,
    #[doc = "0x54 - SCT output counter direction control register"]
    pub outputdirctrl: OUTPUTDIRCTRL,
    #[doc = "0x58 - SCT conflict resolution register"]
    pub res: RES,
    #[doc = "0x5c - SCT DMA request 0 register"]
    pub dma0request: DMA0REQUEST,
    #[doc = "0x60 - SCT DMA request 1 register"]
    pub dma1request: DMA1REQUEST,
    _reserved1: [u8; 140usize],
    #[doc = "0xf0 - SCT event interrupt enable register"]
    pub even: EVEN,
    #[doc = "0xf4 - SCT event flag register"]
    pub evflag: EVFLAG,
    #[doc = "0xf8 - SCT conflict interrupt enable register"]
    pub conen: CONEN,
    #[doc = "0xfc - SCT conflict flag register"]
    pub conflag: CONFLAG,
    #[doc = "0x100 - SCT capture register of capture channel"]
    pub sctcap0: SCTCAP0,
    #[doc = "0x104 - SCT capture register of capture channel"]
    pub sctcap1: SCTCAP1,
    #[doc = "0x108 - SCT capture register of capture channel"]
    pub sctcap2: SCTCAP2,
    #[doc = "0x10c - SCT capture register of capture channel"]
    pub sctcap3: SCTCAP3,
    #[doc = "0x110 - SCT capture register of capture channel"]
    pub sctcap4: SCTCAP4,
    #[doc = "0x114 - SCT capture register of capture channel"]
    pub sctcap5: SCTCAP5,
    #[doc = "0x118 - SCT capture register of capture channel"]
    pub sctcap6: SCTCAP6,
    #[doc = "0x11c - SCT capture register of capture channel"]
    pub sctcap7: SCTCAP7,
    #[doc = "0x120 - SCT capture register of capture channel"]
    pub sctcap8: SCTCAP8,
    #[doc = "0x124 - SCT capture register of capture channel"]
    pub sctcap9: SCTCAP9,
    #[doc = "0x128 - SCT capture register of capture channel"]
    pub sctcap10: SCTCAP10,
    #[doc = "0x12c - SCT capture register of capture channel"]
    pub sctcap11: SCTCAP11,
    #[doc = "0x130 - SCT capture register of capture channel"]
    pub sctcap12: SCTCAP12,
    #[doc = "0x134 - SCT capture register of capture channel"]
    pub sctcap13: SCTCAP13,
    #[doc = "0x138 - SCT capture register of capture channel"]
    pub sctcap14: SCTCAP14,
    #[doc = "0x13c - SCT capture register of capture channel"]
    pub sctcap15: SCTCAP15,
    _reserved2: [u8; 192usize],
    #[doc = "0x200 - SCT capture control register"]
    pub sctcapctrl0: SCTCAPCTRL0,
    #[doc = "0x204 - SCT capture control register"]
    pub sctcapctrl1: SCTCAPCTRL1,
    #[doc = "0x208 - SCT capture control register"]
    pub sctcapctrl2: SCTCAPCTRL2,
    #[doc = "0x20c - SCT capture control register"]
    pub sctcapctrl3: SCTCAPCTRL3,
    #[doc = "0x210 - SCT capture control register"]
    pub sctcapctrl4: SCTCAPCTRL4,
    #[doc = "0x214 - SCT capture control register"]
    pub sctcapctrl5: SCTCAPCTRL5,
    #[doc = "0x218 - SCT capture control register"]
    pub sctcapctrl6: SCTCAPCTRL6,
    #[doc = "0x21c - SCT capture control register"]
    pub sctcapctrl7: SCTCAPCTRL7,
    #[doc = "0x220 - SCT capture control register"]
    pub sctcapctrl8: SCTCAPCTRL8,
    #[doc = "0x224 - SCT capture control register"]
    pub sctcapctrl9: SCTCAPCTRL9,
    #[doc = "0x228 - SCT capture control register"]
    pub sctcapctrl10: SCTCAPCTRL10,
    #[doc = "0x22c - SCT capture control register"]
    pub sctcapctrl11: SCTCAPCTRL11,
    #[doc = "0x230 - SCT capture control register"]
    pub sctcapctrl12: SCTCAPCTRL12,
    #[doc = "0x234 - SCT capture control register"]
    pub sctcapctrl13: SCTCAPCTRL13,
    #[doc = "0x238 - SCT capture control register"]
    pub sctcapctrl14: SCTCAPCTRL14,
    #[doc = "0x23c - SCT capture control register"]
    pub sctcapctrl15: SCTCAPCTRL15,
    _reserved3: [u8; 192usize],
    #[doc = "0x300 - no description available"]
    pub event: [EVENT; 16],
    _reserved4: [u8; 384usize],
    #[doc = "0x500 - no description available"]
    pub out: [OUT; 10],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct EVENT {
    #[doc = "0x00 - SCT event state register 0"]
    pub state: self::event::STATE,
    #[doc = "0x04 - SCT event control register 0"]
    pub ctrl: self::event::CTRL,
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod event;
#[doc = r" Register block"]
#[repr(C)]
pub struct OUT {
    #[doc = "0x00 - SCT output 0 set register"]
    pub set: self::out::SET,
    #[doc = "0x04 - SCT output 0 clear register"]
    pub clr: self::out::CLR,
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod out;
#[doc = "SCT configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "SCT control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "SCT limit event select register"]
pub struct LIMIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT limit event select register"]
pub mod limit;
#[doc = "SCT halt event select register"]
pub struct HALT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT halt event select register"]
pub mod halt;
#[doc = "SCT stop event select register"]
pub struct STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT stop event select register"]
pub mod stop;
#[doc = "SCT start event select register"]
pub struct START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT start event select register"]
pub mod start;
#[doc = "SCT counter register"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT counter register"]
pub mod count;
#[doc = "SCT state register"]
pub struct STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT state register"]
pub mod state;
#[doc = "SCT input register"]
pub struct INPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT input register"]
pub mod input;
#[doc = "SCT match/capture mode register"]
pub struct REGMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match/capture mode register"]
pub mod regmode;
#[doc = "SCT output register"]
pub struct OUTPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output register"]
pub mod output;
#[doc = "SCT output counter direction control register"]
pub struct OUTPUTDIRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "SCT conflict resolution register"]
pub struct RES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "SCT DMA request 0 register"]
pub struct DMA0REQUEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT DMA request 0 register"]
pub mod dma0request;
#[doc = "SCT DMA request 1 register"]
pub struct DMA1REQUEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT DMA request 1 register"]
pub mod dma1request;
#[doc = "SCT event interrupt enable register"]
pub struct EVEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event interrupt enable register"]
pub mod even;
#[doc = "SCT event flag register"]
pub struct EVFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "SCT conflict interrupt enable register"]
pub struct CONEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict interrupt enable register"]
pub mod conen;
#[doc = "SCT conflict flag register"]
pub struct CONFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap0;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch0;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap1;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch1;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap2;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch2;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap3;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch3;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap4;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch4;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap5;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch5;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap6;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch6;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap7;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch7;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap8;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch8;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap9;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch9;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap10;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch10;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap11;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch11;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap12;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch12;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap13;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch13;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap14;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch14;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap15;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch15;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl0;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel0;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl1;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel1;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl2;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel2;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl3;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel3;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl4;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel4;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl5;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel5;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl6;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel6;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl7;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel7;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl8;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel8;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl9;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel9;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl10;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel10;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl11;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel11;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl12;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel12;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl13;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel13;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl14;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel14;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl15;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel15;
