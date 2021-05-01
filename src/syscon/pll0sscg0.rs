#[doc = "Register `PLL0SSCG0` reader"]
pub struct R(crate::R<PLL0SSCG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL0SSCG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PLL0SSCG0_SPEC>> for R {
    fn from(reader: crate::R<PLL0SSCG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL0SSCG0` writer"]
pub struct W(crate::W<PLL0SSCG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL0SSCG0_SPEC>;
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
impl core::convert::From<crate::W<PLL0SSCG0_SPEC>> for W {
    fn from(writer: crate::W<PLL0SSCG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD_LBS` reader - input word of the wrapper bit 31 to 0."]
pub struct MD_LBS_R(crate::FieldReader<u32, u32>);
impl MD_LBS_R {
    pub(crate) fn new(bits: u32) -> Self {
        MD_LBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_LBS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MD_LBS` writer - input word of the wrapper bit 31 to 0."]
pub struct MD_LBS_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_LBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - input word of the wrapper bit 31 to 0."]
    #[inline(always)]
    pub fn md_lbs(&self) -> MD_LBS_R {
        MD_LBS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - input word of the wrapper bit 31 to 0."]
    #[inline(always)]
    pub fn md_lbs(&mut self) -> MD_LBS_W {
        MD_LBS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 Spread Spectrum Wrapper control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0sscg0](index.html) module"]
pub struct PLL0SSCG0_SPEC;
impl crate::RegisterSpec for PLL0SSCG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll0sscg0::R](R) reader structure"]
impl crate::Readable for PLL0SSCG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll0sscg0::W](W) writer structure"]
impl crate::Writable for PLL0SSCG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL0SSCG0 to value 0"]
impl crate::Resettable for PLL0SSCG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
