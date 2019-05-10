#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRO192M_CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct BIAS_TRIMR {
    bits: u8,
}
impl BIAS_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TEMP_TRIMR {
    bits: u8,
}
impl TEMP_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENA_12MHZCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_12MHZCLKR {
    #[doc = "12 MHz clock is disabled."]
    DISABLE,
    #[doc = "12 MHz clock is enabled."]
    ENABLE,
}
impl ENA_12MHZCLKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ENA_12MHZCLKR::DISABLE => false,
            ENA_12MHZCLKR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA_12MHZCLKR {
        match value {
            false => ENA_12MHZCLKR::DISABLE,
            true => ENA_12MHZCLKR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ENA_12MHZCLKR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENA_12MHZCLKR::ENABLE
    }
}
#[doc = "Possible values of the field `ENA_48MHZCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_48MHZCLKR {
    #[doc = "48 MHz clock is disabled."]
    DISABLE,
    #[doc = "48 MHz clock is enabled."]
    ENABLE,
}
impl ENA_48MHZCLKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ENA_48MHZCLKR::DISABLE => false,
            ENA_48MHZCLKR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA_48MHZCLKR {
        match value {
            false => ENA_48MHZCLKR::DISABLE,
            true => ENA_48MHZCLKR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ENA_48MHZCLKR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENA_48MHZCLKR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct DAC_TRIMR {
    bits: u8,
}
impl DAC_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USBCLKADJR {
    bits: bool,
}
impl USBCLKADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USBMODCHGR {
    bits: bool,
}
impl USBMODCHGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ATB_CTRLR {
    bits: u8,
}
impl ATB_CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENA_96MHZCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_96MHZCLKR {
    #[doc = "96 MHz clock is disabled."]
    DISABLE,
    #[doc = "96 MHz clock is enabled."]
    ENABLE,
}
impl ENA_96MHZCLKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ENA_96MHZCLKR::DISABLE => false,
            ENA_96MHZCLKR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENA_96MHZCLKR {
        match value {
            false => ENA_96MHZCLKR::DISABLE,
            true => ENA_96MHZCLKR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ENA_96MHZCLKR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENA_96MHZCLKR::ENABLE
    }
}
#[doc = r" Proxy"]
pub struct _BIAS_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BIAS_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEMP_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMP_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENA_12MHZCLK`"]
pub enum ENA_12MHZCLKW {
    #[doc = "12 MHz clock is disabled."]
    DISABLE,
    #[doc = "12 MHz clock is enabled."]
    ENABLE,
}
impl ENA_12MHZCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA_12MHZCLKW::DISABLE => false,
            ENA_12MHZCLKW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA_12MHZCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_12MHZCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA_12MHZCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "12 MHz clock is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLKW::DISABLE)
    }
    #[doc = "12 MHz clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLKW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENA_48MHZCLK`"]
pub enum ENA_48MHZCLKW {
    #[doc = "48 MHz clock is disabled."]
    DISABLE,
    #[doc = "48 MHz clock is enabled."]
    ENABLE,
}
impl ENA_48MHZCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA_48MHZCLKW::DISABLE => false,
            ENA_48MHZCLKW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA_48MHZCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_48MHZCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA_48MHZCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "48 MHz clock is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_48MHZCLKW::DISABLE)
    }
    #[doc = "48 MHz clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_48MHZCLKW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DAC_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _DAC_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBCLKADJW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCLKADJW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ATB_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ATB_CTRLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENA_96MHZCLK`"]
pub enum ENA_96MHZCLKW {
    #[doc = "96 MHz clock is disabled."]
    DISABLE,
    #[doc = "96 MHz clock is enabled."]
    ENABLE,
}
impl ENA_96MHZCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENA_96MHZCLKW::DISABLE => false,
            ENA_96MHZCLKW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENA_96MHZCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ENA_96MHZCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENA_96MHZCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "96 MHz clock is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLKW::DISABLE)
    }
    #[doc = "96 MHz clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLKW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _WRTRIMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Bias trimming bits (course frequency trimming)."]
    #[inline]
    pub fn bias_trim(&self) -> BIAS_TRIMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BIAS_TRIMR { bits }
    }
    #[doc = "Bits 7:13 - Temperature coefficient trimming bits."]
    #[inline]
    pub fn temp_trim(&self) -> TEMP_TRIMR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TEMP_TRIMR { bits }
    }
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline]
    pub fn ena_12mhzclk(&self) -> ENA_12MHZCLKR {
        ENA_12MHZCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - 48 MHz clock control."]
    #[inline]
    pub fn ena_48mhzclk(&self) -> ENA_48MHZCLKR {
        ENA_48MHZCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Curdac trimming bits (fine frequency trimming) This trim is used to adjust the frequency, given that the bias and temperature trim are set."]
    #[inline]
    pub fn dac_trim(&self) -> DAC_TRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DAC_TRIMR { bits }
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline]
    pub fn usbclkadj(&self) -> USBCLKADJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USBCLKADJR { bits }
    }
    #[doc = "Bit 25 - If this reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be reread until it is 0."]
    #[inline]
    pub fn usbmodchg(&self) -> USBMODCHGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USBMODCHGR { bits }
    }
    #[doc = "Bits 28:29 - Analog Test Bus control."]
    #[inline]
    pub fn atb_ctrl(&self) -> ATB_CTRLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ATB_CTRLR { bits }
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline]
    pub fn ena_96mhzclk(&self) -> ENA_96MHZCLKR {
        ENA_96MHZCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8441882 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Bias trimming bits (course frequency trimming)."]
    #[inline]
    pub fn bias_trim(&mut self) -> _BIAS_TRIMW {
        _BIAS_TRIMW { w: self }
    }
    #[doc = "Bits 7:13 - Temperature coefficient trimming bits."]
    #[inline]
    pub fn temp_trim(&mut self) -> _TEMP_TRIMW {
        _TEMP_TRIMW { w: self }
    }
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline]
    pub fn ena_12mhzclk(&mut self) -> _ENA_12MHZCLKW {
        _ENA_12MHZCLKW { w: self }
    }
    #[doc = "Bit 15 - 48 MHz clock control."]
    #[inline]
    pub fn ena_48mhzclk(&mut self) -> _ENA_48MHZCLKW {
        _ENA_48MHZCLKW { w: self }
    }
    #[doc = "Bits 16:23 - Curdac trimming bits (fine frequency trimming) This trim is used to adjust the frequency, given that the bias and temperature trim are set."]
    #[inline]
    pub fn dac_trim(&mut self) -> _DAC_TRIMW {
        _DAC_TRIMW { w: self }
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline]
    pub fn usbclkadj(&mut self) -> _USBCLKADJW {
        _USBCLKADJW { w: self }
    }
    #[doc = "Bits 28:29 - Analog Test Bus control."]
    #[inline]
    pub fn atb_ctrl(&mut self) -> _ATB_CTRLW {
        _ATB_CTRLW { w: self }
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline]
    pub fn ena_96mhzclk(&mut self) -> _ENA_96MHZCLKW {
        _ENA_96MHZCLKW { w: self }
    }
    #[doc = "Bit 31 - This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
    #[inline]
    pub fn wrtrim(&mut self) -> _WRTRIMW {
        _WRTRIMW { w: self }
    }
}
