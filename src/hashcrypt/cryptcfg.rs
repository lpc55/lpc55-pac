#[doc = "Reader of register CRYPTCFG"]
pub type R = crate::R<u32, super::CRYPTCFG>;
#[doc = "Writer for register CRYPTCFG"]
pub type W = crate::W<u32, super::CRYPTCFG>;
#[doc = "Register CRYPTCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYPTCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSW1ST_OUT`"]
pub type MSW1ST_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSW1ST_OUT`"]
pub struct MSW1ST_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> MSW1ST_OUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SWAPKEY`"]
pub type SWAPKEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAPKEY`"]
pub struct SWAPKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAPKEY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SWAPDAT`"]
pub type SWAPDAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAPDAT`"]
pub struct SWAPDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAPDAT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MSW1ST`"]
pub type MSW1ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSW1ST`"]
pub struct MSW1ST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSW1ST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "AES Cipher mode to use if plain AES\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESMODE_A {
    #[doc = "0: ECB - used as is"]
    ECB = 0,
    #[doc = "1: CBC mode (see details on IV/nonce)"]
    CBC = 1,
    #[doc = "2: CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    CTR = 2,
}
impl From<AESMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: AESMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESMODE`"]
pub type AESMODE_R = crate::R<u8, AESMODE_A>;
impl AESMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AESMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESMODE_A::ECB),
            1 => Val(AESMODE_A::CBC),
            2 => Val(AESMODE_A::CTR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == AESMODE_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == AESMODE_A::CBC
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == AESMODE_A::CTR
    }
}
#[doc = "Write proxy for field `AESMODE`"]
pub struct AESMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AESMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ECB - used as is"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(AESMODE_A::ECB)
    }
    #[doc = "CBC mode (see details on IV/nonce)"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(AESMODE_A::CBC)
    }
    #[doc = "CTR mode (see details on IV/nonce). See also AESCTRPOS."]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut W {
        self.variant(AESMODE_A::CTR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESDECRYPT_A {
    #[doc = "0: Encrypt"]
    ENCRYPT = 0,
    #[doc = "1: Decrypt"]
    DECRYPT = 1,
}
impl From<AESDECRYPT_A> for bool {
    #[inline(always)]
    fn from(variant: AESDECRYPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESDECRYPT`"]
pub type AESDECRYPT_R = crate::R<bool, AESDECRYPT_A>;
impl AESDECRYPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESDECRYPT_A {
        match self.bits {
            false => AESDECRYPT_A::ENCRYPT,
            true => AESDECRYPT_A::DECRYPT,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPT`"]
    #[inline(always)]
    pub fn is_encrypt(&self) -> bool {
        *self == AESDECRYPT_A::ENCRYPT
    }
    #[doc = "Checks if the value of the field is `DECRYPT`"]
    #[inline(always)]
    pub fn is_decrypt(&self) -> bool {
        *self == AESDECRYPT_A::DECRYPT
    }
}
#[doc = "Write proxy for field `AESDECRYPT`"]
pub struct AESDECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDECRYPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESDECRYPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Encrypt"]
    #[inline(always)]
    pub fn encrypt(self) -> &'a mut W {
        self.variant(AESDECRYPT_A::ENCRYPT)
    }
    #[doc = "Decrypt"]
    #[inline(always)]
    pub fn decrypt(self) -> &'a mut W {
        self.variant(AESDECRYPT_A::DECRYPT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESSECRET_A {
    #[doc = "0: User key provided in normal way"]
    NORMAL_WAY = 0,
    #[doc = "1: Secret key provided in hidden way by HW"]
    HIDDEN_WAY = 1,
}
impl From<AESSECRET_A> for bool {
    #[inline(always)]
    fn from(variant: AESSECRET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AESSECRET`"]
pub type AESSECRET_R = crate::R<bool, AESSECRET_A>;
impl AESSECRET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESSECRET_A {
        match self.bits {
            false => AESSECRET_A::NORMAL_WAY,
            true => AESSECRET_A::HIDDEN_WAY,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_WAY`"]
    #[inline(always)]
    pub fn is_normal_way(&self) -> bool {
        *self == AESSECRET_A::NORMAL_WAY
    }
    #[doc = "Checks if the value of the field is `HIDDEN_WAY`"]
    #[inline(always)]
    pub fn is_hidden_way(&self) -> bool {
        *self == AESSECRET_A::HIDDEN_WAY
    }
}
#[doc = "Write proxy for field `AESSECRET`"]
pub struct AESSECRET_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSECRET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESSECRET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "User key provided in normal way"]
    #[inline(always)]
    pub fn normal_way(self) -> &'a mut W {
        self.variant(AESSECRET_A::NORMAL_WAY)
    }
    #[doc = "Secret key provided in hidden way by HW"]
    #[inline(always)]
    pub fn hidden_way(self) -> &'a mut W {
        self.variant(AESSECRET_A::HIDDEN_WAY)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Sets the AES key size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESKEYSZ_A {
    #[doc = "0: 128 bit key"]
    BITS_128 = 0,
    #[doc = "1: 192 bit key"]
    BITS_192 = 1,
    #[doc = "2: 256 bit key"]
    BITS_256 = 2,
}
impl From<AESKEYSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: AESKEYSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESKEYSZ`"]
pub type AESKEYSZ_R = crate::R<u8, AESKEYSZ_A>;
impl AESKEYSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AESKEYSZ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESKEYSZ_A::BITS_128),
            1 => Val(AESKEYSZ_A::BITS_192),
            2 => Val(AESKEYSZ_A::BITS_256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BITS_128`"]
    #[inline(always)]
    pub fn is_bits_128(&self) -> bool {
        *self == AESKEYSZ_A::BITS_128
    }
    #[doc = "Checks if the value of the field is `BITS_192`"]
    #[inline(always)]
    pub fn is_bits_192(&self) -> bool {
        *self == AESKEYSZ_A::BITS_192
    }
    #[doc = "Checks if the value of the field is `BITS_256`"]
    #[inline(always)]
    pub fn is_bits_256(&self) -> bool {
        *self == AESKEYSZ_A::BITS_256
    }
}
#[doc = "Write proxy for field `AESKEYSZ`"]
pub struct AESKEYSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEYSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKEYSZ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "128 bit key"]
    #[inline(always)]
    pub fn bits_128(self) -> &'a mut W {
        self.variant(AESKEYSZ_A::BITS_128)
    }
    #[doc = "192 bit key"]
    #[inline(always)]
    pub fn bits_192(self) -> &'a mut W {
        self.variant(AESKEYSZ_A::BITS_192)
    }
    #[doc = "256 bit key"]
    #[inline(always)]
    pub fn bits_256(self) -> &'a mut W {
        self.variant(AESKEYSZ_A::BITS_256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `AESCTRPOS`"]
pub type AESCTRPOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AESCTRPOS`"]
pub struct AESCTRPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> AESCTRPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `STREAMLAST`"]
pub type STREAMLAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STREAMLAST`"]
pub struct STREAMLAST_W<'a> {
    w: &'a mut W,
}
impl<'a> STREAMLAST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "This sets the ICB size between 32 and 128 bits, using the following rules. Note that the counter is assumed to occupy the low order bits of the IV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ICBSZ_A {
    #[doc = "0: 32 bits of the IV/ctr are used (from 127:96)"]
    BITS_32 = 0,
    #[doc = "1: 64 bits of the IV/ctr are used (from 127:64)"]
    BITS_64 = 1,
    #[doc = "2: 96 bits of the IV/ctr are used (from 127:32)"]
    BITS_96 = 2,
    #[doc = "3: All 128 bits of the IV/ctr are used"]
    BIT_128 = 3,
}
impl From<ICBSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: ICBSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ICBSZ`"]
pub type ICBSZ_R = crate::R<u8, ICBSZ_A>;
impl ICBSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBSZ_A {
        match self.bits {
            0 => ICBSZ_A::BITS_32,
            1 => ICBSZ_A::BITS_64,
            2 => ICBSZ_A::BITS_96,
            3 => ICBSZ_A::BIT_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BITS_32`"]
    #[inline(always)]
    pub fn is_bits_32(&self) -> bool {
        *self == ICBSZ_A::BITS_32
    }
    #[doc = "Checks if the value of the field is `BITS_64`"]
    #[inline(always)]
    pub fn is_bits_64(&self) -> bool {
        *self == ICBSZ_A::BITS_64
    }
    #[doc = "Checks if the value of the field is `BITS_96`"]
    #[inline(always)]
    pub fn is_bits_96(&self) -> bool {
        *self == ICBSZ_A::BITS_96
    }
    #[doc = "Checks if the value of the field is `BIT_128`"]
    #[inline(always)]
    pub fn is_bit_128(&self) -> bool {
        *self == ICBSZ_A::BIT_128
    }
}
#[doc = "Write proxy for field `ICBSZ`"]
pub struct ICBSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ICBSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICBSZ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32 bits of the IV/ctr are used (from 127:96)"]
    #[inline(always)]
    pub fn bits_32(self) -> &'a mut W {
        self.variant(ICBSZ_A::BITS_32)
    }
    #[doc = "64 bits of the IV/ctr are used (from 127:64)"]
    #[inline(always)]
    pub fn bits_64(self) -> &'a mut W {
        self.variant(ICBSZ_A::BITS_64)
    }
    #[doc = "96 bits of the IV/ctr are used (from 127:32)"]
    #[inline(always)]
    pub fn bits_96(self) -> &'a mut W {
        self.variant(ICBSZ_A::BITS_96)
    }
    #[doc = "All 128 bits of the IV/ctr are used"]
    #[inline(always)]
    pub fn bit_128(self) -> &'a mut W {
        self.variant(ICBSZ_A::BIT_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "The size of the ICB-AES stream that can be pushed before needing to compute a new IV/ctr (counter start). This optimizes the performance of the stream of blocks after the 1st.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ICBSTRM_A {
    #[doc = "0: 8 blocks"]
    BLOCKS_8 = 0,
    #[doc = "1: 16 blocks"]
    BLOCKS_16 = 1,
    #[doc = "2: 32 blocks"]
    BLOCKS_32 = 2,
    #[doc = "3: 64 blocks"]
    BLOCKS_64 = 3,
}
impl From<ICBSTRM_A> for u8 {
    #[inline(always)]
    fn from(variant: ICBSTRM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ICBSTRM`"]
pub type ICBSTRM_R = crate::R<u8, ICBSTRM_A>;
impl ICBSTRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICBSTRM_A {
        match self.bits {
            0 => ICBSTRM_A::BLOCKS_8,
            1 => ICBSTRM_A::BLOCKS_16,
            2 => ICBSTRM_A::BLOCKS_32,
            3 => ICBSTRM_A::BLOCKS_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKS_8`"]
    #[inline(always)]
    pub fn is_blocks_8(&self) -> bool {
        *self == ICBSTRM_A::BLOCKS_8
    }
    #[doc = "Checks if the value of the field is `BLOCKS_16`"]
    #[inline(always)]
    pub fn is_blocks_16(&self) -> bool {
        *self == ICBSTRM_A::BLOCKS_16
    }
    #[doc = "Checks if the value of the field is `BLOCKS_32`"]
    #[inline(always)]
    pub fn is_blocks_32(&self) -> bool {
        *self == ICBSTRM_A::BLOCKS_32
    }
    #[doc = "Checks if the value of the field is `BLOCKS_64`"]
    #[inline(always)]
    pub fn is_blocks_64(&self) -> bool {
        *self == ICBSTRM_A::BLOCKS_64
    }
}
#[doc = "Write proxy for field `ICBSTRM`"]
pub struct ICBSTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICBSTRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICBSTRM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8 blocks"]
    #[inline(always)]
    pub fn blocks_8(self) -> &'a mut W {
        self.variant(ICBSTRM_A::BLOCKS_8)
    }
    #[doc = "16 blocks"]
    #[inline(always)]
    pub fn blocks_16(self) -> &'a mut W {
        self.variant(ICBSTRM_A::BLOCKS_16)
    }
    #[doc = "32 blocks"]
    #[inline(always)]
    pub fn blocks_32(self) -> &'a mut W {
        self.variant(ICBSTRM_A::BLOCKS_32)
    }
    #[doc = "64 blocks"]
    #[inline(always)]
    pub fn blocks_64(self) -> &'a mut W {
        self.variant(ICBSTRM_A::BLOCKS_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[inline(always)]
    pub fn msw1st_out(&self) -> MSW1ST_OUT_R {
        MSW1ST_OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If 1, will Swap the key input (bytes in each word)."]
    #[inline(always)]
    pub fn swapkey(&self) -> SWAPKEY_R {
        SWAPKEY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[inline(always)]
    pub fn swapdat(&self) -> SWAPDAT_R {
        SWAPDAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[inline(always)]
    pub fn msw1st(&self) -> MSW1ST_R {
        MSW1ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - AES Cipher mode to use if plain AES"]
    #[inline(always)]
    pub fn aesmode(&self) -> AESMODE_R {
        AESMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
    #[inline(always)]
    pub fn aesdecrypt(&self) -> AESDECRYPT_R {
        AESDECRYPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[inline(always)]
    pub fn aessecret(&self) -> AESSECRET_R {
        AESSECRET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Sets the AES key size"]
    #[inline(always)]
    pub fn aeskeysz(&self) -> AESKEYSZ_R {
        AESKEYSZ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12 - Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[inline(always)]
    pub fn aesctrpos(&self) -> AESCTRPOS_R {
        AESCTRPOS_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[inline(always)]
    pub fn streamlast(&self) -> STREAMLAST_R {
        STREAMLAST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - This sets the ICB size between 32 and 128 bits, using the following rules. Note that the counter is assumed to occupy the low order bits of the IV."]
    #[inline(always)]
    pub fn icbsz(&self) -> ICBSZ_R {
        ICBSZ_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - The size of the ICB-AES stream that can be pushed before needing to compute a new IV/ctr (counter start). This optimizes the performance of the stream of blocks after the 1st."]
    #[inline(always)]
    pub fn icbstrm(&self) -> ICBSTRM_R {
        ICBSTRM_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - If 1, OUTDATA0 will be read Most significant word 1st for AES. Else it will be read in normal little endian - Least significant word 1st. Note: only if allowed by configuration."]
    #[inline(always)]
    pub fn msw1st_out(&mut self) -> MSW1ST_OUT_W {
        MSW1ST_OUT_W { w: self }
    }
    #[doc = "Bit 1 - If 1, will Swap the key input (bytes in each word)."]
    #[inline(always)]
    pub fn swapkey(&mut self) -> SWAPKEY_W {
        SWAPKEY_W { w: self }
    }
    #[doc = "Bit 2 - If 1, will SWAP the data and IV inputs (bytes in each word)."]
    #[inline(always)]
    pub fn swapdat(&mut self) -> SWAPDAT_W {
        SWAPDAT_W { w: self }
    }
    #[doc = "Bit 3 - If 1, load of key, IV, and data is MSW 1st for AES. Else, the words are little endian. Note: only if allowed by configuration."]
    #[inline(always)]
    pub fn msw1st(&mut self) -> MSW1ST_W {
        MSW1ST_W { w: self }
    }
    #[doc = "Bits 4:5 - AES Cipher mode to use if plain AES"]
    #[inline(always)]
    pub fn aesmode(&mut self) -> AESMODE_W {
        AESMODE_W { w: self }
    }
    #[doc = "Bit 6 - AES ECB direction. Only encryption used if CTR mode or manual modes such as CFB"]
    #[inline(always)]
    pub fn aesdecrypt(&mut self) -> AESDECRYPT_W {
        AESDECRYPT_W { w: self }
    }
    #[doc = "Bit 7 - Selects the Hidden Secret key vs. User key, if provided. If security levels are used, only the highest level is permitted to select this."]
    #[inline(always)]
    pub fn aessecret(&mut self) -> AESSECRET_W {
        AESSECRET_W { w: self }
    }
    #[doc = "Bits 8:9 - Sets the AES key size"]
    #[inline(always)]
    pub fn aeskeysz(&mut self) -> AESKEYSZ_W {
        AESKEYSZ_W { w: self }
    }
    #[doc = "Bits 10:12 - Halfword position of 16b counter in IV if AESMODE is CTR (position is fixed for Salsa and ChaCha). Only supports 16b counter, so application must control any additional bytes if using more. The 16-bit counter is read from the IV and incremented by 1 each time. Any other use CTR should use ECB directly and do its own XOR and so on."]
    #[inline(always)]
    pub fn aesctrpos(&mut self) -> AESCTRPOS_W {
        AESCTRPOS_W { w: self }
    }
    #[doc = "Bit 16 - Is 1 if last stream block. If not 1, then the engine will compute the next \"hash\"."]
    #[inline(always)]
    pub fn streamlast(&mut self) -> STREAMLAST_W {
        STREAMLAST_W { w: self }
    }
    #[doc = "Bits 20:21 - This sets the ICB size between 32 and 128 bits, using the following rules. Note that the counter is assumed to occupy the low order bits of the IV."]
    #[inline(always)]
    pub fn icbsz(&mut self) -> ICBSZ_W {
        ICBSZ_W { w: self }
    }
    #[doc = "Bits 22:23 - The size of the ICB-AES stream that can be pushed before needing to compute a new IV/ctr (counter start). This optimizes the performance of the stream of blocks after the 1st."]
    #[inline(always)]
    pub fn icbstrm(&mut self) -> ICBSTRM_W {
        ICBSTRM_W { w: self }
    }
}
