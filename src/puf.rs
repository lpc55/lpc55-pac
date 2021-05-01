#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PUF Control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - PUF Key Index register"]
    pub keyindex: crate::Reg<keyindex::KEYINDEX_SPEC>,
    #[doc = "0x08 - PUF Key Size register"]
    pub keysize: crate::Reg<keysize::KEYSIZE_SPEC>,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - PUF Status register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    _reserved4: [u8; 4usize],
    #[doc = "0x28 - PUF Allow register"]
    pub allow: crate::Reg<allow::ALLOW_SPEC>,
    _reserved5: [u8; 20usize],
    #[doc = "0x40 - PUF Key Input register"]
    pub keyinput: crate::Reg<keyinput::KEYINPUT_SPEC>,
    #[doc = "0x44 - PUF Code Input register"]
    pub codeinput: crate::Reg<codeinput::CODEINPUT_SPEC>,
    #[doc = "0x48 - PUF Code Output register"]
    pub codeoutput: crate::Reg<codeoutput::CODEOUTPUT_SPEC>,
    _reserved8: [u8; 20usize],
    #[doc = "0x60 - PUF Key Output Index register"]
    pub keyoutindex: crate::Reg<keyoutindex::KEYOUTINDEX_SPEC>,
    #[doc = "0x64 - PUF Key Output register"]
    pub keyoutput: crate::Reg<keyoutput::KEYOUTPUT_SPEC>,
    _reserved10: [u8; 116usize],
    #[doc = "0xdc - PUF Interface Status and clear register"]
    pub ifstat: crate::Reg<ifstat::IFSTAT_SPEC>,
    _reserved11: [u8; 28usize],
    #[doc = "0xfc - PUF version register."]
    pub version: crate::Reg<version::VERSION_SPEC>,
    #[doc = "0x100 - PUF Interrupt Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x104 - PUF interrupt status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x108 - PUF RAM Power Control"]
    pub pwrctrl: crate::Reg<pwrctrl::PWRCTRL_SPEC>,
    #[doc = "0x10c - PUF config register for block bits"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    _reserved16: [u8; 240usize],
    #[doc = "0x200 - Only reset in case of full IC reset"]
    pub keylock: crate::Reg<keylock::KEYLOCK_SPEC>,
    #[doc = "0x204 - no description available"]
    pub keyenable: crate::Reg<keyenable::KEYENABLE_SPEC>,
    #[doc = "0x208 - Reinitialize Keys shift registers counters"]
    pub keyreset: crate::Reg<keyreset::KEYRESET_SPEC>,
    #[doc = "0x20c - no description available"]
    pub idxblk_l: crate::Reg<idxblk_l::IDXBLK_L_SPEC>,
    #[doc = "0x210 - no description available"]
    pub idxblk_h_dp: crate::Reg<idxblk_h_dp::IDXBLK_H_DP_SPEC>,
    #[doc = "0x214 - Only reset in case of full IC reset"]
    pub keymask: [crate::Reg<keymask::KEYMASK_SPEC>; 4],
    _reserved22: [u8; 48usize],
    #[doc = "0x254 - no description available"]
    pub idxblk_h: crate::Reg<idxblk_h::IDXBLK_H_SPEC>,
    #[doc = "0x258 - no description available"]
    pub idxblk_l_dp: crate::Reg<idxblk_l_dp::IDXBLK_L_DP_SPEC>,
    #[doc = "0x25c - no description available"]
    pub shift_status: crate::Reg<shift_status::SHIFT_STATUS_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "PUF Control register"]
