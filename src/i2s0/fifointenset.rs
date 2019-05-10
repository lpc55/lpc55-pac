#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOINTENSET {
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
#[doc = "Possible values of the field `TXERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXERRR {
    #[doc = "No interrupt will be generated for a transmit error."]
    DISABLED,
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    ENABLED,
}
impl TXERRR {
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
            TXERRR::DISABLED => false,
            TXERRR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXERRR {
        match value {
            false => TXERRR::DISABLED,
            true => TXERRR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXERRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXERRR::ENABLED
    }
}
#[doc = "Possible values of the field `RXERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERRR {
    #[doc = "No interrupt will be generated for a receive error."]
    DISABLED,
    #[doc = "An interrupt will be generated when a receive error occurs."]
    ENABLED,
}
impl RXERRR {
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
            RXERRR::DISABLED => false,
            RXERRR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXERRR {
        match value {
            false => RXERRR::DISABLED,
            true => RXERRR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXERRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXERRR::ENABLED
    }
}
#[doc = "Possible values of the field `TXLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLVLR {
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    DISABLED,
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    ENABLED,
}
impl TXLVLR {
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
            TXLVLR::DISABLED => false,
            TXLVLR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXLVLR {
        match value {
            false => TXLVLR::DISABLED,
            true => TXLVLR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXLVLR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXLVLR::ENABLED
    }
}
#[doc = "Possible values of the field `RXLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLVLR {
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    DISABLED,
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    ENABLED,
}
impl RXLVLR {
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
            RXLVLR::DISABLED => false,
            RXLVLR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXLVLR {
        match value {
            false => RXLVLR::DISABLED,
            true => RXLVLR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXLVLR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXLVLR::ENABLED
    }
}
#[doc = "Values that can be written to the field `TXERR`"]
pub enum TXERRW {
    #[doc = "No interrupt will be generated for a transmit error."]
    DISABLED,
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    ENABLED,
}
impl TXERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXERRW::DISABLED => false,
            TXERRW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXERRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated for a transmit error."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXERRW::DISABLED)
    }
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXERRW::ENABLED)
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
#[doc = "Values that can be written to the field `RXERR`"]
pub enum RXERRW {
    #[doc = "No interrupt will be generated for a receive error."]
    DISABLED,
    #[doc = "An interrupt will be generated when a receive error occurs."]
    ENABLED,
}
impl RXERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXERRW::DISABLED => false,
            RXERRW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXERRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated for a receive error."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXERRW::DISABLED)
    }
    #[doc = "An interrupt will be generated when a receive error occurs."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXERRW::ENABLED)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXLVL`"]
pub enum TXLVLW {
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    DISABLED,
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    ENABLED,
}
impl TXLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXLVLW::DISABLED => false,
            TXLVLW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXLVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXLVLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVLW::DISABLED)
    }
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVLW::ENABLED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXLVL`"]
pub enum RXLVLW {
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    DISABLED,
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    ENABLED,
}
impl RXLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXLVLW::DISABLED => false,
            RXLVLW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXLVLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVLW::DISABLED)
    }
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVLW::ENABLED)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline]
    pub fn txerr(&self) -> TXERRR {
        TXERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline]
    pub fn rxerr(&self) -> RXERRR {
        RXERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline]
    pub fn txlvl(&self) -> TXLVLR {
        TXLVLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline]
    pub fn rxlvl(&self) -> RXLVLR {
        RXLVLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline]
    pub fn txerr(&mut self) -> _TXERRW {
        _TXERRW { w: self }
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline]
    pub fn rxerr(&mut self) -> _RXERRW {
        _RXERRW { w: self }
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline]
    pub fn txlvl(&mut self) -> _TXLVLW {
        _TXLVLW { w: self }
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline]
    pub fn rxlvl(&mut self) -> _RXLVLW {
        _RXLVLW { w: self }
    }
}
