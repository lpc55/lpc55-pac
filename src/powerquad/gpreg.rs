#[doc = "Register `gpreg[%s]` reader"]
pub struct R(crate::R<GPREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPREG_SPEC>> for R {
    fn from(reader: crate::R<GPREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpreg[%s]` writer"]
pub struct W(crate::W<GPREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPREG_SPEC>;
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
impl core::convert::From<crate::W<GPREG_SPEC>> for W {
    fn from(writer: crate::W<GPREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpreg` reader - General purpose register bank"]
pub struct GPREG_R(crate::FieldReader<u32, u32>);
impl GPREG_R {
    pub(crate) fn new(bits: u32) -> Self {
        GPREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPREG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpreg` writer - General purpose register bank"]
pub struct GPREG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - General purpose register bank"]
    #[inline(always)]
    pub fn gpreg(&self) -> GPREG_R {
        GPREG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose register bank"]
    #[inline(always)]
    pub fn gpreg(&mut self) -> GPREG_W {
        GPREG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose register bank N.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpreg](index.html) module"]
pub struct GPREG_SPEC;
impl crate::RegisterSpec for GPREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpreg::R](R) reader structure"]
impl crate::Readable for GPREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpreg::W](W) writer structure"]
impl crate::Writable for GPREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpreg[%s]
to value 0"]
impl crate::Resettable for GPREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
