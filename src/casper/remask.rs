#[doc = "Register `REMASK` reader"]
pub struct R(crate::R<REMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMASK` writer"]
pub struct W(crate::W<REMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMASK_SPEC>;
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
impl From<crate::W<REMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
pub struct MASK_R(crate::FieldReader<u32, u32>);
impl MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Optional re-mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remask](index.html) module"]
pub struct REMASK_SPEC;
impl crate::RegisterSpec for REMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remask::R](R) reader structure"]
impl crate::Readable for REMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remask::W](W) writer structure"]
impl crate::Writable for REMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REMASK to value 0"]
impl crate::Resettable for REMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
