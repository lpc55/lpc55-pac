#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XO_CAL_CFG {
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
pub struct START_INVR {
    bits: bool,
}
impl START_INVR {
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
pub struct START_OVRR {
    bits: bool,
}
impl START_OVRR {
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
pub struct STARTR {
    bits: bool,
}
impl STARTR {
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
pub struct STOP_INVR {
    bits: bool,
}
impl STOP_INVR {
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
pub struct STOP_CNTR_ENDR {
    bits: bool,
}
impl STOP_CNTR_ENDR {
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
#[doc = "Possible values of the field `XO32K_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XO32K_MODER {
    #[doc = "High speed crystal oscillator (12 MHz- 32 MHz) is used"]
    XO32MHZ,
    #[doc = "32 kHz crystal oscillator calibration is used."]
    XO32KHZ,
}
impl XO32K_MODER {
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
            XO32K_MODER::XO32MHZ => false,
            XO32K_MODER::XO32KHZ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XO32K_MODER {
        match value {
            false => XO32K_MODER::XO32MHZ,
            true => XO32K_MODER::XO32KHZ,
        }
    }
    #[doc = "Checks if the value of the field is `XO32MHZ`"]
    #[inline]
    pub fn is_xo32mhz(&self) -> bool {
        *self == XO32K_MODER::XO32MHZ
    }
    #[doc = "Checks if the value of the field is `XO32KHZ`"]
    #[inline]
    pub fn is_xo32khz(&self) -> bool {
        *self == XO32K_MODER::XO32KHZ
    }
}
#[doc = r" Proxy"]
pub struct _START_INVW<'a> {
    w: &'a mut W,
}
impl<'a> _START_INVW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _START_OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _START_OVRW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STOP_INVW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_INVW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STOP_CNTR_ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_CNTR_ENDW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XO32K_MODE`"]
pub enum XO32K_MODEW {
    #[doc = "High speed crystal oscillator (12 MHz- 32 MHz) is used"]
    XO32MHZ,
    #[doc = "32 kHz crystal oscillator calibration is used."]
    XO32KHZ,
}
impl XO32K_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XO32K_MODEW::XO32MHZ => false,
            XO32K_MODEW::XO32KHZ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XO32K_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _XO32K_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XO32K_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High speed crystal oscillator (12 MHz- 32 MHz) is used"]
    #[inline]
    pub fn xo32mhz(self) -> &'a mut W {
        self.variant(XO32K_MODEW::XO32MHZ)
    }
    #[doc = "32 kHz crystal oscillator calibration is used."]
    #[inline]
    pub fn xo32khz(self) -> &'a mut W {
        self.variant(XO32K_MODEW::XO32KHZ)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Polarity of the externally applied START signal"]
    #[inline]
    pub fn start_inv(&self) -> START_INVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        START_INVR { bits }
    }
    #[doc = "Bit 1 - Override of the START signal."]
    #[inline]
    pub fn start_ovr(&self) -> START_OVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        START_OVRR { bits }
    }
    #[doc = "Bit 2 - Override value of the START signal."]
    #[inline]
    pub fn start(&self) -> STARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STARTR { bits }
    }
    #[doc = "Bit 3 - Polarity of the STOP signal."]
    #[inline]
    pub fn stop_inv(&self) -> STOP_INVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOP_INVR { bits }
    }
    #[doc = "Bit 4 - Generate the external DONE signal when the counter reaches its end."]
    #[inline]
    pub fn stop_cntr_end(&self) -> STOP_CNTR_ENDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOP_CNTR_ENDR { bits }
    }
    #[doc = "Bit 5 - When 0 : High speed crystal oscillator calibration is used. When 1 : 32 kHz crystal oscillator calibration is used."]
    #[inline]
    pub fn xo32k_mode(&self) -> XO32K_MODER {
        XO32K_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Polarity of the externally applied START signal"]
    #[inline]
    pub fn start_inv(&mut self) -> _START_INVW {
        _START_INVW { w: self }
    }
    #[doc = "Bit 1 - Override of the START signal."]
    #[inline]
    pub fn start_ovr(&mut self) -> _START_OVRW {
        _START_OVRW { w: self }
    }
    #[doc = "Bit 2 - Override value of the START signal."]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 3 - Polarity of the STOP signal."]
    #[inline]
    pub fn stop_inv(&mut self) -> _STOP_INVW {
        _STOP_INVW { w: self }
    }
    #[doc = "Bit 4 - Generate the external DONE signal when the counter reaches its end."]
    #[inline]
    pub fn stop_cntr_end(&mut self) -> _STOP_CNTR_ENDW {
        _STOP_CNTR_ENDW { w: self }
    }
    #[doc = "Bit 5 - When 0 : High speed crystal oscillator calibration is used. When 1 : 32 kHz crystal oscillator calibration is used."]
    #[inline]
    pub fn xo32k_mode(&mut self) -> _XO32K_MODEW {
        _XO32K_MODEW { w: self }
    }
}
