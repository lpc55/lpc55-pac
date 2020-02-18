#[doc = "Reader of register DMA1_ITRIG_ENA"]
pub type R = crate::R<u32, super::DMA1_ITRIG_ENA>;
#[doc = "Writer for register DMA1_ITRIG_ENA"]
pub type W = crate::W<u32, super::DMA1_ITRIG_ENA>;
#[doc = "Register DMA1_ITRIG_ENA `reset()`'s with value 0x7fff"]
impl crate::ResetValue for super::DMA1_ITRIG_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff
    }
}
#[doc = "Reader of field `ITRIG_ENA`"]
pub type ITRIG_ENA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ITRIG_ENA`"]
pub struct ITRIG_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIG_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&self) -> ITRIG_ENA_R {
        ITRIG_ENA_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&mut self) -> ITRIG_ENA_W {
        ITRIG_ENA_W { w: self }
    }
}
