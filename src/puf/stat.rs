#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `busy` reader - Indicates that operation is in progress"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUCCESS` reader - Last operation was successful"]
pub struct SUCCESS_R(crate::FieldReader<bool, bool>);
impl SUCCESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUCCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUCCESS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `error` reader - PUF is in the Error state and no operations can be performed"]
pub struct ERROR_R(crate::FieldReader<bool, bool>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYINREQ` reader - Request for next part of key"]
pub struct KEYINREQ_R(crate::FieldReader<bool, bool>);
impl KEYINREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEYINREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYINREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYOUTAVAIL` reader - Next part of key is available"]
pub struct KEYOUTAVAIL_R(crate::FieldReader<bool, bool>);
impl KEYOUTAVAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KEYOUTAVAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYOUTAVAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CODEINREQ` reader - Request for next part of AC/KC"]
pub struct CODEINREQ_R(crate::FieldReader<bool, bool>);
impl CODEINREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CODEINREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CODEINREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CODEOUTAVAIL` reader - Next part of AC/KC is available"]
pub struct CODEOUTAVAIL_R(crate::FieldReader<bool, bool>);
impl CODEOUTAVAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CODEOUTAVAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CODEOUTAVAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates that operation is in progress"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Last operation was successful"]
    #[inline(always)]
    pub fn success(&self) -> SUCCESS_R {
        SUCCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PUF is in the Error state and no operations can be performed"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Request for next part of key"]
    #[inline(always)]
    pub fn keyinreq(&self) -> KEYINREQ_R {
        KEYINREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Next part of key is available"]
    #[inline(always)]
    pub fn keyoutavail(&self) -> KEYOUTAVAIL_R {
        KEYOUTAVAIL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Request for next part of AC/KC"]
    #[inline(always)]
    pub fn codeinreq(&self) -> CODEINREQ_R {
        CODEINREQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Next part of AC/KC is available"]
    #[inline(always)]
    pub fn codeoutavail(&self) -> CODEOUTAVAIL_R {
        CODEOUTAVAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0x01"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
