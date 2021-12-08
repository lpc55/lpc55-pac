#[doc = "Register `INTPTDS` reader"]
pub struct R(crate::R<INTPTDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTPTDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTPTDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTPTDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTPTDS` writer"]
pub struct W(crate::W<INTPTDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTPTDS_SPEC>;
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
impl From<crate::W<INTPTDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTPTDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_SKIP` reader - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub struct INT_SKIP_R(crate::FieldReader<u32, u32>);
impl INT_SKIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INT_SKIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_SKIP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_SKIP` writer - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub struct INT_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_SKIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn int_skip(&self) -> INT_SKIP_R {
        INT_SKIP_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn int_skip(&mut self) -> INT_SKIP_W {
        INT_SKIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Skip map for each INT PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intptds](index.html) module"]
pub struct INTPTDS_SPEC;
impl crate::RegisterSpec for INTPTDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intptds::R](R) reader structure"]
impl crate::Readable for INTPTDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intptds::W](W) writer structure"]
impl crate::Writable for INTPTDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTPTDS to value 0"]
impl crate::Resettable for INTPTDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
