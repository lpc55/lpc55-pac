#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTEN_SPEC>> for R {
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl core::convert::From<crate::W<INTEN_SPEC>> for W {
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READYEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct READYEN_R(crate::FieldReader<bool, bool>);
impl READYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        READYEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READYEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READYEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct READYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> READYEN_W<'a> {
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
#[doc = "Field `SUCCESEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct SUCCESEN_R(crate::FieldReader<bool, bool>);
impl SUCCESEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUCCESEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUCCESEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUCCESEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct SUCCESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUCCESEN_W<'a> {
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
#[doc = "Field `ERROREN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct ERROREN_R(crate::FieldReader<bool, bool>);
impl ERROREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROREN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct ERROREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROREN_W<'a> {
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
#[doc = "Field `KEYINREQEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct KEYINREQEN_R(crate::FieldReader<bool, bool>);
impl KEYINREQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEYINREQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYINREQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYINREQEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct KEYINREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYINREQEN_W<'a> {
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
#[doc = "Field `KEYOUTAVAILEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct KEYOUTAVAILEN_R(crate::FieldReader<bool, bool>);
impl KEYOUTAVAILEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEYOUTAVAILEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYOUTAVAILEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYOUTAVAILEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct KEYOUTAVAILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYOUTAVAILEN_W<'a> {
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
#[doc = "Field `CODEINREQEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct CODEINREQEN_R(crate::FieldReader<bool, bool>);
impl CODEINREQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CODEINREQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CODEINREQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CODEINREQEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct CODEINREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEINREQEN_W<'a> {
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
#[doc = "Field `CODEOUTAVAILEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct CODEOUTAVAILEN_R(crate::FieldReader<bool, bool>);
impl CODEOUTAVAILEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CODEOUTAVAILEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CODEOUTAVAILEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CODEOUTAVAILEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub struct CODEOUTAVAILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEOUTAVAILEN_W<'a> {
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
    #[doc = "Bit 0 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn readyen(&self) -> READYEN_R {
        READYEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn succesen(&self) -> SUCCESEN_R {
        SUCCESEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn erroren(&self) -> ERROREN_R {
        ERROREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyinreqen(&self) -> KEYINREQEN_R {
        KEYINREQEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyoutavailen(&self) -> KEYOUTAVAILEN_R {
        KEYOUTAVAILEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeinreqen(&self) -> CODEINREQEN_R {
        CODEINREQEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeoutavailen(&self) -> CODEOUTAVAILEN_R {
        CODEOUTAVAILEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn readyen(&mut self) -> READYEN_W {
        READYEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn succesen(&mut self) -> SUCCESEN_W {
        SUCCESEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn erroren(&mut self) -> ERROREN_W {
        ERROREN_W { w: self }
    }
    #[doc = "Bit 4 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyinreqen(&mut self) -> KEYINREQEN_W {
        KEYINREQEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyoutavailen(&mut self) -> KEYOUTAVAILEN_W {
        KEYOUTAVAILEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeinreqen(&mut self) -> CODEINREQEN_W {
        CODEINREQEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeoutavailen(&mut self) -> CODEOUTAVAILEN_W {
        CODEOUTAVAILEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
