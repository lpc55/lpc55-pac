#[doc = "LUT0 input 0 MUX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lut_inp](lut_inp) module"]
pub type LUT_INP = crate::Reg<u32, _LUT_INP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT_INP;
#[doc = "`read()` method returns [lut_inp::R](lut_inp::R) reader structure"]
impl crate::Readable for LUT_INP {}
#[doc = "`write(|w| ..)` method takes [lut_inp::W](lut_inp::W) writer structure"]
impl crate::Writable for LUT_INP {}
#[doc = "LUT0 input 0 MUX"]
pub mod lut_inp;
