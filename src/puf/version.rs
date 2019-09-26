#[doc = "Reader of register VERSION"]
pub type R = crate::R<u32, super::VERSION>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Version of the PUF module."]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
