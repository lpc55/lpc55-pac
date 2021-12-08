#[doc = "Register `LOCK` reader"]
pub struct R(crate::R<LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK` writer"]
pub struct W(crate::W<LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_SPEC>;
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
impl From<crate::W<LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: unlock"]
    UNLOCK = 0,
    #[doc = "1: Lock to current security level"]
    LOCK = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCK,
            true => LOCK_A::LOCK,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        **self == LOCK_A::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        **self == LOCK_A::LOCK
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "unlock"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCK)
    }
    #[doc = "Lock to current security level"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCK_A::LOCK)
    }
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
#[doc = "Must be written as 0x73D to change the register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum KEY_A {
    #[doc = "1853: If set during write, will allow lock or unlock"]
    KWY_VALUE = 1853,
}
impl From<KEY_A> for u16 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - Must be written as 0x73D to change the register."]
pub struct KEY_R(crate::FieldReader<u16, KEY_A>);
impl KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            1853 => Some(KEY_A::KWY_VALUE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KWY_VALUE`"]
    #[inline(always)]
    pub fn is_kwy_value(&self) -> bool {
        **self == KEY_A::KWY_VALUE
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u16, KEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Must be written as 0x73D to change the register."]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "If set during write, will allow lock or unlock"]
    #[inline(always)]
    pub fn kwy_value(self) -> &'a mut W {
        self.variant(KEY_A::KWY_VALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 4)) | ((value as u32 & 0x1fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:16 - Must be written as 0x73D to change the register."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 4) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bits 4:16 - Must be written as 0x73D to change the register."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](index.html) module"]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock::R](R) reader structure"]
impl crate::Readable for LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock::W](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
