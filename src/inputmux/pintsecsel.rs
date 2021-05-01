#[doc = "Register `PINTSECSEL[%s]` reader"]
pub struct R(crate::R<PINTSECSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINTSECSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PINTSECSEL_SPEC>> for R {
    fn from(reader: crate::R<PINTSECSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINTSECSEL[%s]` writer"]
pub struct W(crate::W<PINTSECSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINTSECSEL_SPEC>;
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
impl core::convert::From<crate::W<PINTSECSEL_SPEC>> for W {
    fn from(writer: crate::W<PINTSECSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTPIN` reader - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
pub struct INTPIN_R(crate::FieldReader<u8, u8>);
impl INTPIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTPIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTPIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTPIN` writer - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
pub struct INTPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    pub fn intpin(&self) -> INTPIN_R {
        INTPIN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    pub fn intpin(&mut self) -> INTPIN_W {
        INTPIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt secure select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsecsel](index.html) module"]
pub struct PINTSECSEL_SPEC;
impl crate::RegisterSpec for PINTSECSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pintsecsel::R](R) reader structure"]
impl crate::Readable for PINTSECSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pintsecsel::W](W) writer structure"]
impl crate::Writable for PINTSECSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINTSECSEL[%s]
to value 0x3f"]
impl crate::Resettable for PINTSECSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
