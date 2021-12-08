#[doc = "Register `PID` reader"]
pub struct R(crate::R<PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APERTURE` reader - size aperture for the register port on the bus (APB or AHB)."]
pub struct APERTURE_R(crate::FieldReader<u8, u8>);
impl APERTURE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        APERTURE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APERTURE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINOR_REV` reader - Minor revision of module implementation."]
pub struct MINOR_REV_R(crate::FieldReader<u8, u8>);
impl MINOR_REV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MINOR_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINOR_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAJOR_REV` reader - Major revision of module implementation."]
pub struct MAJOR_REV_R(crate::FieldReader<u8, u8>);
impl MAJOR_REV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJOR_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJOR_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` reader - Module identifier for the selected function."]
pub struct ID_R(crate::FieldReader<u16, u16>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - size aperture for the register port on the bus (APB or AHB)."]
    #[inline(always)]
    pub fn aperture(&self) -> APERTURE_R {
        APERTURE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision of module implementation."]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision of module implementation."]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module identifier for the selected function."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Peripheral identification register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](index.html) module"]
pub struct PID_SPEC;
impl crate::RegisterSpec for PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid::R](R) reader structure"]
impl crate::Readable for PID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID to value 0"]
impl crate::Resettable for PID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
