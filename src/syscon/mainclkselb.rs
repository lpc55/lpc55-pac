#[doc = "Register `MAINCLKSELB` reader"]
pub struct R(crate::R<MAINCLKSELB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKSELB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKSELB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKSELB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKSELB` writer"]
pub struct W(crate::W<MAINCLKSELB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKSELB_SPEC>;
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
impl From<crate::W<MAINCLKSELB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKSELB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Main clock source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main Clock A."]
    ENUM_0X0 = 0,
    #[doc = "1: PLL0 clock."]
    ENUM_0X1 = 1,
    #[doc = "2: PLL1 clock."]
    ENUM_0X2 = 2,
    #[doc = "3: Oscillator 32 kHz clock."]
    ENUM_0X3 = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Main clock source select."]
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
            0 => Some(SEL_A::ENUM_0X0),
            1 => Some(SEL_A::ENUM_0X1),
            2 => Some(SEL_A::ENUM_0X2),
            3 => Some(SEL_A::ENUM_0X3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0X0`"]
    #[inline(always)]
    pub fn is_enum_0x0(&self) -> bool {
        **self == SEL_A::ENUM_0X0
    }
    #[doc = "Checks if the value of the field is `ENUM_0X1`"]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        **self == SEL_A::ENUM_0X1
    }
    #[doc = "Checks if the value of the field is `ENUM_0X2`"]
    #[inline(always)]
    pub fn is_enum_0x2(&self) -> bool {
        **self == SEL_A::ENUM_0X2
    }
    #[doc = "Checks if the value of the field is `ENUM_0X3`"]
    #[inline(always)]
    pub fn is_enum_0x3(&self) -> bool {
        **self == SEL_A::ENUM_0X3
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Main clock source select."]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Main Clock A."]
    #[inline(always)]
    pub fn enum_0x0(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X0)
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X1)
    }
    #[doc = "PLL1 clock."]
    #[inline(always)]
    pub fn enum_0x2(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X2)
    }
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn enum_0x3(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Main clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Main clock source select."]
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
#[doc = "Main clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclkselb](index.html) module"]
pub struct MAINCLKSELB_SPEC;
impl crate::RegisterSpec for MAINCLKSELB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclkselb::R](R) reader structure"]
impl crate::Readable for MAINCLKSELB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclkselb::W](W) writer structure"]
impl crate::Writable for MAINCLKSELB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAINCLKSELB to value 0"]
impl crate::Resettable for MAINCLKSELB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
