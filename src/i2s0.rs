#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 3072usize],
    #[doc = "0xc00 - Configuration register 1 for the primary channel pair."]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    #[doc = "0xc04 - Configuration register 2 for the primary channel pair."]
    pub cfg2: crate::Reg<cfg2::CFG2_SPEC>,
    #[doc = "0xc08 - Status register for the primary channel pair."]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    _reserved3: [u8; 16usize],
    #[doc = "0xc1c - Clock divider, used by all channel pairs."]
    pub div: crate::Reg<div::DIV_SPEC>,
    _reserved4: [u8; 480usize],
    #[doc = "0xe00 - FIFO configuration and enable register."]
    pub fifocfg: crate::Reg<fifocfg::FIFOCFG_SPEC>,
    #[doc = "0xe04 - FIFO status register."]
    pub fifostat: crate::Reg<fifostat::FIFOSTAT_SPEC>,
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    pub fifotrig: crate::Reg<fifotrig::FIFOTRIG_SPEC>,
    _reserved7: [u8; 4usize],
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    pub fifointenset: crate::Reg<fifointenset::FIFOINTENSET_SPEC>,
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    pub fifointenclr: crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>,
    #[doc = "0xe18 - FIFO interrupt status register."]
    pub fifointstat: crate::Reg<fifointstat::FIFOINTSTAT_SPEC>,
    _reserved10: [u8; 4usize],
    #[doc = "0xe20 - FIFO write data."]
    pub fifowr: crate::Reg<fifowr::FIFOWR_SPEC>,
    #[doc = "0xe24 - FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fifowr48h: crate::Reg<fifowr48h::FIFOWR48H_SPEC>,
    _reserved12: [u8; 8usize],
    #[doc = "0xe30 - FIFO read data."]
    pub fiford: crate::Reg<fiford::FIFORD_SPEC>,
    #[doc = "0xe34 - FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fiford48h: crate::Reg<fiford48h::FIFORD48H_SPEC>,
    _reserved14: [u8; 8usize],
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    pub fifordnopop: crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>,
    #[doc = "0xe44 - FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fiford48hnopop: crate::Reg<fiford48hnopop::FIFORD48HNOPOP_SPEC>,
    _reserved16: [u8; 436usize],
    #[doc = "0xffc - I2S Module identification"]
    pub id: crate::Reg<id::ID_SPEC>,
}
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "Configuration register 1 for the primary channel pair."]
pub mod cfg1;
#[doc = "CFG2 register accessor: an alias for `Reg<CFG2_SPEC>`"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "Configuration register 2 for the primary channel pair."]
pub mod cfg2;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status register for the primary channel pair."]
pub mod stat;
#[doc = "DIV register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "Clock divider, used by all channel pairs."]
pub mod div;
#[doc = "FIFOCFG register accessor: an alias for `Reg<FIFOCFG_SPEC>`"]
pub type FIFOCFG = crate::Reg<fifocfg::FIFOCFG_SPEC>;
#[doc = "FIFO configuration and enable register."]
pub mod fifocfg;
#[doc = "FIFOSTAT register accessor: an alias for `Reg<FIFOSTAT_SPEC>`"]
pub type FIFOSTAT = crate::Reg<fifostat::FIFOSTAT_SPEC>;
#[doc = "FIFO status register."]
pub mod fifostat;
#[doc = "FIFOTRIG register accessor: an alias for `Reg<FIFOTRIG_SPEC>`"]
pub type FIFOTRIG = crate::Reg<fifotrig::FIFOTRIG_SPEC>;
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub mod fifotrig;
#[doc = "FIFOINTENSET register accessor: an alias for `Reg<FIFOINTENSET_SPEC>`"]
pub type FIFOINTENSET = crate::Reg<fifointenset::FIFOINTENSET_SPEC>;
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub mod fifointenset;
#[doc = "FIFOINTENCLR register accessor: an alias for `Reg<FIFOINTENCLR_SPEC>`"]
pub type FIFOINTENCLR = crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>;
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub mod fifointenclr;
#[doc = "FIFOINTSTAT register accessor: an alias for `Reg<FIFOINTSTAT_SPEC>`"]
pub type FIFOINTSTAT = crate::Reg<fifointstat::FIFOINTSTAT_SPEC>;
#[doc = "FIFO interrupt status register."]
pub mod fifointstat;
#[doc = "FIFOWR register accessor: an alias for `Reg<FIFOWR_SPEC>`"]
pub type FIFOWR = crate::Reg<fifowr::FIFOWR_SPEC>;
#[doc = "FIFO write data."]
pub mod fifowr;
#[doc = "FIFOWR48H register accessor: an alias for `Reg<FIFOWR48H_SPEC>`"]
pub type FIFOWR48H = crate::Reg<fifowr48h::FIFOWR48H_SPEC>;
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fifowr48h;
#[doc = "FIFORD register accessor: an alias for `Reg<FIFORD_SPEC>`"]
pub type FIFORD = crate::Reg<fiford::FIFORD_SPEC>;
#[doc = "FIFO read data."]
pub mod fiford;
#[doc = "FIFORD48H register accessor: an alias for `Reg<FIFORD48H_SPEC>`"]
pub type FIFORD48H = crate::Reg<fiford48h::FIFORD48H_SPEC>;
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48h;
#[doc = "FIFORDNOPOP register accessor: an alias for `Reg<FIFORDNOPOP_SPEC>`"]
pub type FIFORDNOPOP = crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>;
#[doc = "FIFO data read with no FIFO pop."]
pub mod fifordnopop;
#[doc = "FIFORD48HNOPOP register accessor: an alias for `Reg<FIFORD48HNOPOP_SPEC>`"]
pub type FIFORD48HNOPOP = crate::Reg<fiford48hnopop::FIFORD48HNOPOP_SPEC>;
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48hnopop;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "I2S Module identification"]
pub mod id;
