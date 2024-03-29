#[doc = "Register `INTPTDD` reader"]
pub struct R(crate::R<INTPTDD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPTDD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPTDD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPTDD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPTDD` writer"]
pub struct W(crate::W<INTPTDD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPTDD_SPEC>;
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
impl From<crate::W<INTPTDD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPTDD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_DONE` reader - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub struct INT_DONE_R(crate::FieldReader<u32, u32>);
impl INT_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INT_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_DONE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_DONE` writer - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
pub struct INT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_DONE_W<'a> {
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
    pub fn int_done(&self) -> INT_DONE_R {
        INT_DONE_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bit corresponding to a certain PTD will be set to logic 1 as soon as that PTD execution is completed."]
    #[inline(always)]
    pub fn int_done(&mut self) -> INT_DONE_W {
        INT_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Done map for each INT PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intptdd](index.html) module"]
pub struct INTPTDD_SPEC;
impl crate::RegisterSpec for INTPTDD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intptdd::R](R) reader structure"]
impl crate::Readable for INTPTDD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intptdd::W](W) writer structure"]
impl crate::Writable for INTPTDD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTPTDD to value 0"]
impl crate::Resettable for INTPTDD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
