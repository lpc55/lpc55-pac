#[doc = "Reader of register HARDWARESLEEP"]
pub type R = crate::R<u32, super::HARDWARESLEEP>;
#[doc = "Writer for register HARDWARESLEEP"]
pub type W = crate::W<u32, super::HARDWARESLEEP>;
#[doc = "Register HARDWARESLEEP `reset()`'s with value 0"]
impl crate::ResetValue for super::HARDWARESLEEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FORCED`"]
pub type FORCED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCED`"]
pub struct FORCED_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCED_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PERIPHERALS`"]
pub type PERIPHERALS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERIPHERALS`"]
pub struct PERIPHERALS_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPHERALS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SDMA0`"]
pub type SDMA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMA0`"]
pub struct SDMA0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SDMA1`"]
pub type SDMA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDMA1`"]
pub struct SDMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during Deep Sleep and Power-down modes."]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake for Flexcomms."]
    #[inline(always)]
    pub fn peripherals(&self) -> PERIPHERALS_R {
        PERIPHERALS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake for DMA0."]
    #[inline(always)]
    pub fn sdma0(&self) -> SDMA0_R {
        SDMA0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wake for DMA1."]
    #[inline(always)]
    pub fn sdma1(&self) -> SDMA1_R {
        SDMA1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during Deep Sleep and Power-down modes."]
    #[inline(always)]
    pub fn forced(&mut self) -> FORCED_W {
        FORCED_W { w: self }
    }
    #[doc = "Bit 1 - Wake for Flexcomms."]
    #[inline(always)]
    pub fn peripherals(&mut self) -> PERIPHERALS_W {
        PERIPHERALS_W { w: self }
    }
    #[doc = "Bit 3 - Wake for DMA0."]
    #[inline(always)]
    pub fn sdma0(&mut self) -> SDMA0_W {
        SDMA0_W { w: self }
    }
    #[doc = "Bit 5 - Wake for DMA1."]
    #[inline(always)]
    pub fn sdma1(&mut self) -> SDMA1_W {
        SDMA1_W { w: self }
    }
}
