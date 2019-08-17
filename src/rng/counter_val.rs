#[doc = "Reader of register COUNTER_VAL"]
pub type R = crate::R<u32, super::COUNTER_VAL>;
#[doc = "Writer for register COUNTER_VAL"]
pub type W = crate::W<u32, super::COUNTER_VAL>;
#[doc = "Register COUNTER_VAL `reset()`'s with value 0"]
impl crate::ResetValue for super::COUNTER_VAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_RATIO`"]
pub type CLK_RATIO_R = crate::R<u8, u8>;
#[doc = "Reader of field `REFRESH_CNT`"]
pub type REFRESH_CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Gives the ratio between the internal clocks frequencies and the register clock frequency for evaluation and certification purposes."]
    #[inline(always)]
    pub fn clk_ratio(&self) -> CLK_RATIO_R {
        CLK_RATIO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Incremented (till max possible value) each time COUNTER was updated since last reading to any *_NUMBER."]
    #[inline(always)]
    pub fn refresh_cnt(&self) -> REFRESH_CNT_R {
        REFRESH_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {}
