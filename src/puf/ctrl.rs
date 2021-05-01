#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CTRL_SPEC>> for R {
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl core::convert::From<crate::W<CTRL_SPEC>> for W {
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `zeroize` reader - Begin Zeroize operation for PUF and go to Error state"]
pub struct ZEROIZE_R(crate::FieldReader<bool, bool>);
impl ZEROIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZEROIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZEROIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `zeroize` writer - Begin Zeroize operation for PUF and go to Error state"]
pub struct ZEROIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ZEROIZE_W<'a> {
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
#[doc = "Field `enroll` reader - Begin Enroll operation"]
pub struct ENROLL_R(crate::FieldReader<bool, bool>);
impl ENROLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENROLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENROLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enroll` writer - Begin Enroll operation"]
pub struct ENROLL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENROLL_W<'a> {
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
#[doc = "Field `start` reader - Begin Start operation"]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `start` writer - Begin Start operation"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `GENERATEKEY` reader - Begin Set Intrinsic Key operation"]
pub struct GENERATEKEY_R(crate::FieldReader<bool, bool>);
impl GENERATEKEY_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENERATEKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERATEKEY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENERATEKEY` writer - Begin Set Intrinsic Key operation"]
pub struct GENERATEKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERATEKEY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SETKEY` reader - Begin Set User Key operation"]
pub struct SETKEY_R(crate::FieldReader<bool, bool>);
impl SETKEY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETKEY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETKEY` writer - Begin Set User Key operation"]
pub struct SETKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SETKEY_W<'a> {
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
#[doc = "Field `GETKEY` reader - Begin Get Key operation"]
pub struct GETKEY_R(crate::FieldReader<bool, bool>);
impl GETKEY_R {
    pub(crate) fn new(bits: bool) -> Self {
        GETKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GETKEY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GETKEY` writer - Begin Get Key operation"]
pub struct GETKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> GETKEY_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Begin Zeroize operation for PUF and go to Error state"]
    #[inline(always)]
    pub fn zeroize(&self) -> ZEROIZE_R {
        ZEROIZE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Begin Enroll operation"]
    #[inline(always)]
    pub fn enroll(&self) -> ENROLL_R {
        ENROLL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Begin Start operation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Begin Set Intrinsic Key operation"]
    #[inline(always)]
    pub fn generatekey(&self) -> GENERATEKEY_R {
        GENERATEKEY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Begin Set User Key operation"]
    #[inline(always)]
    pub fn setkey(&self) -> SETKEY_R {
        SETKEY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Begin Get Key operation"]
    #[inline(always)]
    pub fn getkey(&self) -> GETKEY_R {
        GETKEY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Begin Zeroize operation for PUF and go to Error state"]
    #[inline(always)]
    pub fn zeroize(&mut self) -> ZEROIZE_W {
        ZEROIZE_W { w: self }
    }
    #[doc = "Bit 1 - Begin Enroll operation"]
    #[inline(always)]
    pub fn enroll(&mut self) -> ENROLL_W {
        ENROLL_W { w: self }
    }
    #[doc = "Bit 2 - Begin Start operation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 3 - Begin Set Intrinsic Key operation"]
    #[inline(always)]
    pub fn generatekey(&mut self) -> GENERATEKEY_W {
        GENERATEKEY_W { w: self }
    }
    #[doc = "Bit 4 - Begin Set User Key operation"]
    #[inline(always)]
    pub fn setkey(&mut self) -> SETKEY_W {
        SETKEY_W { w: self }
    }
    #[doc = "Bit 6 - Begin Get Key operation"]
    #[inline(always)]
    pub fn getkey(&mut self) -> GETKEY_W {
        GETKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
