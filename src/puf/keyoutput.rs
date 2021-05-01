#[doc = "Register `KEYOUTPUT` reader"]
pub struct R(crate::R<KEYOUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYOUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<KEYOUTPUT_SPEC>> for R {
    fn from(reader: crate::R<KEYOUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEYOUT` reader - Key output data"]
pub struct KEYOUT_R(crate::FieldReader<u32, u32>);
impl KEYOUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEYOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYOUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Key output data"]
    #[inline(always)]
    pub fn keyout(&self) -> KEYOUT_R {
        KEYOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "PUF Key Output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyoutput](index.html) module"]
pub struct KEYOUTPUT_SPEC;
impl crate::RegisterSpec for KEYOUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyoutput::R](R) reader structure"]
impl crate::Readable for KEYOUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets KEYOUTPUT to value 0"]
impl crate::Resettable for KEYOUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
