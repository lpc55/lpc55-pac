#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EVTIMER Low Register"]
    pub evtimerl: EVTIMERL,
    #[doc = "0x04 - EVTIMER High Register"]
    pub evtimerh: EVTIMERH,
    #[doc = "0x08 - Local Capture Low Register for CPUn"]
    pub capturen_l: CAPTUREN_L,
    #[doc = "0x0c - Local Capture High Register for CPUn"]
    pub capturen_h: CAPTUREN_H,
    #[doc = "0x10 - Local Match Low Register for CPUn"]
    pub matchn_l: MATCHN_L,
    #[doc = "0x14 - Match High Register for CPUn"]
    pub matchn_h: MATCHN_H,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - OS_EVENT TIMER Control Register for CPUn"]
    pub osevent_ctrl: OSEVENT_CTRL,
}
#[doc = "EVTIMER Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtimerl](evtimerl) module"]
pub type EVTIMERL = crate::Reg<u32, _EVTIMERL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTIMERL;
#[doc = "`read()` method returns [evtimerl::R](evtimerl::R) reader structure"]
impl crate::Readable for EVTIMERL {}
#[doc = "EVTIMER Low Register"]
pub mod evtimerl;
#[doc = "EVTIMER High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtimerh](evtimerh) module"]
pub type EVTIMERH = crate::Reg<u32, _EVTIMERH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTIMERH;
#[doc = "`read()` method returns [evtimerh::R](evtimerh::R) reader structure"]
impl crate::Readable for EVTIMERH {}
#[doc = "`write(|w| ..)` method takes [evtimerh::W](evtimerh::W) writer structure"]
impl crate::Writable for EVTIMERH {}
#[doc = "EVTIMER High Register"]
pub mod evtimerh;
#[doc = "Local Capture Low Register for CPUn\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capturen_l](capturen_l) module"]
pub type CAPTUREN_L = crate::Reg<u32, _CAPTUREN_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTUREN_L;
#[doc = "`read()` method returns [capturen_l::R](capturen_l::R) reader structure"]
impl crate::Readable for CAPTUREN_L {}
#[doc = "Local Capture Low Register for CPUn"]
pub mod capturen_l;
#[doc = "Local Capture High Register for CPUn\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capturen_h](capturen_h) module"]
pub type CAPTUREN_H = crate::Reg<u32, _CAPTUREN_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTUREN_H;
#[doc = "`read()` method returns [capturen_h::R](capturen_h::R) reader structure"]
impl crate::Readable for CAPTUREN_H {}
#[doc = "`write(|w| ..)` method takes [capturen_h::W](capturen_h::W) writer structure"]
impl crate::Writable for CAPTUREN_H {}
#[doc = "Local Capture High Register for CPUn"]
pub mod capturen_h;
#[doc = "Local Match Low Register for CPUn\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchn_l](matchn_l) module"]
pub type MATCHN_L = crate::Reg<u32, _MATCHN_L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHN_L;
#[doc = "`read()` method returns [matchn_l::R](matchn_l::R) reader structure"]
impl crate::Readable for MATCHN_L {}
#[doc = "`write(|w| ..)` method takes [matchn_l::W](matchn_l::W) writer structure"]
impl crate::Writable for MATCHN_L {}
#[doc = "Local Match Low Register for CPUn"]
pub mod matchn_l;
#[doc = "Match High Register for CPUn\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matchn_h](matchn_h) module"]
pub type MATCHN_H = crate::Reg<u32, _MATCHN_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCHN_H;
#[doc = "`read()` method returns [matchn_h::R](matchn_h::R) reader structure"]
impl crate::Readable for MATCHN_H {}
#[doc = "`write(|w| ..)` method takes [matchn_h::W](matchn_h::W) writer structure"]
impl crate::Writable for MATCHN_H {}
#[doc = "Match High Register for CPUn"]
pub mod matchn_h;
#[doc = "OS_EVENT TIMER Control Register for CPUn\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osevent_ctrl](osevent_ctrl) module"]
pub type OSEVENT_CTRL = crate::Reg<u32, _OSEVENT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSEVENT_CTRL;
#[doc = "`read()` method returns [osevent_ctrl::R](osevent_ctrl::R) reader structure"]
impl crate::Readable for OSEVENT_CTRL {}
#[doc = "`write(|w| ..)` method takes [osevent_ctrl::W](osevent_ctrl::W) writer structure"]
impl crate::Writable for OSEVENT_CTRL {}
#[doc = "OS_EVENT TIMER Control Register for CPUn"]
pub mod osevent_ctrl;
