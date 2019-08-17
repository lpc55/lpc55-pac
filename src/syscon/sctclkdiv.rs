#[doc = "Reader of register SCTCLKDIV"]
pub type R = crate::R<u32, super::SCTCLKDIV>;
#[doc = "Writer for register SCTCLKDIV"]
pub type W = crate::W<u32, super::SCTCLKDIV>;
#[doc = "Register SCTCLKDIV `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::SCTCLKDIV {
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
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Possible values of the field `RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_AW {
    #[doc = "Divider is not reset."]
    RELEASED,
    #[doc = "Divider is reset."]
    ASSERTED,
}
impl From<RESET_AW> for bool {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        match variant {
            RESET_AW::RELEASED => false,
            RESET_AW::ASSERTED => true,
        }
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
#[doc = "Possible values of the field `HALT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "Divider clock is running."]
    RUN,
    #[doc = "Divider clock is stoped."]
    HALT,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        match variant {
            HALT_A::RUN => false,
            HALT_A::HALT => true,
        }
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
#[doc = "Possible values of the field `REQFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQFLAG_A {
    #[doc = "Divider clock is stable."]
    STABLE,
    #[doc = "Clock frequency is not stable."]
    ONGOING,
}
impl From<REQFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: REQFLAG_A) -> Self {
        match variant {
            REQFLAG_A::STABLE => false,
            REQFLAG_A::ONGOING => true,
        }
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
    #[doc = "Bits 0:7 - Clock divider value."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
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
    #[doc = "Bits 0:7 - Clock divider value."]
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
