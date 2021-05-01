#[doc = "Register `DMA1_REQ_ENA` reader"]
pub struct R(crate::R<DMA1_REQ_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA1_REQ_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA1_REQ_ENA_SPEC>> for R {
    fn from(reader: crate::R<DMA1_REQ_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA1_REQ_ENA` writer"]
pub struct W(crate::W<DMA1_REQ_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA1_REQ_ENA_SPEC>;
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
impl core::convert::From<crate::W<DMA1_REQ_ENA_SPEC>> for W {
    fn from(writer: crate::W<DMA1_REQ_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ_ENA` reader - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
pub struct REQ_ENA_R(crate::FieldReader<u16, u16>);
impl REQ_ENA_R {
    pub(crate) fn new(bits: u16) -> Self {
        REQ_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQ_ENA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQ_ENA` writer - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
pub struct REQ_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&self) -> REQ_ENA_R {
        REQ_ENA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&mut self) -> REQ_ENA_W {
        REQ_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable DMA1 requests\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_req_ena](index.html) module"]
pub struct DMA1_REQ_ENA_SPEC;
impl crate::RegisterSpec for DMA1_REQ_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma1_req_ena::R](R) reader structure"]
impl crate::Readable for DMA1_REQ_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma1_req_ena::W](W) writer structure"]
impl crate::Writable for DMA1_REQ_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA1_REQ_ENA to value 0x03ff"]
impl crate::Resettable for DMA1_REQ_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
