#[doc = "Reader of register DIGEST0[%s]"]
pub type R = crate::R<u32, super::DIGEST0>;
#[doc = "Reader of field `DIGEST`"]
pub type DIGEST_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
