#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOCFG {
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
#[doc = "Possible values of the field `ENABLETX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLETXR {
    #[doc = "The transmit FIFO is not enabled."]
    DISABLED,
    #[doc = "The transmit FIFO is enabled."]
    ENABLED,
}
impl ENABLETXR {
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
            ENABLETXR::DISABLED => false,
            ENABLETXR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLETXR {
        match value {
            false => ENABLETXR::DISABLED,
            true => ENABLETXR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLETXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLETXR::ENABLED
    }
}
#[doc = "Possible values of the field `ENABLERX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLERXR {
    #[doc = "The receive FIFO is not enabled."]
    DISABLED,
    #[doc = "The receive FIFO is enabled."]
    ENABLED,
}
impl ENABLERXR {
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
            ENABLERXR::DISABLED => false,
            ENABLERXR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLERXR {
        match value {
            false => ENABLERXR::DISABLED,
            true => ENABLERXR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLERXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLERXR::ENABLED
    }
}
#[doc = "Possible values of the field `TXI2SE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXI2SE0R {
    #[doc = "If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair."]
    LAST_VALUE,
    #[doc = "If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred."]
    ZERO,
}
impl TXI2SE0R {
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
            TXI2SE0R::LAST_VALUE => false,
            TXI2SE0R::ZERO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXI2SE0R {
        match value {
            false => TXI2SE0R::LAST_VALUE,
            true => TXI2SE0R::ZERO,
        }
    }
    #[doc = "Checks if the value of the field is `LAST_VALUE`"]
    #[inline]
    pub fn is_last_value(&self) -> bool {
        *self == TXI2SE0R::LAST_VALUE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == TXI2SE0R::ZERO
    }
}
#[doc = "Possible values of the field `PACK48`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACK48R {
    #[doc = "48-bit I2S FIFO entries are handled as all 24-bit values."]
    BIT_24,
    #[doc = "48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values."]
    BIT_32_16,
}
impl PACK48R {
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
            PACK48R::BIT_24 => false,
            PACK48R::BIT_32_16 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PACK48R {
        match value {
            false => PACK48R::BIT_24,
            true => PACK48R::BIT_32_16,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_24`"]
    #[inline]
    pub fn is_bit_24(&self) -> bool {
        *self == PACK48R::BIT_24
    }
    #[doc = "Checks if the value of the field is `BIT_32_16`"]
    #[inline]
    pub fn is_bit_32_16(&self) -> bool {
        *self == PACK48R::BIT_32_16
    }
}
#[doc = r" Value of the field"]
pub struct SIZER {
    bits: u8,
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DMATX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMATXR {
    #[doc = "DMA is not used for the transmit function."]
    DISABLED,
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl DMATXR {
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
            DMATXR::DISABLED => false,
            DMATXR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMATXR {
        match value {
            false => DMATXR::DISABLED,
            true => DMATXR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMATXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMATXR::ENABLED
    }
}
#[doc = "Possible values of the field `DMARX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARXR {
    #[doc = "DMA is not used for the receive function."]
    DISABLED,
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl DMARXR {
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
            DMARXR::DISABLED => false,
            DMARXR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMARXR {
        match value {
            false => DMARXR::DISABLED,
            true => DMARXR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DMARXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DMARXR::ENABLED
    }
}
#[doc = "Possible values of the field `WAKETX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKETXR {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    ENABLED,
}
impl WAKETXR {
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
            WAKETXR::DISABLED => false,
            WAKETXR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKETXR {
        match value {
            false => WAKETXR::DISABLED,
            true => WAKETXR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WAKETXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WAKETXR::ENABLED
    }
}
#[doc = "Possible values of the field `WAKERX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKERXR {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    ENABLED,
}
impl WAKERXR {
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
            WAKERXR::DISABLED => false,
            WAKERXR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKERXR {
        match value {
            false => WAKERXR::DISABLED,
            true => WAKERXR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WAKERXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WAKERXR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct EMPTYTXR {
    bits: bool,
}
impl EMPTYTXR {
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
pub struct EMPTYRXR {
    bits: bool,
}
impl EMPTYRXR {
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
#[doc = "Possible values of the field `POPDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POPDBGR {
    #[doc = "Debug reads of the FIFO do not pop the FIFO."]
    DO_NOT_POP,
    #[doc = "A debug read will cause the FIFO to pop."]
    POP,
}
impl POPDBGR {
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
            POPDBGR::DO_NOT_POP => false,
            POPDBGR::POP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POPDBGR {
        match value {
            false => POPDBGR::DO_NOT_POP,
            true => POPDBGR::POP,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOT_POP`"]
    #[inline]
    pub fn is_do_not_pop(&self) -> bool {
        *self == POPDBGR::DO_NOT_POP
    }
    #[doc = "Checks if the value of the field is `POP`"]
    #[inline]
    pub fn is_pop(&self) -> bool {
        *self == POPDBGR::POP
    }
}
#[doc = "Values that can be written to the field `ENABLETX`"]
pub enum ENABLETXW {
    #[doc = "The transmit FIFO is not enabled."]
    DISABLED,
    #[doc = "The transmit FIFO is enabled."]
    ENABLED,
}
impl ENABLETXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLETXW::DISABLED => false,
            ENABLETXW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLETXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLETXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLETXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmit FIFO is not enabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLETXW::DISABLED)
    }
    #[doc = "The transmit FIFO is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLETXW::ENABLED)
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
#[doc = "Values that can be written to the field `ENABLERX`"]
pub enum ENABLERXW {
    #[doc = "The receive FIFO is not enabled."]
    DISABLED,
    #[doc = "The receive FIFO is enabled."]
    ENABLED,
}
impl ENABLERXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLERXW::DISABLED => false,
            ENABLERXW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLERXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLERXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLERXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receive FIFO is not enabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLERXW::DISABLED)
    }
    #[doc = "The receive FIFO is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLERXW::ENABLED)
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
#[doc = "Values that can be written to the field `TXI2SE0`"]
pub enum TXI2SE0W {
    #[doc = "If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair."]
    LAST_VALUE,
    #[doc = "If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred."]
    ZERO,
}
impl TXI2SE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXI2SE0W::LAST_VALUE => false,
            TXI2SE0W::ZERO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXI2SE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TXI2SE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXI2SE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair."]
    #[inline]
    pub fn last_value(self) -> &'a mut W {
        self.variant(TXI2SE0W::LAST_VALUE)
    }
    #[doc = "If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred."]
    #[inline]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXI2SE0W::ZERO)
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
#[doc = "Values that can be written to the field `PACK48`"]
pub enum PACK48W {
    #[doc = "48-bit I2S FIFO entries are handled as all 24-bit values."]
    BIT_24,
    #[doc = "48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values."]
    BIT_32_16,
}
impl PACK48W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PACK48W::BIT_24 => false,
            PACK48W::BIT_32_16 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PACK48W<'a> {
    w: &'a mut W,
}
impl<'a> _PACK48W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PACK48W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "48-bit I2S FIFO entries are handled as all 24-bit values."]
    #[inline]
    pub fn bit_24(self) -> &'a mut W {
        self.variant(PACK48W::BIT_24)
    }
    #[doc = "48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values."]
    #[inline]
    pub fn bit_32_16(self) -> &'a mut W {
        self.variant(PACK48W::BIT_32_16)
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
#[doc = "Values that can be written to the field `DMATX`"]
pub enum DMATXW {
    #[doc = "DMA is not used for the transmit function."]
    DISABLED,
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl DMATXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMATXW::DISABLED => false,
            DMATXW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMATXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMATXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMATXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is not used for the transmit function."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMATXW::DISABLED)
    }
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMATXW::ENABLED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMARX`"]
pub enum DMARXW {
    #[doc = "DMA is not used for the receive function."]
    DISABLED,
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl DMARXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMARXW::DISABLED => false,
            DMARXW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMARXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMARXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMARXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is not used for the receive function."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMARXW::DISABLED)
    }
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMARXW::ENABLED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAKETX`"]
pub enum WAKETXW {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    ENABLED,
}
impl WAKETXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKETXW::DISABLED => false,
            WAKETXW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKETXW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKETXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKETXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKETXW::DISABLED)
    }
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKETXW::ENABLED)
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
#[doc = "Values that can be written to the field `WAKERX`"]
pub enum WAKERXW {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    ENABLED,
}
impl WAKERXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKERXW::DISABLED => false,
            WAKERXW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKERXW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKERXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKERXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKERXW::DISABLED)
    }
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKERXW::ENABLED)
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
pub struct _EMPTYTXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMPTYTXW<'a> {
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
pub struct _EMPTYRXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMPTYRXW<'a> {
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
#[doc = "Values that can be written to the field `POPDBG`"]
pub enum POPDBGW {
    #[doc = "Debug reads of the FIFO do not pop the FIFO."]
    DO_NOT_POP,
    #[doc = "A debug read will cause the FIFO to pop."]
    POP,
}
impl POPDBGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POPDBGW::DO_NOT_POP => false,
            POPDBGW::POP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POPDBGW<'a> {
    w: &'a mut W,
}
impl<'a> _POPDBGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POPDBGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Debug reads of the FIFO do not pop the FIFO."]
    #[inline]
    pub fn do_not_pop(self) -> &'a mut W {
        self.variant(POPDBGW::DO_NOT_POP)
    }
    #[doc = "A debug read will cause the FIFO to pop."]
    #[inline]
    pub fn pop(self) -> &'a mut W {
        self.variant(POPDBGW::POP)
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline]
    pub fn enabletx(&self) -> ENABLETXR {
        ENABLETXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline]
    pub fn enablerx(&self) -> ENABLERXR {
        ENABLERXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused."]
    #[inline]
    pub fn txi2se0(&self) -> TXI2SE0R {
        TXI2SE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA."]
    #[inline]
    pub fn pack48(&self) -> PACK48R {
        PACK48R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[inline]
    pub fn size(&self) -> SIZER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SIZER { bits }
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline]
    pub fn dmatx(&self) -> DMATXR {
        DMATXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline]
    pub fn dmarx(&self) -> DMARXR {
        DMARXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline]
    pub fn waketx(&self) -> WAKETXR {
        WAKETXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline]
    pub fn wakerx(&self) -> WAKERXR {
        WAKERXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline]
    pub fn emptytx(&self) -> EMPTYTXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EMPTYTXR { bits }
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline]
    pub fn emptyrx(&self) -> EMPTYRXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EMPTYRXR { bits }
    }
    #[doc = "Bit 18 - Pop FIFO for debug reads."]
    #[inline]
    pub fn popdbg(&self) -> POPDBGR {
        POPDBGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline]
    pub fn enabletx(&mut self) -> _ENABLETXW {
        _ENABLETXW { w: self }
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline]
    pub fn enablerx(&mut self) -> _ENABLERXW {
        _ENABLERXW { w: self }
    }
    #[doc = "Bit 2 - Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused."]
    #[inline]
    pub fn txi2se0(&mut self) -> _TXI2SE0W {
        _TXI2SE0W { w: self }
    }
    #[doc = "Bit 3 - Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA."]
    #[inline]
    pub fn pack48(&mut self) -> _PACK48W {
        _PACK48W { w: self }
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline]
    pub fn dmatx(&mut self) -> _DMATXW {
        _DMATXW { w: self }
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline]
    pub fn dmarx(&mut self) -> _DMARXW {
        _DMARXW { w: self }
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline]
    pub fn waketx(&mut self) -> _WAKETXW {
        _WAKETXW { w: self }
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline]
    pub fn wakerx(&mut self) -> _WAKERXW {
        _WAKERXW { w: self }
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline]
    pub fn emptytx(&mut self) -> _EMPTYTXW {
        _EMPTYTXW { w: self }
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline]
    pub fn emptyrx(&mut self) -> _EMPTYRXW {
        _EMPTYRXW { w: self }
    }
    #[doc = "Bit 18 - Pop FIFO for debug reads."]
    #[inline]
    pub fn popdbg(&mut self) -> _POPDBGW {
        _POPDBGW { w: self }
    }
}
