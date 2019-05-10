#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSELID {
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
#[doc = "Possible values of the field `PERSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERSELR {
    #[doc = "No peripheral selected."]
    NO_PERIPH_SELECTED,
    #[doc = "USART function selected."]
    USART,
    #[doc = "SPI function selected."]
    SPI,
    #[doc = "I2C function selected."]
    I2C,
    #[doc = "I2S transmit function selected."]
    I2S_TRANSMIT,
    #[doc = "I2S receive function selected."]
    I2S_RECEIVE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PERSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PERSELR::NO_PERIPH_SELECTED => 0,
            PERSELR::USART => 1,
            PERSELR::SPI => 2,
            PERSELR::I2C => 3,
            PERSELR::I2S_TRANSMIT => 4,
            PERSELR::I2S_RECEIVE => 5,
            PERSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PERSELR {
        match value {
            0 => PERSELR::NO_PERIPH_SELECTED,
            1 => PERSELR::USART,
            2 => PERSELR::SPI,
            3 => PERSELR::I2C,
            4 => PERSELR::I2S_TRANSMIT,
            5 => PERSELR::I2S_RECEIVE,
            i => PERSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_PERIPH_SELECTED`"]
    #[inline]
    pub fn is_no_periph_selected(&self) -> bool {
        *self == PERSELR::NO_PERIPH_SELECTED
    }
    #[doc = "Checks if the value of the field is `USART`"]
    #[inline]
    pub fn is_usart(&self) -> bool {
        *self == PERSELR::USART
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline]
    pub fn is_spi(&self) -> bool {
        *self == PERSELR::SPI
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline]
    pub fn is_i2c(&self) -> bool {
        *self == PERSELR::I2C
    }
    #[doc = "Checks if the value of the field is `I2S_TRANSMIT`"]
    #[inline]
    pub fn is_i2s_transmit(&self) -> bool {
        *self == PERSELR::I2S_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `I2S_RECEIVE`"]
    #[inline]
    pub fn is_i2s_receive(&self) -> bool {
        *self == PERSELR::I2S_RECEIVE
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "Peripheral select can be changed by software."]
    UNLOCKED,
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    LOCKED,
}
impl LOCKR {
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
            LOCKR::UNLOCKED => false,
            LOCKR::LOCKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::UNLOCKED,
            true => LOCKR::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR::LOCKED
    }
}
#[doc = "Possible values of the field `USARTPRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USARTPRESENTR {
    #[doc = "This Flexcomm does not include the USART function."]
    NOT_PRESENT,
    #[doc = "This Flexcomm includes the USART function."]
    PRESENT,
}
impl USARTPRESENTR {
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
            USARTPRESENTR::NOT_PRESENT => false,
            USARTPRESENTR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USARTPRESENTR {
        match value {
            false => USARTPRESENTR::NOT_PRESENT,
            true => USARTPRESENTR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == USARTPRESENTR::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == USARTPRESENTR::PRESENT
    }
}
#[doc = "Possible values of the field `SPIPRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIPRESENTR {
    #[doc = "This Flexcomm does not include the SPI function."]
    NOT_PRESENT,
    #[doc = "This Flexcomm includes the SPI function."]
    PRESENT,
}
impl SPIPRESENTR {
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
            SPIPRESENTR::NOT_PRESENT => false,
            SPIPRESENTR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPIPRESENTR {
        match value {
            false => SPIPRESENTR::NOT_PRESENT,
            true => SPIPRESENTR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == SPIPRESENTR::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == SPIPRESENTR::PRESENT
    }
}
#[doc = "Possible values of the field `I2CPRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CPRESENTR {
    #[doc = "This Flexcomm does not include the I2C function."]
    NOT_PRESENT,
    #[doc = "This Flexcomm includes the I2C function."]
    PRESENT,
}
impl I2CPRESENTR {
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
            I2CPRESENTR::NOT_PRESENT => false,
            I2CPRESENTR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2CPRESENTR {
        match value {
            false => I2CPRESENTR::NOT_PRESENT,
            true => I2CPRESENTR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == I2CPRESENTR::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == I2CPRESENTR::PRESENT
    }
}
#[doc = "Possible values of the field `I2SPRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SPRESENTR {
    #[doc = "This Flexcomm does not include the I2S function."]
    NOT_PRESENT,
    #[doc = "This Flexcomm includes the I2S function."]
    PRESENT,
}
impl I2SPRESENTR {
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
            I2SPRESENTR::NOT_PRESENT => false,
            I2SPRESENTR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2SPRESENTR {
        match value {
            false => I2SPRESENTR::NOT_PRESENT,
            true => I2SPRESENTR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == I2SPRESENTR::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == I2SPRESENTR::PRESENT
    }
}
#[doc = r" Value of the field"]
pub struct IDR {
    bits: u32,
}
impl IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `PERSEL`"]
pub enum PERSELW {
    #[doc = "No peripheral selected."]
    NO_PERIPH_SELECTED,
    #[doc = "USART function selected."]
    USART,
    #[doc = "SPI function selected."]
    SPI,
    #[doc = "I2C function selected."]
    I2C,
    #[doc = "I2S transmit function selected."]
    I2S_TRANSMIT,
    #[doc = "I2S receive function selected."]
    I2S_RECEIVE,
}
impl PERSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERSELW::NO_PERIPH_SELECTED => 0,
            PERSELW::USART => 1,
            PERSELW::SPI => 2,
            PERSELW::I2C => 3,
            PERSELW::I2S_TRANSMIT => 4,
            PERSELW::I2S_RECEIVE => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No peripheral selected."]
    #[inline]
    pub fn no_periph_selected(self) -> &'a mut W {
        self.variant(PERSELW::NO_PERIPH_SELECTED)
    }
    #[doc = "USART function selected."]
    #[inline]
    pub fn usart(self) -> &'a mut W {
        self.variant(PERSELW::USART)
    }
    #[doc = "SPI function selected."]
    #[inline]
    pub fn spi(self) -> &'a mut W {
        self.variant(PERSELW::SPI)
    }
    #[doc = "I2C function selected."]
    #[inline]
    pub fn i2c(self) -> &'a mut W {
        self.variant(PERSELW::I2C)
    }
    #[doc = "I2S transmit function selected."]
    #[inline]
    pub fn i2s_transmit(self) -> &'a mut W {
        self.variant(PERSELW::I2S_TRANSMIT)
    }
    #[doc = "I2S receive function selected."]
    #[inline]
    pub fn i2s_receive(self) -> &'a mut W {
        self.variant(PERSELW::I2S_RECEIVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "Peripheral select can be changed by software."]
    UNLOCKED,
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    LOCKED,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::UNLOCKED => false,
            LOCKW::LOCKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral select can be changed by software."]
    #[inline]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKW::UNLOCKED)
    }
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    #[inline]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKW::LOCKED)
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
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline]
    pub fn persel(&self) -> PERSELR {
        PERSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - USART present indicator. This field is Read-only."]
    #[inline]
    pub fn usartpresent(&self) -> USARTPRESENTR {
        USARTPRESENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - SPI present indicator. This field is Read-only."]
    #[inline]
    pub fn spipresent(&self) -> SPIPRESENTR {
        SPIPRESENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - I2C present indicator. This field is Read-only."]
    #[inline]
    pub fn i2cpresent(&self) -> I2CPRESENTR {
        I2CPRESENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - I 2S present indicator. This field is Read-only."]
    #[inline]
    pub fn i2spresent(&self) -> I2SPRESENTR {
        I2SPRESENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:31 - Flexcomm ID."]
    #[inline]
    pub fn id(&self) -> IDR {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        IDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1052672 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline]
    pub fn persel(&mut self) -> _PERSELW {
        _PERSELW { w: self }
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}
