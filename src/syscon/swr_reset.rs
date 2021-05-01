#[doc = "Register `SWR_RESET` writer"]
pub struct W(crate::W<SWR_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWR_RESET_SPEC>;
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
impl core::convert::From<crate::W<SWR_RESET_SPEC>> for W {
    fn from(writer: crate::W<SWR_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write 0x5A00_0001 to generate a software_reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SWR_RESET_AW {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1509949441: Generate a software reset."]
    ASSERTED = 1509949441,
}
impl From<SWR_RESET_AW> for u32 {
    #[inline(always)]
    fn from(variant: SWR_RESET_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `SWR_RESET` writer - Write 0x5A00_0001 to generate a software_reset."]
pub struct SWR_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWR_RESET_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SWR_RESET_AW::RELEASED)
    }
    #[doc = "Generate a software reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SWR_RESET_AW::ASSERTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write 0x5A00_0001 to generate a software_reset."]
    #[inline(always)]
    pub fn swr_reset(&mut self) -> SWR_RESET_W {
        SWR_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "generate a software_reset\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swr_reset](index.html) module"]
pub struct SWR_RESET_SPEC;
impl crate::RegisterSpec for SWR_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swr_reset::W](W) writer structure"]
impl crate::Writable for SWR_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWR_RESET to value 0"]
impl crate::Resettable for SWR_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
