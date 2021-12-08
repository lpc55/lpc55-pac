#[doc = "Register `MODULE_ID` reader"]
pub struct R(crate::R<MODULE_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULE_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULE_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULE_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APERTURE` reader - Aperture i."]
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
#[doc = "Field `MINOR_REV` reader - Minor revision i."]
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
#[doc = "Field `MAJOR_REV` reader - Major revision i."]
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
#[doc = "Field `ID` reader - Identifier."]
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
    #[doc = "Bits 0:7 - Aperture i."]
    #[inline(always)]
    pub fn aperture(&self) -> APERTURE_R {
        APERTURE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision i."]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision i."]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Identifier."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Controller+Memory module identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [module_id](index.html) module"]
pub struct MODULE_ID_SPEC;
impl crate::RegisterSpec for MODULE_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [module_id::R](R) reader structure"]
impl crate::Readable for MODULE_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULE_ID to value 0xc40f_0800"]
impl crate::Resettable for MODULE_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc40f_0800
    }
}
