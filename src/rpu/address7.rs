#[doc = "Register `ADDRESS7` reader"]
pub struct R(crate::R<ADDRESS7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRESS7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADDRESS7_SPEC>> for R {
    fn from(reader: crate::R<ADDRESS7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRESS7` writer"]
pub struct W(crate::W<ADDRESS7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRESS7_SPEC>;
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
impl core::convert::From<crate::W<ADDRESS7_SPEC>> for W {
    fn from(writer: crate::W<ADDRESS7_SPEC>) -> Self {
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
#[doc = "Replacement address 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address7](index.html) module"]
pub struct ADDRESS7_SPEC;
impl crate::RegisterSpec for ADDRESS7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [address7::R](R) reader structure"]
impl crate::Readable for ADDRESS7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [address7::W](W) writer structure"]
impl crate::Writable for ADDRESS7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRESS7 to value 0"]
impl crate::Resettable for ADDRESS7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
