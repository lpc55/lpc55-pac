#[doc = "Reader of register SHIFT_STATUS"]
pub type R = crate::R<u32, super::SHIFT_STATUS>;
#[doc = "Writer for register SHIFT_STATUS"]
pub type W = crate::W<u32, super::SHIFT_STATUS>;
#[doc = "Register SHIFT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFT_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEY0`"]
pub type KEY0_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY1`"]
pub type KEY1_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY2`"]
pub type KEY2_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY3`"]
pub type KEY3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Index counter from key 0 shift register"]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Index counter from key 1 shift register"]
    #[inline(always)]
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Index counter from key 2 shift register"]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Index counter from key 3 shift register"]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {}
