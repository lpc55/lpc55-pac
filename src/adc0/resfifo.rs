#[doc = "Register `RESFIFO[%s]` reader"]
pub struct R(crate::R<RESFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RESFIFO_SPEC>> for R {
    fn from(reader: crate::R<RESFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D` reader - Data result"]
pub struct D_R(crate::FieldReader<u16, u16>);
impl D_R {
    pub(crate) fn new(bits: u16) -> Self {
        D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSRC_A {
    #[doc = "0: Trigger source 0 initiated this conversion."]
    TSRC_0 = 0,
    #[doc = "1: Trigger source 1 initiated this conversion."]
    TSRC_1 = 1,
    #[doc = "2: Corresponding trigger source initiated this conversion."]
    TSRC_2 = 2,
    #[doc = "3: Corresponding trigger source initiated this conversion."]
    TSRC_3 = 3,
    #[doc = "4: Corresponding trigger source initiated this conversion."]
    TSRC_4 = 4,
    #[doc = "5: Corresponding trigger source initiated this conversion."]
    TSRC_5 = 5,
    #[doc = "6: Corresponding trigger source initiated this conversion."]
    TSRC_6 = 6,
    #[doc = "7: Corresponding trigger source initiated this conversion."]
    TSRC_7 = 7,
    #[doc = "8: Corresponding trigger source initiated this conversion."]
    TSRC_8 = 8,
    #[doc = "9: Corresponding trigger source initiated this conversion."]
    TSRC_9 = 9,
    #[doc = "15: Trigger source 15 initiated this conversion."]
    TSRC_15 = 15,
}
impl From<TSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSRC` reader - Trigger Source"]
pub struct TSRC_R(crate::FieldReader<u8, TSRC_A>);
impl TSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSRC_A> {
        match self.bits {
            0 => Some(TSRC_A::TSRC_0),
            1 => Some(TSRC_A::TSRC_1),
            2 => Some(TSRC_A::TSRC_2),
            3 => Some(TSRC_A::TSRC_3),
            4 => Some(TSRC_A::TSRC_4),
            5 => Some(TSRC_A::TSRC_5),
            6 => Some(TSRC_A::TSRC_6),
            7 => Some(TSRC_A::TSRC_7),
            8 => Some(TSRC_A::TSRC_8),
            9 => Some(TSRC_A::TSRC_9),
            15 => Some(TSRC_A::TSRC_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TSRC_0`"]
    #[inline(always)]
    pub fn is_tsrc_0(&self) -> bool {
        **self == TSRC_A::TSRC_0
    }
    #[doc = "Checks if the value of the field is `TSRC_1`"]
    #[inline(always)]
    pub fn is_tsrc_1(&self) -> bool {
        **self == TSRC_A::TSRC_1
    }
    #[doc = "Checks if the value of the field is `TSRC_2`"]
    #[inline(always)]
    pub fn is_tsrc_2(&self) -> bool {
        **self == TSRC_A::TSRC_2
    }
    #[doc = "Checks if the value of the field is `TSRC_3`"]
    #[inline(always)]
    pub fn is_tsrc_3(&self) -> bool {
        **self == TSRC_A::TSRC_3
    }
    #[doc = "Checks if the value of the field is `TSRC_4`"]
    #[inline(always)]
    pub fn is_tsrc_4(&self) -> bool {
        **self == TSRC_A::TSRC_4
    }
    #[doc = "Checks if the value of the field is `TSRC_5`"]
    #[inline(always)]
    pub fn is_tsrc_5(&self) -> bool {
        **self == TSRC_A::TSRC_5
    }
    #[doc = "Checks if the value of the field is `TSRC_6`"]
    #[inline(always)]
    pub fn is_tsrc_6(&self) -> bool {
        **self == TSRC_A::TSRC_6
    }
    #[doc = "Checks if the value of the field is `TSRC_7`"]
    #[inline(always)]
    pub fn is_tsrc_7(&self) -> bool {
        **self == TSRC_A::TSRC_7
    }
    #[doc = "Checks if the value of the field is `TSRC_8`"]
    #[inline(always)]
    pub fn is_tsrc_8(&self) -> bool {
        **self == TSRC_A::TSRC_8
    }
    #[doc = "Checks if the value of the field is `TSRC_9`"]
    #[inline(always)]
    pub fn is_tsrc_9(&self) -> bool {
        **self == TSRC_A::TSRC_9
    }
    #[doc = "Checks if the value of the field is `TSRC_15`"]
    #[inline(always)]
    pub fn is_tsrc_15(&self) -> bool {
        **self == TSRC_A::TSRC_15
    }
}
impl core::ops::Deref for TSRC_R {
    type Target = crate::FieldReader<u8, TSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Loop count value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOOPCNT_A {
    #[doc = "0: Result is from initial conversion in command."]
    LOOPCNT_0 = 0,
    #[doc = "1: Result is from second conversion in command."]
    LOOPCNT_1 = 1,
    #[doc = "2: Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_2 = 2,
    #[doc = "3: Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_3 = 3,
    #[doc = "4: Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_4 = 4,
    #[doc = "5: Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_5 = 5,
    #[doc = "6: Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_6 = 6,
    #[doc = "7: Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_7 = 7,
    #[doc = "8: Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_8 = 8,
    #[doc = "9: Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_9 = 9,
    #[doc = "15: Result is from 16th conversion in command."]
    LOOPCNT_15 = 15,
}
impl From<LOOPCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: LOOPCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOOPCNT` reader - Loop count value"]
pub struct LOOPCNT_R(crate::FieldReader<u8, LOOPCNT_A>);
impl LOOPCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOOPCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOOPCNT_A> {
        match self.bits {
            0 => Some(LOOPCNT_A::LOOPCNT_0),
            1 => Some(LOOPCNT_A::LOOPCNT_1),
            2 => Some(LOOPCNT_A::LOOPCNT_2),
            3 => Some(LOOPCNT_A::LOOPCNT_3),
            4 => Some(LOOPCNT_A::LOOPCNT_4),
            5 => Some(LOOPCNT_A::LOOPCNT_5),
            6 => Some(LOOPCNT_A::LOOPCNT_6),
            7 => Some(LOOPCNT_A::LOOPCNT_7),
            8 => Some(LOOPCNT_A::LOOPCNT_8),
            9 => Some(LOOPCNT_A::LOOPCNT_9),
            15 => Some(LOOPCNT_A::LOOPCNT_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_0`"]
    #[inline(always)]
    pub fn is_loopcnt_0(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_0
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_1`"]
    #[inline(always)]
    pub fn is_loopcnt_1(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_1
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_2`"]
    #[inline(always)]
    pub fn is_loopcnt_2(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_2
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_3`"]
    #[inline(always)]
    pub fn is_loopcnt_3(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_3
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_4`"]
    #[inline(always)]
    pub fn is_loopcnt_4(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_4
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_5`"]
    #[inline(always)]
    pub fn is_loopcnt_5(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_5
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_6`"]
    #[inline(always)]
    pub fn is_loopcnt_6(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_6
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_7`"]
    #[inline(always)]
    pub fn is_loopcnt_7(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_7
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_8`"]
    #[inline(always)]
    pub fn is_loopcnt_8(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_8
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_9`"]
    #[inline(always)]
    pub fn is_loopcnt_9(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_9
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_15`"]
    #[inline(always)]
    pub fn is_loopcnt_15(&self) -> bool {
        **self == LOOPCNT_A::LOOPCNT_15
    }
}
impl core::ops::Deref for LOOPCNT_R {
    type Target = crate::FieldReader<u8, LOOPCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Command Buffer Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDSRC_A {
    #[doc = "0: Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    CMDSRC_0 = 0,
    #[doc = "1: CMD1 buffer used as control settings for this conversion."]
    CMDSRC_1 = 1,
    #[doc = "2: Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_2 = 2,
    #[doc = "3: Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_3 = 3,
    #[doc = "4: Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_4 = 4,
    #[doc = "5: Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_5 = 5,
    #[doc = "6: Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_6 = 6,
    #[doc = "7: Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_7 = 7,
    #[doc = "8: Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_8 = 8,
    #[doc = "9: Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_9 = 9,
    #[doc = "15: CMD15 buffer used as control settings for this conversion."]
    CMDSRC_15 = 15,
}
impl From<CMDSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMDSRC` reader - Command Buffer Source"]
pub struct CMDSRC_R(crate::FieldReader<u8, CMDSRC_A>);
impl CMDSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMDSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDSRC_A> {
        match self.bits {
            0 => Some(CMDSRC_A::CMDSRC_0),
            1 => Some(CMDSRC_A::CMDSRC_1),
            2 => Some(CMDSRC_A::CMDSRC_2),
            3 => Some(CMDSRC_A::CMDSRC_3),
            4 => Some(CMDSRC_A::CMDSRC_4),
            5 => Some(CMDSRC_A::CMDSRC_5),
            6 => Some(CMDSRC_A::CMDSRC_6),
            7 => Some(CMDSRC_A::CMDSRC_7),
            8 => Some(CMDSRC_A::CMDSRC_8),
            9 => Some(CMDSRC_A::CMDSRC_9),
            15 => Some(CMDSRC_A::CMDSRC_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CMDSRC_0`"]
    #[inline(always)]
    pub fn is_cmdsrc_0(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_0
    }
    #[doc = "Checks if the value of the field is `CMDSRC_1`"]
    #[inline(always)]
    pub fn is_cmdsrc_1(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_1
    }
    #[doc = "Checks if the value of the field is `CMDSRC_2`"]
    #[inline(always)]
    pub fn is_cmdsrc_2(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_2
    }
    #[doc = "Checks if the value of the field is `CMDSRC_3`"]
    #[inline(always)]
    pub fn is_cmdsrc_3(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_3
    }
    #[doc = "Checks if the value of the field is `CMDSRC_4`"]
    #[inline(always)]
    pub fn is_cmdsrc_4(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_4
    }
    #[doc = "Checks if the value of the field is `CMDSRC_5`"]
    #[inline(always)]
    pub fn is_cmdsrc_5(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_5
    }
    #[doc = "Checks if the value of the field is `CMDSRC_6`"]
    #[inline(always)]
    pub fn is_cmdsrc_6(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_6
    }
    #[doc = "Checks if the value of the field is `CMDSRC_7`"]
    #[inline(always)]
    pub fn is_cmdsrc_7(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_7
    }
    #[doc = "Checks if the value of the field is `CMDSRC_8`"]
    #[inline(always)]
    pub fn is_cmdsrc_8(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_8
    }
    #[doc = "Checks if the value of the field is `CMDSRC_9`"]
    #[inline(always)]
    pub fn is_cmdsrc_9(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_9
    }
    #[doc = "Checks if the value of the field is `CMDSRC_15`"]
    #[inline(always)]
    pub fn is_cmdsrc_15(&self) -> bool {
        **self == CMDSRC_A::CMDSRC_15
    }
}
impl core::ops::Deref for CMDSRC_R {
    type Target = crate::FieldReader<u8, CMDSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FIFO entry is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_A {
    #[doc = "0: FIFO is empty. Discard any read from RESFIFO."]
    VALID_0 = 0,
    #[doc = "1: FIFO record read from RESFIFO is valid."]
    VALID_1 = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - FIFO entry is valid"]
pub struct VALID_R(crate::FieldReader<bool, VALID_A>);
impl VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::VALID_0,
            true => VALID_A::VALID_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_0`"]
    #[inline(always)]
    pub fn is_valid_0(&self) -> bool {
        **self == VALID_A::VALID_0
    }
    #[doc = "Checks if the value of the field is `VALID_1`"]
    #[inline(always)]
    pub fn is_valid_1(&self) -> bool {
        **self == VALID_A::VALID_1
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<bool, VALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Data result"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Trigger Source"]
    #[inline(always)]
    pub fn tsrc(&self) -> TSRC_R {
        TSRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Loop count value"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LOOPCNT_R {
        LOOPCNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Command Buffer Source"]
    #[inline(always)]
    pub fn cmdsrc(&self) -> CMDSRC_R {
        CMDSRC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - FIFO entry is valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "ADC Data Result FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resfifo](index.html) module"]
pub struct RESFIFO_SPEC;
impl crate::RegisterSpec for RESFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resfifo::R](R) reader structure"]
impl crate::Readable for RESFIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESFIFO[%s]
to value 0"]
impl crate::Resettable for RESFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
