#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DIEID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct REV_IDR {
    bits: u8,
}
impl REV_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MCO_NUM_IN_DIE_IDR {
    bits: u32,
}
impl MCO_NUM_IN_DIE_IDR {
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
    #[doc = "Bits 0:3 - Chip Metal Revision ID."]
    #[inline]
    pub fn rev_id(&self) -> REV_IDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REV_IDR { bits }
    }
    #[doc = "Bits 4:23 - Chip Number."]
    #[inline]
    pub fn mco_num_in_die_id(&self) -> MCO_NUM_IN_DIE_IDR {
        let bits = {
            const MASK: u32 = 1048575;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        MCO_NUM_IN_DIE_IDR { bits }
    }
}
