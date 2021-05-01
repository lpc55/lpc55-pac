#[doc = "Register `KEYOUTINDEX` reader"]
pub struct R(crate::R<KEYOUTINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYOUTINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<KEYOUTINDEX_SPEC>> for R {
    fn from(reader: crate::R<KEYOUTINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYOUTINDEX` writer"]
pub struct W(crate::W<KEYOUTINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYOUTINDEX_SPEC>;
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
impl core::convert::From<crate::W<KEYOUTINDEX_SPEC>> for W {
    fn from(writer: crate::W<KEYOUTINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYOUTIDX` reader - Key index for the key that is currently output via the Key Output register"]
pub struct KEYOUTIDX_R(crate::FieldReader<u8, u8>);
impl KEYOUTIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYOUTIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYOUTIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Key index for the key that is currently output via the Key Output register"]
    #[inline(always)]
    pub fn keyoutidx(&self) -> KEYOUTIDX_R {
        KEYOUTIDX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Key Output Index register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyoutindex](index.html) module"]
pub struct KEYOUTINDEX_SPEC;
impl crate::RegisterSpec for KEYOUTINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyoutindex::R](R) reader structure"]
impl crate::Readable for KEYOUTINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyoutindex::W](W) writer structure"]
impl crate::Writable for KEYOUTINDEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYOUTINDEX to value 0"]
impl crate::Resettable for KEYOUTINDEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
