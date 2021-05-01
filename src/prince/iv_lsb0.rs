#[doc = "Register `IV_LSB0` writer"]
pub struct W(crate::W<IV_LSB0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV_LSB0_SPEC>;
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
impl core::convert::From<crate::W<IV_LSB0_SPEC>> for W {
    fn from(writer: crate::W<IV_LSB0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVVAL` writer - Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
pub struct IVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> IVVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub fn ivval(&mut self) -> IVVAL_W {
        IVVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initial Vector register for region 0, Least Significant Bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_lsb0](index.html) module"]
pub struct IV_LSB0_SPEC;
impl crate::RegisterSpec for IV_LSB0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [iv_lsb0::W](W) writer structure"]
impl crate::Writable for IV_LSB0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV_LSB0 to value 0"]
impl crate::Resettable for IV_LSB0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
