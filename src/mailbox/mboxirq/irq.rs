#[doc = "Register `IRQ` reader"]
pub struct R(crate::R<IRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRQ_SPEC>> for R {
    fn from(reader: crate::R<IRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ` writer"]
pub struct W(crate::W<IRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_SPEC>;
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
impl core::convert::From<crate::W<IRQ_SPEC>> for W {
    fn from(writer: crate::W<IRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTREQ` reader - If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
pub struct INTREQ_R(crate::FieldReader<u32, u32>);
impl INTREQ_R {
    pub(crate) fn new(bits: u32) -> Self {
        INTREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTREQ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTREQ` writer - If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
pub struct INTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INTREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt request register for the Cortex-M0+ CPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq](index.html) module"]
pub struct IRQ_SPEC;
impl crate::RegisterSpec for IRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq::R](R) reader structure"]
impl crate::Readable for IRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq::W](W) writer structure"]
impl crate::Writable for IRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
