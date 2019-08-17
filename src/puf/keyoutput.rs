#[doc = "Reader of register KEYOUTPUT"]
pub type R = crate::R<u32, super::KEYOUTPUT>;
#[doc = "Reader of field `KEYOUT`"]
pub type KEYOUT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key output data"]
    #[inline(always)]
    pub fn keyout(&self) -> KEYOUT_R {
        KEYOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
