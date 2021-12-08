#[doc = "Register `DMA0_ITRIG_ENA_SET` writer"]
pub struct W(crate::W<DMA0_ITRIG_ENA_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA0_ITRIG_ENA_SET_SPEC>;
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
impl From<crate::W<DMA0_ITRIG_ENA_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA0_ITRIG_ENA_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET` writer - Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA0_ITRIG_ENA register"]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | (value as u32 & 0x003f_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:21 - Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA0_ITRIG_ENA register"]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_itrig_ena_set](index.html) module"]
pub struct DMA0_ITRIG_ENA_SET_SPEC;
impl crate::RegisterSpec for DMA0_ITRIG_ENA_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma0_itrig_ena_set::W](W) writer structure"]
impl crate::Writable for DMA0_ITRIG_ENA_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA0_ITRIG_ENA_SET to value 0"]
impl crate::Resettable for DMA0_ITRIG_ENA_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
