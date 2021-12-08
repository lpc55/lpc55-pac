#[doc = "Register `ATLPTDS` reader"]
pub struct R(crate::R<ATLPTDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATLPTDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATLPTDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATLPTDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATLPTDS` writer"]
pub struct W(crate::W<ATLPTDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATLPTDS_SPEC>;
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
impl From<crate::W<ATLPTDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATLPTDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATL_SKIP` reader - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub struct ATL_SKIP_R(crate::FieldReader<u32, u32>);
impl ATL_SKIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ATL_SKIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATL_SKIP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATL_SKIP` writer - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub struct ATL_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_SKIP_W<'a> {
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
    pub fn atl_skip(&self) -> ATL_SKIP_R {
        ATL_SKIP_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn atl_skip(&mut self) -> ATL_SKIP_W {
        ATL_SKIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Skip map for each ATL PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atlptds](index.html) module"]
pub struct ATLPTDS_SPEC;
impl crate::RegisterSpec for ATLPTDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atlptds::R](R) reader structure"]
impl crate::Readable for ATLPTDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atlptds::W](W) writer structure"]
impl crate::Writable for ATLPTDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATLPTDS to value 0"]
impl crate::Resettable for ATLPTDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
