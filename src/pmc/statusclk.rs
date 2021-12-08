#[doc = "Register `STATUSCLK` reader"]
pub struct R(crate::R<STATUSCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUSCLK` writer"]
pub struct W(crate::W<STATUSCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUSCLK_SPEC>;
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
impl From<crate::W<STATUSCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUSCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32KOK` reader - XTAL oscillator 32 K OK signal."]
pub struct XTAL32KOK_R(crate::FieldReader<bool, bool>);
impl XTAL32KOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32KOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32KOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "XTAL32 KHZ oscillator oscillation failure detection indicator.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL32KOSCFAILURE_A {
    #[doc = "0: No oscillation failure has been detetced since the last time this bit has been cleared.."]
    NOFAIL = 0,
    #[doc = "1: At least one oscillation failure has been detetced since the last time this bit has been cleared.."]
    FAILURE = 1,
}
impl From<XTAL32KOSCFAILURE_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL32KOSCFAILURE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTAL32KOSCFAILURE` reader - XTAL32 KHZ oscillator oscillation failure detection indicator."]
pub struct XTAL32KOSCFAILURE_R(crate::FieldReader<bool, XTAL32KOSCFAILURE_A>);
impl XTAL32KOSCFAILURE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32KOSCFAILURE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL32KOSCFAILURE_A {
        match self.bits {
            false => XTAL32KOSCFAILURE_A::NOFAIL,
            true => XTAL32KOSCFAILURE_A::FAILURE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAIL`"]
    #[inline(always)]
    pub fn is_nofail(&self) -> bool {
        **self == XTAL32KOSCFAILURE_A::NOFAIL
    }
    #[doc = "Checks if the value of the field is `FAILURE`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        **self == XTAL32KOSCFAILURE_A::FAILURE
    }
}
impl core::ops::Deref for XTAL32KOSCFAILURE_R {
    type Target = crate::FieldReader<bool, XTAL32KOSCFAILURE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32KOSCFAILURE` writer - XTAL32 KHZ oscillator oscillation failure detection indicator."]
pub struct XTAL32KOSCFAILURE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32KOSCFAILURE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL32KOSCFAILURE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared.."]
    #[inline(always)]
    pub fn nofail(self) -> &'a mut W {
        self.variant(XTAL32KOSCFAILURE_A::NOFAIL)
    }
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared.."]
    #[inline(always)]
    pub fn failure(self) -> &'a mut W {
        self.variant(XTAL32KOSCFAILURE_A::FAILURE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XTAL oscillator 32 K OK signal."]
    #[inline(always)]
    pub fn xtal32kok(&self) -> XTAL32KOK_R {
        XTAL32KOK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    pub fn xtal32koscfailure(&self) -> XTAL32KOSCFAILURE_R {
        XTAL32KOSCFAILURE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline(always)]
    pub fn xtal32koscfailure(&mut self) -> XTAL32KOSCFAILURE_W {
        XTAL32KOSCFAILURE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusclk](index.html) module"]
pub struct STATUSCLK_SPEC;
impl crate::RegisterSpec for STATUSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusclk::R](R) reader structure"]
impl crate::Readable for STATUSCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statusclk::W](W) writer structure"]
impl crate::Writable for STATUSCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUSCLK to value 0x06"]
impl crate::Resettable for STATUSCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
