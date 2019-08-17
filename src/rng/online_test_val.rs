#[doc = "Reader of register ONLINE_TEST_VAL"]
pub type R = crate::R<u32, super::ONLINE_TEST_VAL>;
#[doc = "Writer for register ONLINE_TEST_VAL"]
pub type W = crate::W<u32, super::ONLINE_TEST_VAL>;
#[doc = "Register ONLINE_TEST_VAL `reset()`'s with value 0"]
impl crate::ResetValue for super::ONLINE_TEST_VAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LIVE_CHI_SQUARED`"]
pub type LIVE_CHI_SQUARED_R = crate::R<u8, u8>;
#[doc = "Reader of field `MIN_CHI_SQUARED`"]
pub type MIN_CHI_SQUARED_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAX_CHI_SQUARED`"]
pub type MAX_CHI_SQUARED_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - This value is updated as described in field 'activate'."]
    #[inline(always)]
    pub fn live_chi_squared(&self) -> LIVE_CHI_SQUARED_R {
        LIVE_CHI_SQUARED_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This field is reset when 'activate'==0."]
    #[inline(always)]
    pub fn min_chi_squared(&self) -> MIN_CHI_SQUARED_R {
        MIN_CHI_SQUARED_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field is reset when 'activate'==0."]
    #[inline(always)]
    pub fn max_chi_squared(&self) -> MAX_CHI_SQUARED_R {
        MAX_CHI_SQUARED_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {}
