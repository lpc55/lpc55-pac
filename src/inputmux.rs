#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input mux register for SCT0 input"]
    pub sct0_inmux: [SCT0_INMUX; 7],
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Capture select registers for TIMER0 inputs"]
    pub timer0captsel: [TIMER0CAPTSEL; 4],
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Capture select registers for TIMER1 inputs"]
    pub timer1captsel: [TIMER1CAPTSEL; 4],
    _reserved2: [u8; 16usize],
    #[doc = "0x60 - Capture select registers for TIMER2 inputs"]
    pub timer2captsel: [TIMER2CAPTSEL; 4],
    _reserved3: [u8; 80usize],
    #[doc = "0xc0 - Pin interrupt select register"]
    pub pintsel: [PINTSEL; 8],
    #[doc = "0xe0 - Trigger select register for DMA0 channel"]
    pub dma0_itrig_inmux: [DMA0_ITRIG_INMUX; 23],
    _reserved4: [u8; 36usize],
    #[doc = "0x160 - DMA0 output trigger selection to become DMA0 trigger"]
    pub dma0_otrig_inmux: [DMA0_OTRIG_INMUX; 4],
    _reserved5: [u8; 16usize],
    #[doc = "0x180 - Selection for frequency measurement reference clock"]
    pub freqmeas_ref: FREQMEAS_REF,
    #[doc = "0x184 - Selection for frequency measurement target clock"]
    pub freqmeas_target: FREQMEAS_TARGET,
    _reserved6: [u8; 24usize],
    #[doc = "0x1a0 - Capture select registers for TIMER3 inputs"]
    pub timer3captsel: [TIMER3CAPTSEL; 4],
    _reserved7: [u8; 16usize],
    #[doc = "0x1c0 - Capture select registers for TIMER4 inputs"]
    pub timer4captsel: [TIMER4CAPTSEL; 4],
    _reserved8: [u8; 16usize],
    #[doc = "0x1e0 - Pin interrupt secure select register"]
    pub pintsecsel: [PINTSECSEL; 2],
    _reserved9: [u8; 24usize],
    #[doc = "0x200 - Trigger select register for DMA1 channel"]
    pub dma1_itrig_inmux: [DMA1_ITRIG_INMUX; 10],
    _reserved10: [u8; 24usize],
    #[doc = "0x240 - DMA1 output trigger selection to become DMA1 trigger"]
    pub dma1_otrig_inmux: [DMA1_OTRIG_INMUX; 4],
    _reserved11: [u8; 1264usize],
    #[doc = "0x740 - Enable DMA0 requests"]
    pub dma0_req_ena: DMA0_REQ_ENA,
    _reserved12: [u8; 4usize],
    #[doc = "0x748 - Set one or several bits in DMA0_REQ_ENA register"]
    pub dma0_req_ena_set: DMA0_REQ_ENA_SET,
    _reserved13: [u8; 4usize],
    #[doc = "0x750 - Clear one or several bits in DMA0_REQ_ENA register"]
    pub dma0_req_ena_clr: DMA0_REQ_ENA_CLR,
    _reserved14: [u8; 12usize],
    #[doc = "0x760 - Enable DMA1 requests"]
    pub dma1_req_ena: DMA1_REQ_ENA,
    _reserved15: [u8; 4usize],
    #[doc = "0x768 - Set one or several bits in DMA1_REQ_ENA register"]
    pub dma1_req_ena_set: DMA1_REQ_ENA_SET,
    _reserved16: [u8; 4usize],
    #[doc = "0x770 - Clear one or several bits in DMA1_REQ_ENA register"]
    pub dma1_req_ena_clr: DMA1_REQ_ENA_CLR,
    _reserved17: [u8; 12usize],
    #[doc = "0x780 - Enable DMA0 triggers"]
    pub dma0_itrig_ena: DMA0_ITRIG_ENA,
    _reserved18: [u8; 4usize],
    #[doc = "0x788 - Set one or several bits in DMA0_ITRIG_ENA register"]
    pub dma0_itrig_ena_set: DMA0_ITRIG_ENA_SET,
    _reserved19: [u8; 4usize],
    #[doc = "0x790 - Clear one or several bits in DMA0_ITRIG_ENA register"]
    pub dma0_itrig_ena_clr: DMA0_ITRIG_ENA_CLR,
    _reserved20: [u8; 12usize],
    #[doc = "0x7a0 - Enable DMA1 triggers"]
    pub dma1_itrig_ena: DMA1_ITRIG_ENA,
    _reserved21: [u8; 4usize],
    #[doc = "0x7a8 - Set one or several bits in DMA1_ITRIG_ENA register"]
    pub dma1_itrig_ena_set: DMA1_ITRIG_ENA_SET,
    _reserved22: [u8; 4usize],
    #[doc = "0x7b0 - Clear one or several bits in DMA1_ITRIG_ENA register"]
    pub dma1_itrig_ena_clr: DMA1_ITRIG_ENA_CLR,
}
#[doc = "Input mux register for SCT0 input"]
pub struct SCT0_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input mux register for SCT0 input"]
pub mod sct0_inmux;
#[doc = "Capture select registers for TIMER0 inputs"]
pub struct TIMER0CAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture select registers for TIMER0 inputs"]
pub mod timer0captsel;
#[doc = "Capture select registers for TIMER1 inputs"]
pub struct TIMER1CAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture select registers for TIMER1 inputs"]
pub mod timer1captsel;
#[doc = "Capture select registers for TIMER2 inputs"]
pub struct TIMER2CAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture select registers for TIMER2 inputs"]
pub mod timer2captsel;
#[doc = "Pin interrupt select register"]
pub struct PINTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt select register"]
pub mod pintsel;
#[doc = "Trigger select register for DMA0 channel"]
pub struct DMA0_ITRIG_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger select register for DMA0 channel"]
pub mod dma0_itrig_inmux;
#[doc = "DMA0 output trigger selection to become DMA0 trigger"]
pub struct DMA0_OTRIG_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA0 output trigger selection to become DMA0 trigger"]
pub mod dma0_otrig_inmux;
#[doc = "Selection for frequency measurement reference clock"]
pub struct FREQMEAS_REF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selection for frequency measurement reference clock"]
pub mod freqmeas_ref;
#[doc = "Selection for frequency measurement target clock"]
pub struct FREQMEAS_TARGET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selection for frequency measurement target clock"]
pub mod freqmeas_target;
#[doc = "Capture select registers for TIMER3 inputs"]
pub struct TIMER3CAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture select registers for TIMER3 inputs"]
pub mod timer3captsel;
#[doc = "Capture select registers for TIMER4 inputs"]
pub struct TIMER4CAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture select registers for TIMER4 inputs"]
pub mod timer4captsel;
#[doc = "Pin interrupt secure select register"]
pub struct PINTSECSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt secure select register"]
pub mod pintsecsel;
#[doc = "Trigger select register for DMA1 channel"]
pub struct DMA1_ITRIG_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger select register for DMA1 channel"]
pub mod dma1_itrig_inmux;
#[doc = "DMA1 output trigger selection to become DMA1 trigger"]
pub struct DMA1_OTRIG_INMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA1 output trigger selection to become DMA1 trigger"]
pub mod dma1_otrig_inmux;
#[doc = "Enable DMA0 requests"]
pub struct DMA0_REQ_ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable DMA0 requests"]
pub mod dma0_req_ena;
#[doc = "Set one or several bits in DMA0_REQ_ENA register"]
pub struct DMA0_REQ_ENA_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set one or several bits in DMA0_REQ_ENA register"]
pub mod dma0_req_ena_set;
#[doc = "Clear one or several bits in DMA0_REQ_ENA register"]
pub struct DMA0_REQ_ENA_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear one or several bits in DMA0_REQ_ENA register"]
pub mod dma0_req_ena_clr;
#[doc = "Enable DMA1 requests"]
pub struct DMA1_REQ_ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable DMA1 requests"]
pub mod dma1_req_ena;
#[doc = "Set one or several bits in DMA1_REQ_ENA register"]
pub struct DMA1_REQ_ENA_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set one or several bits in DMA1_REQ_ENA register"]
pub mod dma1_req_ena_set;
#[doc = "Clear one or several bits in DMA1_REQ_ENA register"]
pub struct DMA1_REQ_ENA_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear one or several bits in DMA1_REQ_ENA register"]
pub mod dma1_req_ena_clr;
#[doc = "Enable DMA0 triggers"]
pub struct DMA0_ITRIG_ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable DMA0 triggers"]
pub mod dma0_itrig_ena;
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register"]
pub struct DMA0_ITRIG_ENA_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register"]
pub mod dma0_itrig_ena_set;
#[doc = "Clear one or several bits in DMA0_ITRIG_ENA register"]
pub struct DMA0_ITRIG_ENA_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear one or several bits in DMA0_ITRIG_ENA register"]
pub mod dma0_itrig_ena_clr;
#[doc = "Enable DMA1 triggers"]
pub struct DMA1_ITRIG_ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable DMA1 triggers"]
pub mod dma1_itrig_ena;
#[doc = "Set one or several bits in DMA1_ITRIG_ENA register"]
pub struct DMA1_ITRIG_ENA_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set one or several bits in DMA1_ITRIG_ENA register"]
pub mod dma1_itrig_ena_set;
#[doc = "Clear one or several bits in DMA1_ITRIG_ENA register"]
pub struct DMA1_ITRIG_ENA_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear one or several bits in DMA1_ITRIG_ENA register"]
pub mod dma1_itrig_ena_clr;
