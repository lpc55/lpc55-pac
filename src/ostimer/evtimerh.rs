#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EVTIMERH {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct EVTIMER_COUNT_VALUER {
    bits: u32,
}
impl EVTIMER_COUNT_VALUER {
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
    #[doc = "Bits 0:31 - A read reflects the current value of the upper 32 bits of the EVTIMER. Note there is physically only one EVTimer, readable from all domains."]
    #[inline]
    pub fn evtimer_count_value(&self) -> EVTIMER_COUNT_VALUER {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        EVTIMER_COUNT_VALUER { bits }
    }
}
