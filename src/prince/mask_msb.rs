#[doc = "Register `MASK_MSB` writer"]
pub struct W(crate::W<MASK_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_MSB_SPEC>;
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
impl core::convert::From<crate::W<MASK_MSB_SPEC>> for W {
    fn from(writer: crate::W<MASK_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASKVAL` writer - Value of the 32 Most Significant Bits of the 64-bit data mask."]
pub struct MASKVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32 Most Significant Bits of the 64-bit data mask."]
    #[inline(always)]
    pub fn maskval(&mut self) -> MASKVAL_W {
        MASKVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Mask register, 32 Most Significant Bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask_msb](index.html) module"]
pub struct MASK_MSB_SPEC;
impl crate::RegisterSpec for MASK_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mask_msb::W](W) writer structure"]
impl crate::Writable for MASK_MSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK_MSB to value 0"]
impl crate::Resettable for MASK_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
