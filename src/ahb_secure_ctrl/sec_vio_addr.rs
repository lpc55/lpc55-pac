#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SEC_VIO_ADDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SEC_VIO_ADDRR {
    bits: u32,
}
impl SEC_VIO_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - security violation address for AHB layer"]
    #[inline]
    pub fn sec_vio_addr(&self) -> SEC_VIO_ADDRR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        SEC_VIO_ADDRR { bits }
    }
}
