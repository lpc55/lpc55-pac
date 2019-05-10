#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WAKEINT {
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
pub struct MASKR {
    bits: u8,
}
impl MASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FILTER_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER_MODER {
    #[doc = "Bypass mode."]
    BYPASS,
    #[doc = "Filter 1 clock period."]
    FILTER1CLK,
    #[doc = "Filter 2 clock period."]
    FILTER2CLK,
    #[doc = "Filter 3 clock period."]
    FILTER3CLK,
}
impl FILTER_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILTER_MODER::BYPASS => 0,
            FILTER_MODER::FILTER1CLK => 1,
            FILTER_MODER::FILTER2CLK => 2,
            FILTER_MODER::FILTER3CLK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILTER_MODER {
        match value {
            0 => FILTER_MODER::BYPASS,
            1 => FILTER_MODER::FILTER1CLK,
            2 => FILTER_MODER::FILTER2CLK,
            3 => FILTER_MODER::FILTER3CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == FILTER_MODER::BYPASS
    }
    #[doc = "Checks if the value of the field is `FILTER1CLK`"]
    #[inline]
    pub fn is_filter1clk(&self) -> bool {
        *self == FILTER_MODER::FILTER1CLK
    }
    #[doc = "Checks if the value of the field is `FILTER2CLK`"]
    #[inline]
    pub fn is_filter2clk(&self) -> bool {
        *self == FILTER_MODER::FILTER2CLK
    }
    #[doc = "Checks if the value of the field is `FILTER3CLK`"]
    #[inline]
    pub fn is_filter3clk(&self) -> bool {
        *self == FILTER_MODER::FILTER3CLK
    }
}
#[doc = r" Value of the field"]
pub struct FILTER_CLKSELR {
    bits: u8,
}
impl FILTER_CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LATCH_ENABLER {
    bits: bool,
}
impl LATCH_ENABLER {
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
pub struct INTR_CLEARR {
    bits: bool,
}
impl INTR_CLEARR {
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
#[doc = r" Proxy"]
pub struct _MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTER_MODE`"]
pub enum FILTER_MODEW {
    #[doc = "Bypass mode."]
    BYPASS,
    #[doc = "Filter 1 clock period."]
    FILTER1CLK,
    #[doc = "Filter 2 clock period."]
    FILTER2CLK,
    #[doc = "Filter 3 clock period."]
    FILTER3CLK,
}
impl FILTER_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FILTER_MODEW::BYPASS => 0,
            FILTER_MODEW::FILTER1CLK => 1,
            FILTER_MODEW::FILTER2CLK => 2,
            FILTER_MODEW::FILTER3CLK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTER_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTER_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bypass mode."]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(FILTER_MODEW::BYPASS)
    }
    #[doc = "Filter 1 clock period."]
    #[inline]
    pub fn filter1clk(self) -> &'a mut W {
        self.variant(FILTER_MODEW::FILTER1CLK)
    }
    #[doc = "Filter 2 clock period."]
    #[inline]
    pub fn filter2clk(self) -> &'a mut W {
        self.variant(FILTER_MODEW::FILTER2CLK)
    }
    #[doc = "Filter 3 clock period."]
    #[inline]
    pub fn filter3clk(self) -> &'a mut W {
        self.variant(FILTER_MODEW::FILTER3CLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTER_CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTER_CLKSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LATCH_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _LATCH_ENABLEW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INTR_CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _INTR_CLEARW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline]
    pub fn mask(&self) -> MASKR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MASKR { bits }
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch"]
    #[inline]
    pub fn filter_mode(&self) -> FILTER_MODER {
        FILTER_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel"]
    #[inline]
    pub fn filter_clksel(&self) -> FILTER_CLKSELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FILTER_CLKSELR { bits }
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline]
    pub fn latch_enable(&self) -> LATCH_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LATCH_ENABLER { bits }
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline]
    pub fn intr_clear(&self) -> INTR_CLEARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTR_CLEARR { bits }
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
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline]
    pub fn mask(&mut self) -> _MASKW {
        _MASKW { w: self }
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch"]
    #[inline]
    pub fn filter_mode(&mut self) -> _FILTER_MODEW {
        _FILTER_MODEW { w: self }
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel"]
    #[inline]
    pub fn filter_clksel(&mut self) -> _FILTER_CLKSELW {
        _FILTER_CLKSELW { w: self }
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline]
    pub fn latch_enable(&mut self) -> _LATCH_ENABLEW {
        _LATCH_ENABLEW { w: self }
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline]
    pub fn intr_clear(&mut self) -> _INTR_CLEARW {
        _INTR_CLEARW { w: self }
    }
}
