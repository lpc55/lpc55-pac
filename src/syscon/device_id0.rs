#[doc = "Reader of register DEVICE_ID0"]
pub type R = crate::R<u32, super::DEVICE_ID0>;
#[doc = "Reader of field `ROM_REV_MINOR`"]
pub type ROM_REV_MINOR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 20:23 - ROM revision."]
    #[inline(always)]
    pub fn rom_rev_minor(&self) -> ROM_REV_MINOR_R {
        ROM_REV_MINOR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
