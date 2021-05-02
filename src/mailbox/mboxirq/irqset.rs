#[doc = "Register `IRQSET` writer"]
pub struct W(crate::W<IRQSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQSET_SPEC>;
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
impl core::convert::From<crate::W<IRQSET_SPEC>> for W {
    fn from(writer: crate::W<IRQSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTREQSET` writer - Writing 1 sets the corresponding bit in the IRQ0 register."]
pub struct INTREQSET_W<'a> {
    w: &'a mut W,
}
impl<'a> INTREQSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing 1 sets the corresponding bit in the IRQ0 register."]
    #[inline(always)]
    pub fn intreqset(&mut self) -> INTREQSET_W {
        INTREQSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set bits in IRQ0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqset](index.html) module"]
pub struct IRQSET_SPEC;
impl crate::RegisterSpec for IRQSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [irqset::W](W) writer structure"]
impl crate::Writable for IRQSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQSET to value 0"]
impl crate::Resettable for IRQSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
