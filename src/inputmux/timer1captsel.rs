#[doc = "Register `TIMER1CAPTSEL[%s]` reader"]
pub struct R(crate::R<TIMER1CAPTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1CAPTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1CAPTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1CAPTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1CAPTSEL[%s]` writer"]
pub struct W(crate::W<TIMER1CAPTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1CAPTSEL_SPEC>;
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
impl From<crate::W<TIMER1CAPTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1CAPTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTSEL` reader - Input number to TIMER%s capture inputs 0 to 4"]
pub struct CAPTSEL_R(crate::FieldReader<u8, u8>);
impl CAPTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAPTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTSEL` writer - Input number to TIMER%s capture inputs 0 to 4"]
pub struct CAPTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input number to TIMER%s capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&self) -> CAPTSEL_R {
        CAPTSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to TIMER%s capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&mut self) -> CAPTSEL_W {
        CAPTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture select registers for TIMER1 inputs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1captsel](index.html) module"]
pub struct TIMER1CAPTSEL_SPEC;
impl crate::RegisterSpec for TIMER1CAPTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1captsel::R](R) reader structure"]
impl crate::Readable for TIMER1CAPTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1captsel::W](W) writer structure"]
impl crate::Writable for TIMER1CAPTSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1CAPTSEL[%s]
to value 0x1f"]
impl crate::Resettable for TIMER1CAPTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
