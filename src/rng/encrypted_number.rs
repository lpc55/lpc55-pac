#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ENCRYPTED_NUMBER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ENCRYPTED_NUMBERR {
    bits: u32,
}
impl ENCRYPTED_NUMBERR {
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
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is pre-computed."]
    #[inline]
    pub fn encrypted_number(&self) -> ENCRYPTED_NUMBERR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        ENCRYPTED_NUMBERR { bits }
    }
}
