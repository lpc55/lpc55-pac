#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTYPE {
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
pub struct CARD0_WIDTH0R {
    bits: bool,
}
impl CARD0_WIDTH0R {
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
pub struct CARD1_WIDTH0R {
    bits: bool,
}
impl CARD1_WIDTH0R {
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
pub struct CARD0_WIDTH1R {
    bits: bool,
}
impl CARD0_WIDTH1R {
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
pub struct CARD1_WIDTH1R {
    bits: bool,
}
impl CARD1_WIDTH1R {
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
pub struct _CARD0_WIDTH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD0_WIDTH0W<'a> {
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
pub struct _CARD1_WIDTH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD1_WIDTH0W<'a> {
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
pub struct _CARD0_WIDTH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD0_WIDTH1W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CARD1_WIDTH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD1_WIDTH1W<'a> {
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 0 - Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline]
    pub fn card0_width0(&self) -> CARD0_WIDTH0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CARD0_WIDTH0R { bits }
    }
    #[doc = "Bit 1 - Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline]
    pub fn card1_width0(&self) -> CARD1_WIDTH0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CARD1_WIDTH0R { bits }
    }
    #[doc = "Bit 16 - Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline]
    pub fn card0_width1(&self) -> CARD0_WIDTH1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CARD0_WIDTH1R { bits }
    }
    #[doc = "Bit 17 - Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline]
    pub fn card1_width1(&self) -> CARD1_WIDTH1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CARD1_WIDTH1R { bits }
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
    #[doc = "Bit 0 - Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline]
    pub fn card0_width0(&mut self) -> _CARD0_WIDTH0W {
        _CARD0_WIDTH0W { w: self }
    }
    #[doc = "Bit 1 - Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline]
    pub fn card1_width0(&mut self) -> _CARD1_WIDTH0W {
        _CARD1_WIDTH0W { w: self }
    }
    #[doc = "Bit 16 - Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline]
    pub fn card0_width1(&mut self) -> _CARD0_WIDTH1W {
        _CARD0_WIDTH1W { w: self }
    }
    #[doc = "Bit 17 - Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline]
    pub fn card1_width1(&mut self) -> _CARD1_WIDTH1W {
        _CARD1_WIDTH1W { w: self }
    }
}
