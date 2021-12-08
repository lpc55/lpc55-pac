#[doc = "Register `DEVICE_ID0` reader"]
pub struct R(crate::R<DEVICE_ID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_ID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICE_ID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICE_ID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ROM_REV_MINOR` reader - ROM revision."]
pub struct ROM_REV_MINOR_R(crate::FieldReader<u8, u8>);
impl ROM_REV_MINOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_REV_MINOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_REV_MINOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 20:23 - ROM revision."]
    #[inline(always)]
    pub fn rom_rev_minor(&self) -> ROM_REV_MINOR_R {
        ROM_REV_MINOR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "Device ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id0](index.html) module"]
pub struct DEVICE_ID0_SPEC;
impl crate::RegisterSpec for DEVICE_ID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_id0::R](R) reader structure"]
impl crate::Readable for DEVICE_ID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICE_ID0 to value 0"]
impl crate::Resettable for DEVICE_ID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
