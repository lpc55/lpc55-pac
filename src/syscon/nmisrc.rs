#[doc = "Register `NMISRC` reader"]
pub struct R(crate::R<NMISRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMISRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NMISRC_SPEC>> for R {
    fn from(reader: crate::R<NMISRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMISRC` writer"]
pub struct W(crate::W<NMISRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMISRC_SPEC>;
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
impl core::convert::From<crate::W<NMISRC_SPEC>> for W {
    fn from(writer: crate::W<NMISRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQCPU0` reader - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
pub struct IRQCPU0_R(crate::FieldReader<u8, u8>);
impl IRQCPU0_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRQCPU0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQCPU0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQCPU0` writer - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
pub struct IRQCPU0_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQCPU0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `IRQCPU1` reader - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
pub struct IRQCPU1_R(crate::FieldReader<u8, u8>);
impl IRQCPU1_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRQCPU1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQCPU1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQCPU1` writer - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
pub struct IRQCPU1_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQCPU1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `NMIENCPU1` reader - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
pub struct NMIENCPU1_R(crate::FieldReader<bool, bool>);
impl NMIENCPU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMIENCPU1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMIENCPU1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIENCPU1` writer - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
pub struct NMIENCPU1_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIENCPU1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `NMIENCPU0` reader - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
pub struct NMIENCPU0_R(crate::FieldReader<bool, bool>);
impl NMIENCPU0_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMIENCPU0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMIENCPU0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIENCPU0` writer - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
pub struct NMIENCPU0_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIENCPU0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub fn irqcpu0(&self) -> IRQCPU0_R {
        IRQCPU0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
    #[inline(always)]
    pub fn irqcpu1(&self) -> IRQCPU1_R {
        IRQCPU1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[inline(always)]
    pub fn nmiencpu1(&self) -> NMIENCPU1_R {
        NMIENCPU1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub fn nmiencpu0(&self) -> NMIENCPU0_R {
        NMIENCPU0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub fn irqcpu0(&mut self) -> IRQCPU0_W {
        IRQCPU0_W { w: self }
    }
    #[doc = "Bits 8:13 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
    #[inline(always)]
    pub fn irqcpu1(&mut self) -> IRQCPU1_W {
        IRQCPU1_W { w: self }
    }
    #[doc = "Bit 30 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[inline(always)]
    pub fn nmiencpu1(&mut self) -> NMIENCPU1_W {
        NMIENCPU1_W { w: self }
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub fn nmiencpu0(&mut self) -> NMIENCPU0_W {
        NMIENCPU0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmisrc](index.html) module"]
pub struct NMISRC_SPEC;
impl crate::RegisterSpec for NMISRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmisrc::R](R) reader structure"]
impl crate::Readable for NMISRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmisrc::W](W) writer structure"]
impl crate::Writable for NMISRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMISRC to value 0"]
impl crate::Resettable for NMISRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
