#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PUF Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - PUF Key Index register"]
    pub keyindex: KEYINDEX,
    #[doc = "0x08 - PUF Key Size register"]
    pub keysize: KEYSIZE,
    _reserved0: [u8; 20usize],
    #[doc = "0x20 - PUF Status register"]
    pub stat: STAT,
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - PUF Allow register"]
    pub allow: ALLOW,
    _reserved2: [u8; 20usize],
    #[doc = "0x40 - PUF Key Input register"]
    pub keyinput: KEYINPUT,
    #[doc = "0x44 - PUF Code Input register"]
    pub codeinput: CODEINPUT,
    #[doc = "0x48 - PUF Code Output register"]
    pub codeoutput: CODEOUTPUT,
    _reserved3: [u8; 20usize],
    #[doc = "0x60 - PUF Key Output Index register"]
    pub keyoutindex: KEYOUTINDEX,
    #[doc = "0x64 - PUF Key Output register"]
    pub keyoutput: KEYOUTPUT,
    _reserved4: [u8; 116usize],
    #[doc = "0xdc - PUF Interface Status and clear register"]
    pub ifstat: IFSTAT,
    _reserved5: [u8; 28usize],
    #[doc = "0xfc - PUF version register."]
    pub version: VERSION,
    #[doc = "0x100 - PUF Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x104 - PUF interrupt status"]
    pub intstat: INTSTAT,
    #[doc = "0x108 - PUF RAM Power Control"]
    pub pwrctrl: PWRCTRL,
    #[doc = "0x10c - PUF config register for block bits"]
    pub cfg: CFG,
    _reserved6: [u8; 240usize],
    #[doc = "0x200 - Only reset in case of full IC reset"]
    pub keylock: KEYLOCK,
    #[doc = "0x204 - no description available"]
    pub keyenable: KEYENABLE,
    #[doc = "0x208 - Reinitialize Keys shift registers counters"]
    pub keyreset: KEYRESET,
    #[doc = "0x20c - no description available"]
    pub idxblk_l: IDXBLK_L,
    #[doc = "0x210 - no description available"]
    pub idxblk_h_dp: IDXBLK_H_DP,
    #[doc = "0x214 - Only reset in case of full IC reset"]
    pub keymask: [KEYMASK; 4],
    _reserved7: [u8; 48usize],
    #[doc = "0x254 - no description available"]
    pub idxblk_h: IDXBLK_H,
    #[doc = "0x258 - no description available"]
    pub idxblk_l_dp: IDXBLK_L_DP,
    #[doc = "0x25c - no description available"]
    pub shift_status: SHIFT_STATUS,
}
#[doc = "PUF Control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Control register"]
pub mod ctrl;
#[doc = "PUF Key Index register"]
pub struct KEYINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Key Index register"]
pub mod keyindex;
#[doc = "PUF Key Size register"]
pub struct KEYSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Key Size register"]
pub mod keysize;
#[doc = "PUF Status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Status register"]
pub mod stat;
#[doc = "PUF Allow register"]
pub struct ALLOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Allow register"]
pub mod allow;
#[doc = "PUF Key Input register"]
pub struct KEYINPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Key Input register"]
pub mod keyinput;
#[doc = "PUF Code Input register"]
pub struct CODEINPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Code Input register"]
pub mod codeinput;
#[doc = "PUF Code Output register"]
pub struct CODEOUTPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Code Output register"]
pub mod codeoutput;
#[doc = "PUF Key Output Index register"]
pub struct KEYOUTINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Key Output Index register"]
pub mod keyoutindex;
#[doc = "PUF Key Output register"]
pub struct KEYOUTPUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Key Output register"]
pub mod keyoutput;
#[doc = "PUF Interface Status and clear register"]
pub struct IFSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Interface Status and clear register"]
pub mod ifstat;
#[doc = "PUF version register."]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF version register."]
pub mod version;
#[doc = "PUF Interrupt Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF Interrupt Enable"]
pub mod inten;
#[doc = "PUF interrupt status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF interrupt status"]
pub mod intstat;
#[doc = "PUF RAM Power Control"]
pub struct PWRCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF RAM Power Control"]
pub mod pwrctrl;
#[doc = "PUF config register for block bits"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUF config register for block bits"]
pub mod cfg;
#[doc = "Only reset in case of full IC reset"]
pub struct KEYLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Only reset in case of full IC reset"]
pub mod keylock;
#[doc = "no description available"]
pub struct KEYENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod keyenable;
#[doc = "Reinitialize Keys shift registers counters"]
pub struct KEYRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reinitialize Keys shift registers counters"]
pub mod keyreset;
#[doc = "no description available"]
pub struct IDXBLK_L {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod idxblk_l;
#[doc = "no description available"]
pub struct IDXBLK_H_DP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod idxblk_h_dp;
#[doc = "Only reset in case of full IC reset"]
pub struct KEYMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Only reset in case of full IC reset"]
pub mod keymask;
#[doc = "no description available"]
pub struct IDXBLK_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod idxblk_h;
#[doc = "no description available"]
pub struct IDXBLK_L_DP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod idxblk_l_dp;
#[doc = "no description available"]
pub struct SHIFT_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod shift_status;
