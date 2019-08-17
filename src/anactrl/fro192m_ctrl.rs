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
#[doc = "Reader of field `BIAS_TRIM`"]
pub type BIAS_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BIAS_TRIM`"]
pub struct BIAS_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `TEMP_TRIM`"]
pub type TEMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEMP_TRIM`"]
pub struct TEMP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 7)) | (((value as u32) & 0x7f) << 7);
        self.w
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
    #[doc = "48 MHz clock is disabled."]
    DISABLE,
    #[doc = "48 MHz clock is enabled."]
    ENABLE,
}
impl From<ENA_48MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_48MHZCLK_A) -> Self {
        match variant {
            ENA_48MHZCLK_A::DISABLE => false,
            ENA_48MHZCLK_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `ENA_48MHZCLK`"]
pub type ENA_48MHZCLK_R = crate::R<bool, ENA_48MHZCLK_A>;
impl ENA_48MHZCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_48MHZCLK_A {
        match self.bits {
            false => ENA_48MHZCLK_A::DISABLE,
            true => ENA_48MHZCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENA_48MHZCLK_A::DISABLE
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
    #[doc = "48 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_48MHZCLK_A::DISABLE)
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
#[doc = "Reader of field `ATB_CTRL`"]
pub type ATB_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATB_CTRL`"]
pub struct ATB_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ATB_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
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
#[doc = "Write proxy for field `WRTRIM`"]
pub struct WRTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTRIM_W<'a> {
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
    #[doc = "Bits 0:5 - Bias trimming bits (course frequency trimming)."]
    #[inline(always)]
    pub fn bias_trim(&self) -> BIAS_TRIM_R {
        BIAS_TRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 7:13 - Temperature coefficient trimming bits."]
    #[inline(always)]
    pub fn temp_trim(&self) -> TEMP_TRIM_R {
        TEMP_TRIM_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
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
    #[doc = "Bits 16:23 - Curdac trimming bits (fine frequency trimming) This trim is used to adjust the frequency, given that the bias and temperature trim are set."]
    #[inline(always)]
    pub fn dac_trim(&self) -> DAC_TRIM_R {
        DAC_TRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub fn usbclkadj(&self) -> USBCLKADJ_R {
        USBCLKADJ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - If this reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be reread until it is 0."]
    #[inline(always)]
    pub fn usbmodchg(&self) -> USBMODCHG_R {
        USBMODCHG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Analog Test Bus control."]
    #[inline(always)]
    pub fn atb_ctrl(&self) -> ATB_CTRL_R {
        ATB_CTRL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&self) -> ENA_96MHZCLK_R {
        ENA_96MHZCLK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Bias trimming bits (course frequency trimming)."]
    #[inline(always)]
    pub fn bias_trim(&mut self) -> BIAS_TRIM_W {
        BIAS_TRIM_W { w: self }
    }
    #[doc = "Bits 7:13 - Temperature coefficient trimming bits."]
    #[inline(always)]
    pub fn temp_trim(&mut self) -> TEMP_TRIM_W {
        TEMP_TRIM_W { w: self }
    }
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
    #[doc = "Bits 16:23 - Curdac trimming bits (fine frequency trimming) This trim is used to adjust the frequency, given that the bias and temperature trim are set."]
    #[inline(always)]
    pub fn dac_trim(&mut self) -> DAC_TRIM_W {
        DAC_TRIM_W { w: self }
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub fn usbclkadj(&mut self) -> USBCLKADJ_W {
        USBCLKADJ_W { w: self }
    }
    #[doc = "Bits 28:29 - Analog Test Bus control."]
    #[inline(always)]
    pub fn atb_ctrl(&mut self) -> ATB_CTRL_W {
        ATB_CTRL_W { w: self }
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&mut self) -> ENA_96MHZCLK_W {
        ENA_96MHZCLK_W { w: self }
    }
    #[doc = "Bit 31 - This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
    #[inline(always)]
    pub fn wrtrim(&mut self) -> WRTRIM_W {
        WRTRIM_W { w: self }
    }
}
