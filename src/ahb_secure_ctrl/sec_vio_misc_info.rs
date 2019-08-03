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
#[doc = "Possible values of the field `SEC_VIO_INFO_WRITE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_INFO_WRITER {
    #[doc = "Read access."]
    READ,
    #[doc = "Write access."]
    WRITE,
}
impl SEC_VIO_INFO_WRITER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SEC_VIO_INFO_WRITER::READ => false,
            SEC_VIO_INFO_WRITER::WRITE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_VIO_INFO_WRITER {
        match value {
            false => SEC_VIO_INFO_WRITER::READ,
            true => SEC_VIO_INFO_WRITER::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == SEC_VIO_INFO_WRITER::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == SEC_VIO_INFO_WRITER::WRITE
    }
}
#[doc = "Possible values of the field `SEC_VIO_INFO_DATA_ACCESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_INFO_DATA_ACCESSR {
    #[doc = "Code access."]
    CODE,
    #[doc = "Data access."]
    DATA,
}
impl SEC_VIO_INFO_DATA_ACCESSR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SEC_VIO_INFO_DATA_ACCESSR::CODE => false,
            SEC_VIO_INFO_DATA_ACCESSR::DATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_VIO_INFO_DATA_ACCESSR {
        match value {
            false => SEC_VIO_INFO_DATA_ACCESSR::CODE,
            true => SEC_VIO_INFO_DATA_ACCESSR::DATA,
        }
    }
    #[doc = "Checks if the value of the field is `CODE`"]
    #[inline]
    pub fn is_code(&self) -> bool {
        *self == SEC_VIO_INFO_DATA_ACCESSR::CODE
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline]
    pub fn is_data(&self) -> bool {
        *self == SEC_VIO_INFO_DATA_ACCESSR::DATA
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
#[doc = "Possible values of the field `SEC_VIO_INFO_MASTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_INFO_MASTERR {
    #[doc = "CPU0 Code."]
    VALUE_0,
    #[doc = "CPU0 System."]
    VALUE_1,
    #[doc = "CPU1 Data."]
    VALUE_2,
    #[doc = "CPU1 System."]
    VALUE_3,
    #[doc = "USB-HS Device."]
    VALUE_4,
    #[doc = "SDMA0."]
    VALUE_5,
    #[doc = "SDIO."]
    VALUE_8,
    #[doc = "PowerQuad."]
    VALUE_9,
    #[doc = "HASH."]
    VALUE_10,
    #[doc = "USB-FS Host."]
    VALUE_11,
    #[doc = "SDMA1."]
    VALUE_12,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEC_VIO_INFO_MASTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEC_VIO_INFO_MASTERR::VALUE_0 => 0,
            SEC_VIO_INFO_MASTERR::VALUE_1 => 1,
            SEC_VIO_INFO_MASTERR::VALUE_2 => 2,
            SEC_VIO_INFO_MASTERR::VALUE_3 => 3,
            SEC_VIO_INFO_MASTERR::VALUE_4 => 4,
            SEC_VIO_INFO_MASTERR::VALUE_5 => 5,
            SEC_VIO_INFO_MASTERR::VALUE_8 => 8,
            SEC_VIO_INFO_MASTERR::VALUE_9 => 9,
            SEC_VIO_INFO_MASTERR::VALUE_10 => 10,
            SEC_VIO_INFO_MASTERR::VALUE_11 => 11,
            SEC_VIO_INFO_MASTERR::VALUE_12 => 12,
            SEC_VIO_INFO_MASTERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEC_VIO_INFO_MASTERR {
        match value {
            0 => SEC_VIO_INFO_MASTERR::VALUE_0,
            1 => SEC_VIO_INFO_MASTERR::VALUE_1,
            2 => SEC_VIO_INFO_MASTERR::VALUE_2,
            3 => SEC_VIO_INFO_MASTERR::VALUE_3,
            4 => SEC_VIO_INFO_MASTERR::VALUE_4,
            5 => SEC_VIO_INFO_MASTERR::VALUE_5,
            8 => SEC_VIO_INFO_MASTERR::VALUE_8,
            9 => SEC_VIO_INFO_MASTERR::VALUE_9,
            10 => SEC_VIO_INFO_MASTERR::VALUE_10,
            11 => SEC_VIO_INFO_MASTERR::VALUE_11,
            12 => SEC_VIO_INFO_MASTERR::VALUE_12,
            i => SEC_VIO_INFO_MASTERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline]
    pub fn is_value_2(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline]
    pub fn is_value_3(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_3
    }
    #[doc = "Checks if the value of the field is `VALUE_4`"]
    #[inline]
    pub fn is_value_4(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_4
    }
    #[doc = "Checks if the value of the field is `VALUE_5`"]
    #[inline]
    pub fn is_value_5(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_5
    }
    #[doc = "Checks if the value of the field is `VALUE_8`"]
    #[inline]
    pub fn is_value_8(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_8
    }
    #[doc = "Checks if the value of the field is `VALUE_9`"]
    #[inline]
    pub fn is_value_9(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_9
    }
    #[doc = "Checks if the value of the field is `VALUE_10`"]
    #[inline]
    pub fn is_value_10(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_10
    }
    #[doc = "Checks if the value of the field is `VALUE_11`"]
    #[inline]
    pub fn is_value_11(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_11
    }
    #[doc = "Checks if the value of the field is `VALUE_12`"]
    #[inline]
    pub fn is_value_12(&self) -> bool {
        *self == SEC_VIO_INFO_MASTERR::VALUE_12
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - security violation access read/write indicator."]
    #[inline]
    pub fn sec_vio_info_write(&self) -> SEC_VIO_INFO_WRITER {
        SEC_VIO_INFO_WRITER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - security violation access data/code indicator."]
    #[inline]
    pub fn sec_vio_info_data_access(&self) -> SEC_VIO_INFO_DATA_ACCESSR {
        SEC_VIO_INFO_DATA_ACCESSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
        SEC_VIO_INFO_MASTERR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
