#[doc = "Register `TCTRL[%s]` reader"]
pub struct R(crate::R<TCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TCTRL_SPEC>> for R {
    fn from(reader: crate::R<TCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCTRL[%s]` writer"]
pub struct W(crate::W<TCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCTRL_SPEC>;
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
impl core::convert::From<crate::W<TCTRL_SPEC>> for W {
    fn from(writer: crate::W<TCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTEN_A {
    #[doc = "0: Hardware trigger source disabled"]
    HTEN_0 = 0,
    #[doc = "1: Hardware trigger source enabled"]
    HTEN_1 = 1,
}
impl From<HTEN_A> for bool {
    #[inline(always)]
    fn from(variant: HTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTEN` reader - Trigger enable"]
pub struct HTEN_R(crate::FieldReader<bool, HTEN_A>);
impl HTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTEN_A {
        match self.bits {
            false => HTEN_A::HTEN_0,
            true => HTEN_A::HTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HTEN_0`"]
    #[inline(always)]
    pub fn is_hten_0(&self) -> bool {
        **self == HTEN_A::HTEN_0
    }
    #[doc = "Checks if the value of the field is `HTEN_1`"]
    #[inline(always)]
    pub fn is_hten_1(&self) -> bool {
        **self == HTEN_A::HTEN_1
    }
}
impl core::ops::Deref for HTEN_R {
    type Target = crate::FieldReader<bool, HTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTEN` writer - Trigger enable"]
pub struct HTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hardware trigger source disabled"]
    #[inline(always)]
    pub fn hten_0(self) -> &'a mut W {
        self.variant(HTEN_A::HTEN_0)
    }
    #[doc = "Hardware trigger source enabled"]
    #[inline(always)]
    pub fn hten_1(self) -> &'a mut W {
        self.variant(HTEN_A::HTEN_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "SAR Result Destination For Channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_SEL_A_A {
    #[doc = "0: Result written to FIFO 0"]
    FIFO_SEL_A_0 = 0,
    #[doc = "1: Result written to FIFO 1"]
    FIFO_SEL_A_1 = 1,
}
impl From<FIFO_SEL_A_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_SEL_A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_SEL_A` reader - SAR Result Destination For Channel A"]
pub struct FIFO_SEL_A_R(crate::FieldReader<bool, FIFO_SEL_A_A>);
impl FIFO_SEL_A_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_SEL_A_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_SEL_A_A {
        match self.bits {
            false => FIFO_SEL_A_A::FIFO_SEL_A_0,
            true => FIFO_SEL_A_A::FIFO_SEL_A_1,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_SEL_A_0`"]
    #[inline(always)]
    pub fn is_fifo_sel_a_0(&self) -> bool {
        **self == FIFO_SEL_A_A::FIFO_SEL_A_0
    }
    #[doc = "Checks if the value of the field is `FIFO_SEL_A_1`"]
    #[inline(always)]
    pub fn is_fifo_sel_a_1(&self) -> bool {
        **self == FIFO_SEL_A_A::FIFO_SEL_A_1
    }
}
impl core::ops::Deref for FIFO_SEL_A_R {
    type Target = crate::FieldReader<bool, FIFO_SEL_A_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_SEL_A` writer - SAR Result Destination For Channel A"]
pub struct FIFO_SEL_A_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_SEL_A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_SEL_A_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Result written to FIFO 0"]
    #[inline(always)]
    pub fn fifo_sel_a_0(self) -> &'a mut W {
        self.variant(FIFO_SEL_A_A::FIFO_SEL_A_0)
    }
    #[doc = "Result written to FIFO 1"]
    #[inline(always)]
    pub fn fifo_sel_a_1(self) -> &'a mut W {
        self.variant(FIFO_SEL_A_A::FIFO_SEL_A_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "SAR Result Destination For Channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_SEL_B_A {
    #[doc = "0: Result written to FIFO 0"]
    FIFO_SEL_B_0 = 0,
    #[doc = "1: Result written to FIFO 1"]
    FIFO_SEL_B_1 = 1,
}
impl From<FIFO_SEL_B_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_SEL_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_SEL_B` reader - SAR Result Destination For Channel B"]
pub struct FIFO_SEL_B_R(crate::FieldReader<bool, FIFO_SEL_B_A>);
impl FIFO_SEL_B_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_SEL_B_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_SEL_B_A {
        match self.bits {
            false => FIFO_SEL_B_A::FIFO_SEL_B_0,
            true => FIFO_SEL_B_A::FIFO_SEL_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_SEL_B_0`"]
    #[inline(always)]
    pub fn is_fifo_sel_b_0(&self) -> bool {
        **self == FIFO_SEL_B_A::FIFO_SEL_B_0
    }
    #[doc = "Checks if the value of the field is `FIFO_SEL_B_1`"]
    #[inline(always)]
    pub fn is_fifo_sel_b_1(&self) -> bool {
        **self == FIFO_SEL_B_A::FIFO_SEL_B_1
    }
}
impl core::ops::Deref for FIFO_SEL_B_R {
    type Target = crate::FieldReader<bool, FIFO_SEL_B_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_SEL_B` writer - SAR Result Destination For Channel B"]
pub struct FIFO_SEL_B_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_SEL_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_SEL_B_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Result written to FIFO 0"]
    #[inline(always)]
    pub fn fifo_sel_b_0(self) -> &'a mut W {
        self.variant(FIFO_SEL_B_A::FIFO_SEL_B_0)
    }
    #[doc = "Result written to FIFO 1"]
    #[inline(always)]
    pub fn fifo_sel_b_1(self) -> &'a mut W {
        self.variant(FIFO_SEL_B_A::FIFO_SEL_B_1)
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
#[doc = "Trigger priority setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TPRI_A {
    #[doc = "0: Set to highest priority, Level 1"]
    TPRI_0 = 0,
    #[doc = "1: Set to corresponding priority level"]
    TPRI_1 = 1,
    #[doc = "2: Set to corresponding priority level"]
    TPRI_2 = 2,
    #[doc = "3: Set to corresponding priority level"]
    TPRI_3 = 3,
    #[doc = "4: Set to corresponding priority level"]
    TPRI_4 = 4,
    #[doc = "5: Set to corresponding priority level"]
    TPRI_5 = 5,
    #[doc = "6: Set to corresponding priority level"]
    TPRI_6 = 6,
    #[doc = "7: Set to corresponding priority level"]
    TPRI_7 = 7,
    #[doc = "8: Set to corresponding priority level"]
    TPRI_8 = 8,
    #[doc = "9: Set to corresponding priority level"]
    TPRI_9 = 9,
    #[doc = "15: Set to lowest priority, Level 16"]
    TPRI_15 = 15,
}
impl From<TPRI_A> for u8 {
    #[inline(always)]
    fn from(variant: TPRI_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TPRI` reader - Trigger priority setting"]
pub struct TPRI_R(crate::FieldReader<u8, TPRI_A>);
impl TPRI_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPRI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPRI_A> {
        match self.bits {
            0 => Some(TPRI_A::TPRI_0),
            1 => Some(TPRI_A::TPRI_1),
            2 => Some(TPRI_A::TPRI_2),
            3 => Some(TPRI_A::TPRI_3),
            4 => Some(TPRI_A::TPRI_4),
            5 => Some(TPRI_A::TPRI_5),
            6 => Some(TPRI_A::TPRI_6),
            7 => Some(TPRI_A::TPRI_7),
            8 => Some(TPRI_A::TPRI_8),
            9 => Some(TPRI_A::TPRI_9),
            15 => Some(TPRI_A::TPRI_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TPRI_0`"]
    #[inline(always)]
    pub fn is_tpri_0(&self) -> bool {
        **self == TPRI_A::TPRI_0
    }
    #[doc = "Checks if the value of the field is `TPRI_1`"]
    #[inline(always)]
    pub fn is_tpri_1(&self) -> bool {
        **self == TPRI_A::TPRI_1
    }
    #[doc = "Checks if the value of the field is `TPRI_2`"]
    #[inline(always)]
    pub fn is_tpri_2(&self) -> bool {
        **self == TPRI_A::TPRI_2
    }
    #[doc = "Checks if the value of the field is `TPRI_3`"]
    #[inline(always)]
    pub fn is_tpri_3(&self) -> bool {
        **self == TPRI_A::TPRI_3
    }
    #[doc = "Checks if the value of the field is `TPRI_4`"]
    #[inline(always)]
    pub fn is_tpri_4(&self) -> bool {
        **self == TPRI_A::TPRI_4
    }
    #[doc = "Checks if the value of the field is `TPRI_5`"]
    #[inline(always)]
    pub fn is_tpri_5(&self) -> bool {
        **self == TPRI_A::TPRI_5
    }
    #[doc = "Checks if the value of the field is `TPRI_6`"]
    #[inline(always)]
    pub fn is_tpri_6(&self) -> bool {
        **self == TPRI_A::TPRI_6
    }
    #[doc = "Checks if the value of the field is `TPRI_7`"]
    #[inline(always)]
    pub fn is_tpri_7(&self) -> bool {
        **self == TPRI_A::TPRI_7
    }
    #[doc = "Checks if the value of the field is `TPRI_8`"]
    #[inline(always)]
    pub fn is_tpri_8(&self) -> bool {
        **self == TPRI_A::TPRI_8
    }
    #[doc = "Checks if the value of the field is `TPRI_9`"]
    #[inline(always)]
    pub fn is_tpri_9(&self) -> bool {
        **self == TPRI_A::TPRI_9
    }
    #[doc = "Checks if the value of the field is `TPRI_15`"]
    #[inline(always)]
    pub fn is_tpri_15(&self) -> bool {
        **self == TPRI_A::TPRI_15
    }
}
impl core::ops::Deref for TPRI_R {
    type Target = crate::FieldReader<u8, TPRI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPRI` writer - Trigger priority setting"]
pub struct TPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> TPRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPRI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set to highest priority, Level 1"]
    #[inline(always)]
    pub fn tpri_0(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_0)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_1(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_1)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_2(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_2)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_3(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_3)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_4(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_4)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_5(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_5)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_6(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_6)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_7(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_7)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_8(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_8)
    }
    #[doc = "Set to corresponding priority level"]
    #[inline(always)]
    pub fn tpri_9(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_9)
    }
    #[doc = "Set to lowest priority, Level 16"]
    #[inline(always)]
    pub fn tpri_15(self) -> &'a mut W {
        self.variant(TPRI_A::TPRI_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RSYNC` reader - Trigger Resync"]
pub struct RSYNC_R(crate::FieldReader<bool, bool>);
impl RSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSYNC` writer - Trigger Resync"]
pub struct RSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TDLY` reader - Trigger delay select"]
pub struct TDLY_R(crate::FieldReader<u8, u8>);
impl TDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDLY` writer - Trigger delay select"]
pub struct TDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> TDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Trigger command select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCMD_A {
    #[doc = "0: Not a valid selection from the command buffer. Trigger event is ignored."]
    TCMD_0 = 0,
    #[doc = "1: CMD1 is executed"]
    TCMD_1 = 1,
    #[doc = "2: Corresponding CMD is executed"]
    TCMD_2 = 2,
    #[doc = "3: Corresponding CMD is executed"]
    TCMD_3 = 3,
    #[doc = "4: Corresponding CMD is executed"]
    TCMD_4 = 4,
    #[doc = "5: Corresponding CMD is executed"]
    TCMD_5 = 5,
    #[doc = "6: Corresponding CMD is executed"]
    TCMD_6 = 6,
    #[doc = "7: Corresponding CMD is executed"]
    TCMD_7 = 7,
    #[doc = "8: Corresponding CMD is executed"]
    TCMD_8 = 8,
    #[doc = "9: Corresponding CMD is executed"]
    TCMD_9 = 9,
    #[doc = "15: CMD15 is executed"]
    TCMD_15 = 15,
}
impl From<TCMD_A> for u8 {
    #[inline(always)]
    fn from(variant: TCMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCMD` reader - Trigger command select"]
pub struct TCMD_R(crate::FieldReader<u8, TCMD_A>);
impl TCMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCMD_A> {
        match self.bits {
            0 => Some(TCMD_A::TCMD_0),
            1 => Some(TCMD_A::TCMD_1),
            2 => Some(TCMD_A::TCMD_2),
            3 => Some(TCMD_A::TCMD_3),
            4 => Some(TCMD_A::TCMD_4),
            5 => Some(TCMD_A::TCMD_5),
            6 => Some(TCMD_A::TCMD_6),
            7 => Some(TCMD_A::TCMD_7),
            8 => Some(TCMD_A::TCMD_8),
            9 => Some(TCMD_A::TCMD_9),
            15 => Some(TCMD_A::TCMD_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCMD_0`"]
    #[inline(always)]
    pub fn is_tcmd_0(&self) -> bool {
        **self == TCMD_A::TCMD_0
    }
    #[doc = "Checks if the value of the field is `TCMD_1`"]
    #[inline(always)]
    pub fn is_tcmd_1(&self) -> bool {
        **self == TCMD_A::TCMD_1
    }
    #[doc = "Checks if the value of the field is `TCMD_2`"]
    #[inline(always)]
    pub fn is_tcmd_2(&self) -> bool {
        **self == TCMD_A::TCMD_2
    }
    #[doc = "Checks if the value of the field is `TCMD_3`"]
    #[inline(always)]
    pub fn is_tcmd_3(&self) -> bool {
        **self == TCMD_A::TCMD_3
    }
    #[doc = "Checks if the value of the field is `TCMD_4`"]
    #[inline(always)]
    pub fn is_tcmd_4(&self) -> bool {
        **self == TCMD_A::TCMD_4
    }
    #[doc = "Checks if the value of the field is `TCMD_5`"]
    #[inline(always)]
    pub fn is_tcmd_5(&self) -> bool {
        **self == TCMD_A::TCMD_5
    }
    #[doc = "Checks if the value of the field is `TCMD_6`"]
    #[inline(always)]
    pub fn is_tcmd_6(&self) -> bool {
        **self == TCMD_A::TCMD_6
    }
    #[doc = "Checks if the value of the field is `TCMD_7`"]
    #[inline(always)]
    pub fn is_tcmd_7(&self) -> bool {
        **self == TCMD_A::TCMD_7
    }
    #[doc = "Checks if the value of the field is `TCMD_8`"]
    #[inline(always)]
    pub fn is_tcmd_8(&self) -> bool {
        **self == TCMD_A::TCMD_8
    }
    #[doc = "Checks if the value of the field is `TCMD_9`"]
    #[inline(always)]
    pub fn is_tcmd_9(&self) -> bool {
        **self == TCMD_A::TCMD_9
    }
    #[doc = "Checks if the value of the field is `TCMD_15`"]
    #[inline(always)]
    pub fn is_tcmd_15(&self) -> bool {
        **self == TCMD_A::TCMD_15
    }
}
impl core::ops::Deref for TCMD_R {
    type Target = crate::FieldReader<u8, TCMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCMD` writer - Trigger command select"]
pub struct TCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    #[inline(always)]
    pub fn tcmd_0(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_0)
    }
    #[doc = "CMD1 is executed"]
    #[inline(always)]
    pub fn tcmd_1(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_1)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_2(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_2)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_3(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_3)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_4(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_4)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_5(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_5)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_6(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_6)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_7(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_7)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_8(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_8)
    }
    #[doc = "Corresponding CMD is executed"]
    #[inline(always)]
    pub fn tcmd_9(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_9)
    }
    #[doc = "CMD15 is executed"]
    #[inline(always)]
    pub fn tcmd_15(self) -> &'a mut W {
        self.variant(TCMD_A::TCMD_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Trigger enable"]
    #[inline(always)]
    pub fn hten(&self) -> HTEN_R {
        HTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SAR Result Destination For Channel A"]
    #[inline(always)]
    pub fn fifo_sel_a(&self) -> FIFO_SEL_A_R {
        FIFO_SEL_A_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SAR Result Destination For Channel B"]
    #[inline(always)]
    pub fn fifo_sel_b(&self) -> FIFO_SEL_B_R {
        FIFO_SEL_B_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Trigger priority setting"]
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Trigger Resync"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Trigger delay select"]
    #[inline(always)]
    pub fn tdly(&self) -> TDLY_R {
        TDLY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Trigger command select"]
    #[inline(always)]
    pub fn tcmd(&self) -> TCMD_R {
        TCMD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger enable"]
    #[inline(always)]
    pub fn hten(&mut self) -> HTEN_W {
        HTEN_W { w: self }
    }
    #[doc = "Bit 1 - SAR Result Destination For Channel A"]
    #[inline(always)]
    pub fn fifo_sel_a(&mut self) -> FIFO_SEL_A_W {
        FIFO_SEL_A_W { w: self }
    }
    #[doc = "Bit 2 - SAR Result Destination For Channel B"]
    #[inline(always)]
    pub fn fifo_sel_b(&mut self) -> FIFO_SEL_B_W {
        FIFO_SEL_B_W { w: self }
    }
    #[doc = "Bits 8:11 - Trigger priority setting"]
    #[inline(always)]
    pub fn tpri(&mut self) -> TPRI_W {
        TPRI_W { w: self }
    }
    #[doc = "Bit 15 - Trigger Resync"]
    #[inline(always)]
    pub fn rsync(&mut self) -> RSYNC_W {
        RSYNC_W { w: self }
    }
    #[doc = "Bits 16:19 - Trigger delay select"]
    #[inline(always)]
    pub fn tdly(&mut self) -> TDLY_W {
        TDLY_W { w: self }
    }
    #[doc = "Bits 24:27 - Trigger command select"]
    #[inline(always)]
    pub fn tcmd(&mut self) -> TCMD_W {
        TCMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl](index.html) module"]
pub struct TCTRL_SPEC;
impl crate::RegisterSpec for TCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tctrl::R](R) reader structure"]
impl crate::Readable for TCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tctrl::W](W) writer structure"]
impl crate::Writable for TCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCTRL[%s]
to value 0"]
impl crate::Resettable for TCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
