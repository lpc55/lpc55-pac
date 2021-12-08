#[doc = "Register `RANDOM_NUMBER` reader"]
pub struct R(crate::R<RANDOM_NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANDOM_NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANDOM_NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANDOM_NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RANDOM_NUMBER` reader - This register contains a random 32 bit number which is computed on demand, at each time it is read."]
pub struct RANDOM_NUMBER_R(crate::FieldReader<u32, u32>);
impl RANDOM_NUMBER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RANDOM_NUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANDOM_NUMBER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is computed on demand, at each time it is read."]
    #[inline(always)]
    pub fn random_number(&self) -> RANDOM_NUMBER_R {
        RANDOM_NUMBER_R::new(self.bits as u32)
    }
}
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_number](index.html) module"]
pub struct RANDOM_NUMBER_SPEC;
impl crate::RegisterSpec for RANDOM_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [random_number::R](R) reader structure"]
impl crate::Readable for RANDOM_NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RANDOM_NUMBER to value 0"]
impl crate::Resettable for RANDOM_NUMBER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
