#[doc = "Reader of register sec_vio_addr[%s]"]
pub type R = crate::R<u32, super::SEC_VIO_ADDR>;
#[doc = "Reader of field `SEC_VIO_ADDR`"]
pub type SEC_VIO_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - security violation address for AHB port"]
    #[inline(always)]
    pub fn sec_vio_addr(&self) -> SEC_VIO_ADDR_R {
        SEC_VIO_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
