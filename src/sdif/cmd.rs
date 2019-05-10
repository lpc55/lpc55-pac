#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMD {
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
pub struct CMD_INDEXR {
    bits: u8,
}
impl CMD_INDEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESPONSE_EXPECTR {
    bits: bool,
}
impl RESPONSE_EXPECTR {
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
pub struct RESPONSE_LENGTHR {
    bits: bool,
}
impl RESPONSE_LENGTHR {
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
pub struct CHECK_RESPONSE_CRCR {
    bits: bool,
}
impl CHECK_RESPONSE_CRCR {
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
pub struct DATA_EXPECTEDR {
    bits: bool,
}
impl DATA_EXPECTEDR {
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
pub struct READ_WRITER {
    bits: bool,
}
impl READ_WRITER {
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
pub struct TRANSFER_MODER {
    bits: bool,
}
impl TRANSFER_MODER {
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
pub struct SEND_AUTO_STOPR {
    bits: bool,
}
impl SEND_AUTO_STOPR {
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
pub struct WAIT_PRVDATA_COMPLETER {
    bits: bool,
}
impl WAIT_PRVDATA_COMPLETER {
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
pub struct STOP_ABORT_CMDR {
    bits: bool,
}
impl STOP_ABORT_CMDR {
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
pub struct SEND_INITIALIZATIONR {
    bits: bool,
}
impl SEND_INITIALIZATIONR {
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
#[doc = "Possible values of the field `CARD_NUMBER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_NUMBERR {
    #[doc = "Command will be execute on SDCARD 0"]
    CARD0,
    #[doc = "Command will be execute on SDCARD 1"]
    CARD1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CARD_NUMBERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CARD_NUMBERR::CARD0 => 0,
            CARD_NUMBERR::CARD1 => 1,
            CARD_NUMBERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CARD_NUMBERR {
        match value {
            0 => CARD_NUMBERR::CARD0,
            1 => CARD_NUMBERR::CARD1,
            i => CARD_NUMBERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CARD0`"]
    #[inline]
    pub fn is_card0(&self) -> bool {
        *self == CARD_NUMBERR::CARD0
    }
    #[doc = "Checks if the value of the field is `CARD1`"]
    #[inline]
    pub fn is_card1(&self) -> bool {
        *self == CARD_NUMBERR::CARD1
    }
}
#[doc = r" Value of the field"]
pub struct UPDATE_CLOCK_REGISTERS_ONLYR {
    bits: bool,
}
impl UPDATE_CLOCK_REGISTERS_ONLYR {
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
pub struct READ_CEATA_DEVICER {
    bits: bool,
}
impl READ_CEATA_DEVICER {
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
pub struct CCS_EXPECTEDR {
    bits: bool,
}
impl CCS_EXPECTEDR {
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
pub struct ENABLE_BOOTR {
    bits: bool,
}
impl ENABLE_BOOTR {
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
pub struct EXPECT_BOOT_ACKR {
    bits: bool,
}
impl EXPECT_BOOT_ACKR {
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
pub struct DISABLE_BOOTR {
    bits: bool,
}
impl DISABLE_BOOTR {
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
pub struct BOOT_MODER {
    bits: bool,
}
impl BOOT_MODER {
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
pub struct VOLT_SWITCHR {
    bits: bool,
}
impl VOLT_SWITCHR {
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
pub struct USE_HOLD_REGR {
    bits: bool,
}
impl USE_HOLD_REGR {
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
pub struct START_CMDR {
    bits: bool,
}
impl START_CMDR {
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
pub struct _CMD_INDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_INDEXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESPONSE_EXPECTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPONSE_EXPECTW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESPONSE_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPONSE_LENGTHW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHECK_RESPONSE_CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CHECK_RESPONSE_CRCW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_EXPECTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_EXPECTEDW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _READ_WRITEW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_WRITEW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRANSFER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRANSFER_MODEW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEND_AUTO_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_AUTO_STOPW<'a> {
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
pub struct _WAIT_PRVDATA_COMPLETEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAIT_PRVDATA_COMPLETEW<'a> {
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
pub struct _STOP_ABORT_CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_ABORT_CMDW<'a> {
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
#[doc = r" Proxy"]
pub struct _SEND_INITIALIZATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_INITIALIZATIONW<'a> {
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
#[doc = "Values that can be written to the field `CARD_NUMBER`"]
pub enum CARD_NUMBERW {
    #[doc = "Command will be execute on SDCARD 0"]
    CARD0,
    #[doc = "Command will be execute on SDCARD 1"]
    CARD1,
}
impl CARD_NUMBERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CARD_NUMBERW::CARD0 => 0,
            CARD_NUMBERW::CARD1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CARD_NUMBERW<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_NUMBERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CARD_NUMBERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Command will be execute on SDCARD 0"]
    #[inline]
    pub fn card0(self) -> &'a mut W {
        self.variant(CARD_NUMBERW::CARD0)
    }
    #[doc = "Command will be execute on SDCARD 1"]
    #[inline]
    pub fn card1(self) -> &'a mut W {
        self.variant(CARD_NUMBERW::CARD1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UPDATE_CLOCK_REGISTERS_ONLYW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDATE_CLOCK_REGISTERS_ONLYW<'a> {
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
#[doc = r" Proxy"]
pub struct _READ_CEATA_DEVICEW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_CEATA_DEVICEW<'a> {
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
pub struct _CCS_EXPECTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _CCS_EXPECTEDW<'a> {
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
pub struct _ENABLE_BOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_BOOTW<'a> {
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
#[doc = r" Proxy"]
pub struct _EXPECT_BOOT_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _EXPECT_BOOT_ACKW<'a> {
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
#[doc = r" Proxy"]
pub struct _DISABLE_BOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_BOOTW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_MODEW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VOLT_SWITCHW<'a> {
    w: &'a mut W,
}
impl<'a> _VOLT_SWITCHW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USE_HOLD_REGW<'a> {
    w: &'a mut W,
}
impl<'a> _USE_HOLD_REGW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _START_CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _START_CMDW<'a> {
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
    #[doc = "Bits 0:5 - Command index."]
    #[inline]
    pub fn cmd_index(&self) -> CMD_INDEXR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMD_INDEXR { bits }
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline]
    pub fn response_expect(&self) -> RESPONSE_EXPECTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESPONSE_EXPECTR { bits }
    }
    #[doc = "Bit 7 - Response length."]
    #[inline]
    pub fn response_length(&self) -> RESPONSE_LENGTHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESPONSE_LENGTHR { bits }
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline]
    pub fn check_response_crc(&self) -> CHECK_RESPONSE_CRCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHECK_RESPONSE_CRCR { bits }
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline]
    pub fn data_expected(&self) -> DATA_EXPECTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATA_EXPECTEDR { bits }
    }
    #[doc = "Bit 10 - read/write."]
    #[inline]
    pub fn read_write(&self) -> READ_WRITER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        READ_WRITER { bits }
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline]
    pub fn transfer_mode(&self) -> TRANSFER_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRANSFER_MODER { bits }
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline]
    pub fn send_auto_stop(&self) -> SEND_AUTO_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEND_AUTO_STOPR { bits }
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline]
    pub fn wait_prvdata_complete(&self) -> WAIT_PRVDATA_COMPLETER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAIT_PRVDATA_COMPLETER { bits }
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline]
    pub fn stop_abort_cmd(&self) -> STOP_ABORT_CMDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOP_ABORT_CMDR { bits }
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline]
    pub fn send_initialization(&self) -> SEND_INITIALIZATIONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEND_INITIALIZATIONR { bits }
    }
    #[doc = "Bits 16:20 - Specifies the card number of SDCARD for which the current Command is being executed"]
    #[inline]
    pub fn card_number(&self) -> CARD_NUMBERR {
        CARD_NUMBERR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline]
    pub fn update_clock_registers_only(&self) -> UPDATE_CLOCK_REGISTERS_ONLYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UPDATE_CLOCK_REGISTERS_ONLYR { bits }
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline]
    pub fn read_ceata_device(&self) -> READ_CEATA_DEVICER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        READ_CEATA_DEVICER { bits }
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline]
    pub fn ccs_expected(&self) -> CCS_EXPECTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CCS_EXPECTEDR { bits }
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline]
    pub fn enable_boot(&self) -> ENABLE_BOOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLE_BOOTR { bits }
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline]
    pub fn expect_boot_ack(&self) -> EXPECT_BOOT_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXPECT_BOOT_ACKR { bits }
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline]
    pub fn disable_boot(&self) -> DISABLE_BOOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISABLE_BOOTR { bits }
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline]
    pub fn boot_mode(&self) -> BOOT_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOT_MODER { bits }
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline]
    pub fn volt_switch(&self) -> VOLT_SWITCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VOLT_SWITCHR { bits }
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline]
    pub fn use_hold_reg(&self) -> USE_HOLD_REGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USE_HOLD_REGR { bits }
    }
    #[doc = "Bit 31 - Start command."]
    #[inline]
    pub fn start_cmd(&self) -> START_CMDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        START_CMDR { bits }
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
    #[doc = "Bits 0:5 - Command index."]
    #[inline]
    pub fn cmd_index(&mut self) -> _CMD_INDEXW {
        _CMD_INDEXW { w: self }
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline]
    pub fn response_expect(&mut self) -> _RESPONSE_EXPECTW {
        _RESPONSE_EXPECTW { w: self }
    }
    #[doc = "Bit 7 - Response length."]
    #[inline]
    pub fn response_length(&mut self) -> _RESPONSE_LENGTHW {
        _RESPONSE_LENGTHW { w: self }
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline]
    pub fn check_response_crc(&mut self) -> _CHECK_RESPONSE_CRCW {
        _CHECK_RESPONSE_CRCW { w: self }
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline]
    pub fn data_expected(&mut self) -> _DATA_EXPECTEDW {
        _DATA_EXPECTEDW { w: self }
    }
    #[doc = "Bit 10 - read/write."]
    #[inline]
    pub fn read_write(&mut self) -> _READ_WRITEW {
        _READ_WRITEW { w: self }
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline]
    pub fn transfer_mode(&mut self) -> _TRANSFER_MODEW {
        _TRANSFER_MODEW { w: self }
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline]
    pub fn send_auto_stop(&mut self) -> _SEND_AUTO_STOPW {
        _SEND_AUTO_STOPW { w: self }
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline]
    pub fn wait_prvdata_complete(&mut self) -> _WAIT_PRVDATA_COMPLETEW {
        _WAIT_PRVDATA_COMPLETEW { w: self }
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline]
    pub fn stop_abort_cmd(&mut self) -> _STOP_ABORT_CMDW {
        _STOP_ABORT_CMDW { w: self }
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline]
    pub fn send_initialization(&mut self) -> _SEND_INITIALIZATIONW {
        _SEND_INITIALIZATIONW { w: self }
    }
    #[doc = "Bits 16:20 - Specifies the card number of SDCARD for which the current Command is being executed"]
    #[inline]
    pub fn card_number(&mut self) -> _CARD_NUMBERW {
        _CARD_NUMBERW { w: self }
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline]
    pub fn update_clock_registers_only(&mut self) -> _UPDATE_CLOCK_REGISTERS_ONLYW {
        _UPDATE_CLOCK_REGISTERS_ONLYW { w: self }
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline]
    pub fn read_ceata_device(&mut self) -> _READ_CEATA_DEVICEW {
        _READ_CEATA_DEVICEW { w: self }
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline]
    pub fn ccs_expected(&mut self) -> _CCS_EXPECTEDW {
        _CCS_EXPECTEDW { w: self }
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline]
    pub fn enable_boot(&mut self) -> _ENABLE_BOOTW {
        _ENABLE_BOOTW { w: self }
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline]
    pub fn expect_boot_ack(&mut self) -> _EXPECT_BOOT_ACKW {
        _EXPECT_BOOT_ACKW { w: self }
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline]
    pub fn disable_boot(&mut self) -> _DISABLE_BOOTW {
        _DISABLE_BOOTW { w: self }
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline]
    pub fn boot_mode(&mut self) -> _BOOT_MODEW {
        _BOOT_MODEW { w: self }
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline]
    pub fn volt_switch(&mut self) -> _VOLT_SWITCHW {
        _VOLT_SWITCHW { w: self }
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline]
    pub fn use_hold_reg(&mut self) -> _USE_HOLD_REGW {
        _USE_HOLD_REGW { w: self }
    }
    #[doc = "Bit 31 - Start command."]
    #[inline]
    pub fn start_cmd(&mut self) -> _START_CMDW {
        _START_CMDW { w: self }
    }
}