pub mod ctrl;
#[doc = "KEYINDEX register accessor: an alias for `Reg<KEYINDEX_SPEC>`"]
pub type KEYINDEX = crate::Reg<keyindex::KEYINDEX_SPEC>;
#[doc = "PUF Key Index register"]
pub mod keyindex;
#[doc = "KEYSIZE register accessor: an alias for `Reg<KEYSIZE_SPEC>`"]
pub type KEYSIZE = crate::Reg<keysize::KEYSIZE_SPEC>;
#[doc = "PUF Key Size register"]
pub mod keysize;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "PUF Status register"]
pub mod stat;
#[doc = "ALLOW register accessor: an alias for `Reg<ALLOW_SPEC>`"]
pub type ALLOW = crate::Reg<allow::ALLOW_SPEC>;
#[doc = "PUF Allow register"]
pub mod allow;
#[doc = "KEYINPUT register accessor: an alias for `Reg<KEYINPUT_SPEC>`"]
pub type KEYINPUT = crate::Reg<keyinput::KEYINPUT_SPEC>;
#[doc = "PUF Key Input register"]
pub mod keyinput;
#[doc = "CODEINPUT register accessor: an alias for `Reg<CODEINPUT_SPEC>`"]
pub type CODEINPUT = crate::Reg<codeinput::CODEINPUT_SPEC>;
#[doc = "PUF Code Input register"]
pub mod codeinput;
#[doc = "CODEOUTPUT register accessor: an alias for `Reg<CODEOUTPUT_SPEC>`"]
pub type CODEOUTPUT = crate::Reg<codeoutput::CODEOUTPUT_SPEC>;
#[doc = "PUF Code Output register"]
pub mod codeoutput;
#[doc = "KEYOUTINDEX register accessor: an alias for `Reg<KEYOUTINDEX_SPEC>`"]
pub type KEYOUTINDEX = crate::Reg<keyoutindex::KEYOUTINDEX_SPEC>;
#[doc = "PUF Key Output Index register"]
pub mod keyoutindex;
#[doc = "KEYOUTPUT register accessor: an alias for `Reg<KEYOUTPUT_SPEC>`"]
pub type KEYOUTPUT = crate::Reg<keyoutput::KEYOUTPUT_SPEC>;
#[doc = "PUF Key Output register"]
pub mod keyoutput;
#[doc = "IFSTAT register accessor: an alias for `Reg<IFSTAT_SPEC>`"]
pub type IFSTAT = crate::Reg<ifstat::IFSTAT_SPEC>;
#[doc = "PUF Interface Status and clear register"]
pub mod ifstat;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "PUF version register."]
pub mod version;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "PUF Interrupt Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "PUF interrupt status"]
pub mod intstat;
#[doc = "PWRCTRL register accessor: an alias for `Reg<PWRCTRL_SPEC>`"]
pub type PWRCTRL = crate::Reg<pwrctrl::PWRCTRL_SPEC>;
#[doc = "PUF RAM Power Control"]
pub mod pwrctrl;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "PUF config register for block bits"]
pub mod cfg;
#[doc = "KEYLOCK register accessor: an alias for `Reg<KEYLOCK_SPEC>`"]
pub type KEYLOCK = crate::Reg<keylock::KEYLOCK_SPEC>;
#[doc = "Only reset in case of full IC reset"]
pub mod keylock;
#[doc = "KEYENABLE register accessor: an alias for `Reg<KEYENABLE_SPEC>`"]
pub type KEYENABLE = crate::Reg<keyenable::KEYENABLE_SPEC>;
#[doc = "no description available"]
pub mod keyenable;
#[doc = "KEYRESET register accessor: an alias for `Reg<KEYRESET_SPEC>`"]
pub type KEYRESET = crate::Reg<keyreset::KEYRESET_SPEC>;
#[doc = "Reinitialize Keys shift registers counters"]
pub mod keyreset;
#[doc = "IDXBLK_L register accessor: an alias for `Reg<IDXBLK_L_SPEC>`"]
pub type IDXBLK_L = crate::Reg<idxblk_l::IDXBLK_L_SPEC>;
#[doc = "no description available"]
pub mod idxblk_l;
#[doc = "IDXBLK_H_DP register accessor: an alias for `Reg<IDXBLK_H_DP_SPEC>`"]
pub type IDXBLK_H_DP = crate::Reg<idxblk_h_dp::IDXBLK_H_DP_SPEC>;
#[doc = "no description available"]
pub mod idxblk_h_dp;
#[doc = "KEYMASK register accessor: an alias for `Reg<KEYMASK_SPEC>`"]
pub type KEYMASK = crate::Reg<keymask::KEYMASK_SPEC>;
#[doc = "Only reset in case of full IC reset"]
pub mod keymask;
#[doc = "IDXBLK_H register accessor: an alias for `Reg<IDXBLK_H_SPEC>`"]
pub type IDXBLK_H = crate::Reg<idxblk_h::IDXBLK_H_SPEC>;
#[doc = "no description available"]
pub mod idxblk_h;
#[doc = "IDXBLK_L_DP register accessor: an alias for `Reg<IDXBLK_L_DP_SPEC>`"]
pub type IDXBLK_L_DP = crate::Reg<idxblk_l_dp::IDXBLK_L_DP_SPEC>;
#[doc = "no description available"]
pub mod idxblk_l_dp;
#[doc = "SHIFT_STATUS register accessor: an alias for `Reg<SHIFT_STATUS_SPEC>`"]
pub type SHIFT_STATUS = crate::Reg<shift_status::SHIFT_STATUS_SPEC>;
#[doc = "no description available"]
pub mod shift_status;
