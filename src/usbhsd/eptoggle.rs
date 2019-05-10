#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EPTOGGLE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TOGGLER {
    bits: u32,
}
impl TOGGLER {
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
    #[doc = "Bits 0:29 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline]
    pub fn toggle(&self) -> TOGGLER {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TOGGLER { bits }
    }
}
