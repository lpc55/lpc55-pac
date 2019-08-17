#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Base address register for output region"]
    pub outbase: OUTBASE,
    #[doc = "0x04 - Output format"]
    pub outformat: OUTFORMAT,
    #[doc = "0x08 - Base address register for temp region"]
    pub tmpbase: TMPBASE,
    #[doc = "0x0c - Temp format"]
    pub tmpformat: TMPFORMAT,
    #[doc = "0x10 - Base address register for input A region"]
    pub inabase: INABASE,
    #[doc = "0x14 - Input A format"]
    pub inaformat: INAFORMAT,
    #[doc = "0x18 - Base address register for input B region"]
    pub inbbase: INBBASE,
    #[doc = "0x1c - Input B format"]
    pub inbformat: INBFORMAT,
    _reserved8: [u8; 224usize],
    #[doc = "0x100 - PowerQuad Control register"]
    pub control: CONTROL,
    #[doc = "0x104 - Length register"]
    pub length: LENGTH,
    #[doc = "0x108 - Pre-scale register"]
    pub cppre: CPPRE,
    #[doc = "0x10c - Misc register"]
    pub misc: MISC,
    #[doc = "0x110 - Cursory register"]
    pub cursory: CURSORY,
    _reserved13: [u8; 108usize],
    #[doc = "0x180 - Cordic input X register"]
    pub cordic_x: CORDIC_X,
    #[doc = "0x184 - Cordic input Y register"]
    pub cordic_y: CORDIC_Y,
    #[doc = "0x188 - Cordic input Z register"]
    pub cordic_z: CORDIC_Z,
    #[doc = "0x18c - Read/Write register where error statuses are captured (sticky)"]
    pub errstat: ERRSTAT,
    #[doc = "0x190 - INTERRUPT enable register"]
    pub intren: INTREN,
    #[doc = "0x194 - Event Enable register"]
    pub eventen: EVENTEN,
    #[doc = "0x198 - INTERRUPT STATUS register"]
    pub intrstat: INTRSTAT,
    _reserved20: [u8; 100usize],
    #[doc = "0x200 - General purpose register bank N."]
    pub gpreg: [GPREG; 16],
    #[doc = "0x240 - Compute register bank"]
    pub compreg: [COMPREG; 8],
}
#[doc = "Base address register for output region\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outbase](outbase) module"]
pub type OUTBASE = crate::Reg<u32, _OUTBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTBASE;
#[doc = "`read()` method returns [outbase::R](outbase::R) reader structure"]
impl crate::Readable for OUTBASE {}
#[doc = "`write(|w| ..)` method takes [outbase::W](outbase::W) writer structure"]
impl crate::Writable for OUTBASE {}
#[doc = "Base address register for output region"]
pub mod outbase;
#[doc = "Output format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outformat](outformat) module"]
pub type OUTFORMAT = crate::Reg<u32, _OUTFORMAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTFORMAT;
#[doc = "`read()` method returns [outformat::R](outformat::R) reader structure"]
impl crate::Readable for OUTFORMAT {}
#[doc = "`write(|w| ..)` method takes [outformat::W](outformat::W) writer structure"]
impl crate::Writable for OUTFORMAT {}
#[doc = "Output format"]
pub mod outformat;
#[doc = "Base address register for temp region\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tmpbase](tmpbase) module"]
pub type TMPBASE = crate::Reg<u32, _TMPBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMPBASE;
#[doc = "`read()` method returns [tmpbase::R](tmpbase::R) reader structure"]
impl crate::Readable for TMPBASE {}
#[doc = "`write(|w| ..)` method takes [tmpbase::W](tmpbase::W) writer structure"]
impl crate::Writable for TMPBASE {}
#[doc = "Base address register for temp region"]
pub mod tmpbase;
#[doc = "Temp format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tmpformat](tmpformat) module"]
pub type TMPFORMAT = crate::Reg<u32, _TMPFORMAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMPFORMAT;
#[doc = "`read()` method returns [tmpformat::R](tmpformat::R) reader structure"]
impl crate::Readable for TMPFORMAT {}
#[doc = "`write(|w| ..)` method takes [tmpformat::W](tmpformat::W) writer structure"]
impl crate::Writable for TMPFORMAT {}
#[doc = "Temp format"]
pub mod tmpformat;
#[doc = "Base address register for input A region\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inabase](inabase) module"]
pub type INABASE = crate::Reg<u32, _INABASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INABASE;
#[doc = "`read()` method returns [inabase::R](inabase::R) reader structure"]
impl crate::Readable for INABASE {}
#[doc = "`write(|w| ..)` method takes [inabase::W](inabase::W) writer structure"]
impl crate::Writable for INABASE {}
#[doc = "Base address register for input A region"]
pub mod inabase;
#[doc = "Input A format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inaformat](inaformat) module"]
pub type INAFORMAT = crate::Reg<u32, _INAFORMAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INAFORMAT;
#[doc = "`read()` method returns [inaformat::R](inaformat::R) reader structure"]
impl crate::Readable for INAFORMAT {}
#[doc = "`write(|w| ..)` method takes [inaformat::W](inaformat::W) writer structure"]
impl crate::Writable for INAFORMAT {}
#[doc = "Input A format"]
pub mod inaformat;
#[doc = "Base address register for input B region\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inbbase](inbbase) module"]
pub type INBBASE = crate::Reg<u32, _INBBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INBBASE;
#[doc = "`read()` method returns [inbbase::R](inbbase::R) reader structure"]
impl crate::Readable for INBBASE {}
#[doc = "`write(|w| ..)` method takes [inbbase::W](inbbase::W) writer structure"]
impl crate::Writable for INBBASE {}
#[doc = "Base address register for input B region"]
pub mod inbbase;
#[doc = "Input B format\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inbformat](inbformat) module"]
pub type INBFORMAT = crate::Reg<u32, _INBFORMAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INBFORMAT;
#[doc = "`read()` method returns [inbformat::R](inbformat::R) reader structure"]
impl crate::Readable for INBFORMAT {}
#[doc = "`write(|w| ..)` method takes [inbformat::W](inbformat::W) writer structure"]
impl crate::Writable for INBFORMAT {}
#[doc = "Input B format"]
pub mod inbformat;
#[doc = "PowerQuad Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "PowerQuad Control register"]
pub mod control;
#[doc = "Length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [length](length) module"]
pub type LENGTH = crate::Reg<u32, _LENGTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LENGTH;
#[doc = "`read()` method returns [length::R](length::R) reader structure"]
impl crate::Readable for LENGTH {}
#[doc = "`write(|w| ..)` method takes [length::W](length::W) writer structure"]
impl crate::Writable for LENGTH {}
#[doc = "Length register"]
pub mod length;
#[doc = "Pre-scale register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cppre](cppre) module"]
pub type CPPRE = crate::Reg<u32, _CPPRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPPRE;
#[doc = "`read()` method returns [cppre::R](cppre::R) reader structure"]
impl crate::Readable for CPPRE {}
#[doc = "`write(|w| ..)` method takes [cppre::W](cppre::W) writer structure"]
impl crate::Writable for CPPRE {}
#[doc = "Pre-scale register"]
pub mod cppre;
#[doc = "Misc register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc](misc) module"]
pub type MISC = crate::Reg<u32, _MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC;
#[doc = "`read()` method returns [misc::R](misc::R) reader structure"]
impl crate::Readable for MISC {}
#[doc = "`write(|w| ..)` method takes [misc::W](misc::W) writer structure"]
impl crate::Writable for MISC {}
#[doc = "Misc register"]
pub mod misc;
#[doc = "Cursory register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cursory](cursory) module"]
pub type CURSORY = crate::Reg<u32, _CURSORY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURSORY;
#[doc = "`read()` method returns [cursory::R](cursory::R) reader structure"]
impl crate::Readable for CURSORY {}
#[doc = "`write(|w| ..)` method takes [cursory::W](cursory::W) writer structure"]
impl crate::Writable for CURSORY {}
#[doc = "Cursory register"]
pub mod cursory;
#[doc = "Cordic input X register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cordic_x](cordic_x) module"]
pub type CORDIC_X = crate::Reg<u32, _CORDIC_X>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CORDIC_X;
#[doc = "`read()` method returns [cordic_x::R](cordic_x::R) reader structure"]
impl crate::Readable for CORDIC_X {}
#[doc = "`write(|w| ..)` method takes [cordic_x::W](cordic_x::W) writer structure"]
impl crate::Writable for CORDIC_X {}
#[doc = "Cordic input X register"]
pub mod cordic_x;
#[doc = "Cordic input Y register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cordic_y](cordic_y) module"]
pub type CORDIC_Y = crate::Reg<u32, _CORDIC_Y>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CORDIC_Y;
#[doc = "`read()` method returns [cordic_y::R](cordic_y::R) reader structure"]
impl crate::Readable for CORDIC_Y {}
#[doc = "`write(|w| ..)` method takes [cordic_y::W](cordic_y::W) writer structure"]
impl crate::Writable for CORDIC_Y {}
#[doc = "Cordic input Y register"]
pub mod cordic_y;
#[doc = "Cordic input Z register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cordic_z](cordic_z) module"]
pub type CORDIC_Z = crate::Reg<u32, _CORDIC_Z>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CORDIC_Z;
#[doc = "`read()` method returns [cordic_z::R](cordic_z::R) reader structure"]
impl crate::Readable for CORDIC_Z {}
#[doc = "`write(|w| ..)` method takes [cordic_z::W](cordic_z::W) writer structure"]
impl crate::Writable for CORDIC_Z {}
#[doc = "Cordic input Z register"]
pub mod cordic_z;
#[doc = "Read/Write register where error statuses are captured (sticky)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [errstat](errstat) module"]
pub type ERRSTAT = crate::Reg<u32, _ERRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRSTAT;
#[doc = "`read()` method returns [errstat::R](errstat::R) reader structure"]
impl crate::Readable for ERRSTAT {}
#[doc = "`write(|w| ..)` method takes [errstat::W](errstat::W) writer structure"]
impl crate::Writable for ERRSTAT {}
#[doc = "Read/Write register where error statuses are captured (sticky)"]
pub mod errstat;
#[doc = "INTERRUPT enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intren](intren) module"]
pub type INTREN = crate::Reg<u32, _INTREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTREN;
#[doc = "`read()` method returns [intren::R](intren::R) reader structure"]
impl crate::Readable for INTREN {}
#[doc = "`write(|w| ..)` method takes [intren::W](intren::W) writer structure"]
impl crate::Writable for INTREN {}
#[doc = "INTERRUPT enable register"]
pub mod intren;
#[doc = "Event Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eventen](eventen) module"]
pub type EVENTEN = crate::Reg<u32, _EVENTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTEN;
#[doc = "`read()` method returns [eventen::R](eventen::R) reader structure"]
impl crate::Readable for EVENTEN {}
#[doc = "`write(|w| ..)` method takes [eventen::W](eventen::W) writer structure"]
impl crate::Writable for EVENTEN {}
#[doc = "Event Enable register"]
pub mod eventen;
#[doc = "INTERRUPT STATUS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intrstat](intrstat) module"]
pub type INTRSTAT = crate::Reg<u32, _INTRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRSTAT;
#[doc = "`read()` method returns [intrstat::R](intrstat::R) reader structure"]
impl crate::Readable for INTRSTAT {}
#[doc = "`write(|w| ..)` method takes [intrstat::W](intrstat::W) writer structure"]
impl crate::Writable for INTRSTAT {}
#[doc = "INTERRUPT STATUS register"]
pub mod intrstat;
#[doc = "General purpose register bank N.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpreg](gpreg) module"]
pub type GPREG = crate::Reg<u32, _GPREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREG;
#[doc = "`read()` method returns [gpreg::R](gpreg::R) reader structure"]
impl crate::Readable for GPREG {}
#[doc = "`write(|w| ..)` method takes [gpreg::W](gpreg::W) writer structure"]
impl crate::Writable for GPREG {}
#[doc = "General purpose register bank N."]
pub mod gpreg;
#[doc = "Compute register bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [compreg](compreg) module"]
pub type COMPREG = crate::Reg<u32, _COMPREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPREG;
#[doc = "`read()` method returns [compreg::R](compreg::R) reader structure"]
impl crate::Readable for COMPREG {}
#[doc = "`write(|w| ..)` method takes [compreg::W](compreg::W) writer structure"]
impl crate::Writable for COMPREG {}
#[doc = "Compute register bank"]
pub mod compreg;
