#[doc = "Reader of register EPTOGGLE"]
pub type R = crate::R<u32, super::EPTOGGLE>;
#[doc = "Reader of field `TOGGLE`"]
pub type TOGGLE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn toggle(&self) -> TOGGLE_R {
        TOGGLE_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
