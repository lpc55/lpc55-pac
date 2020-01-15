#[doc = "Reader of register RESFIFO[%s]"]
pub type R = crate::R<u32, super::RESFIFO>;
#[doc = "Reader of field `D`"]
pub type D_R = crate::R<u16, u16>;
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
#[doc = "Reader of field `TSRC`"]
pub type TSRC_R = crate::R<u8, TSRC_A>;
impl TSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSRC_A::TSRC_0),
            1 => Val(TSRC_A::TSRC_1),
            2 => Val(TSRC_A::TSRC_2),
            3 => Val(TSRC_A::TSRC_3),
            4 => Val(TSRC_A::TSRC_4),
            5 => Val(TSRC_A::TSRC_5),
            6 => Val(TSRC_A::TSRC_6),
            7 => Val(TSRC_A::TSRC_7),
            8 => Val(TSRC_A::TSRC_8),
            9 => Val(TSRC_A::TSRC_9),
            15 => Val(TSRC_A::TSRC_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TSRC_0`"]
    #[inline(always)]
    pub fn is_tsrc_0(&self) -> bool {
        *self == TSRC_A::TSRC_0
    }
    #[doc = "Checks if the value of the field is `TSRC_1`"]
    #[inline(always)]
    pub fn is_tsrc_1(&self) -> bool {
        *self == TSRC_A::TSRC_1
    }
    #[doc = "Checks if the value of the field is `TSRC_2`"]
    #[inline(always)]
    pub fn is_tsrc_2(&self) -> bool {
        *self == TSRC_A::TSRC_2
    }
    #[doc = "Checks if the value of the field is `TSRC_3`"]
    #[inline(always)]
    pub fn is_tsrc_3(&self) -> bool {
        *self == TSRC_A::TSRC_3
    }
    #[doc = "Checks if the value of the field is `TSRC_4`"]
    #[inline(always)]
    pub fn is_tsrc_4(&self) -> bool {
        *self == TSRC_A::TSRC_4
    }
    #[doc = "Checks if the value of the field is `TSRC_5`"]
    #[inline(always)]
    pub fn is_tsrc_5(&self) -> bool {
        *self == TSRC_A::TSRC_5
    }
    #[doc = "Checks if the value of the field is `TSRC_6`"]
    #[inline(always)]
    pub fn is_tsrc_6(&self) -> bool {
        *self == TSRC_A::TSRC_6
    }
    #[doc = "Checks if the value of the field is `TSRC_7`"]
    #[inline(always)]
    pub fn is_tsrc_7(&self) -> bool {
        *self == TSRC_A::TSRC_7
    }
    #[doc = "Checks if the value of the field is `TSRC_8`"]
    #[inline(always)]
    pub fn is_tsrc_8(&self) -> bool {
        *self == TSRC_A::TSRC_8
    }
    #[doc = "Checks if the value of the field is `TSRC_9`"]
    #[inline(always)]
    pub fn is_tsrc_9(&self) -> bool {
        *self == TSRC_A::TSRC_9
    }
    #[doc = "Checks if the value of the field is `TSRC_15`"]
    #[inline(always)]
    pub fn is_tsrc_15(&self) -> bool {
        *self == TSRC_A::TSRC_15
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
#[doc = "Reader of field `LOOPCNT`"]
pub type LOOPCNT_R = crate::R<u8, LOOPCNT_A>;
impl LOOPCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOOPCNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOOPCNT_A::LOOPCNT_0),
            1 => Val(LOOPCNT_A::LOOPCNT_1),
            2 => Val(LOOPCNT_A::LOOPCNT_2),
            3 => Val(LOOPCNT_A::LOOPCNT_3),
            4 => Val(LOOPCNT_A::LOOPCNT_4),
            5 => Val(LOOPCNT_A::LOOPCNT_5),
            6 => Val(LOOPCNT_A::LOOPCNT_6),
            7 => Val(LOOPCNT_A::LOOPCNT_7),
            8 => Val(LOOPCNT_A::LOOPCNT_8),
            9 => Val(LOOPCNT_A::LOOPCNT_9),
            15 => Val(LOOPCNT_A::LOOPCNT_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_0`"]
    #[inline(always)]
    pub fn is_loopcnt_0(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_0
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_1`"]
    #[inline(always)]
    pub fn is_loopcnt_1(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_1
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_2`"]
    #[inline(always)]
    pub fn is_loopcnt_2(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_2
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_3`"]
    #[inline(always)]
    pub fn is_loopcnt_3(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_3
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_4`"]
    #[inline(always)]
    pub fn is_loopcnt_4(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_4
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_5`"]
    #[inline(always)]
    pub fn is_loopcnt_5(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_5
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_6`"]
    #[inline(always)]
    pub fn is_loopcnt_6(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_6
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_7`"]
    #[inline(always)]
    pub fn is_loopcnt_7(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_7
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_8`"]
    #[inline(always)]
    pub fn is_loopcnt_8(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_8
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_9`"]
    #[inline(always)]
    pub fn is_loopcnt_9(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_9
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_15`"]
    #[inline(always)]
    pub fn is_loopcnt_15(&self) -> bool {
        *self == LOOPCNT_A::LOOPCNT_15
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
#[doc = "Reader of field `CMDSRC`"]
pub type CMDSRC_R = crate::R<u8, CMDSRC_A>;
impl CMDSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMDSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMDSRC_A::CMDSRC_0),
            1 => Val(CMDSRC_A::CMDSRC_1),
            2 => Val(CMDSRC_A::CMDSRC_2),
            3 => Val(CMDSRC_A::CMDSRC_3),
            4 => Val(CMDSRC_A::CMDSRC_4),
            5 => Val(CMDSRC_A::CMDSRC_5),
            6 => Val(CMDSRC_A::CMDSRC_6),
            7 => Val(CMDSRC_A::CMDSRC_7),
            8 => Val(CMDSRC_A::CMDSRC_8),
            9 => Val(CMDSRC_A::CMDSRC_9),
            15 => Val(CMDSRC_A::CMDSRC_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMDSRC_0`"]
    #[inline(always)]
    pub fn is_cmdsrc_0(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_0
    }
    #[doc = "Checks if the value of the field is `CMDSRC_1`"]
    #[inline(always)]
    pub fn is_cmdsrc_1(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_1
    }
    #[doc = "Checks if the value of the field is `CMDSRC_2`"]
    #[inline(always)]
    pub fn is_cmdsrc_2(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_2
    }
    #[doc = "Checks if the value of the field is `CMDSRC_3`"]
    #[inline(always)]
    pub fn is_cmdsrc_3(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_3
    }
    #[doc = "Checks if the value of the field is `CMDSRC_4`"]
    #[inline(always)]
    pub fn is_cmdsrc_4(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_4
    }
    #[doc = "Checks if the value of the field is `CMDSRC_5`"]
    #[inline(always)]
    pub fn is_cmdsrc_5(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_5
    }
    #[doc = "Checks if the value of the field is `CMDSRC_6`"]
    #[inline(always)]
    pub fn is_cmdsrc_6(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_6
    }
    #[doc = "Checks if the value of the field is `CMDSRC_7`"]
    #[inline(always)]
    pub fn is_cmdsrc_7(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_7
    }
    #[doc = "Checks if the value of the field is `CMDSRC_8`"]
    #[inline(always)]
    pub fn is_cmdsrc_8(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_8
    }
    #[doc = "Checks if the value of the field is `CMDSRC_9`"]
    #[inline(always)]
    pub fn is_cmdsrc_9(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_9
    }
    #[doc = "Checks if the value of the field is `CMDSRC_15`"]
    #[inline(always)]
    pub fn is_cmdsrc_15(&self) -> bool {
        *self == CMDSRC_A::CMDSRC_15
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
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, VALID_A>;
impl VALID_R {
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
        *self == VALID_A::VALID_0
    }
    #[doc = "Checks if the value of the field is `VALID_1`"]
    #[inline(always)]
    pub fn is_valid_1(&self) -> bool {
        *self == VALID_A::VALID_1
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
