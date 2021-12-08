#[doc = "Register `ATLPTDD` reader"]
pub struct R(crate::R<ATLPTDD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATLPTDD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATLPTDD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATLPTDD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATLPTDD` writer"]
pub struct W(crate::W<ATLPTDD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATLPTDD_SPEC>;
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
impl From<crate::W<ATLPTDD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATLPTDD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATL_DONE` reader - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub struct ATL_DONE_R(crate::FieldReader<u32, u32>);
impl ATL_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ATL_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATL_DONE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATL_DONE` writer - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub struct ATL_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_DONE_W<'a> {
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
    pub fn atl_done(&self) -> ATL_DONE_R {
        ATL_DONE_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn atl_done(&mut self) -> ATL_DONE_W {
        ATL_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Done map for each ATL PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atlptdd](index.html) module"]
pub struct ATLPTDD_SPEC;
impl crate::RegisterSpec for ATLPTDD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atlptdd::R](R) reader structure"]
impl crate::Readable for ATLPTDD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atlptdd::W](W) writer structure"]
impl crate::Writable for ATLPTDD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATLPTDD to value 0"]
impl crate::Resettable for ATLPTDD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
