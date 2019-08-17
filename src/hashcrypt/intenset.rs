#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `WAITING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITING_A {
    #[doc = "Will not interrupt when waiting."]
    NO_INTERRUPT,
    #[doc = "Will interrupt when waiting"]
    INTERRUPT,
}
impl From<WAITING_A> for bool {
    #[inline(always)]
    fn from(variant: WAITING_A) -> Self {
        match variant {
            WAITING_A::NO_INTERRUPT => false,
            WAITING_A::INTERRUPT => true,
        }
    }
}
#[doc = "Reader of field `WAITING`"]
pub type WAITING_R = crate::R<bool, WAITING_A>;
impl WAITING_R {
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
        *self == WAITING_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WAITING_A::INTERRUPT
    }
}
#[doc = "Write proxy for field `WAITING`"]
pub struct WAITING_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITING_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `DIGEST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGEST_A {
    #[doc = "Will not interrupt when Digest is ready"]
    NO_INTERRUPT,
    #[doc = "Will interrupt when Digest is ready. Interrupt cleared by writing more data, starting a new Hash, or disabling (done)."]
    INTERRUPT,
}
impl From<DIGEST_A> for bool {
    #[inline(always)]
    fn from(variant: DIGEST_A) -> Self {
        match variant {
            DIGEST_A::NO_INTERRUPT => false,
            DIGEST_A::INTERRUPT => true,
        }
    }
}
#[doc = "Reader of field `DIGEST`"]
pub type DIGEST_R = crate::R<bool, DIGEST_A>;
impl DIGEST_R {
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
        *self == DIGEST_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == DIGEST_A::INTERRUPT
    }
}
#[doc = "Write proxy for field `DIGEST`"]
pub struct DIGEST_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIGEST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "Will not interrupt on Error."]
    NOT_INTERRUPT,
    #[doc = "Will interrupt on Error (until cleared)."]
    INTERRUPT,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        match variant {
            ERROR_A::NOT_INTERRUPT => false,
            ERROR_A::INTERRUPT => true,
        }
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, ERROR_A>;
impl ERROR_R {
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
        *self == ERROR_A::NOT_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == ERROR_A::INTERRUPT
    }
}
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
}
