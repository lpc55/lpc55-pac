#[doc = "Register `CV%s` reader"]
pub struct R(crate::R<CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CV%s` writer"]
pub struct W(crate::W<CV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CV_SPEC>;
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
impl From<crate::W<CV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVL` reader - Compare Value Low."]
pub struct CVL_R(crate::FieldReader<u16, u16>);
impl CVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVL` writer - Compare Value Low."]
pub struct CVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CVH` reader - Compare Value High."]
pub struct CVH_R(crate::FieldReader<u16, u16>);
impl CVH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CVH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVH` writer - Compare Value High."]
pub struct CVH_W<'a> {
    w: &'a mut W,
}
impl<'a> CVH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Compare Value Low."]
    #[inline(always)]
    pub fn cvl(&self) -> CVL_R {
        CVL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Compare Value High."]
    #[inline(always)]
    pub fn cvh(&self) -> CVH_R {
        CVH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Value Low."]
    #[inline(always)]
    pub fn cvl(&mut self) -> CVL_W {
        CVL_W { w: self }
    }
    #[doc = "Bits 16:31 - Compare Value High."]
    #[inline(always)]
    pub fn cvh(&mut self) -> CVH_W {
        CVH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](index.html) module"]
pub struct CV_SPEC;
impl crate::RegisterSpec for CV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cv::R](R) reader structure"]
impl crate::Readable for CV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cv::W](W) writer structure"]
impl crate::Writable for CV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CV%s to value 0"]
impl crate::Resettable for CV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
