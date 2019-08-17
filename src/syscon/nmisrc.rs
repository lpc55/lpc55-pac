#[doc = "Reader of register NMISRC"]
pub type R = crate::R<u32, super::NMISRC>;
#[doc = "Writer for register NMISRC"]
pub type W = crate::W<u32, super::NMISRC>;
#[doc = "Register NMISRC `reset()`'s with value 0"]
impl crate::ResetValue for super::NMISRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRQCPU0`"]
pub type IRQCPU0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQCPU0`"]
pub struct IRQCPU0_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQCPU0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `IRQCPU1`"]
pub type IRQCPU1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQCPU1`"]
pub struct IRQCPU1_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQCPU1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NMIENCPU1`"]
pub type NMIENCPU1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMIENCPU1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `NMIENCPU0`"]
pub type NMIENCPU0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NMIENCPU0`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
