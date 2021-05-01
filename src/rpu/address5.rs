#[doc = "Register `ADDRESS5` reader"]
pub struct R(crate::R<ADDRESS5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRESS5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADDRESS5_SPEC>> for R {
    fn from(reader: crate::R<ADDRESS5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRESS5` writer"]
pub struct W(crate::W<ADDRESS5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRESS5_SPEC>;
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
impl core::convert::From<crate::W<ADDRESS5_SPEC>> for W {
    fn from(writer: crate::W<ADDRESS5_SPEC>) -> Self {
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
#[doc = "Replacement address 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address5](index.html) module"]
pub struct ADDRESS5_SPEC;
impl crate::RegisterSpec for ADDRESS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [address5::R](R) reader structure"]
impl crate::Readable for ADDRESS5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [address5::W](W) writer structure"]
impl crate::Writable for ADDRESS5_SPEC {
    type Writer = W;
}
