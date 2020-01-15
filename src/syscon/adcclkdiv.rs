#[doc = "Reader of register ADCCLKDIV"]
pub type R = crate::R<u32, super::ADCCLKDIV>;
#[doc = "Writer for register ADCCLKDIV"]
pub type W = crate::W<u32, super::ADCCLKDIV>;
#[doc = "Register ADCCLKDIV `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::ADCCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0000
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Resets the divider counter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_AW {
    #[doc = "0: Divider is not reset."]
    RELEASED = 0,
    #[doc = "1: Divider is reset."]
    ASSERTED = 1,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Divider is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(RESET_AW::RELEASED)
    }
    #[doc = "Divider is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RESET_AW::ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Halts the divider counter.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: Divider clock is running."]
    RUN = 0,
    #[doc = "1: Divider clock is stoped."]
    HALT = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HALT`"]
pub type HALT_R = crate::R<bool, HALT_A>;
impl HALT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::RUN,
            true => HALT_A::HALT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == HALT_A::RUN
    }
    #[doc = "Checks if the value of the field is `HALT`"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == HALT_A::HALT
    }
}
#[doc = "Write proxy for field `HALT`"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Divider clock is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(HALT_A::RUN)
    }
    #[doc = "Divider clock is stoped."]
    #[inline(always)]
    pub fn halt(self) -> &'a mut W {
        self.variant(HALT_A::HALT)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Divider status flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQFLAG_A {
    #[doc = "0: Divider clock is stable."]
    STABLE = 0,
    #[doc = "1: Clock frequency is not stable."]
    ONGOING = 1,
}
impl From<REQFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: REQFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REQFLAG`"]
pub type REQFLAG_R = crate::R<bool, REQFLAG_A>;
impl REQFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQFLAG_A {
        match self.bits {
            false => REQFLAG_A::STABLE,
            true => REQFLAG_A::ONGOING,
        }
    }
    #[doc = "Checks if the value of the field is `STABLE`"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == REQFLAG_A::STABLE
    }
    #[doc = "Checks if the value of the field is `ONGOING`"]
    #[inline(always)]
    pub fn is_ongoing(&self) -> bool {
        *self == REQFLAG_A::ONGOING
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock divider value."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Divider status flag."]
    #[inline(always)]
    pub fn reqflag(&self) -> REQFLAG_R {
        REQFLAG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock divider value."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bit 29 - Resets the divider counter."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
}
