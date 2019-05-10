#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RANDOM_NUMBER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RANDOM_NUMBERR {
    bits: u32,
}
impl RANDOM_NUMBERR {
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
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is computed on demand, at each time it is read."]
    #[inline]
    pub fn random_number(&self) -> RANDOM_NUMBERR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RANDOM_NUMBERR { bits }
    }
}
