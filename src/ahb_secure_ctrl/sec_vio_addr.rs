#[doc = "Register `sec_vio_addr[%s]` reader"]
pub struct R(crate::R<SEC_VIO_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_VIO_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_VIO_ADDR_SPEC>> for R {
    fn from(reader: crate::R<SEC_VIO_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC_VIO_ADDR` reader - security violation address for AHB port"]
pub struct SEC_VIO_ADDR_R(crate::FieldReader<u32, u32>);
impl SEC_VIO_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        SEC_VIO_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_VIO_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - security violation address for AHB port"]
    #[inline(always)]
    pub fn sec_vio_addr(&self) -> SEC_VIO_ADDR_R {
        SEC_VIO_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "most recent security violation address for AHB port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_vio_addr](index.html) module"]
pub struct SEC_VIO_ADDR_SPEC;
impl crate::RegisterSpec for SEC_VIO_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_vio_addr::R](R) reader structure"]
impl crate::Readable for SEC_VIO_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sec_vio_addr[%s]
to value 0"]
impl crate::Resettable for SEC_VIO_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
