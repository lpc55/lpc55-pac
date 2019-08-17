#[doc = "Reader of register KEYOUTINDEX"]
pub type R = crate::R<u32, super::KEYOUTINDEX>;
#[doc = "Writer for register KEYOUTINDEX"]
pub type W = crate::W<u32, super::KEYOUTINDEX>;
#[doc = "Register KEYOUTINDEX `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYOUTINDEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEYOUTIDX`"]
pub type KEYOUTIDX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Key index for the key that is currently output via the Key Output register"]
    #[inline(always)]
    pub fn keyoutidx(&self) -> KEYOUTIDX_R {
        KEYOUTIDX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {}
