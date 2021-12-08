#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Indicates if should interrupt when waiting for data input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITING_A {
    #[doc = "0: Will not interrupt when waiting."]
    NO_INTERRUPT = 0,
    #[doc = "1: Will interrupt when waiting"]
    INTERRUPT = 1,
}
impl From<WAITING_A> for bool {
    #[inline(always)]
    fn from(variant: WAITING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITING` reader - Indicates if should interrupt when waiting for data input."]
pub struct WAITING_R(crate::FieldReader<bool, WAITING_A>);
impl WAITING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITING_A {
        match self.bits {
            false => WAITING_A::NO_INTERRUPT,
            true => WAITING_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        **self == WAITING_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        **self == WAITING_A::INTERRUPT
    }
}
impl core::ops::Deref for WAITING_R {
    type Target = crate::FieldReader<bool, WAITING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITING` writer - Indicates if should interrupt when waiting for data input."]
pub struct WAITING_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Will not interrupt when waiting."]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(WAITING_A::NO_INTERRUPT)
    }
    #[doc = "Will interrupt when waiting"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WAITING_A::INTERRUPT)
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
#[doc = "Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGEST_A {
    #[doc = "0: Will not interrupt when Digest is ready"]
    NO_INTERRUPT = 0,
    #[doc = "1: Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    INTERRUPT = 1,
}
impl From<DIGEST_A> for bool {
    #[inline(always)]
    fn from(variant: DIGEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGEST` reader - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
pub struct DIGEST_R(crate::FieldReader<bool, DIGEST_A>);
impl DIGEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIGEST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIGEST_A {
        match self.bits {
            false => DIGEST_A::NO_INTERRUPT,
            true => DIGEST_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        **self == DIGEST_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        **self == DIGEST_A::INTERRUPT
    }
}
impl core::ops::Deref for DIGEST_R {
    type Target = crate::FieldReader<bool, DIGEST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIGEST` writer - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
pub struct DIGEST_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIGEST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Will not interrupt when Digest is ready"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(DIGEST_A::NO_INTERRUPT)
    }
    #[doc = "Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(DIGEST_A::INTERRUPT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Indicates if should interrupt on an ERROR (as defined in Status)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Will not interrupt on Error."]
    NOT_INTERRUPT = 0,
    #[doc = "1: Will interrupt on Error (until cleared)."]
    INTERRUPT = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Indicates if should interrupt on an ERROR (as defined in Status)"]
pub struct ERROR_R(crate::FieldReader<bool, ERROR_A>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::NOT_INTERRUPT,
            true => ERROR_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INTERRUPT`"]
    #[inline(always)]
    pub fn is_not_interrupt(&self) -> bool {
        **self == ERROR_A::NOT_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        **self == ERROR_A::INTERRUPT
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` writer - Indicates if should interrupt on an ERROR (as defined in Status)"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Will not interrupt on Error."]
    #[inline(always)]
    pub fn not_interrupt(self) -> &'a mut W {
        self.variant(ERROR_A::NOT_INTERRUPT)
    }
    #[doc = "Will interrupt on Error (until cleared)."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(ERROR_A::INTERRUPT)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if should interrupt when waiting for data input."]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates if should interrupt on an ERROR (as defined in Status)"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if should interrupt when waiting for data input."]
    #[inline(always)]
    pub fn waiting(&mut self) -> WAITING_W {
        WAITING_W { w: self }
    }
    #[doc = "Bit 1 - Indicates if should interrupt when Digest (or Outdata) is ready (completed a hash/crypto or completed a full sequence)."]
    #[inline(always)]
    pub fn digest(&mut self) -> DIGEST_W {
        DIGEST_W { w: self }
    }
    #[doc = "Bit 2 - Indicates if should interrupt on an ERROR (as defined in Status)"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write 1 to enable interrupts; reads back with which are set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
