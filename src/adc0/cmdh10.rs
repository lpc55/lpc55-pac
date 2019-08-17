#[doc = "Reader of register CMDH10"]
pub type R = crate::R<u32, super::CMDH10>;
#[doc = "Writer for register CMDH10"]
pub type W = crate::W<u32, super::CMDH10>;
#[doc = "Register CMDH10 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDH10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `WAIT_TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_TRIG_A {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1,
}
impl From<WAIT_TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_TRIG_A) -> Self {
        match variant {
            WAIT_TRIG_A::WAIT_TRIG_0 => false,
            WAIT_TRIG_A::WAIT_TRIG_1 => true,
        }
    }
}
#[doc = "Reader of field `WAIT_TRIG`"]
pub type WAIT_TRIG_R = crate::R<bool, WAIT_TRIG_A>;
impl WAIT_TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_TRIG_A {
        match self.bits {
            false => WAIT_TRIG_A::WAIT_TRIG_0,
            true => WAIT_TRIG_A::WAIT_TRIG_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAIT_TRIG_0`"]
    #[inline(always)]
    pub fn is_wait_trig_0(&self) -> bool {
        *self == WAIT_TRIG_A::WAIT_TRIG_0
    }
    #[doc = "Checks if the value of the field is `WAIT_TRIG_1`"]
    #[inline(always)]
    pub fn is_wait_trig_1(&self) -> bool {
        *self == WAIT_TRIG_A::WAIT_TRIG_1
    }
}
#[doc = "Write proxy for field `WAIT_TRIG`"]
pub struct WAIT_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAIT_TRIG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This command will be automatically executed."]
    #[inline(always)]
    pub fn wait_trig_0(self) -> &'a mut W {
        self.variant(WAIT_TRIG_A::WAIT_TRIG_0)
    }
    #[doc = "The active trigger must be asserted again before executing this command."]
    #[inline(always)]
    pub fn wait_trig_1(self) -> &'a mut W {
        self.variant(WAIT_TRIG_A::WAIT_TRIG_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `LWI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LWI_A {
    #[doc = "Auto channel increment disabled"]
    LWI_0,
    #[doc = "Auto channel increment enabled"]
    LWI_1,
}
impl From<LWI_A> for bool {
    #[inline(always)]
    fn from(variant: LWI_A) -> Self {
        match variant {
            LWI_A::LWI_0 => false,
            LWI_A::LWI_1 => true,
        }
    }
}
#[doc = "Reader of field `LWI`"]
pub type LWI_R = crate::R<bool, LWI_A>;
impl LWI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LWI_A {
        match self.bits {
            false => LWI_A::LWI_0,
            true => LWI_A::LWI_1,
        }
    }
    #[doc = "Checks if the value of the field is `LWI_0`"]
    #[inline(always)]
    pub fn is_lwi_0(&self) -> bool {
        *self == LWI_A::LWI_0
    }
    #[doc = "Checks if the value of the field is `LWI_1`"]
    #[inline(always)]
    pub fn is_lwi_1(&self) -> bool {
        *self == LWI_A::LWI_1
    }
}
#[doc = "Write proxy for field `LWI`"]
pub struct LWI_W<'a> {
    w: &'a mut W,
}
impl<'a> LWI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LWI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Auto channel increment disabled"]
    #[inline(always)]
    pub fn lwi_0(self) -> &'a mut W {
        self.variant(LWI_A::LWI_0)
    }
    #[doc = "Auto channel increment enabled"]
    #[inline(always)]
    pub fn lwi_1(self) -> &'a mut W {
        self.variant(LWI_A::LWI_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `STS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STS_A {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7,
}
impl From<STS_A> for u8 {
    #[inline(always)]
    fn from(variant: STS_A) -> Self {
        match variant {
            STS_A::STS_0 => 0,
            STS_A::STS_1 => 1,
            STS_A::STS_2 => 2,
            STS_A::STS_3 => 3,
            STS_A::STS_4 => 4,
            STS_A::STS_5 => 5,
            STS_A::STS_6 => 6,
            STS_A::STS_7 => 7,
        }
    }
}
#[doc = "Reader of field `STS`"]
pub type STS_R = crate::R<u8, STS_A>;
impl STS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS_A {
        match self.bits {
            0 => STS_A::STS_0,
            1 => STS_A::STS_1,
            2 => STS_A::STS_2,
            3 => STS_A::STS_3,
            4 => STS_A::STS_4,
            5 => STS_A::STS_5,
            6 => STS_A::STS_6,
            7 => STS_A::STS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STS_0`"]
    #[inline(always)]
    pub fn is_sts_0(&self) -> bool {
        *self == STS_A::STS_0
    }
    #[doc = "Checks if the value of the field is `STS_1`"]
    #[inline(always)]
    pub fn is_sts_1(&self) -> bool {
        *self == STS_A::STS_1
    }
    #[doc = "Checks if the value of the field is `STS_2`"]
    #[inline(always)]
    pub fn is_sts_2(&self) -> bool {
        *self == STS_A::STS_2
    }
    #[doc = "Checks if the value of the field is `STS_3`"]
    #[inline(always)]
    pub fn is_sts_3(&self) -> bool {
        *self == STS_A::STS_3
    }
    #[doc = "Checks if the value of the field is `STS_4`"]
    #[inline(always)]
    pub fn is_sts_4(&self) -> bool {
        *self == STS_A::STS_4
    }
    #[doc = "Checks if the value of the field is `STS_5`"]
    #[inline(always)]
    pub fn is_sts_5(&self) -> bool {
        *self == STS_A::STS_5
    }
    #[doc = "Checks if the value of the field is `STS_6`"]
    #[inline(always)]
    pub fn is_sts_6(&self) -> bool {
        *self == STS_A::STS_6
    }
    #[doc = "Checks if the value of the field is `STS_7`"]
    #[inline(always)]
    pub fn is_sts_7(&self) -> bool {
        *self == STS_A::STS_7
    }
}
#[doc = "Write proxy for field `STS`"]
pub struct STS_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    #[inline(always)]
    pub fn sts_0(self) -> &'a mut W {
        self.variant(STS_A::STS_0)
    }
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_1(self) -> &'a mut W {
        self.variant(STS_A::STS_1)
    }
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_2(self) -> &'a mut W {
        self.variant(STS_A::STS_2)
    }
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_3(self) -> &'a mut W {
        self.variant(STS_A::STS_3)
    }
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_4(self) -> &'a mut W {
        self.variant(STS_A::STS_4)
    }
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_5(self) -> &'a mut W {
        self.variant(STS_A::STS_5)
    }
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_6(self) -> &'a mut W {
        self.variant(STS_A::STS_6)
    }
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn sts_7(self) -> &'a mut W {
        self.variant(STS_A::STS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `AVGS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVGS_A {
    #[doc = "Single conversion."]
    AVGS_0,
    #[doc = "2 conversions averaged."]
    AVGS_1,
    #[doc = "4 conversions averaged."]
    AVGS_2,
    #[doc = "8 conversions averaged."]
    AVGS_3,
    #[doc = "16 conversions averaged."]
    AVGS_4,
    #[doc = "32 conversions averaged."]
    AVGS_5,
    #[doc = "64 conversions averaged."]
    AVGS_6,
    #[doc = "128 conversions averaged."]
    AVGS_7,
}
impl From<AVGS_A> for u8 {
    #[inline(always)]
    fn from(variant: AVGS_A) -> Self {
        match variant {
            AVGS_A::AVGS_0 => 0,
            AVGS_A::AVGS_1 => 1,
            AVGS_A::AVGS_2 => 2,
            AVGS_A::AVGS_3 => 3,
            AVGS_A::AVGS_4 => 4,
            AVGS_A::AVGS_5 => 5,
            AVGS_A::AVGS_6 => 6,
            AVGS_A::AVGS_7 => 7,
        }
    }
}
#[doc = "Reader of field `AVGS`"]
pub type AVGS_R = crate::R<u8, AVGS_A>;
impl AVGS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGS_A {
        match self.bits {
            0 => AVGS_A::AVGS_0,
            1 => AVGS_A::AVGS_1,
            2 => AVGS_A::AVGS_2,
            3 => AVGS_A::AVGS_3,
            4 => AVGS_A::AVGS_4,
            5 => AVGS_A::AVGS_5,
            6 => AVGS_A::AVGS_6,
            7 => AVGS_A::AVGS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVGS_0`"]
    #[inline(always)]
    pub fn is_avgs_0(&self) -> bool {
        *self == AVGS_A::AVGS_0
    }
    #[doc = "Checks if the value of the field is `AVGS_1`"]
    #[inline(always)]
    pub fn is_avgs_1(&self) -> bool {
        *self == AVGS_A::AVGS_1
    }
    #[doc = "Checks if the value of the field is `AVGS_2`"]
    #[inline(always)]
    pub fn is_avgs_2(&self) -> bool {
        *self == AVGS_A::AVGS_2
    }
    #[doc = "Checks if the value of the field is `AVGS_3`"]
    #[inline(always)]
    pub fn is_avgs_3(&self) -> bool {
        *self == AVGS_A::AVGS_3
    }
    #[doc = "Checks if the value of the field is `AVGS_4`"]
    #[inline(always)]
    pub fn is_avgs_4(&self) -> bool {
        *self == AVGS_A::AVGS_4
    }
    #[doc = "Checks if the value of the field is `AVGS_5`"]
    #[inline(always)]
    pub fn is_avgs_5(&self) -> bool {
        *self == AVGS_A::AVGS_5
    }
    #[doc = "Checks if the value of the field is `AVGS_6`"]
    #[inline(always)]
    pub fn is_avgs_6(&self) -> bool {
        *self == AVGS_A::AVGS_6
    }
    #[doc = "Checks if the value of the field is `AVGS_7`"]
    #[inline(always)]
    pub fn is_avgs_7(&self) -> bool {
        *self == AVGS_A::AVGS_7
    }
}
#[doc = "Write proxy for field `AVGS`"]
pub struct AVGS_W<'a> {
    w: &'a mut W,
}
impl<'a> AVGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVGS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single conversion."]
    #[inline(always)]
    pub fn avgs_0(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_0)
    }
    #[doc = "2 conversions averaged."]
    #[inline(always)]
    pub fn avgs_1(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_1)
    }
    #[doc = "4 conversions averaged."]
    #[inline(always)]
    pub fn avgs_2(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_2)
    }
    #[doc = "8 conversions averaged."]
    #[inline(always)]
    pub fn avgs_3(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_3)
    }
    #[doc = "16 conversions averaged."]
    #[inline(always)]
    pub fn avgs_4(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_4)
    }
    #[doc = "32 conversions averaged."]
    #[inline(always)]
    pub fn avgs_5(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_5)
    }
    #[doc = "64 conversions averaged."]
    #[inline(always)]
    pub fn avgs_6(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_6)
    }
    #[doc = "128 conversions averaged."]
    #[inline(always)]
    pub fn avgs_7(self) -> &'a mut W {
        self.variant(AVGS_A::AVGS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOP_A {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15,
}
impl From<LOOP_A> for u8 {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        match variant {
            LOOP_A::LOOP_0 => 0,
            LOOP_A::LOOP_1 => 1,
            LOOP_A::LOOP_2 => 2,
            LOOP_A::LOOP_3 => 3,
            LOOP_A::LOOP_4 => 4,
            LOOP_A::LOOP_5 => 5,
            LOOP_A::LOOP_6 => 6,
            LOOP_A::LOOP_7 => 7,
            LOOP_A::LOOP_8 => 8,
            LOOP_A::LOOP_9 => 9,
            LOOP_A::LOOP_15 => 15,
        }
    }
}
#[doc = "Reader of field `LOOP`"]
pub type LOOP_R = crate::R<u8, LOOP_A>;
impl LOOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOOP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOOP_A::LOOP_0),
            1 => Val(LOOP_A::LOOP_1),
            2 => Val(LOOP_A::LOOP_2),
            3 => Val(LOOP_A::LOOP_3),
            4 => Val(LOOP_A::LOOP_4),
            5 => Val(LOOP_A::LOOP_5),
            6 => Val(LOOP_A::LOOP_6),
            7 => Val(LOOP_A::LOOP_7),
            8 => Val(LOOP_A::LOOP_8),
            9 => Val(LOOP_A::LOOP_9),
            15 => Val(LOOP_A::LOOP_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOOP_0`"]
    #[inline(always)]
    pub fn is_loop_0(&self) -> bool {
        *self == LOOP_A::LOOP_0
    }
    #[doc = "Checks if the value of the field is `LOOP_1`"]
    #[inline(always)]
    pub fn is_loop_1(&self) -> bool {
        *self == LOOP_A::LOOP_1
    }
    #[doc = "Checks if the value of the field is `LOOP_2`"]
    #[inline(always)]
    pub fn is_loop_2(&self) -> bool {
        *self == LOOP_A::LOOP_2
    }
    #[doc = "Checks if the value of the field is `LOOP_3`"]
    #[inline(always)]
    pub fn is_loop_3(&self) -> bool {
        *self == LOOP_A::LOOP_3
    }
    #[doc = "Checks if the value of the field is `LOOP_4`"]
    #[inline(always)]
    pub fn is_loop_4(&self) -> bool {
        *self == LOOP_A::LOOP_4
    }
    #[doc = "Checks if the value of the field is `LOOP_5`"]
    #[inline(always)]
    pub fn is_loop_5(&self) -> bool {
        *self == LOOP_A::LOOP_5
    }
    #[doc = "Checks if the value of the field is `LOOP_6`"]
    #[inline(always)]
    pub fn is_loop_6(&self) -> bool {
        *self == LOOP_A::LOOP_6
    }
    #[doc = "Checks if the value of the field is `LOOP_7`"]
    #[inline(always)]
    pub fn is_loop_7(&self) -> bool {
        *self == LOOP_A::LOOP_7
    }
    #[doc = "Checks if the value of the field is `LOOP_8`"]
    #[inline(always)]
    pub fn is_loop_8(&self) -> bool {
        *self == LOOP_A::LOOP_8
    }
    #[doc = "Checks if the value of the field is `LOOP_9`"]
    #[inline(always)]
    pub fn is_loop_9(&self) -> bool {
        *self == LOOP_A::LOOP_9
    }
    #[doc = "Checks if the value of the field is `LOOP_15`"]
    #[inline(always)]
    pub fn is_loop_15(&self) -> bool {
        *self == LOOP_A::LOOP_15
    }
}
#[doc = "Write proxy for field `LOOP`"]
pub struct LOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Looping not enabled. Command executes 1 time."]
    #[inline(always)]
    pub fn loop_0(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_0)
    }
    #[doc = "Loop 1 time. Command executes 2 times."]
    #[inline(always)]
    pub fn loop_1(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_1)
    }
    #[doc = "Loop 2 times. Command executes 3 times."]
    #[inline(always)]
    pub fn loop_2(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_2)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_3(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_3)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_4(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_4)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_5(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_5)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_6(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_6)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_7(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_7)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_8(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_8)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline(always)]
    pub fn loop_9(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_9)
    }
    #[doc = "Loop 15 times. Command executes 16 times."]
    #[inline(always)]
    pub fn loop_15(self) -> &'a mut W {
        self.variant(LOOP_A::LOOP_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `NEXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEXT_A {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15,
}
impl From<NEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: NEXT_A) -> Self {
        match variant {
            NEXT_A::NEXT_0 => 0,
            NEXT_A::NEXT_1 => 1,
            NEXT_A::NEXT_2 => 2,
            NEXT_A::NEXT_3 => 3,
            NEXT_A::NEXT_4 => 4,
            NEXT_A::NEXT_5 => 5,
            NEXT_A::NEXT_6 => 6,
            NEXT_A::NEXT_7 => 7,
            NEXT_A::NEXT_8 => 8,
            NEXT_A::NEXT_9 => 9,
            NEXT_A::NEXT_15 => 15,
        }
    }
}
#[doc = "Reader of field `NEXT`"]
pub type NEXT_R = crate::R<u8, NEXT_A>;
impl NEXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NEXT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NEXT_A::NEXT_0),
            1 => Val(NEXT_A::NEXT_1),
            2 => Val(NEXT_A::NEXT_2),
            3 => Val(NEXT_A::NEXT_3),
            4 => Val(NEXT_A::NEXT_4),
            5 => Val(NEXT_A::NEXT_5),
            6 => Val(NEXT_A::NEXT_6),
            7 => Val(NEXT_A::NEXT_7),
            8 => Val(NEXT_A::NEXT_8),
            9 => Val(NEXT_A::NEXT_9),
            15 => Val(NEXT_A::NEXT_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NEXT_0`"]
    #[inline(always)]
    pub fn is_next_0(&self) -> bool {
        *self == NEXT_A::NEXT_0
    }
    #[doc = "Checks if the value of the field is `NEXT_1`"]
    #[inline(always)]
    pub fn is_next_1(&self) -> bool {
        *self == NEXT_A::NEXT_1
    }
    #[doc = "Checks if the value of the field is `NEXT_2`"]
    #[inline(always)]
    pub fn is_next_2(&self) -> bool {
        *self == NEXT_A::NEXT_2
    }
    #[doc = "Checks if the value of the field is `NEXT_3`"]
    #[inline(always)]
    pub fn is_next_3(&self) -> bool {
        *self == NEXT_A::NEXT_3
    }
    #[doc = "Checks if the value of the field is `NEXT_4`"]
    #[inline(always)]
    pub fn is_next_4(&self) -> bool {
        *self == NEXT_A::NEXT_4
    }
    #[doc = "Checks if the value of the field is `NEXT_5`"]
    #[inline(always)]
    pub fn is_next_5(&self) -> bool {
        *self == NEXT_A::NEXT_5
    }
    #[doc = "Checks if the value of the field is `NEXT_6`"]
    #[inline(always)]
    pub fn is_next_6(&self) -> bool {
        *self == NEXT_A::NEXT_6
    }
    #[doc = "Checks if the value of the field is `NEXT_7`"]
    #[inline(always)]
    pub fn is_next_7(&self) -> bool {
        *self == NEXT_A::NEXT_7
    }
    #[doc = "Checks if the value of the field is `NEXT_8`"]
    #[inline(always)]
    pub fn is_next_8(&self) -> bool {
        *self == NEXT_A::NEXT_8
    }
    #[doc = "Checks if the value of the field is `NEXT_9`"]
    #[inline(always)]
    pub fn is_next_9(&self) -> bool {
        *self == NEXT_A::NEXT_9
    }
    #[doc = "Checks if the value of the field is `NEXT_15`"]
    #[inline(always)]
    pub fn is_next_15(&self) -> bool {
        *self == NEXT_A::NEXT_15
    }
}
#[doc = "Write proxy for field `NEXT`"]
pub struct NEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEXT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    #[inline(always)]
    pub fn next_0(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_0)
    }
    #[doc = "Select CMD1 command buffer register as next command."]
    #[inline(always)]
    pub fn next_1(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_1)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_2(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_2)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_3(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_3)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_4(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_4)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_5(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_5)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_6(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_6)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_7(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_7)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_8(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_8)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline(always)]
    pub fn next_9(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_9)
    }
    #[doc = "Select CMD15 command buffer register as next command."]
    #[inline(always)]
    pub fn next_15(self) -> &'a mut W {
        self.variant(NEXT_A::NEXT_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Wait for trigger assertion before execution."]
    #[inline(always)]
    pub fn wait_trig(&self) -> WAIT_TRIG_R {
        WAIT_TRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loop with Increment"]
    #[inline(always)]
    pub fn lwi(&self) -> LWI_R {
        LWI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Sample Time Select"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Hardware Average Select"]
    #[inline(always)]
    pub fn avgs(&self) -> AVGS_R {
        AVGS_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - Loop Count Select"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Next Command Select"]
    #[inline(always)]
    pub fn next(&self) -> NEXT_R {
        NEXT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Wait for trigger assertion before execution."]
    #[inline(always)]
    pub fn wait_trig(&mut self) -> WAIT_TRIG_W {
        WAIT_TRIG_W { w: self }
    }
    #[doc = "Bit 7 - Loop with Increment"]
    #[inline(always)]
    pub fn lwi(&mut self) -> LWI_W {
        LWI_W { w: self }
    }
    #[doc = "Bits 8:10 - Sample Time Select"]
    #[inline(always)]
    pub fn sts(&mut self) -> STS_W {
        STS_W { w: self }
    }
    #[doc = "Bits 12:14 - Hardware Average Select"]
    #[inline(always)]
    pub fn avgs(&mut self) -> AVGS_W {
        AVGS_W { w: self }
    }
    #[doc = "Bits 16:19 - Loop Count Select"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LOOP_W {
        LOOP_W { w: self }
    }
    #[doc = "Bits 24:27 - Next Command Select"]
    #[inline(always)]
    pub fn next(&mut self) -> NEXT_W {
        NEXT_W { w: self }
    }
}
