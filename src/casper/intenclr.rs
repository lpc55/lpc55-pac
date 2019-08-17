#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u32, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "If written 0, ignored"]
    IGNORED,
    #[doc = "If written 1, do not Interrupt when done"]
    NO_INTERRUPT,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        match variant {
            DONE_A::IGNORED => false,
            DONE_A::NO_INTERRUPT => true,
        }
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, DONE_A>;
impl DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::IGNORED,
            true => DONE_A::NO_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORED`"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        *self == DONE_A::IGNORED
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == DONE_A::NO_INTERRUPT
    }
}
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If written 0, ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(DONE_A::IGNORED)
    }
    #[doc = "If written 1, do not Interrupt when done"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(DONE_A::NO_INTERRUPT)
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
impl R {
    #[doc = "Bit 0 - Written to clear an interrupt set with INTENSET."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Written to clear an interrupt set with INTENSET."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
}
