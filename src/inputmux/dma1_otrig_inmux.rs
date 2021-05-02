#[doc = "Register `DMA1_OTRIG_INMUX[%s]` reader"]
pub struct R(crate::R<DMA1_OTRIG_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA1_OTRIG_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA1_OTRIG_INMUX_SPEC>> for R {
    fn from(reader: crate::R<DMA1_OTRIG_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA1_OTRIG_INMUX[%s]` writer"]
pub struct W(crate::W<DMA1_OTRIG_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA1_OTRIG_INMUX_SPEC>;
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
impl core::convert::From<crate::W<DMA1_OTRIG_INMUX_SPEC>> for W {
    fn from(writer: crate::W<DMA1_OTRIG_INMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP` reader - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
pub struct INP_R(crate::FieldReader<u8, u8>);
impl INP_R {
    pub(crate) fn new(bits: u8) -> Self {
        INP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP` writer - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
pub struct INP_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub fn inp(&mut self) -> INP_W {
        INP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA1 output trigger selection to become DMA1 trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_otrig_inmux](index.html) module"]
pub struct DMA1_OTRIG_INMUX_SPEC;
impl crate::RegisterSpec for DMA1_OTRIG_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma1_otrig_inmux::R](R) reader structure"]
impl crate::Readable for DMA1_OTRIG_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma1_otrig_inmux::W](W) writer structure"]
impl crate::Writable for DMA1_OTRIG_INMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA1_OTRIG_INMUX[%s]
to value 0x0f"]
impl crate::Resettable for DMA1_OTRIG_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
