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
#[doc = "Possible values of the field `MCLKIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLKIO_A {
    #[doc = "input mode."]
    INPUT,
    #[doc = "output mode."]
    OUTPUT,
}
impl From<MCLKIO_A> for u32 {
    #[inline(always)]
    fn from(variant: MCLKIO_A) -> Self {
        match variant {
            MCLKIO_A::INPUT => 0,
            MCLKIO_A::OUTPUT => 1,
        }
    }
}
#[doc = "Reader of field `MCLKIO`"]
pub type MCLKIO_R = crate::R<u32, MCLKIO_A>;
impl MCLKIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, MCLKIO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCLKIO_A::INPUT),
            1 => Val(MCLKIO_A::OUTPUT),
            i => Res(i),
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
        unsafe { self.bits(variant.into()) }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MCLK control."]
    #[inline(always)]
    pub fn mclkio(&self) -> MCLKIO_R {
        MCLKIO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MCLK control."]
    #[inline(always)]
    pub fn mclkio(&mut self) -> MCLKIO_W {
        MCLKIO_W { w: self }
    }
}
