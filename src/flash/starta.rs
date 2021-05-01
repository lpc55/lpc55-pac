#[doc = "Register `STARTA` reader"]
pub struct R(crate::R<STARTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STARTA_SPEC>> for R {
    fn from(reader: crate::R<STARTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTA` writer"]
pub struct W(crate::W<STARTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTA_SPEC>;
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
impl core::convert::From<crate::W<STARTA_SPEC>> for W {
    fn from(writer: crate::W<STARTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTA` reader - Address / Start address for commands that take an address (range) as a parameter."]
pub struct STARTA_R(crate::FieldReader<u32, u32>);
impl STARTA_R {
    pub(crate) fn new(bits: u32) -> Self {
        STARTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTA` writer - Address / Start address for commands that take an address (range) as a parameter."]
pub struct STARTA_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Address / Start address for commands that take an address (range) as a parameter."]
    #[inline(always)]
    pub fn starta(&self) -> STARTA_R {
        STARTA_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Address / Start address for commands that take an address (range) as a parameter."]
    #[inline(always)]
    pub fn starta(&mut self) -> STARTA_W {
        STARTA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "start (or only) address for next flash command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starta](index.html) module"]
pub struct STARTA_SPEC;
impl crate::RegisterSpec for STARTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starta::R](R) reader structure"]
impl crate::Readable for STARTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starta::W](W) writer structure"]
impl crate::Writable for STARTA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTA to value 0"]
impl crate::Resettable for STARTA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
