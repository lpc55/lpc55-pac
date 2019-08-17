#[doc = "Reader of register ENCRYPTED_NUMBER"]
pub type R = crate::R<u32, super::ENCRYPTED_NUMBER>;
#[doc = "Reader of field `ENCRYPTED_NUMBER`"]
pub type ENCRYPTED_NUMBER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is pre-computed."]
    #[inline(always)]
    pub fn encrypted_number(&self) -> ENCRYPTED_NUMBER_R {
        ENCRYPTED_NUMBER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
