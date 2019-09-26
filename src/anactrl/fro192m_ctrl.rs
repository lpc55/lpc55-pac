#[doc = "Reader of register FRO192M_CTRL"]
pub type R = crate::R<u32, super::FRO192M_CTRL>;
#[doc = "Writer for register FRO192M_CTRL"]
pub type W = crate::W<u32, super::FRO192M_CTRL>;
#[doc = "Register FRO192M_CTRL `reset()`'s with value 0x0080_d01a"]
impl crate::ResetValue for super::FRO192M_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0080_d01a
    }
}
#[doc = "Possible values of the field `ENA_12MHZCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_12MHZCLK_A {
    #[doc = "12 MHz clock is disabled."]
    DISABLE,
    #[doc = "12 MHz clock is enabled."]
    ENABLE,
}
impl From<ENA_12MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_12MHZCLK_A) -> Self {
        match variant {
            ENA_12MHZCLK_A::DISABLE => false,
            ENA_12MHZCLK_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ENA_12MHZCLK`"]
pub type ENA_12MHZCLK_R = crate::R<bool, ENA_12MHZCLK_A>;
impl ENA_12MHZCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_12MHZCLK_A {
        match self.bits {
            false => ENA_12MHZCLK_A::DISABLE,
            true => ENA_12MHZCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENA_12MHZCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENA_12MHZCLK_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENA_12MHZCLK`"]
pub struct ENA_12MHZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_12MHZCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_12MHZCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "12 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLK_A::DISABLE)
    }
    #[doc = "12 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLK_A::ENABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `ENA_48MHZCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_48MHZCLK_A {
    #[doc = "48 MHz clock is enabled."]
    ENABLE,
}
impl From<ENA_48MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_48MHZCLK_A) -> Self {
        match variant {
            ENA_48MHZCLK_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ENA_48MHZCLK`"]
pub type ENA_48MHZCLK_R = crate::R<bool, ENA_48MHZCLK_A>;
impl ENA_48MHZCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ENA_48MHZCLK_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ENA_48MHZCLK_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENA_48MHZCLK_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENA_48MHZCLK`"]
pub struct ENA_48MHZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_48MHZCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_48MHZCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "48 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_48MHZCLK_A::ENABLE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DAC_TRIM`"]
pub type DAC_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC_TRIM`"]
pub struct DAC_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `USBCLKADJ`"]
pub type USBCLKADJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBCLKADJ`"]
pub struct USBCLKADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCLKADJ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `USBMODCHG`"]
pub type USBMODCHG_R = crate::R<bool, bool>;
#[doc = "Possible values of the field `ENA_96MHZCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_96MHZCLK_A {
    #[doc = "96 MHz clock is disabled."]
    DISABLE,
    #[doc = "96 MHz clock is enabled."]
    ENABLE,
}
impl From<ENA_96MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_96MHZCLK_A) -> Self {
        match variant {
            ENA_96MHZCLK_A::DISABLE => false,
            ENA_96MHZCLK_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ENA_96MHZCLK`"]
pub type ENA_96MHZCLK_R = crate::R<bool, ENA_96MHZCLK_A>;
impl ENA_96MHZCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_96MHZCLK_A {
        match self.bits {
            false => ENA_96MHZCLK_A::DISABLE,
            true => ENA_96MHZCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENA_96MHZCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENA_96MHZCLK_A::ENABLE
    }
}
#[doc = "Write proxy for field `ENA_96MHZCLK`"]
pub struct ENA_96MHZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_96MHZCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_96MHZCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "96 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLK_A::DISABLE)
    }
    #[doc = "96 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLK_A::ENABLE)
    }
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
impl R {
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    pub fn ena_12mhzclk(&self) -> ENA_12MHZCLK_R {
        ENA_12MHZCLK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 48 MHz clock control."]
    #[inline(always)]
    pub fn ena_48mhzclk(&self) -> ENA_48MHZCLK_R {
        ENA_48MHZCLK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn dac_trim(&self) -> DAC_TRIM_R {
        DAC_TRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub fn usbclkadj(&self) -> USBCLKADJ_R {
        USBCLKADJ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
    #[inline(always)]
    pub fn usbmodchg(&self) -> USBMODCHG_R {
        USBMODCHG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&self) -> ENA_96MHZCLK_R {
        ENA_96MHZCLK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    pub fn ena_12mhzclk(&mut self) -> ENA_12MHZCLK_W {
        ENA_12MHZCLK_W { w: self }
    }
    #[doc = "Bit 15 - 48 MHz clock control."]
    #[inline(always)]
    pub fn ena_48mhzclk(&mut self) -> ENA_48MHZCLK_W {
        ENA_48MHZCLK_W { w: self }
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn dac_trim(&mut self) -> DAC_TRIM_W {
        DAC_TRIM_W { w: self }
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub fn usbclkadj(&mut self) -> USBCLKADJ_W {
        USBCLKADJ_W { w: self }
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&mut self) -> ENA_96MHZCLK_W {
        ENA_96MHZCLK_W { w: self }
    }
}
