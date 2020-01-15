#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
    pub caplength_chipid: CAPLENGTH_CHIPID,
    #[doc = "0x04 - Host Controller Structural Parameters"]
    pub hcsparams: HCSPARAMS,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Frame Length Adjustment"]
    pub fladj_frindex: FLADJ_FRINDEX,
    #[doc = "0x10 - Memory base address where ATL PTD0 is stored"]
    pub atlptd: ATLPTD,
    #[doc = "0x14 - Memory base address where ISO PTD0 is stored"]
    pub isoptd: ISOPTD,
    #[doc = "0x18 - Memory base address where INT PTD0 is stored"]
    pub intptd: INTPTD,
    #[doc = "0x1c - Memory base address that indicates the start of the data payload buffers"]
    pub datapayload: DATAPAYLOAD,
    #[doc = "0x20 - USB Command register"]
    pub usbcmd: USBCMD,
    #[doc = "0x24 - USB Interrupt Status register"]
    pub usbsts: USBSTS,
    #[doc = "0x28 - USB Interrupt Enable register"]
    pub usbintr: USBINTR,
    #[doc = "0x2c - Port Status and Control register"]
    pub portsc1: PORTSC1,
    #[doc = "0x30 - Done map for each ATL PTD"]
    pub atlptdd: ATLPTDD,
    #[doc = "0x34 - Skip map for each ATL PTD"]
    pub atlptds: ATLPTDS,
    #[doc = "0x38 - Done map for each ISO PTD"]
    pub isoptdd: ISOPTDD,
    #[doc = "0x3c - Skip map for each ISO PTD"]
    pub isoptds: ISOPTDS,
    #[doc = "0x40 - Done map for each INT PTD"]
    pub intptdd: INTPTDD,
    #[doc = "0x44 - Skip map for each INT PTD"]
    pub intptds: INTPTDS,
    #[doc = "0x48 - Marks the last PTD in the list for ISO, INT and ATL"]
    pub lastptd: LASTPTD,
    _reserved18: [u8; 4usize],
    #[doc = "0x50 - Controls the port if it is attached to the host block or the device block"]
    pub portmode: PORTMODE,
}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caplength_chipid](caplength_chipid) module"]
pub type CAPLENGTH_CHIPID = crate::Reg<u32, _CAPLENGTH_CHIPID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPLENGTH_CHIPID;
#[doc = "`read()` method returns [caplength_chipid::R](caplength_chipid::R) reader structure"]
impl crate::Readable for CAPLENGTH_CHIPID {}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
pub mod caplength_chipid;
#[doc = "Host Controller Structural Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsparams](hcsparams) module"]
pub type HCSPARAMS = crate::Reg<u32, _HCSPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPARAMS;
#[doc = "`read()` method returns [hcsparams::R](hcsparams::R) reader structure"]
impl crate::Readable for HCSPARAMS {}
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "Frame Length Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fladj_frindex](fladj_frindex) module"]
pub type FLADJ_FRINDEX = crate::Reg<u32, _FLADJ_FRINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLADJ_FRINDEX;
#[doc = "`read()` method returns [fladj_frindex::R](fladj_frindex::R) reader structure"]
impl crate::Readable for FLADJ_FRINDEX {}
#[doc = "`write(|w| ..)` method takes [fladj_frindex::W](fladj_frindex::W) writer structure"]
impl crate::Writable for FLADJ_FRINDEX {}
#[doc = "Frame Length Adjustment"]
pub mod fladj_frindex;
#[doc = "Memory base address where ATL PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atlptd](atlptd) module"]
pub type ATLPTD = crate::Reg<u32, _ATLPTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATLPTD;
#[doc = "`read()` method returns [atlptd::R](atlptd::R) reader structure"]
impl crate::Readable for ATLPTD {}
#[doc = "`write(|w| ..)` method takes [atlptd::W](atlptd::W) writer structure"]
impl crate::Writable for ATLPTD {}
#[doc = "Memory base address where ATL PTD0 is stored"]
pub mod atlptd;
#[doc = "Memory base address where ISO PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoptd](isoptd) module"]
pub type ISOPTD = crate::Reg<u32, _ISOPTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISOPTD;
#[doc = "`read()` method returns [isoptd::R](isoptd::R) reader structure"]
impl crate::Readable for ISOPTD {}
#[doc = "`write(|w| ..)` method takes [isoptd::W](isoptd::W) writer structure"]
impl crate::Writable for ISOPTD {}
#[doc = "Memory base address where ISO PTD0 is stored"]
pub mod isoptd;
#[doc = "Memory base address where INT PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intptd](intptd) module"]
pub type INTPTD = crate::Reg<u32, _INTPTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPTD;
#[doc = "`read()` method returns [intptd::R](intptd::R) reader structure"]
impl crate::Readable for INTPTD {}
#[doc = "`write(|w| ..)` method takes [intptd::W](intptd::W) writer structure"]
impl crate::Writable for INTPTD {}
#[doc = "Memory base address where INT PTD0 is stored"]
pub mod intptd;
#[doc = "Memory base address that indicates the start of the data payload buffers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datapayload](datapayload) module"]
pub type DATAPAYLOAD = crate::Reg<u32, _DATAPAYLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAPAYLOAD;
#[doc = "`read()` method returns [datapayload::R](datapayload::R) reader structure"]
impl crate::Readable for DATAPAYLOAD {}
#[doc = "`write(|w| ..)` method takes [datapayload::W](datapayload::W) writer structure"]
impl crate::Writable for DATAPAYLOAD {}
#[doc = "Memory base address that indicates the start of the data payload buffers"]
pub mod datapayload;
#[doc = "USB Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcmd](usbcmd) module"]
pub type USBCMD = crate::Reg<u32, _USBCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCMD;
#[doc = "`read()` method returns [usbcmd::R](usbcmd::R) reader structure"]
impl crate::Readable for USBCMD {}
#[doc = "`write(|w| ..)` method takes [usbcmd::W](usbcmd::W) writer structure"]
impl crate::Writable for USBCMD {}
#[doc = "USB Command register"]
pub mod usbcmd;
#[doc = "USB Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsts](usbsts) module"]
pub type USBSTS = crate::Reg<u32, _USBSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBSTS;
#[doc = "`read()` method returns [usbsts::R](usbsts::R) reader structure"]
impl crate::Readable for USBSTS {}
#[doc = "`write(|w| ..)` method takes [usbsts::W](usbsts::W) writer structure"]
impl crate::Writable for USBSTS {}
#[doc = "USB Interrupt Status register"]
pub mod usbsts;
#[doc = "USB Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbintr](usbintr) module"]
pub type USBINTR = crate::Reg<u32, _USBINTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBINTR;
#[doc = "`read()` method returns [usbintr::R](usbintr::R) reader structure"]
impl crate::Readable for USBINTR {}
#[doc = "`write(|w| ..)` method takes [usbintr::W](usbintr::W) writer structure"]
impl crate::Writable for USBINTR {}
#[doc = "USB Interrupt Enable register"]
pub mod usbintr;
#[doc = "Port Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsc1](portsc1) module"]
pub type PORTSC1 = crate::Reg<u32, _PORTSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTSC1;
#[doc = "`read()` method returns [portsc1::R](portsc1::R) reader structure"]
impl crate::Readable for PORTSC1 {}
#[doc = "`write(|w| ..)` method takes [portsc1::W](portsc1::W) writer structure"]
impl crate::Writable for PORTSC1 {}
#[doc = "Port Status and Control register"]
pub mod portsc1;
#[doc = "Done map for each ATL PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atlptdd](atlptdd) module"]
pub type ATLPTDD = crate::Reg<u32, _ATLPTDD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATLPTDD;
#[doc = "`read()` method returns [atlptdd::R](atlptdd::R) reader structure"]
impl crate::Readable for ATLPTDD {}
#[doc = "`write(|w| ..)` method takes [atlptdd::W](atlptdd::W) writer structure"]
impl crate::Writable for ATLPTDD {}
#[doc = "Done map for each ATL PTD"]
pub mod atlptdd;
#[doc = "Skip map for each ATL PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atlptds](atlptds) module"]
pub type ATLPTDS = crate::Reg<u32, _ATLPTDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATLPTDS;
#[doc = "`read()` method returns [atlptds::R](atlptds::R) reader structure"]
impl crate::Readable for ATLPTDS {}
#[doc = "`write(|w| ..)` method takes [atlptds::W](atlptds::W) writer structure"]
impl crate::Writable for ATLPTDS {}
#[doc = "Skip map for each ATL PTD"]
pub mod atlptds;
#[doc = "Done map for each ISO PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoptdd](isoptdd) module"]
pub type ISOPTDD = crate::Reg<u32, _ISOPTDD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISOPTDD;
#[doc = "`read()` method returns [isoptdd::R](isoptdd::R) reader structure"]
impl crate::Readable for ISOPTDD {}
#[doc = "`write(|w| ..)` method takes [isoptdd::W](isoptdd::W) writer structure"]
impl crate::Writable for ISOPTDD {}
#[doc = "Done map for each ISO PTD"]
pub mod isoptdd;
#[doc = "Skip map for each ISO PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoptds](isoptds) module"]
pub type ISOPTDS = crate::Reg<u32, _ISOPTDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISOPTDS;
#[doc = "`read()` method returns [isoptds::R](isoptds::R) reader structure"]
impl crate::Readable for ISOPTDS {}
#[doc = "`write(|w| ..)` method takes [isoptds::W](isoptds::W) writer structure"]
impl crate::Writable for ISOPTDS {}
#[doc = "Skip map for each ISO PTD"]
pub mod isoptds;
#[doc = "Done map for each INT PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intptdd](intptdd) module"]
pub type INTPTDD = crate::Reg<u32, _INTPTDD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPTDD;
#[doc = "`read()` method returns [intptdd::R](intptdd::R) reader structure"]
impl crate::Readable for INTPTDD {}
#[doc = "`write(|w| ..)` method takes [intptdd::W](intptdd::W) writer structure"]
impl crate::Writable for INTPTDD {}
#[doc = "Done map for each INT PTD"]
pub mod intptdd;
#[doc = "Skip map for each INT PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intptds](intptds) module"]
pub type INTPTDS = crate::Reg<u32, _INTPTDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPTDS;
#[doc = "`read()` method returns [intptds::R](intptds::R) reader structure"]
impl crate::Readable for INTPTDS {}
#[doc = "`write(|w| ..)` method takes [intptds::W](intptds::W) writer structure"]
impl crate::Writable for INTPTDS {}
#[doc = "Skip map for each INT PTD"]
pub mod intptds;
#[doc = "Marks the last PTD in the list for ISO, INT and ATL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lastptd](lastptd) module"]
pub type LASTPTD = crate::Reg<u32, _LASTPTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LASTPTD;
#[doc = "`read()` method returns [lastptd::R](lastptd::R) reader structure"]
impl crate::Readable for LASTPTD {}
#[doc = "`write(|w| ..)` method takes [lastptd::W](lastptd::W) writer structure"]
impl crate::Writable for LASTPTD {}
#[doc = "Marks the last PTD in the list for ISO, INT and ATL"]
pub mod lastptd;
#[doc = "Controls the port if it is attached to the host block or the device block\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portmode](portmode) module"]
pub type PORTMODE = crate::Reg<u32, _PORTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTMODE;
#[doc = "`read()` method returns [portmode::R](portmode::R) reader structure"]
impl crate::Readable for PORTMODE {}
#[doc = "`write(|w| ..)` method takes [portmode::W](portmode::W) writer structure"]
impl crate::Writable for PORTMODE {}
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub mod portmode;
