#[doc = "Register `ADCCLKSEL` reader"]
pub struct R(crate::R<ADCCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCLKSEL` writer"]
pub struct W(crate::W<ADCCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCLKSEL_SPEC>;
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
impl From<crate::W<ADCCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCLKSEL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SEL` reader - ADC clock source select"]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::MAINCLK),
            1 => Some(SEL_A::PLL0),
            2 => Some(SEL_A::FRO96),
            4 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAINCLK`"]
    #[inline(always)]
    pub fn is_mainclk(&self) -> bool {
        **self == SEL_A::MAINCLK
    }
    #[doc = "Checks if the value of the field is `PLL0`"]
    #[inline(always)]
    pub fn is_pll0(&self) -> bool {
        **self == SEL_A::PLL0
    }
    #[doc = "Checks if the value of the field is `FRO96`"]
    #[inline(always)]
    pub fn is_fro96(&self) -> bool {
        **self == SEL_A::FRO96
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SEL_A::NONE
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - ADC clock source select"]
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
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclksel](index.html) module"]
pub struct ADCCLKSEL_SPEC;
impl crate::RegisterSpec for ADCCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcclksel::R](R) reader structure"]
impl crate::Readable for ADCCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcclksel::W](W) writer structure"]
impl crate::Writable for ADCCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCLKSEL to value 0x07"]
impl crate::Resettable for ADCCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
