#[doc = "Register `ISOPTDD` reader"]
pub struct R(crate::R<ISOPTDD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOPTDD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOPTDD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOPTDD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISOPTDD` writer"]
pub struct W(crate::W<ISOPTDD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOPTDD_SPEC>;
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
impl From<crate::W<ISOPTDD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOPTDD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO_DONE` reader - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub struct ISO_DONE_R(crate::FieldReader<u32, u32>);
impl ISO_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ISO_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISO_DONE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISO_DONE` writer - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub struct ISO_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_DONE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn iso_done(&self) -> ISO_DONE_R {
        ISO_DONE_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn iso_done(&mut self) -> ISO_DONE_W {
        ISO_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Done map for each ISO PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoptdd](index.html) module"]
pub struct ISOPTDD_SPEC;
impl crate::RegisterSpec for ISOPTDD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isoptdd::R](R) reader structure"]
impl crate::Readable for ISOPTDD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isoptdd::W](W) writer structure"]
impl crate::Writable for ISOPTDD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISOPTDD to value 0"]
impl crate::Resettable for ISOPTDD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
