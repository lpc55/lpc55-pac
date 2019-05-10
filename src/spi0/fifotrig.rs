#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOTRIG {
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
#[doc = "Possible values of the field `TXLVLENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLVLENAR {
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    DISABLED,
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    ENABLED,
}
impl TXLVLENAR {
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
            TXLVLENAR::DISABLED => false,
            TXLVLENAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXLVLENAR {
        match value {
            false => TXLVLENAR::DISABLED,
            true => TXLVLENAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXLVLENAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXLVLENAR::ENABLED
    }
}
#[doc = "Possible values of the field `RXLVLENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLVLENAR {
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    DISABLED,
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    ENABLED,
}
impl RXLVLENAR {
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
            RXLVLENAR::DISABLED => false,
            RXLVLENAR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXLVLENAR {
        match value {
            false => RXLVLENAR::DISABLED,
            true => RXLVLENAR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXLVLENAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXLVLENAR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct TXLVLR {
    bits: u8,
}
impl TXLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXLVLR {
    bits: u8,
}
impl RXLVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TXLVLENA`"]
pub enum TXLVLENAW {
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    DISABLED,
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    ENABLED,
}
impl TXLVLENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXLVLENAW::DISABLED => false,
            TXLVLENAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXLVLENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TXLVLENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXLVLENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVLENAW::DISABLED)
    }
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVLENAW::ENABLED)
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
#[doc = "Values that can be written to the field `RXLVLENA`"]
pub enum RXLVLENAW {
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    DISABLED,
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    ENABLED,
}
impl RXLVLENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXLVLENAW::DISABLED => false,
            RXLVLENAW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXLVLENAW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLVLENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXLVLENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVLENAW::DISABLED)
    }
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVLENAW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _TXLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXLVLW<'a> {
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
pub struct _RXLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLVLW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline]
    pub fn txlvlena(&self) -> TXLVLENAR {
        TXLVLENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline]
    pub fn rxlvlena(&self) -> RXLVLENAR {
        RXLVLENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline]
    pub fn txlvl(&self) -> TXLVLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXLVLR { bits }
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline]
    pub fn rxlvl(&self) -> RXLVLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXLVLR { bits }
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
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline]
    pub fn txlvlena(&mut self) -> _TXLVLENAW {
        _TXLVLENAW { w: self }
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline]
    pub fn rxlvlena(&mut self) -> _RXLVLENAW {
        _RXLVLENAW { w: self }
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline]
    pub fn txlvl(&mut self) -> _TXLVLW {
        _TXLVLW { w: self }
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline]
    pub fn rxlvl(&mut self) -> _RXLVLW {
        _RXLVLW { w: self }
    }
}
