#[doc = "Reader of register CODEOUTPUT"]
pub type R = crate::R<u32, super::CODEOUTPUT>;
#[doc = "Reader of field `CODEOUT`"]
pub type CODEOUT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AC/KC output data"]
    #[inline(always)]
    pub fn codeout(&self) -> CODEOUT_R {
        CODEOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
