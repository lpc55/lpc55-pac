#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input mux register for SCT0 input"]
    pub sct0_inmux: [crate::Reg<sct0_inmux::SCT0_INMUX_SPEC>; 7],
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Capture select registers for TIMER0 inputs"]
    pub timer0captsel: [crate::Reg<timer0captsel::TIMER0CAPTSEL_SPEC>; 4],
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - Capture select registers for TIMER1 inputs"]
    pub timer1captsel: [crate::Reg<timer1captsel::TIMER1CAPTSEL_SPEC>; 4],
    _reserved3: [u8; 16usize],
    #[doc = "0x60 - Capture select registers for TIMER2 inputs"]
    pub timer2captsel: [crate::Reg<timer2captsel::TIMER2CAPTSEL_SPEC>; 4],
    _reserved4: [u8; 80usize],
    #[doc = "0xc0 - Pin interrupt select register"]
    pub pintsel: [crate::Reg<pintsel::PINTSEL_SPEC>; 8],
    #[doc = "0xe0 - Trigger select register for DMA0 channel"]
    pub dma0_itrig_inmux: [crate::Reg<dma0_itrig_inmux::DMA0_ITRIG_INMUX_SPEC>; 23],
    _reserved6: [u8; 36usize],
    #[doc = "0x160 - DMA0 output trigger selection to become DMA0 trigger"]
    pub dma0_otrig_inmux: [crate::Reg<dma0_otrig_inmux::DMA0_OTRIG_INMUX_SPEC>; 4],
    _reserved7: [u8; 16usize],
    #[doc = "0x180 - Selection for frequency measurement reference clock"]
    pub freqmeas_ref: crate::Reg<freqmeas_ref::FREQMEAS_REF_SPEC>,
    #[doc = "0x184 - Selection for frequency measurement target clock"]
    pub freqmeas_target: crate::Reg<freqmeas_target::FREQMEAS_TARGET_SPEC>,
    _reserved9: [u8; 24usize],
    #[doc = "0x1a0 - Capture select registers for TIMER3 inputs"]
    pub timer3captsel: [crate::Reg<timer3captsel::TIMER3CAPTSEL_SPEC>; 4],
    _reserved10: [u8; 16usize],
    #[doc = "0x1c0 - Capture select registers for TIMER4 inputs"]
    pub timer4captsel: [crate::Reg<timer4captsel::TIMER4CAPTSEL_SPEC>; 4],
    _reserved11: [u8; 16usize],
    #[doc = "0x1e0 - Pin interrupt secure select register"]
    pub pintsecsel: [crate::Reg<pintsecsel::PINTSECSEL_SPEC>; 2],
    _reserved12: [u8; 24usize],
    #[doc = "0x200 - Trigger select register for DMA1 channel"]
    pub dma1_itrig_inmux: [crate::Reg<dma1_itrig_inmux::DMA1_ITRIG_INMUX_SPEC>; 10],
    _reserved13: [u8; 24usize],
    #[doc = "0x240 - DMA1 output trigger selection to become DMA1 trigger"]
    pub dma1_otrig_inmux: [crate::Reg<dma1_otrig_inmux::DMA1_OTRIG_INMUX_SPEC>; 4],
    _reserved14: [u8; 1264usize],
    #[doc = "0x740 - Enable DMA0 requests"]
    pub dma0_req_ena: crate::Reg<dma0_req_ena::DMA0_REQ_ENA_SPEC>,
    _reserved15: [u8; 4usize],
    #[doc = "0x748 - Set one or several bits in DMA0_REQ_ENA register"]
    pub dma0_req_ena_set: crate::Reg<dma0_req_ena_set::DMA0_REQ_ENA_SET_SPEC>,
    _reserved16: [u8; 4usize],
    #[doc = "0x750 - Clear one or several bits in DMA0_REQ_ENA register"]
    pub dma0_req_ena_clr: crate::Reg<dma0_req_ena_clr::DMA0_REQ_ENA_CLR_SPEC>,
    _reserved17: [u8; 12usize],
    #[doc = "0x760 - Enable DMA1 requests"]
    pub dma1_req_ena: crate::Reg<dma1_req_ena::DMA1_REQ_ENA_SPEC>,
    _reserved18: [u8; 4usize],
    #[doc = "0x768 - Set one or several bits in DMA1_REQ_ENA register"]
    pub dma1_req_ena_set: crate::Reg<dma1_req_ena_set::DMA1_REQ_ENA_SET_SPEC>,
    _reserved19: [u8; 4usize],
    #[doc = "0x770 - Clear one or several bits in DMA1_REQ_ENA register"]
    pub dma1_req_ena_clr: crate::Reg<dma1_req_ena_clr::DMA1_REQ_ENA_CLR_SPEC>,
    _reserved20: [u8; 12usize],
    #[doc = "0x780 - Enable DMA0 triggers"]
    pub dma0_itrig_ena: crate::Reg<dma0_itrig_ena::DMA0_ITRIG_ENA_SPEC>,
    _reserved21: [u8; 4usize],
    #[doc = "0x788 - Set one or several bits in DMA0_ITRIG_ENA register"]
    pub dma0_itrig_ena_set: crate::Reg<dma0_itrig_ena_set::DMA0_ITRIG_ENA_SET_SPEC>,
    _reserved22: [u8; 4usize],
    #[doc = "0x790 - Clear one or several bits in DMA0_ITRIG_ENA register"]
    pub dma0_itrig_ena_clr: crate::Reg<dma0_itrig_ena_clr::DMA0_ITRIG_ENA_CLR_SPEC>,
    _reserved23: [u8; 12usize],
    #[doc = "0x7a0 - Enable DMA1 triggers"]
    pub dma1_itrig_ena: crate::Reg<dma1_itrig_ena::DMA1_ITRIG_ENA_SPEC>,
    _reserved24: [u8; 4usize],
    #[doc = "0x7a8 - Set one or several bits in DMA1_ITRIG_ENA register"]
    pub dma1_itrig_ena_set: crate::Reg<dma1_itrig_ena_set::DMA1_ITRIG_ENA_SET_SPEC>,
    _reserved25: [u8; 4usize],
    #[doc = "0x7b0 - Clear one or several bits in DMA1_ITRIG_ENA register"]
    pub dma1_itrig_ena_clr: crate::Reg<dma1_itrig_ena_clr::DMA1_ITRIG_ENA_CLR_SPEC>,
}
#[doc = "SCT0_INMUX register accessor: an alias for `Reg<SCT0_INMUX_SPEC>`"]
pub type SCT0_INMUX = crate::Reg<sct0_inmux::SCT0_INMUX_SPEC>;
#[doc = "Input mux register for SCT0 input"]
pub mod sct0_inmux;
#[doc = "TIMER0CAPTSEL register accessor: an alias for `Reg<TIMER0CAPTSEL_SPEC>`"]
pub type TIMER0CAPTSEL = crate::Reg<timer0captsel::TIMER0CAPTSEL_SPEC>;
#[doc = "Capture select registers for TIMER0 inputs"]
pub mod timer0captsel;
#[doc = "TIMER1CAPTSEL register accessor: an alias for `Reg<TIMER1CAPTSEL_SPEC>`"]
pub type TIMER1CAPTSEL = crate::Reg<timer1captsel::TIMER1CAPTSEL_SPEC>;
#[doc = "Capture select registers for TIMER1 inputs"]
pub mod timer1captsel;
#[doc = "TIMER2CAPTSEL register accessor: an alias for `Reg<TIMER2CAPTSEL_SPEC>`"]
pub type TIMER2CAPTSEL = crate::Reg<timer2captsel::TIMER2CAPTSEL_SPEC>;
#[doc = "Capture select registers for TIMER2 inputs"]
pub mod timer2captsel;
#[doc = "PINTSEL register accessor: an alias for `Reg<PINTSEL_SPEC>`"]
pub type PINTSEL = crate::Reg<pintsel::PINTSEL_SPEC>;
#[doc = "Pin interrupt select register"]
pub mod pintsel;
#[doc = "DMA0_ITRIG_INMUX register accessor: an alias for `Reg<DMA0_ITRIG_INMUX_SPEC>`"]
pub type DMA0_ITRIG_INMUX = crate::Reg<dma0_itrig_inmux::DMA0_ITRIG_INMUX_SPEC>;
#[doc = "Trigger select register for DMA0 channel"]
pub mod dma0_itrig_inmux;
#[doc = "DMA0_OTRIG_INMUX register accessor: an alias for `Reg<DMA0_OTRIG_INMUX_SPEC>`"]
pub type DMA0_OTRIG_INMUX = crate::Reg<dma0_otrig_inmux::DMA0_OTRIG_INMUX_SPEC>;
#[doc = "DMA0 output trigger selection to become DMA0 trigger"]
pub mod dma0_otrig_inmux;
#[doc = "FREQMEAS_REF register accessor: an alias for `Reg<FREQMEAS_REF_SPEC>`"]
pub type FREQMEAS_REF = crate::Reg<freqmeas_ref::FREQMEAS_REF_SPEC>;
#[doc = "Selection for frequency measurement reference clock"]
pub mod freqmeas_ref;
#[doc = "FREQMEAS_TARGET register accessor: an alias for `Reg<FREQMEAS_TARGET_SPEC>`"]
pub type FREQMEAS_TARGET = crate::Reg<freqmeas_target::FREQMEAS_TARGET_SPEC>;
#[doc = "Selection for frequency measurement target clock"]
pub mod freqmeas_target;
#[doc = "TIMER3CAPTSEL register accessor: an alias for `Reg<TIMER3CAPTSEL_SPEC>`"]
pub type TIMER3CAPTSEL = crate::Reg<timer3captsel::TIMER3CAPTSEL_SPEC>;
#[doc = "Capture select registers for TIMER3 inputs"]
pub mod timer3captsel;
#[doc = "TIMER4CAPTSEL register accessor: an alias for `Reg<TIMER4CAPTSEL_SPEC>`"]
pub type TIMER4CAPTSEL = crate::Reg<timer4captsel::TIMER4CAPTSEL_SPEC>;
#[doc = "Capture select registers for TIMER4 inputs"]
pub mod timer4captsel;
#[doc = "PINTSECSEL register accessor: an alias for `Reg<PINTSECSEL_SPEC>`"]
pub type PINTSECSEL = crate::Reg<pintsecsel::PINTSECSEL_SPEC>;
#[doc = "Pin interrupt secure select register"]
pub mod pintsecsel;
#[doc = "DMA1_ITRIG_INMUX register accessor: an alias for `Reg<DMA1_ITRIG_INMUX_SPEC>`"]
pub type DMA1_ITRIG_INMUX = crate::Reg<dma1_itrig_inmux::DMA1_ITRIG_INMUX_SPEC>;
#[doc = "Trigger select register for DMA1 channel"]
pub mod dma1_itrig_inmux;
#[doc = "DMA1_OTRIG_INMUX register accessor: an alias for `Reg<DMA1_OTRIG_INMUX_SPEC>`"]
pub type DMA1_OTRIG_INMUX = crate::Reg<dma1_otrig_inmux::DMA1_OTRIG_INMUX_SPEC>;
#[doc = "DMA1 output trigger selection to become DMA1 trigger"]
pub mod dma1_otrig_inmux;
#[doc = "DMA0_REQ_ENA register accessor: an alias for `Reg<DMA0_REQ_ENA_SPEC>`"]
pub type DMA0_REQ_ENA = crate::Reg<dma0_req_ena::DMA0_REQ_ENA_SPEC>;
#[doc = "Enable DMA0 requests"]
pub mod dma0_req_ena;
#[doc = "DMA0_REQ_ENA_SET register accessor: an alias for `Reg<DMA0_REQ_ENA_SET_SPEC>`"]
pub type DMA0_REQ_ENA_SET = crate::Reg<dma0_req_ena_set::DMA0_REQ_ENA_SET_SPEC>;
#[doc = "Set one or several bits in DMA0_REQ_ENA register"]
pub mod dma0_req_ena_set;
#[doc = "DMA0_REQ_ENA_CLR register accessor: an alias for `Reg<DMA0_REQ_ENA_CLR_SPEC>`"]
pub type DMA0_REQ_ENA_CLR = crate::Reg<dma0_req_ena_clr::DMA0_REQ_ENA_CLR_SPEC>;
#[doc = "Clear one or several bits in DMA0_REQ_ENA register"]
pub mod dma0_req_ena_clr;
#[doc = "DMA1_REQ_ENA register accessor: an alias for `Reg<DMA1_REQ_ENA_SPEC>`"]
pub type DMA1_REQ_ENA = crate::Reg<dma1_req_ena::DMA1_REQ_ENA_SPEC>;
#[doc = "Enable DMA1 requests"]
pub mod dma1_req_ena;
#[doc = "DMA1_REQ_ENA_SET register accessor: an alias for `Reg<DMA1_REQ_ENA_SET_SPEC>`"]
pub type DMA1_REQ_ENA_SET = crate::Reg<dma1_req_ena_set::DMA1_REQ_ENA_SET_SPEC>;
#[doc = "Set one or several bits in DMA1_REQ_ENA register"]
pub mod dma1_req_ena_set;
#[doc = "DMA1_REQ_ENA_CLR register accessor: an alias for `Reg<DMA1_REQ_ENA_CLR_SPEC>`"]
pub type DMA1_REQ_ENA_CLR = crate::Reg<dma1_req_ena_clr::DMA1_REQ_ENA_CLR_SPEC>;
#[doc = "Clear one or several bits in DMA1_REQ_ENA register"]
pub mod dma1_req_ena_clr;
#[doc = "DMA0_ITRIG_ENA register accessor: an alias for `Reg<DMA0_ITRIG_ENA_SPEC>`"]
pub type DMA0_ITRIG_ENA = crate::Reg<dma0_itrig_ena::DMA0_ITRIG_ENA_SPEC>;
#[doc = "Enable DMA0 triggers"]
pub mod dma0_itrig_ena;
#[doc = "DMA0_ITRIG_ENA_SET register accessor: an alias for `Reg<DMA0_ITRIG_ENA_SET_SPEC>`"]
pub type DMA0_ITRIG_ENA_SET = crate::Reg<dma0_itrig_ena_set::DMA0_ITRIG_ENA_SET_SPEC>;
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register"]
pub mod dma0_itrig_ena_set;
#[doc = "DMA0_ITRIG_ENA_CLR register accessor: an alias for `Reg<DMA0_ITRIG_ENA_CLR_SPEC>`"]
pub type DMA0_ITRIG_ENA_CLR = crate::Reg<dma0_itrig_ena_clr::DMA0_ITRIG_ENA_CLR_SPEC>;
#[doc = "Clear one or several bits in DMA0_ITRIG_ENA register"]
pub mod dma0_itrig_ena_clr;
#[doc = "DMA1_ITRIG_ENA register accessor: an alias for `Reg<DMA1_ITRIG_ENA_SPEC>`"]
pub type DMA1_ITRIG_ENA = crate::Reg<dma1_itrig_ena::DMA1_ITRIG_ENA_SPEC>;
#[doc = "Enable DMA1 triggers"]
pub mod dma1_itrig_ena;
#[doc = "DMA1_ITRIG_ENA_SET register accessor: an alias for `Reg<DMA1_ITRIG_ENA_SET_SPEC>`"]
pub type DMA1_ITRIG_ENA_SET = crate::Reg<dma1_itrig_ena_set::DMA1_ITRIG_ENA_SET_SPEC>;
#[doc = "Set one or several bits in DMA1_ITRIG_ENA register"]
pub mod dma1_itrig_ena_set;
#[doc = "DMA1_ITRIG_ENA_CLR register accessor: an alias for `Reg<DMA1_ITRIG_ENA_CLR_SPEC>`"]
pub type DMA1_ITRIG_ENA_CLR = crate::Reg<dma1_itrig_ena_clr::DMA1_ITRIG_ENA_CLR_SPEC>;
#[doc = "Clear one or several bits in DMA1_ITRIG_ENA register"]
pub mod dma1_itrig_ena_clr;
