#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CRYPTCFG {
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
pub struct MSW1ST_OUTR {
    bits: bool,
}
impl MSW1ST_OUTR {
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
pub struct SWAPKEYR {
    bits: bool,
}
impl SWAPKEYR {
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
pub struct SWAPDATR {
    bits: bool,
}
impl SWAPDATR {
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
pub struct MSW1STR {
    bits: bool,
}
impl MSW1STR {
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
#[doc = "Possible values of the field `AESMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESMODER {
    #[doc = "ECB - used as is"]
    ECB,
    #[doc = "CBC mode (see details on IV/nonce)"]
    CBC,
    #[doc = "CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    CTR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AESMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AESMODER::ECB => 0,
            AESMODER::CBC => 1,
            AESMODER::CTR => 2,
            AESMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AESMODER {
        match value {
            0 => AESMODER::ECB,
            1 => AESMODER::CBC,
            2 => AESMODER::CTR,
            i => AESMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline]
    pub fn is_ecb(&self) -> bool {
        *self == AESMODER::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline]
    pub fn is_cbc(&self) -> bool {
        *self == AESMODER::CBC
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline]
    pub fn is_ctr(&self) -> bool {
        *self == AESMODER::CTR
    }
}
#[doc = "Possible values of the field `AESDECRYPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESDECRYPTR {
    #[doc = "Encrypt"]
    AESDECRYPT_0,
    #[doc = "Decrypt"]
    DECRYPT,
}
impl AESDECRYPTR {
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
            AESDECRYPTR::AESDECRYPT_0 => false,
            AESDECRYPTR::DECRYPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AESDECRYPTR {
        match value {
            false => AESDECRYPTR::AESDECRYPT_0,
            true => AESDECRYPTR::DECRYPT,
        }
    }
    #[doc = "Checks if the value of the field is `AESDECRYPT_0`"]
    #[inline]
    pub fn is_aesdecrypt_0(&self) -> bool {
        *self == AESDECRYPTR::AESDECRYPT_0
    }
    #[doc = "Checks if the value of the field is `DECRYPT`"]
    #[inline]
    pub fn is_decrypt(&self) -> bool {
        *self == AESDECRYPTR::DECRYPT
    }
}
#[doc = "Possible values of the field `AESSECRET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESSECRETR {
    #[doc = "User key provided in normal way"]
    NORMAL_WAY,
    #[doc = "Secret key provided in hidden way by HW"]
    AESSECRET_1,
}
impl AESSECRETR {
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
            AESSECRETR::NORMAL_WAY => false,
            AESSECRETR::AESSECRET_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AESSECRETR {
        match value {
            false => AESSECRETR::NORMAL_WAY,
            true => AESSECRETR::AESSECRET_1,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_WAY`"]
    #[inline]
    pub fn is_normal_way(&self) -> bool {
        *self == AESSECRETR::NORMAL_WAY
    }
    #[doc = "Checks if the value of the field is `AESSECRET_1`"]
    #[inline]
    pub fn is_aessecret_1(&self) -> bool {
        *self == AESSECRETR::AESSECRET_1
    }
}
#[doc = "Possible values of the field `AESKEYSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESKEYSZR {
    #[doc = "128 bit key"]
    BITS_128,
    #[doc = "192 bit key"]
    BITS_192,
    #[doc = "256 bit key"]
    BITS_256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AESKEYSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AESKEYSZR::BITS_128 => 0,
            AESKEYSZR::BITS_192 => 1,
            AESKEYSZR::BITS_256 => 2,
            AESKEYSZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AESKEYSZR {
        match value {
            0 => AESKEYSZR::BITS_128,
            1 => AESKEYSZR::BITS_192,
            2 => AESKEYSZR::BITS_256,
            i => AESKEYSZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BITS_128`"]
    #[inline]
    pub fn is_bits_128(&self) -> bool {
        *self == AESKEYSZR::BITS_128
    }
    #[doc = "Checks if the value of the field is `BITS_192`"]
    #[inline]
    pub fn is_bits_192(&self) -> bool {
        *self == AESKEYSZR::BITS_192
    }
    #[doc = "Checks if the value of the field is `BITS_256`"]
    #[inline]
    pub fn is_bits_256(&self) -> bool {
        *self == AESKEYSZR::BITS_256
    }
}
#[doc = r" Value of the field"]
pub struct AESCTRPOSR {
    bits: u8,
}
impl AESCTRPOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STREAMLASTR {
    bits: bool,
}
impl STREAMLASTR {
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
pub struct XSALSAR {
    bits: bool,
}
impl XSALSAR {
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
#[doc = "Possible values of the field `ICBSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICBSZR {
    #[doc = "32 bits of the IV/ctr are used (from 127:96)"]
    BITS_32,
    #[doc = "64 bits of the IV/ctr are used (from 127:64)"]
    BITS_64,
    #[doc = "96 bits of the IV/ctr are used (from 127:32)"]
    BITS_96,
    #[doc = "All 128 bits of the IV/ctr are used"]
    BIT_128,
}
impl ICBSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICBSZR::BITS_32 => 0,
            ICBSZR::BITS_64 => 1,
            ICBSZR::BITS_96 => 2,
            ICBSZR::BIT_128 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICBSZR {
        match value {
            0 => ICBSZR::BITS_32,
            1 => ICBSZR::BITS_64,
            2 => ICBSZR::BITS_96,
            3 => ICBSZR::BIT_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BITS_32`"]
    #[inline]
    pub fn is_bits_32(&self) -> bool {
        *self == ICBSZR::BITS_32
    }
    #[doc = "Checks if the value of the field is `BITS_64`"]
    #[inline]
    pub fn is_bits_64(&self) -> bool {
        *self == ICBSZR::BITS_64
    }
    #[doc = "Checks if the value of the field is `BITS_96`"]
    #[inline]
    pub fn is_bits_96(&self) -> bool {
        *self == ICBSZR::BITS_96
    }
    #[doc = "Checks if the value of the field is `BIT_128`"]
    #[inline]
    pub fn is_bit_128(&self) -> bool {
        *self == ICBSZR::BIT_128
    }
}
#[doc = "Possible values of the field `ICBSTRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICBSTRMR {
    #[doc = "8 blocks"]
    BLOCKS_8,
    #[doc = "16 blocks"]
    BLOCKS_16,
    #[doc = "32 blocks"]
    BLOCKS_32,
    #[doc = "64 blocks"]
    BLOCKS_64,
}
impl ICBSTRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICBSTRMR::BLOCKS_8 => 0,
            ICBSTRMR::BLOCKS_16 => 1,
            ICBSTRMR::BLOCKS_32 => 2,
            ICBSTRMR::BLOCKS_64 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICBSTRMR {
        match value {
            0 => ICBSTRMR::BLOCKS_8,
            1 => ICBSTRMR::BLOCKS_16,
            2 => ICBSTRMR::BLOCKS_32,
            3 => ICBSTRMR::BLOCKS_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKS_8`"]
    #[inline]
    pub fn is_blocks_8(&self) -> bool {
        *self == ICBSTRMR::BLOCKS_8
    }
    #[doc = "Checks if the value of the field is `BLOCKS_16`"]
    #[inline]
    pub fn is_blocks_16(&self) -> bool {
        *self == ICBSTRMR::BLOCKS_16
    }
    #[doc = "Checks if the value of the field is `BLOCKS_32`"]
    #[inline]
    pub fn is_blocks_32(&self) -> bool {
        *self == ICBSTRMR::BLOCKS_32
    }
    #[doc = "Checks if the value of the field is `BLOCKS_64`"]
    #[inline]
    pub fn is_blocks_64(&self) -> bool {
        *self == ICBSTRMR::BLOCKS_64
    }
}
#[doc = r" Proxy"]
pub struct _MSW1ST_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _MSW1ST_OUTW<'a> {
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
pub struct _SWAPKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPKEYW<'a> {
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
pub struct _SWAPDATW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPDATW<'a> {
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
#[doc = r" Proxy"]
pub struct _MSW1STW<'a> {
    w: &'a mut W,
}
impl<'a> _MSW1STW<'a> {
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
#[doc = "Values that can be written to the field `AESMODE`"]
pub enum AESMODEW {
    #[doc = "ECB - used as is"]
    ECB,
    #[doc = "CBC mode (see details on IV/nonce)"]
    CBC,
    #[doc = "CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    CTR,
}
impl AESMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AESMODEW::ECB => 0,
            AESMODEW::CBC => 1,
            AESMODEW::CTR => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AESMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _AESMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AESMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ECB - used as is"]
    #[inline]
    pub fn ecb(self) -> &'a mut W {
        self.variant(AESMODEW::ECB)
    }
    #[doc = "CBC mode (see details on IV/nonce)"]
    #[inline]
    pub fn cbc(self) -> &'a mut W {
        self.variant(AESMODEW::CBC)
    }
    #[doc = "CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    #[inline]
    pub fn ctr(self) -> &'a mut W {
        self.variant(AESMODEW::CTR)
    }
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
#[doc = "Values that can be written to the field `AESDECRYPT`"]
pub enum AESDECRYPTW {
    #[doc = "Encrypt"]
    AESDECRYPT_0,
    #[doc = "Decrypt"]
    DECRYPT,
}
impl AESDECRYPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AESDECRYPTW::AESDECRYPT_0 => false,
            AESDECRYPTW::DECRYPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AESDECRYPTW<'a> {
    w: &'a mut W,
}
impl<'a> _AESDECRYPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AESDECRYPTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Encrypt"]
    #[inline]
    pub fn aesdecrypt_0(self) -> &'a mut W {
        self.variant(AESDECRYPTW::AESDECRYPT_0)
    }
    #[doc = "Decrypt"]
    #[inline]
    pub fn decrypt(self) -> &'a mut W {
        self.variant(AESDECRYPTW::DECRYPT)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AESSECRET`"]
pub enum AESSECRETW {
    #[doc = "User key provided in normal way"]
    NORMAL_WAY,
    #[doc = "Secret key provided in hidden way by HW"]
    AESSECRET_1,
}
impl AESSECRETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AESSECRETW::NORMAL_WAY => false,
            AESSECRETW::AESSECRET_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AESSECRETW<'a> {
    w: &'a mut W,
}
impl<'a> _AESSECRETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AESSECRETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "User key provided in normal way"]
    #[inline]
    pub fn normal_way(self) -> &'a mut W {
        self.variant(AESSECRETW::NORMAL_WAY)
    }
    #[doc = "Secret key provided in hidden way by HW"]
    #[inline]
    pub fn aessecret_1(self) -> &'a mut W {
        self.variant(AESSECRETW::AESSECRET_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AESKEYSZ`"]
pub enum AESKEYSZW {
    #[doc = "128 bit key"]
    BITS_128,
    #[doc = "192 bit key"]
    BITS_192,
    #[doc = "256 bit key"]
    BITS_256,
}
impl AESKEYSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AESKEYSZW::BITS_128 => 0,
            AESKEYSZW::BITS_192 => 1,
            AESKEYSZW::BITS_256 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AESKEYSZW<'a> {
    w: &'a mut W,
}
impl<'a> _AESKEYSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AESKEYSZW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "128 bit key"]
    #[inline]
    pub fn bits_128(self) -> &'a mut W {
        self.variant(AESKEYSZW::BITS_128)
    }
    #[doc = "192 bit key"]
    #[inline]
    pub fn bits_192(self) -> &'a mut W {
        self.variant(AESKEYSZW::BITS_192)
    }
    #[doc = "256 bit key"]
    #[inline]
    pub fn bits_256(self) -> &'a mut W {
        self.variant(AESKEYSZW::BITS_256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AESCTRPOSW<'a> {
    w: &'a mut W,
}
impl<'a> _AESCTRPOSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STREAMLASTW<'a> {
    w: &'a mut W,
}
impl<'a> _STREAMLASTW<'a> {
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
pub struct _XSALSAW<'a> {
    w: &'a mut W,
}
impl<'a> _XSALSAW<'a> {
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
#[doc = "Values that can be written to the field `ICBSZ`"]
pub enum ICBSZW {
    #[doc = "32 bits of the IV/ctr are used (from 127:96)"]
    BITS_32,
    #[doc = "64 bits of the IV/ctr are used (from 127:64)"]
    BITS_64,
    #[doc = "96 bits of the IV/ctr are used (from 127:32)"]
    BITS_96,
    #[doc = "All 128 bits of the IV/ctr are used"]
    BIT_128,
}
impl ICBSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICBSZW::BITS_32 => 0,
            ICBSZW::BITS_64 => 1,
            ICBSZW::BITS_96 => 2,
            ICBSZW::BIT_128 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICBSZW<'a> {
    w: &'a mut W,
}
impl<'a> _ICBSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICBSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "32 bits of the IV/ctr are used (from 127:96)"]
    #[inline]
    pub fn bits_32(self) -> &'a mut W {
        self.variant(ICBSZW::BITS_32)
    }
    #[doc = "64 bits of the IV/ctr are used (from 127:64)"]
    #[inline]
    pub fn bits_64(self) -> &'a mut W {
        self.variant(ICBSZW::BITS_64)
    }
    #[doc = "96 bits of the IV/ctr are used (from 127:32)"]
    #[inline]
    pub fn bits_96(self) -> &'a mut W {
        self.variant(ICBSZW::BITS_96)
    }
    #[doc = "All 128 bits of the IV/ctr are used"]
    #[inline]
    pub fn bit_128(self) -> &'a mut W {
        self.variant(ICBSZW::BIT_128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICBSTRM`"]
pub enum ICBSTRMW {
    #[doc = "8 blocks"]
    BLOCKS_8,
    #[doc = "16 blocks"]
    BLOCKS_16,
    #[doc = "32 blocks"]
    BLOCKS_32,
    #[doc = "64 blocks"]
    BLOCKS_64,
}
impl ICBSTRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ICBSTRMW::BLOCKS_8 => 0,
            ICBSTRMW::BLOCKS_16 => 1,
            ICBSTRMW::BLOCKS_32 => 2,
            ICBSTRMW::BLOCKS_64 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICBSTRMW<'a> {
    w: &'a mut W,
}
impl<'a> _ICBSTRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICBSTRMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 blocks"]
    #[inline]
    pub fn blocks_8(self) -> &'a mut W {
        self.variant(ICBSTRMW::BLOCKS_8)
    }
    #[doc = "16 blocks"]
    #[inline]
    pub fn blocks_16(self) -> &'a mut W {
        self.variant(ICBSTRMW::BLOCKS_16)
    }
    #[doc = "32 blocks"]
    #[inline]
    pub fn blocks_32(self) -> &'a mut W {
        self.variant(ICBSTRMW::BLOCKS_32)
    }
    #[doc = "64 blocks"]
    #[inline]
    pub fn blocks_64(self) -> &'a mut W {
        self.variant(ICBSTRMW::BLOCKS_64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
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
    #[doc = "Bit 0 - If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[inline]
    pub fn msw1st_out(&self) -> MSW1ST_OUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSW1ST_OUTR { bits }
    }
    #[doc = "Bit 1 - If 1, will Swap the key input (bytes in each word)."]
    #[inline]
    pub fn swapkey(&self) -> SWAPKEYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAPKEYR { bits }
    }
    #[doc = "Bit 2 - If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[inline]
    pub fn swapdat(&self) -> SWAPDATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAPDATR { bits }
    }
    #[doc = "Bit 3 - If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[inline]
    pub fn msw1st(&self) -> MSW1STR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSW1STR { bits }
    }
    #[doc = "Bits 4:5 - AES Cipher mode to use if plain AES"]
    #[inline]
    pub fn aesmode(&self) -> AESMODER {
        AESMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
    #[inline]
    pub fn aesdecrypt(&self) -> AESDECRYPTR {
        AESDECRYPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[inline]
    pub fn aessecret(&self) -> AESSECRETR {
        AESSECRETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Sets the AES key size"]
    #[inline]
    pub fn aeskeysz(&self) -> AESKEYSZR {
        AESKEYSZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:12 - Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[inline]
    pub fn aesctrpos(&self) -> AESCTRPOSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AESCTRPOSR { bits }
    }
    #[doc = "Bit 16 - Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[inline]
    pub fn streamlast(&self) -> STREAMLASTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STREAMLASTR { bits }
    }
    #[doc = "Bit 17 - Is 1 if XSalsa 128b NONCE to be used vs. 64b"]
    #[inline]
    pub fn xsalsa(&self) -> XSALSAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XSALSAR { bits }
    }
    #[doc = "Bits 20:21 - This sets the ICB size between 32 and 128 bits, using the following rules. Note that the counter is assumed to occupy the low order bits of the IV."]
    #[inline]
    pub fn icbsz(&self) -> ICBSZR {
        ICBSZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - The size of the ICB-AES stream that can be pushed before needing to compute a new IV/ctr (counter start). This optimizes the performance of the stream of blocks after the 1st."]
    #[inline]
    pub fn icbstrm(&self) -> ICBSTRMR {
        ICBSTRMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[inline]
    pub fn msw1st_out(&mut self) -> _MSW1ST_OUTW {
        _MSW1ST_OUTW { w: self }
    }
    #[doc = "Bit 1 - If 1, will Swap the key input (bytes in each word)."]
    #[inline]
    pub fn swapkey(&mut self) -> _SWAPKEYW {
        _SWAPKEYW { w: self }
    }
    #[doc = "Bit 2 - If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[inline]
    pub fn swapdat(&mut self) -> _SWAPDATW {
        _SWAPDATW { w: self }
    }
    #[doc = "Bit 3 - If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[inline]
    pub fn msw1st(&mut self) -> _MSW1STW {
        _MSW1STW { w: self }
    }
    #[doc = "Bits 4:5 - AES Cipher mode to use if plain AES"]
    #[inline]
    pub fn aesmode(&mut self) -> _AESMODEW {
        _AESMODEW { w: self }
    }
    #[doc = "Bit 6 - AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
    #[inline]
    pub fn aesdecrypt(&mut self) -> _AESDECRYPTW {
        _AESDECRYPTW { w: self }
    }
    #[doc = "Bit 7 - Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[inline]
    pub fn aessecret(&mut self) -> _AESSECRETW {
        _AESSECRETW { w: self }
    }
    #[doc = "Bits 8:9 - Sets the AES key size"]
    #[inline]
    pub fn aeskeysz(&mut self) -> _AESKEYSZW {
        _AESKEYSZW { w: self }
    }
    #[doc = "Bits 10:12 - Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[inline]
    pub fn aesctrpos(&mut self) -> _AESCTRPOSW {
        _AESCTRPOSW { w: self }
    }
    #[doc = "Bit 16 - Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[inline]
    pub fn streamlast(&mut self) -> _STREAMLASTW {
        _STREAMLASTW { w: self }
    }
    #[doc = "Bit 17 - Is 1 if XSalsa 128b NONCE to be used vs. 64b"]
    #[inline]
    pub fn xsalsa(&mut self) -> _XSALSAW {
        _XSALSAW { w: self }
    }
    #[doc = "Bits 20:21 - This sets the ICB size between 32 and 128 bits, using the following rules. Note that the counter is assumed to occupy the low order bits of the IV."]
    #[inline]
    pub fn icbsz(&mut self) -> _ICBSZW {
        _ICBSZW { w: self }
    }
    #[doc = "Bits 22:23 - The size of the ICB-AES stream that can be pushed before needing to compute a new IV/ctr (counter start). This optimizes the performance of the stream of blocks after the 1st."]
    #[inline]
    pub fn icbstrm(&mut self) -> _ICBSTRMW {
        _ICBSTRMW { w: self }
    }
}
