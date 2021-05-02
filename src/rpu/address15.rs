#[doc = "Register `ADDRESS15` reader"]
pub struct R(crate::R<ADDRESS15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRESS15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADDRESS15_SPEC>> for R {
    fn from(reader: crate::R<ADDRESS15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRESS15` writer"]
pub struct W(crate::W<ADDRESS15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRESS15_SPEC>;
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
impl core::convert::From<crate::W<ADDRESS15_SPEC>> for W {
    fn from(writer: crate::W<ADDRESS15_SPEC>) -> Self {
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
#[doc = "Replacement address 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address15](index.html) module"]
pub struct ADDRESS15_SPEC;
impl crate::RegisterSpec for ADDRESS15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [address15::R](R) reader structure"]
impl crate::Readable for ADDRESS15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [address15::W](W) writer structure"]
impl crate::Writable for ADDRESS15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRESS15 to value 0"]
impl crate::Resettable for ADDRESS15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
