#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - ADC Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x14 - ADC Status Register"]
    pub stat: STAT,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x1c - DMA Enable Register"]
    pub de: DE,
    #[doc = "0x20 - ADC Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x24 - ADC Pause Register"]
    pub pause: PAUSE,
    _reserved1: [u8; 12usize],
    #[doc = "0x34 - Software Trigger Register"]
    pub swtrig: SWTRIG,
    #[doc = "0x38 - Trigger Status Register"]
    pub tstat: TSTAT,
    _reserved2: [u8; 4usize],
    #[doc = "0x40 - ADC Offset Trim Register"]
    pub ofstrim: OFSTRIM,
    _reserved3: [u8; 92usize],
    #[doc = "0xa0 - Trigger Control Register"]
    pub tctrl: [TCTRL; 16],
    #[doc = "0xe0 - FIFO Control Register"]
    pub fctrl: [FCTRL; 2],
    _reserved4: [u8; 8usize],
    #[doc = "0xf0 - Gain Calibration Control"]
    pub gcc: [GCC; 2],
    #[doc = "0xf8 - Gain Calculation Result"]
    pub gcr: [GCR; 2],
    #[doc = "0x100 - ADC Command Low Buffer Register"]
    pub cmdl1: CMDL1,
    #[doc = "0x104 - ADC Command High Buffer Register"]
    pub cmdh1: CMDH1,
    #[doc = "0x108 - ADC Command Low Buffer Register"]
    pub cmdl2: CMDL2,
    #[doc = "0x10c - ADC Command High Buffer Register"]
    pub cmdh2: CMDH2,
    #[doc = "0x110 - ADC Command Low Buffer Register"]
    pub cmdl3: CMDL3,
    #[doc = "0x114 - ADC Command High Buffer Register"]
    pub cmdh3: CMDH3,
    #[doc = "0x118 - ADC Command Low Buffer Register"]
    pub cmdl4: CMDL4,
    #[doc = "0x11c - ADC Command High Buffer Register"]
    pub cmdh4: CMDH4,
    #[doc = "0x120 - ADC Command Low Buffer Register"]
    pub cmdl5: CMDL5,
    #[doc = "0x124 - ADC Command High Buffer Register"]
    pub cmdh5: CMDH5,
    #[doc = "0x128 - ADC Command Low Buffer Register"]
    pub cmdl6: CMDL6,
    #[doc = "0x12c - ADC Command High Buffer Register"]
    pub cmdh6: CMDH6,
    #[doc = "0x130 - ADC Command Low Buffer Register"]
    pub cmdl7: CMDL7,
    #[doc = "0x134 - ADC Command High Buffer Register"]
    pub cmdh7: CMDH7,
    #[doc = "0x138 - ADC Command Low Buffer Register"]
    pub cmdl8: CMDL8,
    #[doc = "0x13c - ADC Command High Buffer Register"]
    pub cmdh8: CMDH8,
    #[doc = "0x140 - ADC Command Low Buffer Register"]
    pub cmdl9: CMDL9,
    #[doc = "0x144 - ADC Command High Buffer Register"]
    pub cmdh9: CMDH9,
    #[doc = "0x148 - ADC Command Low Buffer Register"]
    pub cmdl10: CMDL10,
    #[doc = "0x14c - ADC Command High Buffer Register"]
    pub cmdh10: CMDH10,
    #[doc = "0x150 - ADC Command Low Buffer Register"]
    pub cmdl11: CMDL11,
    #[doc = "0x154 - ADC Command High Buffer Register"]
    pub cmdh11: CMDH11,
    #[doc = "0x158 - ADC Command Low Buffer Register"]
    pub cmdl12: CMDL12,
    #[doc = "0x15c - ADC Command High Buffer Register"]
    pub cmdh12: CMDH12,
    #[doc = "0x160 - ADC Command Low Buffer Register"]
    pub cmdl13: CMDL13,
    #[doc = "0x164 - ADC Command High Buffer Register"]
    pub cmdh13: CMDH13,
    #[doc = "0x168 - ADC Command Low Buffer Register"]
    pub cmdl14: CMDL14,
    #[doc = "0x16c - ADC Command High Buffer Register"]
    pub cmdh14: CMDH14,
    #[doc = "0x170 - ADC Command Low Buffer Register"]
    pub cmdl15: CMDL15,
    #[doc = "0x174 - ADC Command High Buffer Register"]
    pub cmdh15: CMDH15,
    _reserved5: [u8; 136usize],
    #[doc = "0x200 - Compare Value Register"]
    pub cv1: CV,
    #[doc = "0x204 - Compare Value Register"]
    pub cv2: CV,
    #[doc = "0x208 - Compare Value Register"]
    pub cv3: CV,
    #[doc = "0x20c - Compare Value Register"]
    pub cv4: CV,
    _reserved6: [u8; 240usize],
    #[doc = "0x300 - ADC Data Result FIFO Register"]
    pub resfifo: [RESFIFO; 2],
    _reserved7: [u8; 248usize],
    #[doc = "0x400 - Calibration General A-Side Registers"]
    pub cal_gar: [CAL_GAR; 33],
    _reserved8: [u8; 124usize],
    #[doc = "0x500 - Calibration General B-Side Registers"]
    pub cal_gbr: [CAL_GBR; 33],
    _reserved9: [u8; 2680usize],
    #[doc = "0xffc - ADC Test Register"]
    pub tst: TST,
}
#[doc = "Version ID Register"]
pub struct VERID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register"]
pub struct PARAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "ADC Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Control Register"]
pub mod ctrl;
#[doc = "ADC Status Register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Status Register"]
pub mod stat;
#[doc = "Interrupt Enable Register"]
pub struct IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "DMA Enable Register"]
pub struct DE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Enable Register"]
pub mod de;
#[doc = "ADC Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Configuration Register"]
pub mod cfg;
#[doc = "ADC Pause Register"]
pub struct PAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Pause Register"]
pub mod pause;
#[doc = "Software Trigger Register"]
pub struct SWTRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Trigger Register"]
pub mod swtrig;
#[doc = "Trigger Status Register"]
pub struct TSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Status Register"]
pub mod tstat;
#[doc = "ADC Offset Trim Register"]
pub struct OFSTRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Offset Trim Register"]
pub mod ofstrim;
#[doc = "Trigger Control Register"]
pub struct TCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Control Register"]
pub mod tctrl;
#[doc = "FIFO Control Register"]
pub struct FCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Control Register"]
pub mod fctrl;
#[doc = "Gain Calibration Control"]
pub struct GCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Gain Calibration Control"]
pub mod gcc;
#[doc = "Gain Calculation Result"]
pub struct GCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Gain Calculation Result"]
pub mod gcr;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl1;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh1;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl2;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh2;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl3;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh3;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl4;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh4;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl5;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh5;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl6;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh6;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl7;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh7;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl8;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh8;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl9;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh9;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl10;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh10;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl11;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh11;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl12;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh12;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl13;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh13;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl14;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh14;
#[doc = "ADC Command Low Buffer Register"]
pub struct CMDL15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl15;
#[doc = "ADC Command High Buffer Register"]
pub struct CMDH15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh15;
#[doc = "Compare Value Register"]
pub struct CV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Value Register"]
pub mod cv;
#[doc = "ADC Data Result FIFO Register"]
pub struct RESFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Data Result FIFO Register"]
pub mod resfifo;
#[doc = "Calibration General A-Side Registers"]
pub struct CAL_GAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration General A-Side Registers"]
pub mod cal_gar;
#[doc = "Calibration General B-Side Registers"]
pub struct CAL_GBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration General B-Side Registers"]
pub mod cal_gbr;
#[doc = "ADC Test Register"]
pub struct TST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Test Register"]
pub mod tst;
