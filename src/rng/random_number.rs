#[doc = "Reader of register RANDOM_NUMBER"]
pub type R = crate::R<u32, super::RANDOM_NUMBER>;
#[doc = "Reader of field `RANDOM_NUMBER`"]
pub type RANDOM_NUMBER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is computed on demand, at each time it is read."]
    #[inline(always)]
    pub fn random_number(&self) -> RANDOM_NUMBER_R {
        RANDOM_NUMBER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
