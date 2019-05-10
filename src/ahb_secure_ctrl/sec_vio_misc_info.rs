#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SEC_VIO_MISC_INFO {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SEC_VIO_INFO_WRITER {
    bits: bool,
}
impl SEC_VIO_INFO_WRITER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SEC_VIO_INFO_DATA_ACCESSR {
    bits: bool,
}
impl SEC_VIO_INFO_DATA_ACCESSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SEC_VIO_INFO_MASTER_SEC_LEVELR {
    bits: u8,
}
impl SEC_VIO_INFO_MASTER_SEC_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEC_VIO_INFO_MASTERR {
    bits: u8,
}
impl SEC_VIO_INFO_MASTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - security violation access read/write indicator, 0: read, 1: write"]
    #[inline]
    pub fn sec_vio_info_write(&self) -> SEC_VIO_INFO_WRITER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEC_VIO_INFO_WRITER { bits }
    }
    #[doc = "Bit 1 - security violation access data/code indicator, 0: code, 1"]
    #[inline]
    pub fn sec_vio_info_data_access(&self) -> SEC_VIO_INFO_DATA_ACCESSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SEC_VIO_INFO_DATA_ACCESSR { bits }
    }
    #[doc = "Bits 4:7 - bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level"]
    #[inline]
    pub fn sec_vio_info_master_sec_level(&self) -> SEC_VIO_INFO_MASTER_SEC_LEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEC_VIO_INFO_MASTER_SEC_LEVELR { bits }
    }
    #[doc = "Bits 8:11 - security violation master number"]
    #[inline]
    pub fn sec_vio_info_master(&self) -> SEC_VIO_INFO_MASTERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEC_VIO_INFO_MASTERR { bits }
    }
}
