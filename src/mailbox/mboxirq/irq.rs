#[doc = "Reader of register IRQ"]
pub type R = crate::R<u32, super::IRQ>;
#[doc = "Writer for register IRQ"]
pub type W = crate::W<u32, super::IRQ>;
#[doc = "Register IRQ `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTREQ`"]
pub type INTREQ_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INTREQ`"]
pub struct INTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INTREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
    #[inline(always)]
    pub fn intreq(&self) -> INTREQ_R {
        INTREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
    #[inline(always)]
    pub fn intreq(&mut self) -> INTREQ_W {
        INTREQ_W { w: self }
    }
}
