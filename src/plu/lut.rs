#[doc = "LUTn input x MUX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lut_inp_mux](lut_inp_mux) module"]
pub type LUT_INP_MUX = crate::Reg<u32, _LUT_INP_MUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUT_INP_MUX;
#[doc = "`read()` method returns [lut_inp_mux::R](lut_inp_mux::R) reader structure"]
impl crate::Readable for LUT_INP_MUX {}
#[doc = "`write(|w| ..)` method takes [lut_inp_mux::W](lut_inp_mux::W) writer structure"]
impl crate::Writable for LUT_INP_MUX {}
#[doc = "LUTn input x MUX"]
pub mod lut_inp_mux;
