#[doc = "Register `DMA0_ITRIG_ENA` reader"]
pub struct R(crate::R<DMA0_ITRIG_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA0_ITRIG_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA0_ITRIG_ENA_SPEC>> for R {
    fn from(reader: crate::R<DMA0_ITRIG_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA0_ITRIG_ENA` writer"]
pub struct W(crate::W<DMA0_ITRIG_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA0_ITRIG_ENA_SPEC>;
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
impl core::convert::From<crate::W<DMA0_ITRIG_ENA_SPEC>> for W {
    fn from(writer: crate::W<DMA0_ITRIG_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITRIG_ENA` reader - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
pub struct ITRIG_ENA_R(crate::FieldReader<u32, u32>);
impl ITRIG_ENA_R {
    pub(crate) fn new(bits: u32) -> Self {
        ITRIG_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITRIG_ENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITRIG_ENA` writer - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
pub struct ITRIG_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIG_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&self) -> ITRIG_ENA_R {
        ITRIG_ENA_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&mut self) -> ITRIG_ENA_W {
        ITRIG_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable DMA0 triggers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_itrig_ena](index.html) module"]
pub struct DMA0_ITRIG_ENA_SPEC;
impl crate::RegisterSpec for DMA0_ITRIG_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma0_itrig_ena::R](R) reader structure"]
impl crate::Readable for DMA0_ITRIG_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma0_itrig_ena::W](W) writer structure"]
impl crate::Writable for DMA0_ITRIG_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA0_ITRIG_ENA to value 0x003f_ffff"]
impl crate::Resettable for DMA0_ITRIG_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x003f_ffff
    }
}
