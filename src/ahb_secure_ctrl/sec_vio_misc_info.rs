#[doc = "Reader of register sec_vio_misc_info[%s]"]
pub type R = crate::R<u32, super::SEC_VIO_MISC_INFO>;
#[doc = "Possible values of the field `SEC_VIO_INFO_WRITE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_INFO_WRITE_A {
    #[doc = "Read access."]
    READ,
    #[doc = "Write access."]
    WRITE,
}
impl From<SEC_VIO_INFO_WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_VIO_INFO_WRITE_A) -> Self {
        match variant {
            SEC_VIO_INFO_WRITE_A::READ => false,
            SEC_VIO_INFO_WRITE_A::WRITE => true,
        }
    }
}
#[doc = "Reader of field `SEC_VIO_INFO_WRITE`"]
pub type SEC_VIO_INFO_WRITE_R = crate::R<bool, SEC_VIO_INFO_WRITE_A>;
impl SEC_VIO_INFO_WRITE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_VIO_INFO_WRITE_A {
        match self.bits {
            false => SEC_VIO_INFO_WRITE_A::READ,
            true => SEC_VIO_INFO_WRITE_A::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == SEC_VIO_INFO_WRITE_A::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == SEC_VIO_INFO_WRITE_A::WRITE
    }
}
#[doc = "Possible values of the field `SEC_VIO_INFO_DATA_ACCESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_INFO_DATA_ACCESS_A {
    #[doc = "Code access."]
    CODE,
    #[doc = "Data access."]
    DATA,
}
impl From<SEC_VIO_INFO_DATA_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_VIO_INFO_DATA_ACCESS_A) -> Self {
        match variant {
            SEC_VIO_INFO_DATA_ACCESS_A::CODE => false,
            SEC_VIO_INFO_DATA_ACCESS_A::DATA => true,
        }
    }
}
#[doc = "Reader of field `SEC_VIO_INFO_DATA_ACCESS`"]
pub type SEC_VIO_INFO_DATA_ACCESS_R = crate::R<bool, SEC_VIO_INFO_DATA_ACCESS_A>;
impl SEC_VIO_INFO_DATA_ACCESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_VIO_INFO_DATA_ACCESS_A {
        match self.bits {
            false => SEC_VIO_INFO_DATA_ACCESS_A::CODE,
            true => SEC_VIO_INFO_DATA_ACCESS_A::DATA,
        }
    }
    #[doc = "Checks if the value of the field is `CODE`"]
    #[inline(always)]
    pub fn is_code(&self) -> bool {
        *self == SEC_VIO_INFO_DATA_ACCESS_A::CODE
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == SEC_VIO_INFO_DATA_ACCESS_A::DATA
    }
}
#[doc = "Reader of field `SEC_VIO_INFO_MASTER_SEC_LEVEL`"]
pub type SEC_VIO_INFO_MASTER_SEC_LEVEL_R = crate::R<u8, u8>;
#[doc = "Possible values of the field `SEC_VIO_INFO_MASTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_INFO_MASTER_A {
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
}
impl From<SEC_VIO_INFO_MASTER_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_VIO_INFO_MASTER_A) -> Self {
        match variant {
            SEC_VIO_INFO_MASTER_A::VALUE_0 => 0,
            SEC_VIO_INFO_MASTER_A::VALUE_1 => 1,
            SEC_VIO_INFO_MASTER_A::VALUE_2 => 2,
            SEC_VIO_INFO_MASTER_A::VALUE_3 => 3,
            SEC_VIO_INFO_MASTER_A::VALUE_4 => 4,
            SEC_VIO_INFO_MASTER_A::VALUE_5 => 5,
            SEC_VIO_INFO_MASTER_A::VALUE_8 => 8,
            SEC_VIO_INFO_MASTER_A::VALUE_9 => 9,
            SEC_VIO_INFO_MASTER_A::VALUE_10 => 10,
            SEC_VIO_INFO_MASTER_A::VALUE_11 => 11,
            SEC_VIO_INFO_MASTER_A::VALUE_12 => 12,
        }
    }
}
#[doc = "Reader of field `SEC_VIO_INFO_MASTER`"]
pub type SEC_VIO_INFO_MASTER_R = crate::R<u8, SEC_VIO_INFO_MASTER_A>;
impl SEC_VIO_INFO_MASTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEC_VIO_INFO_MASTER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEC_VIO_INFO_MASTER_A::VALUE_0),
            1 => Val(SEC_VIO_INFO_MASTER_A::VALUE_1),
            2 => Val(SEC_VIO_INFO_MASTER_A::VALUE_2),
            3 => Val(SEC_VIO_INFO_MASTER_A::VALUE_3),
            4 => Val(SEC_VIO_INFO_MASTER_A::VALUE_4),
            5 => Val(SEC_VIO_INFO_MASTER_A::VALUE_5),
            8 => Val(SEC_VIO_INFO_MASTER_A::VALUE_8),
            9 => Val(SEC_VIO_INFO_MASTER_A::VALUE_9),
            10 => Val(SEC_VIO_INFO_MASTER_A::VALUE_10),
            11 => Val(SEC_VIO_INFO_MASTER_A::VALUE_11),
            12 => Val(SEC_VIO_INFO_MASTER_A::VALUE_12),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_3
    }
    #[doc = "Checks if the value of the field is `VALUE_4`"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_4
    }
    #[doc = "Checks if the value of the field is `VALUE_5`"]
    #[inline(always)]
    pub fn is_value_5(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_5
    }
    #[doc = "Checks if the value of the field is `VALUE_8`"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_8
    }
    #[doc = "Checks if the value of the field is `VALUE_9`"]
    #[inline(always)]
    pub fn is_value_9(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_9
    }
    #[doc = "Checks if the value of the field is `VALUE_10`"]
    #[inline(always)]
    pub fn is_value_10(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_10
    }
    #[doc = "Checks if the value of the field is `VALUE_11`"]
    #[inline(always)]
    pub fn is_value_11(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_11
    }
    #[doc = "Checks if the value of the field is `VALUE_12`"]
    #[inline(always)]
    pub fn is_value_12(&self) -> bool {
        *self == SEC_VIO_INFO_MASTER_A::VALUE_12
    }
}
impl R {
    #[doc = "Bit 0 - security violation access read/write indicator."]
    #[inline(always)]
    pub fn sec_vio_info_write(&self) -> SEC_VIO_INFO_WRITE_R {
        SEC_VIO_INFO_WRITE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - security violation access data/code indicator."]
    #[inline(always)]
    pub fn sec_vio_info_data_access(&self) -> SEC_VIO_INFO_DATA_ACCESS_R {
        SEC_VIO_INFO_DATA_ACCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level"]
    #[inline(always)]
    pub fn sec_vio_info_master_sec_level(&self) -> SEC_VIO_INFO_MASTER_SEC_LEVEL_R {
        SEC_VIO_INFO_MASTER_SEC_LEVEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - security violation master number"]
    #[inline(always)]
    pub fn sec_vio_info_master(&self) -> SEC_VIO_INFO_MASTER_R {
        SEC_VIO_INFO_MASTER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
