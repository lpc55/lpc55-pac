#[doc = "Register `VALUE1` reader"]
pub struct R(crate::R<VALUE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VALUE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VALUE1_SPEC>> for R {
    fn from(reader: crate::R<VALUE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VALUE1` writer"]
pub struct W(crate::W<VALUE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VALUE1_SPEC>;
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
impl core::convert::From<crate::W<VALUE1_SPEC>> for W {
    fn from(writer: crate::W<VALUE1_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value replacement 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value1](index.html) module"]
pub struct VALUE1_SPEC;
impl crate::RegisterSpec for VALUE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [value1::R](R) reader structure"]
impl crate::Readable for VALUE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [value1::W](W) writer structure"]
impl crate::Writable for VALUE1_SPEC {
    type Writer = W;
}
