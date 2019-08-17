#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUAL`"]
pub type DUAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHB`"]
pub type AHB_R = crate::R<bool, bool>;
#[doc = "Reader of field `SHA512`"]
pub type SHA512_R = crate::R<bool, bool>;
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, bool>;
#[doc = "Reader of field `AESKEY`"]
pub type AESKEY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SECRET`"]
pub type SECRET_R = crate::R<bool, bool>;
#[doc = "Reader of field `SALSA`"]
pub type SALSA_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHACHA`"]
pub type CHACHA_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICB`"]
pub type ICB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 if DMA is connected"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 if AHB Master is enabled"]
    #[inline(always)]
    pub fn ahb(&self) -> AHB_R {
        AHB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1 if SHA2-512 included"]
    #[inline(always)]
    pub fn sha512(&self) -> SHA512_R {
        SHA512_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 if AES 128 included"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1 if AES 192 and 256 also included"]
    #[inline(always)]
    pub fn aeskey(&self) -> AESKEY_R {
        AESKEY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1 if AES Secret key available"]
    #[inline(always)]
    pub fn secret(&self) -> SECRET_R {
        SECRET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1 if Salsa included"]
    #[inline(always)]
    pub fn salsa(&self) -> SALSA_R {
        SALSA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 1 if ChaCha included"]
    #[inline(always)]
    pub fn chacha(&self) -> CHACHA_R {
        CHACHA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 1 if ICB over AES included"]
    #[inline(always)]
    pub fn icb(&self) -> ICB_R {
        ICB_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {}
