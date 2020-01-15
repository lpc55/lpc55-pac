#[doc = "Reader of register MCLKIO"]
pub type R = crate::R<u32, super::MCLKIO>;
#[doc = "Writer for register MCLKIO"]
pub type W = crate::W<u32, super::MCLKIO>;
#[doc = "Register MCLKIO `reset()`'s with value 0"]
impl crate::ResetValue for super::MCLKIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCLK control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKIO_A {
    #[doc = "0: input mode."]
    INPUT = 0,
    #[doc = "1: output mode."]
    OUTPUT = 1,
}
impl From<MCLKIO_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCLKIO`"]
pub type MCLKIO_R = crate::R<bool, MCLKIO_A>;
impl MCLKIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKIO_A {
        match self.bits {
            false => MCLKIO_A::INPUT,
            true => MCLKIO_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MCLKIO_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MCLKIO_A::OUTPUT
    }
}
#[doc = "Write proxy for field `MCLKIO`"]
pub struct MCLKIO_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "input mode."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MCLKIO_A::INPUT)
    }
    #[doc = "output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MCLKIO_A::OUTPUT)
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
    #[doc = "Bit 0 - MCLK control."]
    #[inline(always)]
    pub fn mclkio(&self) -> MCLKIO_R {
        MCLKIO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCLK control."]
    #[inline(always)]
    pub fn mclkio(&mut self) -> MCLKIO_W {
        MCLKIO_W { w: self }
    }
}
