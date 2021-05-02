#[doc = "Register `ADDRESS6` reader"]
pub struct R(crate::R<ADDRESS6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRESS6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADDRESS6_SPEC>> for R {
    fn from(reader: crate::R<ADDRESS6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRESS6` writer"]
pub struct W(crate::W<ADDRESS6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRESS6_SPEC>;
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
impl core::convert::From<crate::W<ADDRESS6_SPEC>> for W {
    fn from(writer: crate::W<ADDRESS6_SPEC>) -> Self {
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
#[doc = "Replacement address 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address6](index.html) module"]
pub struct ADDRESS6_SPEC;
impl crate::RegisterSpec for ADDRESS6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [address6::R](R) reader structure"]
impl crate::Readable for ADDRESS6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [address6::W](W) writer structure"]
impl crate::Writable for ADDRESS6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRESS6 to value 0"]
impl crate::Resettable for ADDRESS6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
