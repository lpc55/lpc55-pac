#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTCOSC32K {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "FRO 32 KHz."]
    FRO32K,
    #[doc = "XTAL 32KHz."]
    XTAL32K,
}
impl SELR {
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
            SELR::FRO32K => false,
            SELR::XTAL32K => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELR {
        match value {
            false => SELR::FRO32K,
            true => SELR::XTAL32K,
        }
    }
    #[doc = "Checks if the value of the field is `FRO32K`"]
    #[inline]
    pub fn is_fro32k(&self) -> bool {
        *self == SELR::FRO32K
    }
    #[doc = "Checks if the value of the field is `XTAL32K`"]
    #[inline]
    pub fn is_xtal32k(&self) -> bool {
        *self == SELR::XTAL32K
    }
}
#[doc = r" Value of the field"]
pub struct CLK1KHZDIVR {
    bits: u8,
}
impl CLK1KHZDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLK1KHZDIVUPDATEREQR {
    bits: bool,
}
impl CLK1KHZDIVUPDATEREQR {
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
pub struct CLK1HZDIVR {
    bits: u16,
}
impl CLK1HZDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLK1HZDIVHALTR {
    bits: bool,
}
impl CLK1HZDIVHALTR {
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
pub struct CLK1HZDIVUPDATEREQR {
    bits: bool,
}
impl CLK1HZDIVUPDATEREQR {
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
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "FRO 32 KHz."]
    FRO32K,
    #[doc = "XTAL 32KHz."]
    XTAL32K,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELW::FRO32K => false,
            SELW::XTAL32K => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FRO 32 KHz."]
    #[inline]
    pub fn fro32k(self) -> &'a mut W {
        self.variant(SELW::FRO32K)
    }
    #[doc = "XTAL 32KHz."]
    #[inline]
    pub fn xtal32k(self) -> &'a mut W {
        self.variant(SELW::XTAL32K)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLK1KHZDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK1KHZDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLK1KHZDIVUPDATEREQW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK1KHZDIVUPDATEREQW<'a> {
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
pub struct _CLK1HZDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK1HZDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLK1HZDIVHALTW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK1HZDIVHALTW<'a> {
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
pub struct _CLK1HZDIVUPDATEREQW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK1HZDIVUPDATEREQW<'a> {
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
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline]
    pub fn clk1khzdiv(&self) -> CLK1KHZDIVR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLK1KHZDIVR { bits }
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline]
    pub fn clk1khzdivupdatereq(&self) -> CLK1KHZDIVUPDATEREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK1KHZDIVUPDATEREQR { bits }
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline]
    pub fn clk1hzdiv(&self) -> CLK1HZDIVR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CLK1HZDIVR { bits }
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline]
    pub fn clk1hzdivhalt(&self) -> CLK1HZDIVHALTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK1HZDIVHALTR { bits }
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline]
    pub fn clk1hzdivupdatereq(&self) -> CLK1HZDIVUPDATEREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK1HZDIVUPDATEREQR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 67043336 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline]
    pub fn clk1khzdiv(&mut self) -> _CLK1KHZDIVW {
        _CLK1KHZDIVW { w: self }
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline]
    pub fn clk1khzdivupdatereq(&mut self) -> _CLK1KHZDIVUPDATEREQW {
        _CLK1KHZDIVUPDATEREQW { w: self }
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline]
    pub fn clk1hzdiv(&mut self) -> _CLK1HZDIVW {
        _CLK1HZDIVW { w: self }
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline]
    pub fn clk1hzdivhalt(&mut self) -> _CLK1HZDIVHALTW {
        _CLK1HZDIVHALTW { w: self }
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline]
    pub fn clk1hzdivupdatereq(&mut self) -> _CLK1HZDIVUPDATEREQW {
        _CLK1HZDIVUPDATEREQW { w: self }
    }
}
