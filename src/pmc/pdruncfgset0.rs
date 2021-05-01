#[doc = "Register `PDRUNCFGSET0` writer"]
pub struct W(crate::W<PDRUNCFGSET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFGSET0_SPEC>;
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
impl core::convert::From<crate::W<PDRUNCFGSET0_SPEC>> for W {
    fn from(writer: crate::W<PDRUNCFGSET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDRUNCFGSET0` writer - Writing ones to this register sets the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
pub struct PDRUNCFGSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDRUNCFGSET0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register sets the corresponding bit or bits in the PDRUNCFG0 register, if they are implemented."]
    #[inline(always)]
    pub fn pdruncfgset0(&mut self) -> PDRUNCFGSET0_W {
        PDRUNCFGSET0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgset0](index.html) module"]
pub struct PDRUNCFGSET0_SPEC;
impl crate::RegisterSpec for PDRUNCFGSET0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pdruncfgset0::W](W) writer structure"]
impl crate::Writable for PDRUNCFGSET0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDRUNCFGSET0 to value 0"]
impl crate::Resettable for PDRUNCFGSET0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
