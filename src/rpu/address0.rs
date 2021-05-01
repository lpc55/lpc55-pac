#[doc = "Register `ADDRESS0` reader"]
pub struct R(crate::R<ADDRESS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRESS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADDRESS0_SPEC>> for R {
    fn from(reader: crate::R<ADDRESS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRESS0` writer"]
pub struct W(crate::W<ADDRESS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRESS0_SPEC>;
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
impl core::convert::From<crate::W<ADDRESS0_SPEC>> for W {
    fn from(writer: crate::W<ADDRESS0_SPEC>) -> Self {
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
#[doc = "Replacement address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address0](index.html) module"]
pub struct ADDRESS0_SPEC;
impl crate::RegisterSpec for ADDRESS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [address0::R](R) reader structure"]
impl crate::Readable for ADDRESS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [address0::W](W) writer structure"]
impl crate::Writable for ADDRESS0_SPEC {
    type Writer = W;
}
