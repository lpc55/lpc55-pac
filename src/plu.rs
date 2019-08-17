#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub lut0: LUT,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - no description available"]
    pub lut1: LUT,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - no description available"]
    pub lut2: LUT,
    _reserved3: [u8; 12usize],
    #[doc = "0x60 - no description available"]
    pub lut3: LUT,
    _reserved4: [u8; 12usize],
    #[doc = "0x80 - no description available"]
    pub lut4: LUT,
    _reserved5: [u8; 12usize],
    #[doc = "0xa0 - no description available"]
    pub lut5: LUT,
    _reserved6: [u8; 12usize],
    #[doc = "0xc0 - no description available"]
    pub lut6: LUT,
    _reserved7: [u8; 12usize],
    #[doc = "0xe0 - no description available"]
    pub lut7: LUT,
    _reserved8: [u8; 12usize],
    #[doc = "0x100 - no description available"]
    pub lut8: LUT,
    _reserved9: [u8; 12usize],
    #[doc = "0x120 - no description available"]
    pub lut9: LUT,
    _reserved10: [u8; 12usize],
    #[doc = "0x140 - no description available"]
    pub lut10: LUT,
    _reserved11: [u8; 12usize],
    #[doc = "0x160 - no description available"]
    pub lut11: LUT,
    _reserved12: [u8; 12usize],
    #[doc = "0x180 - no description available"]
    pub lut12: LUT,
    _reserved13: [u8; 12usize],
    #[doc = "0x1a0 - no description available"]
    pub lut13: LUT,
    _reserved14: [u8; 12usize],
    #[doc = "0x1c0 - no description available"]
    pub lut14: LUT,
    _reserved15: [u8; 12usize],
    #[doc = "0x1e0 - no description available"]
    pub lut15: LUT,
    _reserved16: [u8; 12usize],
    #[doc = "0x200 - no description available"]
    pub lut16: LUT,
    _reserved17: [u8; 12usize],
    #[doc = "0x220 - no description available"]
    pub lut17: LUT,
    _reserved18: [u8; 12usize],
    #[doc = "0x240 - no description available"]
    pub lut18: LUT,
    _reserved19: [u8; 12usize],
    #[doc = "0x260 - no description available"]
    pub lut19: LUT,
    _reserved20: [u8; 12usize],
    #[doc = "0x280 - no description available"]
    pub lut20: LUT,
    _reserved21: [u8; 12usize],
    #[doc = "0x2a0 - no description available"]
    pub lut21: LUT,
    _reserved22: [u8; 12usize],
    #[doc = "0x2c0 - no description available"]
    pub lut22: LUT,
    _reserved23: [u8; 12usize],
    #[doc = "0x2e0 - no description available"]
    pub lut23: LUT,
    _reserved24: [u8; 12usize],
    #[doc = "0x300 - no description available"]
    pub lut24: LUT,
    _reserved25: [u8; 12usize],
    #[doc = "0x320 - no description available"]
    pub lut25: LUT,
    _reserved26: [u8; 1228usize],
    #[doc = "0x800 - Specifies the Truth Table contents for LUT0"]
    pub lut_truth: [LUT_TRUTH; 26],
    _reserved27: [u8; 152usize],
    #[doc = "0x900 - Provides the current state of the 8 designated PLU Outputs."]
    pub outputs: OUTPUTS,
    #[doc = "0x904 - Wakeup interrupt control for PLU"]
    pub wakeint: WAKEINT,
    _reserved29: [u8; 760usize],
    #[doc = "0xc00 - Selects the source to be connected to PLU Output 0"]
    pub output_mux: [OUTPUT_MUX; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct LUT {
    #[doc = "0x00 - LUT0 input 0 MUX"]
    pub lut_inp: [self::lut::LUT_INP; 5],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod lut;
#[doc = "Specifies the Truth Table contents for LUT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lut_truth](lut_truth) module"]
pub type LUT_TRUTH = crate::Reg<u32, _LUT_TRUTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT_TRUTH;
#[doc = "`read()` method returns [lut_truth::R](lut_truth::R) reader structure"]
impl crate::Readable for LUT_TRUTH {}
#[doc = "`write(|w| ..)` method takes [lut_truth::W](lut_truth::W) writer structure"]
impl crate::Writable for LUT_TRUTH {}
#[doc = "Specifies the Truth Table contents for LUT0"]
pub mod lut_truth;
#[doc = "Provides the current state of the 8 designated PLU Outputs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outputs](outputs) module"]
pub type OUTPUTS = crate::Reg<u32, _OUTPUTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUTS;
#[doc = "`read()` method returns [outputs::R](outputs::R) reader structure"]
impl crate::Readable for OUTPUTS {}
#[doc = "`write(|w| ..)` method takes [outputs::W](outputs::W) writer structure"]
impl crate::Writable for OUTPUTS {}
#[doc = "Provides the current state of the 8 designated PLU Outputs."]
pub mod outputs;
#[doc = "Wakeup interrupt control for PLU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wakeint](wakeint) module"]
pub type WAKEINT = crate::Reg<u32, _WAKEINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAKEINT;
#[doc = "`read()` method returns [wakeint::R](wakeint::R) reader structure"]
impl crate::Readable for WAKEINT {}
#[doc = "`write(|w| ..)` method takes [wakeint::W](wakeint::W) writer structure"]
impl crate::Writable for WAKEINT {}
#[doc = "Wakeup interrupt control for PLU"]
pub mod wakeint;
#[doc = "Selects the source to be connected to PLU Output 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [output_mux](output_mux) module"]
pub type OUTPUT_MUX = crate::Reg<u32, _OUTPUT_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUT_MUX;
#[doc = "`read()` method returns [output_mux::R](output_mux::R) reader structure"]
impl crate::Readable for OUTPUT_MUX {}
#[doc = "`write(|w| ..)` method takes [output_mux::W](output_mux::W) writer structure"]
impl crate::Writable for OUTPUT_MUX {}
#[doc = "Selects the source to be connected to PLU Output 0"]
pub mod output_mux;
