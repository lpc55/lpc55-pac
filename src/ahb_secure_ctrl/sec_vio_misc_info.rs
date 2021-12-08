#[doc = "Register `sec_vio_misc_info[%s]` reader"]
pub struct R(crate::R<SEC_VIO_MISC_INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_VIO_MISC_INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_VIO_MISC_INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_VIO_MISC_INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "security violation access read/write indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_INFO_WRITE_A {
    #[doc = "0: Read access."]
    READ = 0,
    #[doc = "1: Write access."]
    WRITE = 1,
}
impl From<SEC_VIO_INFO_WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_VIO_INFO_WRITE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC_VIO_INFO_WRITE` reader - security violation access read/write indicator."]
pub struct SEC_VIO_INFO_WRITE_R(crate::FieldReader<bool, SEC_VIO_INFO_WRITE_A>);
impl SEC_VIO_INFO_WRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEC_VIO_INFO_WRITE_R(crate::FieldReader::new(bits))
    }
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
        **self == SEC_VIO_INFO_WRITE_A::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == SEC_VIO_INFO_WRITE_A::WRITE
    }
}
impl core::ops::Deref for SEC_VIO_INFO_WRITE_R {
    type Target = crate::FieldReader<bool, SEC_VIO_INFO_WRITE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "security violation access data/code indicator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_INFO_DATA_ACCESS_A {
    #[doc = "0: Code access."]
    CODE = 0,
    #[doc = "1: Data access."]
    DATA = 1,
}
impl From<SEC_VIO_INFO_DATA_ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_VIO_INFO_DATA_ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC_VIO_INFO_DATA_ACCESS` reader - security violation access data/code indicator."]
pub struct SEC_VIO_INFO_DATA_ACCESS_R(crate::FieldReader<bool, SEC_VIO_INFO_DATA_ACCESS_A>);
impl SEC_VIO_INFO_DATA_ACCESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEC_VIO_INFO_DATA_ACCESS_R(crate::FieldReader::new(bits))
    }
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
        **self == SEC_VIO_INFO_DATA_ACCESS_A::CODE
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        **self == SEC_VIO_INFO_DATA_ACCESS_A::DATA
    }
}
impl core::ops::Deref for SEC_VIO_INFO_DATA_ACCESS_R {
    type Target = crate::FieldReader<bool, SEC_VIO_INFO_DATA_ACCESS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC_VIO_INFO_MASTER_SEC_LEVEL` reader - bit \\[5:4\\]: master sec level and privilege level bit \\[7:6\\]: anti-pol value for master sec level and privilege level"]
pub struct SEC_VIO_INFO_MASTER_SEC_LEVEL_R(crate::FieldReader<u8, u8>);
impl SEC_VIO_INFO_MASTER_SEC_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEC_VIO_INFO_MASTER_SEC_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_VIO_INFO_MASTER_SEC_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "security violation master number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_VIO_INFO_MASTER_A {
    #[doc = "0: CPU0 Code."]
    VALUE_0 = 0,
    #[doc = "1: CPU0 System."]
    VALUE_1 = 1,
    #[doc = "2: CPU1 Data."]
    VALUE_2 = 2,
    #[doc = "3: CPU1 System."]
    VALUE_3 = 3,
    #[doc = "4: USB-HS Device."]
    VALUE_4 = 4,
    #[doc = "5: SDMA0."]
    VALUE_5 = 5,
    #[doc = "8: SDIO."]
    VALUE_8 = 8,
    #[doc = "9: PowerQuad."]
    VALUE_9 = 9,
    #[doc = "10: HASH."]
    VALUE_10 = 10,
    #[doc = "11: USB-FS Host."]
    VALUE_11 = 11,
    #[doc = "12: SDMA1."]
    VALUE_12 = 12,
}
impl From<SEC_VIO_INFO_MASTER_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_VIO_INFO_MASTER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEC_VIO_INFO_MASTER` reader - security violation master number"]
pub struct SEC_VIO_INFO_MASTER_R(crate::FieldReader<u8, SEC_VIO_INFO_MASTER_A>);
impl SEC_VIO_INFO_MASTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEC_VIO_INFO_MASTER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_VIO_INFO_MASTER_A> {
        match self.bits {
            0 => Some(SEC_VIO_INFO_MASTER_A::VALUE_0),
            1 => Some(SEC_VIO_INFO_MASTER_A::VALUE_1),
            2 => Some(SEC_VIO_INFO_MASTER_A::VALUE_2),
            3 => Some(SEC_VIO_INFO_MASTER_A::VALUE_3),
            4 => Some(SEC_VIO_INFO_MASTER_A::VALUE_4),
            5 => Some(SEC_VIO_INFO_MASTER_A::VALUE_5),
            8 => Some(SEC_VIO_INFO_MASTER_A::VALUE_8),
            9 => Some(SEC_VIO_INFO_MASTER_A::VALUE_9),
            10 => Some(SEC_VIO_INFO_MASTER_A::VALUE_10),
            11 => Some(SEC_VIO_INFO_MASTER_A::VALUE_11),
            12 => Some(SEC_VIO_INFO_MASTER_A::VALUE_12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_3
    }
    #[doc = "Checks if the value of the field is `VALUE_4`"]
    #[inline(always)]
    pub fn is_value_4(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_4
    }
    #[doc = "Checks if the value of the field is `VALUE_5`"]
    #[inline(always)]
    pub fn is_value_5(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_5
    }
    #[doc = "Checks if the value of the field is `VALUE_8`"]
    #[inline(always)]
    pub fn is_value_8(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_8
    }
    #[doc = "Checks if the value of the field is `VALUE_9`"]
    #[inline(always)]
    pub fn is_value_9(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_9
    }
    #[doc = "Checks if the value of the field is `VALUE_10`"]
    #[inline(always)]
    pub fn is_value_10(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_10
    }
    #[doc = "Checks if the value of the field is `VALUE_11`"]
    #[inline(always)]
    pub fn is_value_11(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_11
    }
    #[doc = "Checks if the value of the field is `VALUE_12`"]
    #[inline(always)]
    pub fn is_value_12(&self) -> bool {
        **self == SEC_VIO_INFO_MASTER_A::VALUE_12
    }
}
impl core::ops::Deref for SEC_VIO_INFO_MASTER_R {
    type Target = crate::FieldReader<u8, SEC_VIO_INFO_MASTER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "most recent security violation miscellaneous information for AHB port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_vio_misc_info](index.html) module"]
pub struct SEC_VIO_MISC_INFO_SPEC;
impl crate::RegisterSpec for SEC_VIO_MISC_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_vio_misc_info::R](R) reader structure"]
impl crate::Readable for SEC_VIO_MISC_INFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sec_vio_misc_info[%s]
to value 0"]
impl crate::Resettable for SEC_VIO_MISC_INFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
