#[doc = "Reader of register HCPERIODCURRENTED"]
pub type R = crate::R<u32, super::HCPERIODCURRENTED>;
#[doc = "Writer for register HCPERIODCURRENTED"]
pub type W = crate::W<u32, super::HCPERIODCURRENTED>;
#[doc = "Register HCPERIODCURRENTED `reset()`'s with value 0"]
impl crate::ResetValue for super::HCPERIODCURRENTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCED`"]
pub type PCED_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 4:31 - The content of this register is updated by HC after a periodic ED is processed."]
    #[inline(always)]
    pub fn pced(&self) -> PCED_R {
        PCED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {}
