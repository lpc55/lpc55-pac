#[doc = "Register `CMDH2` reader"]
pub struct R(crate::R<CMDH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CMDH2_SPEC>> for R {
    fn from(reader: crate::R<CMDH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDH2` writer"]
pub struct W(crate::W<CMDH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<CMDH2_SPEC>> for W {
    fn from(writer: crate::W<CMDH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Compare Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPEN_A {
    #[doc = "0: Compare disabled."]
    CMPEN_0 = 0,
    #[doc = "2: Compare enabled. Store on true."]
    CMPEN_2 = 2,
    #[doc = "3: Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 3,
}
impl From<CMPEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPEN` reader - Compare Function Enable"]
pub struct CMPEN_R(crate::FieldReader<u8, CMPEN_A>);
impl CMPEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPEN_A> {
        match self.bits {
            0 => Some(CMPEN_A::CMPEN_0),
            2 => Some(CMPEN_A::CMPEN_2),
            3 => Some(CMPEN_A::CMPEN_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CMPEN_0`"]
    #[inline(always)]
    pub fn is_cmpen_0(&self) -> bool {
        **self == CMPEN_A::CMPEN_0
    }
    #[doc = "Checks if the value of the field is `CMPEN_2`"]
    #[inline(always)]
    pub fn is_cmpen_2(&self) -> bool {
        **self == CMPEN_A::CMPEN_2
    }
    #[doc = "Checks if the value of the field is `CMPEN_3`"]
    #[inline(always)]
    pub fn is_cmpen_3(&self) -> bool {
        **self == CMPEN_A::CMPEN_3
    }
}
impl core::ops::Deref for CMPEN_R {
    type Target = crate::FieldReader<u8, CMPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPEN` writer - Compare Function Enable"]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Compare disabled."]
    #[inline(always)]
    pub fn cmpen_0(self) -> &'a mut W {
        self.variant(CMPEN_A::CMPEN_0)
    }
    #[doc = "Compare enabled. Store on true."]
    #[inline(always)]
    pub fn cmpen_2(self) -> &'a mut W {
        self.variant(CMPEN_A::CMPEN_2)
    }
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    #[inline(always)]
    pub fn cmpen_3(self) -> &'a mut W {
        self.variant(CMPEN_A::CMPEN_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Wait for trigger assertion before execution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_TRIG_A {
    #[doc = "0: This command will be automatically executed."]
    WAIT_TRIG_0 = 0,
    #[doc = "1: The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1 = 1,
}
impl From<WAIT_TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_TRIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAIT_TRIG` reader - Wait for trigger assertion before execution."]
pub struct WAIT_TRIG_R(crate::FieldReader<bool, WAIT_TRIG_A>);
impl WAIT_TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_TRIG_R(crate::FieldReader::new(bits))
    }
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
        **self == WAIT_TRIG_A::WAIT_TRIG_0
    }
    #[doc = "Checks if the value of the field is `WAIT_TRIG_1`"]
    #[inline(always)]
    pub fn is_wait_trig_1(&self) -> bool {
        **self == WAIT_TRIG_A::WAIT_TRIG_1
    }
}
impl core::ops::Deref for WAIT_TRIG_R {
    type Target = crate::FieldReader<bool, WAIT_TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_TRIG` writer - Wait for trigger assertion before execution."]
pub struct WAIT_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAIT_TRIG_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Loop with Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LWI_A {
    #[doc = "0: Auto channel increment disabled"]
    LWI_0 = 0,
    #[doc = "1: Auto channel increment enabled"]
    LWI_1 = 1,
}
impl From<LWI_A> for bool {
    #[inline(always)]
    fn from(variant: LWI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LWI` reader - Loop with Increment"]
pub struct LWI_R(crate::FieldReader<bool, LWI_A>);
impl LWI_R {
    pub(crate) fn new(bits: bool) -> Self {
        LWI_R(crate::FieldReader::new(bits))
    }
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
        **self == LWI_A::LWI_0
    }
    #[doc = "Checks if the value of the field is `LWI_1`"]
    #[inline(always)]
    pub fn is_lwi_1(&self) -> bool {
        **self == LWI_A::LWI_1
    }
}
impl core::ops::Deref for LWI_R {
    type Target = crate::FieldReader<bool, LWI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LWI` writer - Loop with Increment"]
pub struct LWI_W<'a> {
    w: &'a mut W,
}
impl<'a> LWI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LWI_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Sample Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STS_A {
    #[doc = "0: Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0,
    #[doc = "1: 3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 1,
    #[doc = "2: 3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 2,
    #[doc = "3: 3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 3,
    #[doc = "4: 3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 4,
    #[doc = "5: 3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 5,
    #[doc = "6: 3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 6,
    #[doc = "7: 3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 7,
}
impl From<STS_A> for u8 {
    #[inline(always)]
    fn from(variant: STS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STS` reader - Sample Time Select"]
pub struct STS_R(crate::FieldReader<u8, STS_A>);
impl STS_R {
    pub(crate) fn new(bits: u8) -> Self {
        STS_R(crate::FieldReader::new(bits))
    }
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
        **self == STS_A::STS_0
    }
    #[doc = "Checks if the value of the field is `STS_1`"]
    #[inline(always)]
    pub fn is_sts_1(&self) -> bool {
        **self == STS_A::STS_1
    }
    #[doc = "Checks if the value of the field is `STS_2`"]
    #[inline(always)]
    pub fn is_sts_2(&self) -> bool {
        **self == STS_A::STS_2
    }
    #[doc = "Checks if the value of the field is `STS_3`"]
    #[inline(always)]
    pub fn is_sts_3(&self) -> bool {
        **self == STS_A::STS_3
    }
    #[doc = "Checks if the value of the field is `STS_4`"]
    #[inline(always)]
    pub fn is_sts_4(&self) -> bool {
        **self == STS_A::STS_4
    }
    #[doc = "Checks if the value of the field is `STS_5`"]
    #[inline(always)]
    pub fn is_sts_5(&self) -> bool {
        **self == STS_A::STS_5
    }
    #[doc = "Checks if the value of the field is `STS_6`"]
    #[inline(always)]
    pub fn is_sts_6(&self) -> bool {
        **self == STS_A::STS_6
    }
    #[doc = "Checks if the value of the field is `STS_7`"]
    #[inline(always)]
    pub fn is_sts_7(&self) -> bool {
        **self == STS_A::STS_7
    }
}
impl core::ops::Deref for STS_R {
    type Target = crate::FieldReader<u8, STS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STS` writer - Sample Time Select"]
pub struct STS_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STS_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Hardware Average Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AVGS_A {
    #[doc = "0: Single conversion."]
    AVGS_0 = 0,
    #[doc = "1: 2 conversions averaged."]
    AVGS_1 = 1,
    #[doc = "2: 4 conversions averaged."]
    AVGS_2 = 2,
    #[doc = "3: 8 conversions averaged."]
    AVGS_3 = 3,
    #[doc = "4: 16 conversions averaged."]
    AVGS_4 = 4,
    #[doc = "5: 32 conversions averaged."]
    AVGS_5 = 5,
    #[doc = "6: 64 conversions averaged."]
    AVGS_6 = 6,
    #[doc = "7: 128 conversions averaged."]
    AVGS_7 = 7,
}
impl From<AVGS_A> for u8 {
    #[inline(always)]
    fn from(variant: AVGS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AVGS` reader - Hardware Average Select"]
pub struct AVGS_R(crate::FieldReader<u8, AVGS_A>);
impl AVGS_R {
    pub(crate) fn new(bits: u8) -> Self {
        AVGS_R(crate::FieldReader::new(bits))
    }
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
        **self == AVGS_A::AVGS_0
    }
    #[doc = "Checks if the value of the field is `AVGS_1`"]
    #[inline(always)]
    pub fn is_avgs_1(&self) -> bool {
        **self == AVGS_A::AVGS_1
    }
    #[doc = "Checks if the value of the field is `AVGS_2`"]
    #[inline(always)]
    pub fn is_avgs_2(&self) -> bool {
        **self == AVGS_A::AVGS_2
    }
    #[doc = "Checks if the value of the field is `AVGS_3`"]
    #[inline(always)]
    pub fn is_avgs_3(&self) -> bool {
        **self == AVGS_A::AVGS_3
    }
    #[doc = "Checks if the value of the field is `AVGS_4`"]
    #[inline(always)]
    pub fn is_avgs_4(&self) -> bool {
        **self == AVGS_A::AVGS_4
    }
    #[doc = "Checks if the value of the field is `AVGS_5`"]
    #[inline(always)]
    pub fn is_avgs_5(&self) -> bool {
        **self == AVGS_A::AVGS_5
    }
    #[doc = "Checks if the value of the field is `AVGS_6`"]
    #[inline(always)]
    pub fn is_avgs_6(&self) -> bool {
        **self == AVGS_A::AVGS_6
    }
    #[doc = "Checks if the value of the field is `AVGS_7`"]
    #[inline(always)]
    pub fn is_avgs_7(&self) -> bool {
        **self == AVGS_A::AVGS_7
    }
}
impl core::ops::Deref for AVGS_R {
    type Target = crate::FieldReader<u8, AVGS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVGS` writer - Hardware Average Select"]
pub struct AVGS_W<'a> {
    w: &'a mut W,
}
impl<'a> AVGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVGS_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Loop Count Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOOP_A {
    #[doc = "0: Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0,
    #[doc = "1: Loop 1 time. Command executes 2 times."]
    LOOP_1 = 1,
    #[doc = "2: Loop 2 times. Command executes 3 times."]
    LOOP_2 = 2,
    #[doc = "3: Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 3,
    #[doc = "4: Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 4,
    #[doc = "5: Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 5,
    #[doc = "6: Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 6,
    #[doc = "7: Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 7,
    #[doc = "8: Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 8,
    #[doc = "9: Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 9,
    #[doc = "15: Loop 15 times. Command executes 16 times."]
    LOOP_15 = 15,
}
impl From<LOOP_A> for u8 {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOOP` reader - Loop Count Select"]
pub struct LOOP_R(crate::FieldReader<u8, LOOP_A>);
impl LOOP_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOOP_A> {
        match self.bits {
            0 => Some(LOOP_A::LOOP_0),
            1 => Some(LOOP_A::LOOP_1),
            2 => Some(LOOP_A::LOOP_2),
            3 => Some(LOOP_A::LOOP_3),
            4 => Some(LOOP_A::LOOP_4),
            5 => Some(LOOP_A::LOOP_5),
            6 => Some(LOOP_A::LOOP_6),
            7 => Some(LOOP_A::LOOP_7),
            8 => Some(LOOP_A::LOOP_8),
            9 => Some(LOOP_A::LOOP_9),
            15 => Some(LOOP_A::LOOP_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOOP_0`"]
    #[inline(always)]
    pub fn is_loop_0(&self) -> bool {
        **self == LOOP_A::LOOP_0
    }
    #[doc = "Checks if the value of the field is `LOOP_1`"]
    #[inline(always)]
    pub fn is_loop_1(&self) -> bool {
        **self == LOOP_A::LOOP_1
    }
    #[doc = "Checks if the value of the field is `LOOP_2`"]
    #[inline(always)]
    pub fn is_loop_2(&self) -> bool {
        **self == LOOP_A::LOOP_2
    }
    #[doc = "Checks if the value of the field is `LOOP_3`"]
    #[inline(always)]
    pub fn is_loop_3(&self) -> bool {
        **self == LOOP_A::LOOP_3
    }
    #[doc = "Checks if the value of the field is `LOOP_4`"]
    #[inline(always)]
    pub fn is_loop_4(&self) -> bool {
        **self == LOOP_A::LOOP_4
    }
    #[doc = "Checks if the value of the field is `LOOP_5`"]
    #[inline(always)]
    pub fn is_loop_5(&self) -> bool {
        **self == LOOP_A::LOOP_5
    }
    #[doc = "Checks if the value of the field is `LOOP_6`"]
    #[inline(always)]
    pub fn is_loop_6(&self) -> bool {
        **self == LOOP_A::LOOP_6
    }
    #[doc = "Checks if the value of the field is `LOOP_7`"]
    #[inline(always)]
    pub fn is_loop_7(&self) -> bool {
        **self == LOOP_A::LOOP_7
    }
    #[doc = "Checks if the value of the field is `LOOP_8`"]
    #[inline(always)]
    pub fn is_loop_8(&self) -> bool {
        **self == LOOP_A::LOOP_8
    }
    #[doc = "Checks if the value of the field is `LOOP_9`"]
    #[inline(always)]
    pub fn is_loop_9(&self) -> bool {
        **self == LOOP_A::LOOP_9
    }
    #[doc = "Checks if the value of the field is `LOOP_15`"]
    #[inline(always)]
    pub fn is_loop_15(&self) -> bool {
        **self == LOOP_A::LOOP_15
    }
}
impl core::ops::Deref for LOOP_R {
    type Target = crate::FieldReader<u8, LOOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOP` writer - Loop Count Select"]
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
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Next Command Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NEXT_A {
    #[doc = "0: No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0,
    #[doc = "1: Select CMD1 command buffer register as next command."]
    NEXT_1 = 1,
    #[doc = "2: Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 2,
    #[doc = "3: Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 3,
    #[doc = "4: Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 4,
    #[doc = "5: Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 5,
    #[doc = "6: Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 6,
    #[doc = "7: Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 7,
    #[doc = "8: Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 8,
    #[doc = "9: Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 9,
    #[doc = "15: Select CMD15 command buffer register as next command."]
    NEXT_15 = 15,
}
impl From<NEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: NEXT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NEXT` reader - Next Command Select"]
pub struct NEXT_R(crate::FieldReader<u8, NEXT_A>);
impl NEXT_R {
    pub(crate) fn new(bits: u8) -> Self {
        NEXT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NEXT_A> {
        match self.bits {
            0 => Some(NEXT_A::NEXT_0),
            1 => Some(NEXT_A::NEXT_1),
            2 => Some(NEXT_A::NEXT_2),
            3 => Some(NEXT_A::NEXT_3),
            4 => Some(NEXT_A::NEXT_4),
            5 => Some(NEXT_A::NEXT_5),
            6 => Some(NEXT_A::NEXT_6),
            7 => Some(NEXT_A::NEXT_7),
            8 => Some(NEXT_A::NEXT_8),
            9 => Some(NEXT_A::NEXT_9),
            15 => Some(NEXT_A::NEXT_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEXT_0`"]
    #[inline(always)]
    pub fn is_next_0(&self) -> bool {
        **self == NEXT_A::NEXT_0
    }
    #[doc = "Checks if the value of the field is `NEXT_1`"]
    #[inline(always)]
    pub fn is_next_1(&self) -> bool {
        **self == NEXT_A::NEXT_1
    }
    #[doc = "Checks if the value of the field is `NEXT_2`"]
    #[inline(always)]
    pub fn is_next_2(&self) -> bool {
        **self == NEXT_A::NEXT_2
    }
    #[doc = "Checks if the value of the field is `NEXT_3`"]
    #[inline(always)]
    pub fn is_next_3(&self) -> bool {
        **self == NEXT_A::NEXT_3
    }
    #[doc = "Checks if the value of the field is `NEXT_4`"]
    #[inline(always)]
    pub fn is_next_4(&self) -> bool {
        **self == NEXT_A::NEXT_4
    }
    #[doc = "Checks if the value of the field is `NEXT_5`"]
    #[inline(always)]
    pub fn is_next_5(&self) -> bool {
        **self == NEXT_A::NEXT_5
    }
    #[doc = "Checks if the value of the field is `NEXT_6`"]
    #[inline(always)]
    pub fn is_next_6(&self) -> bool {
        **self == NEXT_A::NEXT_6
    }
    #[doc = "Checks if the value of the field is `NEXT_7`"]
    #[inline(always)]
    pub fn is_next_7(&self) -> bool {
        **self == NEXT_A::NEXT_7
    }
    #[doc = "Checks if the value of the field is `NEXT_8`"]
    #[inline(always)]
    pub fn is_next_8(&self) -> bool {
        **self == NEXT_A::NEXT_8
    }
    #[doc = "Checks if the value of the field is `NEXT_9`"]
    #[inline(always)]
    pub fn is_next_9(&self) -> bool {
        **self == NEXT_A::NEXT_9
    }
    #[doc = "Checks if the value of the field is `NEXT_15`"]
    #[inline(always)]
    pub fn is_next_15(&self) -> bool {
        **self == NEXT_A::NEXT_15
    }
}
impl core::ops::Deref for NEXT_R {
    type Target = crate::FieldReader<u8, NEXT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEXT` writer - Next Command Select"]
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
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Compare Function Enable"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 0x03) as u8)
    }
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
    #[doc = "Bits 0:1 - Compare Function Enable"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Command High Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdh2](index.html) module"]
pub struct CMDH2_SPEC;
impl crate::RegisterSpec for CMDH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdh2::R](R) reader structure"]
impl crate::Readable for CMDH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdh2::W](W) writer structure"]
impl crate::Writable for CMDH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDH2 to value 0"]
impl crate::Resettable for CMDH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
