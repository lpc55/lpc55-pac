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
#[doc = "Possible values of the field `DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "Do not interrupt when done"]
    NO_INTERRUPT,
    #[doc = "Interrupt when done"]
    INTERRUPT,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        match variant {
            DONE_A::NO_INTERRUPT => false,
            DONE_A::INTERRUPT => true,
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
            false => DONE_A::NO_INTERRUPT,
            true => DONE_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == DONE_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == DONE_A::INTERRUPT
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
    #[doc = "Do not interrupt when done"]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(DONE_A::NO_INTERRUPT)
    }
    #[doc = "Interrupt when done"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(DONE_A::INTERRUPT)
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
    #[doc = "Bit 0 - Set if the accelerator should interrupt when done."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set if the accelerator should interrupt when done."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
}
