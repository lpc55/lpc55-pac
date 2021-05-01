#[doc = "Register `OUTBASE` reader"]
pub struct R(crate::R<OUTBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OUTBASE_SPEC>> for R {
    fn from(reader: crate::R<OUTBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTBASE` writer"]
pub struct W(crate::W<OUTBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTBASE_SPEC>;
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
impl core::convert::From<crate::W<OUTBASE_SPEC>> for W {
    fn from(writer: crate::W<OUTBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `outbase` reader - Base address register for the output region"]
pub struct OUTBASE_R(crate::FieldReader<u32, u32>);
impl OUTBASE_R {
    pub(crate) fn new(bits: u32) -> Self {
        OUTBASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTBASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `outbase` writer - Base address register for the output region"]
pub struct OUTBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Base address register for the output region"]
    #[inline(always)]
    pub fn outbase(&self) -> OUTBASE_R {
        OUTBASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address register for the output region"]
    #[inline(always)]
    pub fn outbase(&mut self) -> OUTBASE_W {
        OUTBASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base address register for output region\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outbase](index.html) module"]
pub struct OUTBASE_SPEC;
impl crate::RegisterSpec for OUTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outbase::R](R) reader structure"]
impl crate::Readable for OUTBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outbase::W](W) writer structure"]
impl crate::Writable for OUTBASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTBASE to value 0"]
impl crate::Resettable for OUTBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
