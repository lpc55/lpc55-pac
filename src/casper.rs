#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Contains the offsets of AB and CD in the RAM."]
    pub ctrl0: CTRL0,
    #[doc = "0x04 - Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
    pub loader: LOADER,
    #[doc = "0x0c - Indicates operational status and would contain the carry bit if used."]
    pub status: STATUS,
    #[doc = "0x10 - Sets interrupts"]
    pub intenset: INTENSET,
    #[doc = "0x14 - Clears interrupts"]
    pub intenclr: INTENCLR,
    #[doc = "0x18 - Interrupt status bits (mask of INTENSET and STATUS)"]
    pub intstat: INTSTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - A register"]
    pub areg: AREG,
    #[doc = "0x24 - B register"]
    pub breg: BREG,
    #[doc = "0x28 - C register"]
    pub creg: CREG,
    #[doc = "0x2c - D register"]
    pub dreg: DREG,
    #[doc = "0x30 - Result register 0"]
    pub res0: RES0,
    #[doc = "0x34 - Result register 1"]
    pub res1: RES1,
    #[doc = "0x38 - Result register 2"]
    pub res2: RES2,
    #[doc = "0x3c - Result register 3"]
    pub res3: RES3,
    _reserved15: [u8; 32usize],
    #[doc = "0x60 - Optional mask register"]
    pub mask: MASK,
    #[doc = "0x64 - Optional re-mask register"]
    pub remask: REMASK,
    _reserved17: [u8; 24usize],
    #[doc = "0x80 - Security lock register"]
    pub lock: LOCK,
}
#[doc = "Contains the offsets of AB and CD in the RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](ctrl0) module"]
pub type CTRL0 = crate::Reg<u32, _CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL0;
#[doc = "`read()` method returns [ctrl0::R](ctrl0::R) reader structure"]
impl crate::Readable for CTRL0 {}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](ctrl0::W) writer structure"]
impl crate::Writable for CTRL0 {}
#[doc = "Contains the offsets of AB and CD in the RAM."]
pub mod ctrl0;
#[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
pub mod ctrl1;
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loader](loader) module"]
pub type LOADER = crate::Reg<u32, _LOADER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOADER;
#[doc = "`read()` method returns [loader::R](loader::R) reader structure"]
impl crate::Readable for LOADER {}
#[doc = "`write(|w| ..)` method takes [loader::W](loader::W) writer structure"]
impl crate::Writable for LOADER {}
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
pub mod loader;
#[doc = "Indicates operational status and would contain the carry bit if used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Indicates operational status and would contain the carry bit if used."]
pub mod status;
#[doc = "Sets interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Sets interrupts"]
pub mod intenset;
#[doc = "Clears interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Clears interrupts"]
pub mod intenclr;
#[doc = "Interrupt status bits (mask of INTENSET and STATUS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "Interrupt status bits (mask of INTENSET and STATUS)"]
pub mod intstat;
#[doc = "A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [areg](areg) module"]
pub type AREG = crate::Reg<u32, _AREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AREG;
#[doc = "`read()` method returns [areg::R](areg::R) reader structure"]
impl crate::Readable for AREG {}
#[doc = "`write(|w| ..)` method takes [areg::W](areg::W) writer structure"]
impl crate::Writable for AREG {}
#[doc = "A register"]
pub mod areg;
#[doc = "B register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [breg](breg) module"]
pub type BREG = crate::Reg<u32, _BREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BREG;
#[doc = "`read()` method returns [breg::R](breg::R) reader structure"]
impl crate::Readable for BREG {}
#[doc = "`write(|w| ..)` method takes [breg::W](breg::W) writer structure"]
impl crate::Writable for BREG {}
#[doc = "B register"]
pub mod breg;
#[doc = "C register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [creg](creg) module"]
pub type CREG = crate::Reg<u32, _CREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CREG;
#[doc = "`read()` method returns [creg::R](creg::R) reader structure"]
impl crate::Readable for CREG {}
#[doc = "`write(|w| ..)` method takes [creg::W](creg::W) writer structure"]
impl crate::Writable for CREG {}
#[doc = "C register"]
pub mod creg;
#[doc = "D register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dreg](dreg) module"]
pub type DREG = crate::Reg<u32, _DREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DREG;
#[doc = "`read()` method returns [dreg::R](dreg::R) reader structure"]
impl crate::Readable for DREG {}
#[doc = "`write(|w| ..)` method takes [dreg::W](dreg::W) writer structure"]
impl crate::Writable for DREG {}
#[doc = "D register"]
pub mod dreg;
#[doc = "Result register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res0](res0) module"]
pub type RES0 = crate::Reg<u32, _RES0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES0;
#[doc = "`read()` method returns [res0::R](res0::R) reader structure"]
impl crate::Readable for RES0 {}
#[doc = "`write(|w| ..)` method takes [res0::W](res0::W) writer structure"]
impl crate::Writable for RES0 {}
#[doc = "Result register 0"]
pub mod res0;
#[doc = "Result register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res1](res1) module"]
pub type RES1 = crate::Reg<u32, _RES1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES1;
#[doc = "`read()` method returns [res1::R](res1::R) reader structure"]
impl crate::Readable for RES1 {}
#[doc = "`write(|w| ..)` method takes [res1::W](res1::W) writer structure"]
impl crate::Writable for RES1 {}
#[doc = "Result register 1"]
pub mod res1;
#[doc = "Result register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res2](res2) module"]
pub type RES2 = crate::Reg<u32, _RES2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES2;
#[doc = "`read()` method returns [res2::R](res2::R) reader structure"]
impl crate::Readable for RES2 {}
#[doc = "`write(|w| ..)` method takes [res2::W](res2::W) writer structure"]
impl crate::Writable for RES2 {}
#[doc = "Result register 2"]
pub mod res2;
#[doc = "Result register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res3](res3) module"]
pub type RES3 = crate::Reg<u32, _RES3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES3;
#[doc = "`read()` method returns [res3::R](res3::R) reader structure"]
impl crate::Readable for RES3 {}
#[doc = "`write(|w| ..)` method takes [res3::W](res3::W) writer structure"]
impl crate::Writable for RES3 {}
#[doc = "Result register 3"]
pub mod res3;
#[doc = "Optional mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "Optional mask register"]
pub mod mask;
#[doc = "Optional re-mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remask](remask) module"]
pub type REMASK = crate::Reg<u32, _REMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REMASK;
#[doc = "`read()` method returns [remask::R](remask::R) reader structure"]
impl crate::Readable for REMASK {}
#[doc = "`write(|w| ..)` method takes [remask::W](remask::W) writer structure"]
impl crate::Writable for REMASK {}
#[doc = "Optional re-mask register"]
pub mod remask;
#[doc = "Security lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Security lock register"]
pub mod lock;
