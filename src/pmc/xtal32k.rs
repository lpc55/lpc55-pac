#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XTAL32K {
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
pub struct IREFR {
    bits: u8,
}
impl IREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TESTR {
    bits: bool,
}
impl TESTR {
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
pub struct IBIASR {
    bits: u8,
}
impl IBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AMPLR {
    bits: u8,
}
impl AMPLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CAPBANKINR {
    bits: u8,
}
impl CAPBANKINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CAPBANKOUTR {
    bits: u8,
}
impl CAPBANKOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CAPTESTSTARTSRCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTSTARTSRCSELR {
    #[doc = "Sourced from CAPTESTSTART."]
    CAPSTART,
    #[doc = "Sourced from calibration."]
    CALIB,
}
impl CAPTESTSTARTSRCSELR {
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
            CAPTESTSTARTSRCSELR::CAPSTART => false,
            CAPTESTSTARTSRCSELR::CALIB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTESTSTARTSRCSELR {
        match value {
            false => CAPTESTSTARTSRCSELR::CAPSTART,
            true => CAPTESTSTARTSRCSELR::CALIB,
        }
    }
    #[doc = "Checks if the value of the field is `CAPSTART`"]
    #[inline]
    pub fn is_capstart(&self) -> bool {
        *self == CAPTESTSTARTSRCSELR::CAPSTART
    }
    #[doc = "Checks if the value of the field is `CALIB`"]
    #[inline]
    pub fn is_calib(&self) -> bool {
        *self == CAPTESTSTARTSRCSELR::CALIB
    }
}
#[doc = r" Value of the field"]
pub struct CAPTESTSTARTR {
    bits: bool,
}
impl CAPTESTSTARTR {
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
pub struct CAPTESTENABLER {
    bits: bool,
}
impl CAPTESTENABLER {
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
#[doc = "Possible values of the field `CAPTESTOSCINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTOSCINSELR {
    #[doc = "Oscillator output pin (osc_out)."]
    OSCOUT,
    #[doc = "Oscillator input pin (osc_in)."]
    OSCIN,
}
impl CAPTESTOSCINSELR {
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
            CAPTESTOSCINSELR::OSCOUT => false,
            CAPTESTOSCINSELR::OSCIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTESTOSCINSELR {
        match value {
            false => CAPTESTOSCINSELR::OSCOUT,
            true => CAPTESTOSCINSELR::OSCIN,
        }
    }
    #[doc = "Checks if the value of the field is `OSCOUT`"]
    #[inline]
    pub fn is_oscout(&self) -> bool {
        *self == CAPTESTOSCINSELR::OSCOUT
    }
    #[doc = "Checks if the value of the field is `OSCIN`"]
    #[inline]
    pub fn is_oscin(&self) -> bool {
        *self == CAPTESTOSCINSELR::OSCIN
    }
}
#[doc = r" Proxy"]
pub struct _IREFW<'a> {
    w: &'a mut W,
}
impl<'a> _IREFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TESTW<'a> {
    w: &'a mut W,
}
impl<'a> _TESTW<'a> {
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
pub struct _IBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _IBIASW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AMPLW<'a> {
    w: &'a mut W,
}
impl<'a> _AMPLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAPBANKINW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPBANKINW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAPBANKOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPBANKOUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTESTSTARTSRCSEL`"]
pub enum CAPTESTSTARTSRCSELW {
    #[doc = "Sourced from CAPTESTSTART."]
    CAPSTART,
    #[doc = "Sourced from calibration."]
    CALIB,
}
impl CAPTESTSTARTSRCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTESTSTARTSRCSELW::CAPSTART => false,
            CAPTESTSTARTSRCSELW::CALIB => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTESTSTARTSRCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTESTSTARTSRCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTESTSTARTSRCSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sourced from CAPTESTSTART."]
    #[inline]
    pub fn capstart(self) -> &'a mut W {
        self.variant(CAPTESTSTARTSRCSELW::CAPSTART)
    }
    #[doc = "Sourced from calibration."]
    #[inline]
    pub fn calib(self) -> &'a mut W {
        self.variant(CAPTESTSTARTSRCSELW::CALIB)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAPTESTSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTESTSTARTW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAPTESTENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTESTENABLEW<'a> {
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
#[doc = "Values that can be written to the field `CAPTESTOSCINSEL`"]
pub enum CAPTESTOSCINSELW {
    #[doc = "Oscillator output pin (osc_out)."]
    OSCOUT,
    #[doc = "Oscillator input pin (osc_in)."]
    OSCIN,
}
impl CAPTESTOSCINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTESTOSCINSELW::OSCOUT => false,
            CAPTESTOSCINSELW::OSCIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTESTOSCINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTESTOSCINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTESTOSCINSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Oscillator output pin (osc_out)."]
    #[inline]
    pub fn oscout(self) -> &'a mut W {
        self.variant(CAPTESTOSCINSELW::OSCOUT)
    }
    #[doc = "Oscillator input pin (osc_in)."]
    #[inline]
    pub fn oscin(self) -> &'a mut W {
        self.variant(CAPTESTOSCINSELW::OSCIN)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bits 1:2 - reference output current selection inputs."]
    #[inline]
    pub fn iref(&self) -> IREFR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IREFR { bits }
    }
    #[doc = "Bit 3 - Oscillator Test Mode."]
    #[inline]
    pub fn test(&self) -> TESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TESTR { bits }
    }
    #[doc = "Bits 4:5 - bias current selection inputs."]
    #[inline]
    pub fn ibias(&self) -> IBIASR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIASR { bits }
    }
    #[doc = "Bits 6:7 - oscillator amplitude selection inputs."]
    #[inline]
    pub fn ampl(&self) -> AMPLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AMPLR { bits }
    }
    #[doc = "Bits 8:14 - Capa bank setting input."]
    #[inline]
    pub fn capbankin(&self) -> CAPBANKINR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CAPBANKINR { bits }
    }
    #[doc = "Bits 15:21 - Capa bank setting output."]
    #[inline]
    pub fn capbankout(&self) -> CAPBANKOUTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CAPBANKOUTR { bits }
    }
    #[doc = "Bit 22 - Source selection for xo32k_captest_start_ao_set."]
    #[inline]
    pub fn capteststartsrcsel(&self) -> CAPTESTSTARTSRCSELR {
        CAPTESTSTARTSRCSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Start test."]
    #[inline]
    pub fn capteststart(&self) -> CAPTESTSTARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAPTESTSTARTR { bits }
    }
    #[doc = "Bit 24 - Enable signal for cap test."]
    #[inline]
    pub fn captestenable(&self) -> CAPTESTENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAPTESTENABLER { bits }
    }
    #[doc = "Bit 25 - Select the input for test."]
    #[inline]
    pub fn captestoscinsel(&self) -> CAPTESTOSCINSELR {
        CAPTESTOSCINSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2113618 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 1:2 - reference output current selection inputs."]
    #[inline]
    pub fn iref(&mut self) -> _IREFW {
        _IREFW { w: self }
    }
    #[doc = "Bit 3 - Oscillator Test Mode."]
    #[inline]
    pub fn test(&mut self) -> _TESTW {
        _TESTW { w: self }
    }
    #[doc = "Bits 4:5 - bias current selection inputs."]
    #[inline]
    pub fn ibias(&mut self) -> _IBIASW {
        _IBIASW { w: self }
    }
    #[doc = "Bits 6:7 - oscillator amplitude selection inputs."]
    #[inline]
    pub fn ampl(&mut self) -> _AMPLW {
        _AMPLW { w: self }
    }
    #[doc = "Bits 8:14 - Capa bank setting input."]
    #[inline]
    pub fn capbankin(&mut self) -> _CAPBANKINW {
        _CAPBANKINW { w: self }
    }
    #[doc = "Bits 15:21 - Capa bank setting output."]
    #[inline]
    pub fn capbankout(&mut self) -> _CAPBANKOUTW {
        _CAPBANKOUTW { w: self }
    }
    #[doc = "Bit 22 - Source selection for xo32k_captest_start_ao_set."]
    #[inline]
    pub fn capteststartsrcsel(&mut self) -> _CAPTESTSTARTSRCSELW {
        _CAPTESTSTARTSRCSELW { w: self }
    }
    #[doc = "Bit 23 - Start test."]
    #[inline]
    pub fn capteststart(&mut self) -> _CAPTESTSTARTW {
        _CAPTESTSTARTW { w: self }
    }
    #[doc = "Bit 24 - Enable signal for cap test."]
    #[inline]
    pub fn captestenable(&mut self) -> _CAPTESTENABLEW {
        _CAPTESTENABLEW { w: self }
    }
    #[doc = "Bit 25 - Select the input for test."]
    #[inline]
    pub fn captestoscinsel(&mut self) -> _CAPTESTOSCINSELW {
        _CAPTESTOSCINSELW { w: self }
    }
}
