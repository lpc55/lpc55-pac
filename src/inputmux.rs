#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input mux register for SCT0 input"]
    pub sct0_inmux: [SCT0_INMUX; 7],
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Capture select registers for TIMER0 inputs"]
    pub timer0captsel: [TIMER0CAPTSEL; 4],
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - Capture select registers for TIMER1 inputs"]
    pub timer1captsel: [TIMER1CAPTSEL; 4],
    _reserved3: [u8; 16usize],
    #[doc = "0x60 - Capture select registers for TIMER2 inputs"]
    pub timer2captsel: [TIMER2CAPTSEL; 4],
    _reserved4: [u8; 80usize],
    #[doc = "0xc0 - Pin interrupt select register"]
    pub pintsel: [PINTSEL; 8],
    #[doc = "0xe0 - Trigger select register for DMA0 channel"]
    pub dma0_itrig_inmux: [DMA0_ITRIG_INMUX; 23],
    _reserved6: [u8; 36usize],
    #[doc = "0x160 - DMA0 output trigger selection to become DMA0 trigger"]
    pub dma0_otrig_inmux: [DMA0_OTRIG_INMUX; 4],
    _reserved7: [u8; 16usize],
    #[doc = "0x180 - Selection for frequency measurement reference clock"]
    pub freqmeas_ref: FREQMEAS_REF,
    #[doc = "0x184 - Selection for frequency measurement target clock"]
    pub freqmeas_target: FREQMEAS_TARGET,
    _reserved9: [u8; 24usize],
    #[doc = "0x1a0 - Capture select registers for TIMER3 inputs"]
    pub timer3captsel: [TIMER3CAPTSEL; 4],
    _reserved10: [u8; 16usize],
    #[doc = "0x1c0 - Capture select registers for TIMER4 inputs"]
    pub timer4captsel: [TIMER4CAPTSEL; 4],
    _reserved11: [u8; 16usize],
    #[doc = "0x1e0 - Pin interrupt secure select register"]
    pub pintsecsel: [PINTSECSEL; 2],
    _reserved12: [u8; 24usize],
    #[doc = "0x200 - Trigger select register for DMA1 channel"]
    pub dma1_itrig_inmux: [DMA1_ITRIG_INMUX; 10],
    _reserved13: [u8; 24usize],
    #[doc = "0x240 - DMA1 output trigger selection to become DMA1 trigger"]
    pub dma1_otrig_inmux: [DMA1_OTRIG_INMUX; 4],
    _reserved14: [u8; 1264usize],
    #[doc = "0x740 - Enable DMA0 requests"]
    pub dma0_req_ena: DMA0_REQ_ENA,
    _reserved15: [u8; 4usize],
    #[doc = "0x748 - Set one or several bits in DMA0_REQ_ENA register"]
    pub dma0_req_ena_set: DMA0_REQ_ENA_SET,
    _reserved16: [u8; 4usize],
    #[doc = "0x750 - Clear one or several bits in DMA0_REQ_ENA register"]
    pub dma0_req_ena_clr: DMA0_REQ_ENA_CLR,
    _reserved17: [u8; 12usize],
    #[doc = "0x760 - Enable DMA1 requests"]
    pub dma1_req_ena: DMA1_REQ_ENA,
    _reserved18: [u8; 4usize],
    #[doc = "0x768 - Set one or several bits in DMA1_REQ_ENA register"]
    pub dma1_req_ena_set: DMA1_REQ_ENA_SET,
    _reserved19: [u8; 4usize],
    #[doc = "0x770 - Clear one or several bits in DMA1_REQ_ENA register"]
    pub dma1_req_ena_clr: DMA1_REQ_ENA_CLR,
    _reserved20: [u8; 12usize],
    #[doc = "0x780 - Enable DMA0 triggers"]
    pub dma0_itrig_ena: DMA0_ITRIG_ENA,
    _reserved21: [u8; 4usize],
    #[doc = "0x788 - Set one or several bits in DMA0_ITRIG_ENA register"]
    pub dma0_itrig_ena_set: DMA0_ITRIG_ENA_SET,
    _reserved22: [u8; 4usize],
    #[doc = "0x790 - Clear one or several bits in DMA0_ITRIG_ENA register"]
    pub dma0_itrig_ena_clr: DMA0_ITRIG_ENA_CLR,
    _reserved23: [u8; 12usize],
    #[doc = "0x7a0 - Enable DMA1 triggers"]
    pub dma1_itrig_ena: DMA1_ITRIG_ENA,
    _reserved24: [u8; 4usize],
    #[doc = "0x7a8 - Set one or several bits in DMA1_ITRIG_ENA register"]
    pub dma1_itrig_ena_set: DMA1_ITRIG_ENA_SET,
    _reserved25: [u8; 4usize],
    #[doc = "0x7b0 - Clear one or several bits in DMA1_ITRIG_ENA register"]
    pub dma1_itrig_ena_clr: DMA1_ITRIG_ENA_CLR,
}
#[doc = "Input mux register for SCT0 input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sct0_inmux](sct0_inmux) module"]
pub type SCT0_INMUX = crate::Reg<u32, _SCT0_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCT0_INMUX;
#[doc = "`read()` method returns [sct0_inmux::R](sct0_inmux::R) reader structure"]
impl crate::Readable for SCT0_INMUX {}
#[doc = "`write(|w| ..)` method takes [sct0_inmux::W](sct0_inmux::W) writer structure"]
impl crate::Writable for SCT0_INMUX {}
#[doc = "Input mux register for SCT0 input"]
pub mod sct0_inmux;
#[doc = "Capture select registers for TIMER0 inputs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0captsel](timer0captsel) module"]
pub type TIMER0CAPTSEL = crate::Reg<u32, _TIMER0CAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER0CAPTSEL;
#[doc = "`read()` method returns [timer0captsel::R](timer0captsel::R) reader structure"]
impl crate::Readable for TIMER0CAPTSEL {}
#[doc = "`write(|w| ..)` method takes [timer0captsel::W](timer0captsel::W) writer structure"]
impl crate::Writable for TIMER0CAPTSEL {}
#[doc = "Capture select registers for TIMER0 inputs"]
pub mod timer0captsel;
#[doc = "Capture select registers for TIMER1 inputs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1captsel](timer1captsel) module"]
pub type TIMER1CAPTSEL = crate::Reg<u32, _TIMER1CAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1CAPTSEL;
#[doc = "`read()` method returns [timer1captsel::R](timer1captsel::R) reader structure"]
impl crate::Readable for TIMER1CAPTSEL {}
#[doc = "`write(|w| ..)` method takes [timer1captsel::W](timer1captsel::W) writer structure"]
impl crate::Writable for TIMER1CAPTSEL {}
#[doc = "Capture select registers for TIMER1 inputs"]
pub mod timer1captsel;
#[doc = "Capture select registers for TIMER2 inputs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2captsel](timer2captsel) module"]
pub type TIMER2CAPTSEL = crate::Reg<u32, _TIMER2CAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2CAPTSEL;
#[doc = "`read()` method returns [timer2captsel::R](timer2captsel::R) reader structure"]
impl crate::Readable for TIMER2CAPTSEL {}
#[doc = "`write(|w| ..)` method takes [timer2captsel::W](timer2captsel::W) writer structure"]
impl crate::Writable for TIMER2CAPTSEL {}
#[doc = "Capture select registers for TIMER2 inputs"]
pub mod timer2captsel;
#[doc = "Pin interrupt select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsel](pintsel) module"]
pub type PINTSEL = crate::Reg<u32, _PINTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTSEL;
#[doc = "`read()` method returns [pintsel::R](pintsel::R) reader structure"]
impl crate::Readable for PINTSEL {}
#[doc = "`write(|w| ..)` method takes [pintsel::W](pintsel::W) writer structure"]
impl crate::Writable for PINTSEL {}
#[doc = "Pin interrupt select register"]
pub mod pintsel;
#[doc = "Trigger select register for DMA0 channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_itrig_inmux](dma0_itrig_inmux) module"]
pub type DMA0_ITRIG_INMUX = crate::Reg<u32, _DMA0_ITRIG_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0_ITRIG_INMUX;
#[doc = "`read()` method returns [dma0_itrig_inmux::R](dma0_itrig_inmux::R) reader structure"]
impl crate::Readable for DMA0_ITRIG_INMUX {}
#[doc = "`write(|w| ..)` method takes [dma0_itrig_inmux::W](dma0_itrig_inmux::W) writer structure"]
impl crate::Writable for DMA0_ITRIG_INMUX {}
#[doc = "Trigger select register for DMA0 channel"]
pub mod dma0_itrig_inmux;
#[doc = "DMA0 output trigger selection to become DMA0 trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_otrig_inmux](dma0_otrig_inmux) module"]
pub type DMA0_OTRIG_INMUX = crate::Reg<u32, _DMA0_OTRIG_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0_OTRIG_INMUX;
#[doc = "`read()` method returns [dma0_otrig_inmux::R](dma0_otrig_inmux::R) reader structure"]
impl crate::Readable for DMA0_OTRIG_INMUX {}
#[doc = "`write(|w| ..)` method takes [dma0_otrig_inmux::W](dma0_otrig_inmux::W) writer structure"]
impl crate::Writable for DMA0_OTRIG_INMUX {}
#[doc = "DMA0 output trigger selection to become DMA0 trigger"]
pub mod dma0_otrig_inmux;
#[doc = "Selection for frequency measurement reference clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqmeas_ref](freqmeas_ref) module"]
pub type FREQMEAS_REF = crate::Reg<u32, _FREQMEAS_REF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQMEAS_REF;
#[doc = "`read()` method returns [freqmeas_ref::R](freqmeas_ref::R) reader structure"]
impl crate::Readable for FREQMEAS_REF {}
#[doc = "`write(|w| ..)` method takes [freqmeas_ref::W](freqmeas_ref::W) writer structure"]
impl crate::Writable for FREQMEAS_REF {}
#[doc = "Selection for frequency measurement reference clock"]
pub mod freqmeas_ref;
#[doc = "Selection for frequency measurement target clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqmeas_target](freqmeas_target) module"]
pub type FREQMEAS_TARGET = crate::Reg<u32, _FREQMEAS_TARGET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQMEAS_TARGET;
#[doc = "`read()` method returns [freqmeas_target::R](freqmeas_target::R) reader structure"]
impl crate::Readable for FREQMEAS_TARGET {}
#[doc = "`write(|w| ..)` method takes [freqmeas_target::W](freqmeas_target::W) writer structure"]
impl crate::Writable for FREQMEAS_TARGET {}
#[doc = "Selection for frequency measurement target clock"]
pub mod freqmeas_target;
#[doc = "Capture select registers for TIMER3 inputs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer3captsel](timer3captsel) module"]
pub type TIMER3CAPTSEL = crate::Reg<u32, _TIMER3CAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER3CAPTSEL;
#[doc = "`read()` method returns [timer3captsel::R](timer3captsel::R) reader structure"]
impl crate::Readable for TIMER3CAPTSEL {}
#[doc = "`write(|w| ..)` method takes [timer3captsel::W](timer3captsel::W) writer structure"]
impl crate::Writable for TIMER3CAPTSEL {}
#[doc = "Capture select registers for TIMER3 inputs"]
pub mod timer3captsel;
#[doc = "Capture select registers for TIMER4 inputs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer4captsel](timer4captsel) module"]
pub type TIMER4CAPTSEL = crate::Reg<u32, _TIMER4CAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER4CAPTSEL;
#[doc = "`read()` method returns [timer4captsel::R](timer4captsel::R) reader structure"]
impl crate::Readable for TIMER4CAPTSEL {}
#[doc = "`write(|w| ..)` method takes [timer4captsel::W](timer4captsel::W) writer structure"]
impl crate::Writable for TIMER4CAPTSEL {}
#[doc = "Capture select registers for TIMER4 inputs"]
pub mod timer4captsel;
#[doc = "Pin interrupt secure select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsecsel](pintsecsel) module"]
pub type PINTSECSEL = crate::Reg<u32, _PINTSECSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTSECSEL;
#[doc = "`read()` method returns [pintsecsel::R](pintsecsel::R) reader structure"]
impl crate::Readable for PINTSECSEL {}
#[doc = "`write(|w| ..)` method takes [pintsecsel::W](pintsecsel::W) writer structure"]
impl crate::Writable for PINTSECSEL {}
#[doc = "Pin interrupt secure select register"]
pub mod pintsecsel;
#[doc = "Trigger select register for DMA1 channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_itrig_inmux](dma1_itrig_inmux) module"]
pub type DMA1_ITRIG_INMUX = crate::Reg<u32, _DMA1_ITRIG_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1_ITRIG_INMUX;
#[doc = "`read()` method returns [dma1_itrig_inmux::R](dma1_itrig_inmux::R) reader structure"]
impl crate::Readable for DMA1_ITRIG_INMUX {}
#[doc = "`write(|w| ..)` method takes [dma1_itrig_inmux::W](dma1_itrig_inmux::W) writer structure"]
impl crate::Writable for DMA1_ITRIG_INMUX {}
#[doc = "Trigger select register for DMA1 channel"]
pub mod dma1_itrig_inmux;
#[doc = "DMA1 output trigger selection to become DMA1 trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_otrig_inmux](dma1_otrig_inmux) module"]
pub type DMA1_OTRIG_INMUX = crate::Reg<u32, _DMA1_OTRIG_INMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1_OTRIG_INMUX;
#[doc = "`read()` method returns [dma1_otrig_inmux::R](dma1_otrig_inmux::R) reader structure"]
impl crate::Readable for DMA1_OTRIG_INMUX {}
#[doc = "`write(|w| ..)` method takes [dma1_otrig_inmux::W](dma1_otrig_inmux::W) writer structure"]
impl crate::Writable for DMA1_OTRIG_INMUX {}
#[doc = "DMA1 output trigger selection to become DMA1 trigger"]
pub mod dma1_otrig_inmux;
#[doc = "Enable DMA0 requests\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_req_ena](dma0_req_ena) module"]
pub type DMA0_REQ_ENA = crate::Reg<u32, _DMA0_REQ_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0_REQ_ENA;
#[doc = "`read()` method returns [dma0_req_ena::R](dma0_req_ena::R) reader structure"]
impl crate::Readable for DMA0_REQ_ENA {}
#[doc = "`write(|w| ..)` method takes [dma0_req_ena::W](dma0_req_ena::W) writer structure"]
impl crate::Writable for DMA0_REQ_ENA {}
#[doc = "Enable DMA0 requests"]
pub mod dma0_req_ena;
#[doc = "Set one or several bits in DMA0_REQ_ENA register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_req_ena_set](dma0_req_ena_set) module"]
pub type DMA0_REQ_ENA_SET = crate::Reg<u32, _DMA0_REQ_ENA_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0_REQ_ENA_SET;
#[doc = "`write(|w| ..)` method takes [dma0_req_ena_set::W](dma0_req_ena_set::W) writer structure"]
impl crate::Writable for DMA0_REQ_ENA_SET {}
#[doc = "Set one or several bits in DMA0_REQ_ENA register"]
pub mod dma0_req_ena_set;
#[doc = "Clear one or several bits in DMA0_REQ_ENA register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_req_ena_clr](dma0_req_ena_clr) module"]
pub type DMA0_REQ_ENA_CLR = crate::Reg<u32, _DMA0_REQ_ENA_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0_REQ_ENA_CLR;
#[doc = "`write(|w| ..)` method takes [dma0_req_ena_clr::W](dma0_req_ena_clr::W) writer structure"]
impl crate::Writable for DMA0_REQ_ENA_CLR {}
#[doc = "Clear one or several bits in DMA0_REQ_ENA register"]
pub mod dma0_req_ena_clr;
#[doc = "Enable DMA1 requests\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_req_ena](dma1_req_ena) module"]
pub type DMA1_REQ_ENA = crate::Reg<u32, _DMA1_REQ_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1_REQ_ENA;
#[doc = "`read()` method returns [dma1_req_ena::R](dma1_req_ena::R) reader structure"]
impl crate::Readable for DMA1_REQ_ENA {}
#[doc = "`write(|w| ..)` method takes [dma1_req_ena::W](dma1_req_ena::W) writer structure"]
impl crate::Writable for DMA1_REQ_ENA {}
#[doc = "Enable DMA1 requests"]
pub mod dma1_req_ena;
#[doc = "Set one or several bits in DMA1_REQ_ENA register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_req_ena_set](dma1_req_ena_set) module"]
pub type DMA1_REQ_ENA_SET = crate::Reg<u32, _DMA1_REQ_ENA_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1_REQ_ENA_SET;
#[doc = "`write(|w| ..)` method takes [dma1_req_ena_set::W](dma1_req_ena_set::W) writer structure"]
impl crate::Writable for DMA1_REQ_ENA_SET {}
#[doc = "Set one or several bits in DMA1_REQ_ENA register"]
pub mod dma1_req_ena_set;
#[doc = "Clear one or several bits in DMA1_REQ_ENA register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_req_ena_clr](dma1_req_ena_clr) module"]
pub type DMA1_REQ_ENA_CLR = crate::Reg<u32, _DMA1_REQ_ENA_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1_REQ_ENA_CLR;
#[doc = "`write(|w| ..)` method takes [dma1_req_ena_clr::W](dma1_req_ena_clr::W) writer structure"]
impl crate::Writable for DMA1_REQ_ENA_CLR {}
#[doc = "Clear one or several bits in DMA1_REQ_ENA register"]
pub mod dma1_req_ena_clr;
#[doc = "Enable DMA0 triggers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_itrig_ena](dma0_itrig_ena) module"]
pub type DMA0_ITRIG_ENA = crate::Reg<u32, _DMA0_ITRIG_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0_ITRIG_ENA;
#[doc = "`read()` method returns [dma0_itrig_ena::R](dma0_itrig_ena::R) reader structure"]
impl crate::Readable for DMA0_ITRIG_ENA {}
#[doc = "`write(|w| ..)` method takes [dma0_itrig_ena::W](dma0_itrig_ena::W) writer structure"]
impl crate::Writable for DMA0_ITRIG_ENA {}
#[doc = "Enable DMA0 triggers"]
pub mod dma0_itrig_ena;
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_itrig_ena_set](dma0_itrig_ena_set) module"]
pub type DMA0_ITRIG_ENA_SET = crate::Reg<u32, _DMA0_ITRIG_ENA_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0_ITRIG_ENA_SET;
#[doc = "`write(|w| ..)` method takes [dma0_itrig_ena_set::W](dma0_itrig_ena_set::W) writer structure"]
impl crate::Writable for DMA0_ITRIG_ENA_SET {}
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register"]
pub mod dma0_itrig_ena_set;
#[doc = "Clear one or several bits in DMA0_ITRIG_ENA register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_itrig_ena_clr](dma0_itrig_ena_clr) module"]
pub type DMA0_ITRIG_ENA_CLR = crate::Reg<u32, _DMA0_ITRIG_ENA_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0_ITRIG_ENA_CLR;
#[doc = "`write(|w| ..)` method takes [dma0_itrig_ena_clr::W](dma0_itrig_ena_clr::W) writer structure"]
impl crate::Writable for DMA0_ITRIG_ENA_CLR {}
#[doc = "Clear one or several bits in DMA0_ITRIG_ENA register"]
pub mod dma0_itrig_ena_clr;
#[doc = "Enable DMA1 triggers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_itrig_ena](dma1_itrig_ena) module"]
pub type DMA1_ITRIG_ENA = crate::Reg<u32, _DMA1_ITRIG_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1_ITRIG_ENA;
#[doc = "`read()` method returns [dma1_itrig_ena::R](dma1_itrig_ena::R) reader structure"]
impl crate::Readable for DMA1_ITRIG_ENA {}
#[doc = "`write(|w| ..)` method takes [dma1_itrig_ena::W](dma1_itrig_ena::W) writer structure"]
impl crate::Writable for DMA1_ITRIG_ENA {}
#[doc = "Enable DMA1 triggers"]
pub mod dma1_itrig_ena;
#[doc = "Set one or several bits in DMA1_ITRIG_ENA register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_itrig_ena_set](dma1_itrig_ena_set) module"]
pub type DMA1_ITRIG_ENA_SET = crate::Reg<u32, _DMA1_ITRIG_ENA_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1_ITRIG_ENA_SET;
#[doc = "`write(|w| ..)` method takes [dma1_itrig_ena_set::W](dma1_itrig_ena_set::W) writer structure"]
impl crate::Writable for DMA1_ITRIG_ENA_SET {}
#[doc = "Set one or several bits in DMA1_ITRIG_ENA register"]
pub mod dma1_itrig_ena_set;
#[doc = "Clear one or several bits in DMA1_ITRIG_ENA register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_itrig_ena_clr](dma1_itrig_ena_clr) module"]
pub type DMA1_ITRIG_ENA_CLR = crate::Reg<u32, _DMA1_ITRIG_ENA_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1_ITRIG_ENA_CLR;
#[doc = "`write(|w| ..)` method takes [dma1_itrig_ena_clr::W](dma1_itrig_ena_clr::W) writer structure"]
impl crate::Writable for DMA1_ITRIG_ENA_CLR {}
#[doc = "Clear one or several bits in DMA1_ITRIG_ENA register"]
pub mod dma1_itrig_ena_clr;
