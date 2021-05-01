#[doc = "Register `MEMADDR` reader"]
pub struct R(crate::R<MEMADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MEMADDR_SPEC>> for R {
    fn from(reader: crate::R<MEMADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMADDR` writer"]
pub struct W(crate::W<MEMADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMADDR_SPEC>;
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
impl core::convert::From<crate::W<MEMADDR_SPEC>> for W {
    fn from(writer: crate::W<MEMADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE` reader - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
pub struct BASE_R(crate::FieldReader<u32, u32>);
impl BASE_R {
    pub(crate) fn new(bits: u32) -> Self {
        BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BASE` writer - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
pub struct BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W {
        BASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address to start memory access from (if available).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memaddr](index.html) module"]
pub struct MEMADDR_SPEC;
impl crate::RegisterSpec for MEMADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memaddr::R](R) reader structure"]
impl crate::Readable for MEMADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memaddr::W](W) writer structure"]
impl crate::Writable for MEMADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMADDR to value 0"]
impl crate::Resettable for MEMADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
