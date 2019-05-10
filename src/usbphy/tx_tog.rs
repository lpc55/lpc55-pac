#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TX_TOG {
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
#[doc = "Possible values of the field `D_CAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D_CALR {
    #[doc = "Maximum current, approximately 19% above nominal."]
    VALUE0,
    #[doc = "Nominal"]
    VALUE7,
    #[doc = "Minimum current, approximately 19% below nominal."]
    VALUE15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl D_CALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            D_CALR::VALUE0 => 0,
            D_CALR::VALUE7 => 7,
            D_CALR::VALUE15 => 15,
            D_CALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> D_CALR {
        match value {
            0 => D_CALR::VALUE0,
            7 => D_CALR::VALUE7,
            15 => D_CALR::VALUE15,
            i => D_CALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == D_CALR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == D_CALR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == D_CALR::VALUE15
    }
}
#[doc = r" Value of the field"]
pub struct TXCAL45DMR {
    bits: u8,
}
impl TXCAL45DMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXENCAL45DNR {
    bits: bool,
}
impl TXENCAL45DNR {
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
pub struct TXCAL45DPR {
    bits: u8,
}
impl TXCAL45DPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXENCAL45DPR {
    bits: bool,
}
impl TXENCAL45DPR {
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
#[doc = "Values that can be written to the field `D_CAL`"]
pub enum D_CALW {
    #[doc = "Maximum current, approximately 19% above nominal."]
    VALUE0,
    #[doc = "Nominal"]
    VALUE7,
    #[doc = "Minimum current, approximately 19% below nominal."]
    VALUE15,
}
impl D_CALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            D_CALW::VALUE0 => 0,
            D_CALW::VALUE7 => 7,
            D_CALW::VALUE15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _D_CALW<'a> {
    w: &'a mut W,
}
impl<'a> _D_CALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: D_CALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Maximum current, approximately 19% above nominal."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(D_CALW::VALUE0)
    }
    #[doc = "Nominal"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(D_CALW::VALUE7)
    }
    #[doc = "Minimum current, approximately 19% below nominal."]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(D_CALW::VALUE15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXCAL45DMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCAL45DMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXENCAL45DNW<'a> {
    w: &'a mut W,
}
impl<'a> _TXENCAL45DNW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXCAL45DPW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCAL45DPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXENCAL45DPW<'a> {
    w: &'a mut W,
}
impl<'a> _TXENCAL45DPW<'a> {
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline]
    pub fn d_cal(&self) -> D_CALR {
        D_CALR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline]
    pub fn txcal45dm(&self) -> TXCAL45DMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCAL45DMR { bits }
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline]
    pub fn txencal45dn(&self) -> TXENCAL45DNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXENCAL45DNR { bits }
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline]
    pub fn txcal45dp(&self) -> TXCAL45DPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCAL45DPR { bits }
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline]
    pub fn txencal45dp(&self) -> TXENCAL45DPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXENCAL45DPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 167773186 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline]
    pub fn d_cal(&mut self) -> _D_CALW {
        _D_CALW { w: self }
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline]
    pub fn txcal45dm(&mut self) -> _TXCAL45DMW {
        _TXCAL45DMW { w: self }
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline]
    pub fn txencal45dn(&mut self) -> _TXENCAL45DNW {
        _TXENCAL45DNW { w: self }
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline]
    pub fn txcal45dp(&mut self) -> _TXCAL45DPW {
        _TXCAL45DPW { w: self }
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline]
    pub fn txencal45dp(&mut self) -> _TXENCAL45DPW {
        _TXENCAL45DPW { w: self }
    }
}
