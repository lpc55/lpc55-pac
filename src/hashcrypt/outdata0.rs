#[doc = "Reader of register OUTDATA0[%s]"]
pub type R = crate::R<u32, super::OUTDATA0>;
#[doc = "Reader of field `DIGEST_OUTPUT`"]
pub type DIGEST_OUTPUT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
    #[inline(always)]
    pub fn digest_output(&self) -> DIGEST_OUTPUT_R {
        DIGEST_OUTPUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
