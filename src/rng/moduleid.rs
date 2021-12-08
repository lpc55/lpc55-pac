#[doc = "Register `MODULEID` reader"]
pub struct R(crate::R<MODULEID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODULEID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODULEID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODULEID_SPEC>) -> Self {
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
#[doc = "Field `MIN_REV` reader - Minor revision i."]
pub struct MIN_REV_R(crate::FieldReader<u8, u8>);
impl MIN_REV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MIN_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAJ_REV` reader - Major revision i."]
pub struct MAJ_REV_R(crate::FieldReader<u8, u8>);
impl MAJ_REV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJ_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJ_REV_R {
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
    pub fn min_rev(&self) -> MIN_REV_R {
        MIN_REV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision i."]
    #[inline(always)]
    pub fn maj_rev(&self) -> MAJ_REV_R {
        MAJ_REV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Identifier."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "IP identifier\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moduleid](index.html) module"]
pub struct MODULEID_SPEC;
impl crate::RegisterSpec for MODULEID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [moduleid::R](R) reader structure"]
impl crate::Readable for MODULEID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODULEID to value 0xa0b8_3200"]
impl crate::Resettable for MODULEID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa0b8_3200
    }
}
