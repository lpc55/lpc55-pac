#[doc = "Reader of register VERSION"]
pub type R = crate::R<u32, super::VERSION>;
#[doc = "Reader of field `KEYOUT`"]
pub type KEYOUT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Version of the PUF module."]
    #[inline(always)]
    pub fn keyout(&self) -> KEYOUT_R {
        KEYOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
