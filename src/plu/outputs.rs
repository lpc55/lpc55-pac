#[doc = "Reader of register OUTPUTS"]
pub type R = crate::R<u32, super::OUTPUTS>;
#[doc = "Writer for register OUTPUTS"]
pub type W = crate::W<u32, super::OUTPUTS>;
#[doc = "Register OUTPUTS `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTPUTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTPUT_STATE`"]
pub type OUTPUT_STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Provides the current state of the 8 designated PLU Outputs.."]
    #[inline(always)]
    pub fn output_state(&self) -> OUTPUT_STATE_R {
        OUTPUT_STATE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
