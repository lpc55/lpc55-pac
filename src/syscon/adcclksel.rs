#[doc = "Reader of register ADCCLKSEL"]
pub type R = crate::R<u32, super::ADCCLKSEL>;
#[doc = "Writer for register ADCCLKSEL"]
pub type W = crate::W<u32, super::ADCCLKSEL>;
#[doc = "Register ADCCLKSEL `reset()`'s with value 0x07"]
impl crate::ResetValue for super::ADCCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "ADC clock source select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main clk."]
    MAINCLK = 0,
    #[doc = "1: PLL0 clk."]
    PLL0 = 1,
    #[doc = "2: FRO 96 MHZ clk."]
    FRO96 = 2,
    #[doc = "4: No clk."]
    NONE = 4,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::MAINCLK),
            1 => Val(SEL_A::PLL0),
            2 => Val(SEL_A::FRO96),
            4 => Val(SEL_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MAINCLK`"]
    #[inline(always)]
    pub fn is_mainclk(&self) -> bool {
        *self == SEL_A::MAINCLK
    }
    #[doc = "Checks if the value of the field is `PLL0`"]
    #[inline(always)]
    pub fn is_pll0(&self) -> bool {
        *self == SEL_A::PLL0
    }
    #[doc = "Checks if the value of the field is `FRO96`"]
    #[inline(always)]
    pub fn is_fro96(&self) -> bool {
        *self == SEL_A::FRO96
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Main clk."]
    #[inline(always)]
    pub fn mainclk(self) -> &'a mut W {
        self.variant(SEL_A::MAINCLK)
    }
    #[doc = "PLL0 clk."]
    #[inline(always)]
    pub fn pll0(self) -> &'a mut W {
        self.variant(SEL_A::PLL0)
    }
    #[doc = "FRO 96 MHZ clk."]
    #[inline(always)]
    pub fn fro96(self) -> &'a mut W {
        self.variant(SEL_A::FRO96)
    }
    #[doc = "No clk."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC clock source select"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC clock source select"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
