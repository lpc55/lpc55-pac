#[doc = "Register `CODEINPUT` writer"]
pub struct W(crate::W<CODEINPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODEINPUT_SPEC>;
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
impl core::convert::From<crate::W<CODEINPUT_SPEC>> for W {
    fn from(writer: crate::W<CODEINPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODEIN` writer - AC/KC input data"]
pub struct CODEIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - AC/KC input data"]
    #[inline(always)]
    pub fn codein(&mut self) -> CODEIN_W {
        CODEIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Code Input register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codeinput](index.html) module"]
pub struct CODEINPUT_SPEC;
impl crate::RegisterSpec for CODEINPUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [codeinput::W](W) writer structure"]
impl crate::Writable for CODEINPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CODEINPUT to value 0"]
impl crate::Resettable for CODEINPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
