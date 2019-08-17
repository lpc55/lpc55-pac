#[doc = "Reader of register OUTDATA1[%s]"]
pub type R = crate::R<u32, super::OUTDATA1>;
#[doc = "Reader of field `OUTPUT`"]
pub type OUTPUT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - One word of the 2nd half of the output when used."]
    #[inline(always)]
    pub fn output(&self) -> OUTPUT_R {
        OUTPUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
