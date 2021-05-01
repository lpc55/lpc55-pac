#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTSTAT_SPEC>> for R {
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTAT` writer"]
pub struct W(crate::W<INTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTAT_SPEC>;
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
impl core::convert::From<crate::W<INTSTAT_SPEC>> for W {
    fn from(writer: crate::W<INTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY` reader - Triggers on falling edge of busy, write 1 to clear"]
pub struct READY_R(crate::FieldReader<bool, bool>);
impl READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READY` writer - Triggers on falling edge of busy, write 1 to clear"]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SUCCESS` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct SUCCESS_R(crate::FieldReader<bool, bool>);
impl SUCCESS_R {
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
#[doc = "Field `SUCCESS` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct SUCCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUCCESS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ERROR` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct ERROR_R(crate::FieldReader<bool, bool>);
impl ERROR_R {
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
#[doc = "Field `ERROR` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `KEYINREQ` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct KEYINREQ_R(crate::FieldReader<bool, bool>);
impl KEYINREQ_R {
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
#[doc = "Field `KEYINREQ` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct KEYINREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYINREQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `KEYOUTAVAIL` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct KEYOUTAVAIL_R(crate::FieldReader<bool, bool>);
impl KEYOUTAVAIL_R {
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
#[doc = "Field `KEYOUTAVAIL` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct KEYOUTAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYOUTAVAIL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CODEINREQ` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct CODEINREQ_R(crate::FieldReader<bool, bool>);
impl CODEINREQ_R {
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
#[doc = "Field `CODEINREQ` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct CODEINREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEINREQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CODEOUTAVAIL` reader - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct CODEOUTAVAIL_R(crate::FieldReader<bool, bool>);
impl CODEOUTAVAIL_R {
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
#[doc = "Field `CODEOUTAVAIL` writer - Level sensitive interrupt, cleared when interrupt source clears"]
pub struct CODEOUTAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEOUTAVAIL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Triggers on falling edge of busy, write 1 to clear"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn success(&self) -> SUCCESS_R {
        SUCCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyinreq(&self) -> KEYINREQ_R {
        KEYINREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyoutavail(&self) -> KEYOUTAVAIL_R {
        KEYOUTAVAIL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeinreq(&self) -> CODEINREQ_R {
        CODEINREQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeoutavail(&self) -> CODEOUTAVAIL_R {
        CODEOUTAVAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Triggers on falling edge of busy, write 1 to clear"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Bit 1 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn success(&mut self) -> SUCCESS_W {
        SUCCESS_W { w: self }
    }
    #[doc = "Bit 2 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 4 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyinreq(&mut self) -> KEYINREQ_W {
        KEYINREQ_W { w: self }
    }
    #[doc = "Bit 5 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyoutavail(&mut self) -> KEYOUTAVAIL_W {
        KEYOUTAVAIL_W { w: self }
    }
    #[doc = "Bit 6 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeinreq(&mut self) -> CODEINREQ_W {
        CODEINREQ_W { w: self }
    }
    #[doc = "Bit 7 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeoutavail(&mut self) -> CODEOUTAVAIL_W {
        CODEOUTAVAIL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstat::W](W) writer structure"]
impl crate::Writable for INTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
