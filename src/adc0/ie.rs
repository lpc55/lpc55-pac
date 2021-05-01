#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IE_SPEC>> for R {
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl core::convert::From<crate::W<IE_SPEC>> for W {
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO 0 Watermark Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMIE0_A {
    #[doc = "0: FIFO 0 watermark interrupts are not enabled."]
    FWMIE0_0 = 0,
    #[doc = "1: FIFO 0 watermark interrupts are enabled."]
    FWMIE0_1 = 1,
}
impl From<FWMIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FWMIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWMIE0` reader - FIFO 0 Watermark Interrupt Enable"]
pub struct FWMIE0_R(crate::FieldReader<bool, FWMIE0_A>);
impl FWMIE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWMIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMIE0_A {
        match self.bits {
            false => FWMIE0_A::FWMIE0_0,
            true => FWMIE0_A::FWMIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMIE0_0`"]
    #[inline(always)]
    pub fn is_fwmie0_0(&self) -> bool {
        **self == FWMIE0_A::FWMIE0_0
    }
    #[doc = "Checks if the value of the field is `FWMIE0_1`"]
    #[inline(always)]
    pub fn is_fwmie0_1(&self) -> bool {
        **self == FWMIE0_A::FWMIE0_1
    }
}
impl core::ops::Deref for FWMIE0_R {
    type Target = crate::FieldReader<bool, FWMIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWMIE0` writer - FIFO 0 Watermark Interrupt Enable"]
pub struct FWMIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FWMIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWMIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn fwmie0_0(self) -> &'a mut W {
        self.variant(FWMIE0_A::FWMIE0_0)
    }
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn fwmie0_1(self) -> &'a mut W {
        self.variant(FWMIE0_A::FWMIE0_1)
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
#[doc = "Result FIFO 0 Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFIE0_A {
    #[doc = "0: FIFO 0 overflow interrupts are not enabled."]
    FOFIE0_0 = 0,
    #[doc = "1: FIFO 0 overflow interrupts are enabled."]
    FOFIE0_1 = 1,
}
impl From<FOFIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FOFIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOFIE0` reader - Result FIFO 0 Overflow Interrupt Enable"]
pub struct FOFIE0_R(crate::FieldReader<bool, FOFIE0_A>);
impl FOFIE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOFIE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFIE0_A {
        match self.bits {
            false => FOFIE0_A::FOFIE0_0,
            true => FOFIE0_A::FOFIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFIE0_0`"]
    #[inline(always)]
    pub fn is_fofie0_0(&self) -> bool {
        **self == FOFIE0_A::FOFIE0_0
    }
    #[doc = "Checks if the value of the field is `FOFIE0_1`"]
    #[inline(always)]
    pub fn is_fofie0_1(&self) -> bool {
        **self == FOFIE0_A::FOFIE0_1
    }
}
impl core::ops::Deref for FOFIE0_R {
    type Target = crate::FieldReader<bool, FOFIE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOFIE0` writer - Result FIFO 0 Overflow Interrupt Enable"]
pub struct FOFIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FOFIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOFIE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    #[inline(always)]
    pub fn fofie0_0(self) -> &'a mut W {
        self.variant(FOFIE0_A::FOFIE0_0)
    }
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    #[inline(always)]
    pub fn fofie0_1(self) -> &'a mut W {
        self.variant(FOFIE0_A::FOFIE0_1)
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
#[doc = "FIFO1 Watermark Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMIE1_A {
    #[doc = "0: FIFO1 watermark interrupts are not enabled."]
    FWMIE1_0 = 0,
    #[doc = "1: FIFO1 watermark interrupts are enabled."]
    FWMIE1_1 = 1,
}
impl From<FWMIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FWMIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWMIE1` reader - FIFO1 Watermark Interrupt Enable"]
pub struct FWMIE1_R(crate::FieldReader<bool, FWMIE1_A>);
impl FWMIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWMIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMIE1_A {
        match self.bits {
            false => FWMIE1_A::FWMIE1_0,
            true => FWMIE1_A::FWMIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMIE1_0`"]
    #[inline(always)]
    pub fn is_fwmie1_0(&self) -> bool {
        **self == FWMIE1_A::FWMIE1_0
    }
    #[doc = "Checks if the value of the field is `FWMIE1_1`"]
    #[inline(always)]
    pub fn is_fwmie1_1(&self) -> bool {
        **self == FWMIE1_A::FWMIE1_1
    }
}
impl core::ops::Deref for FWMIE1_R {
    type Target = crate::FieldReader<bool, FWMIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWMIE1` writer - FIFO1 Watermark Interrupt Enable"]
pub struct FWMIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FWMIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWMIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn fwmie1_0(self) -> &'a mut W {
        self.variant(FWMIE1_A::FWMIE1_0)
    }
    #[doc = "FIFO1 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn fwmie1_1(self) -> &'a mut W {
        self.variant(FWMIE1_A::FWMIE1_1)
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
#[doc = "Result FIFO1 Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFIE1_A {
    #[doc = "0: No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_0 = 0,
    #[doc = "1: At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_1 = 1,
}
impl From<FOFIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FOFIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOFIE1` reader - Result FIFO1 Overflow Interrupt Enable"]
pub struct FOFIE1_R(crate::FieldReader<bool, FOFIE1_A>);
impl FOFIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOFIE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFIE1_A {
        match self.bits {
            false => FOFIE1_A::FOFIE1_0,
            true => FOFIE1_A::FOFIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFIE1_0`"]
    #[inline(always)]
    pub fn is_fofie1_0(&self) -> bool {
        **self == FOFIE1_A::FOFIE1_0
    }
    #[doc = "Checks if the value of the field is `FOFIE1_1`"]
    #[inline(always)]
    pub fn is_fofie1_1(&self) -> bool {
        **self == FOFIE1_A::FOFIE1_1
    }
}
impl core::ops::Deref for FOFIE1_R {
    type Target = crate::FieldReader<bool, FOFIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FOFIE1` writer - Result FIFO1 Overflow Interrupt Enable"]
pub struct FOFIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FOFIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOFIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fofie1_0(self) -> &'a mut W {
        self.variant(FOFIE1_A::FOFIE1_0)
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fofie1_1(self) -> &'a mut W {
        self.variant(FOFIE1_A::FOFIE1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Trigger Exception Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEXC_IE_A {
    #[doc = "0: Trigger exception interrupts are disabled."]
    TEXC_IE_0 = 0,
    #[doc = "1: Trigger exception interrupts are enabled."]
    TEXC_IE_1 = 1,
}
impl From<TEXC_IE_A> for bool {
    #[inline(always)]
    fn from(variant: TEXC_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEXC_IE` reader - Trigger Exception Interrupt Enable"]
pub struct TEXC_IE_R(crate::FieldReader<bool, TEXC_IE_A>);
impl TEXC_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEXC_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEXC_IE_A {
        match self.bits {
            false => TEXC_IE_A::TEXC_IE_0,
            true => TEXC_IE_A::TEXC_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEXC_IE_0`"]
    #[inline(always)]
    pub fn is_texc_ie_0(&self) -> bool {
        **self == TEXC_IE_A::TEXC_IE_0
    }
    #[doc = "Checks if the value of the field is `TEXC_IE_1`"]
    #[inline(always)]
    pub fn is_texc_ie_1(&self) -> bool {
        **self == TEXC_IE_A::TEXC_IE_1
    }
}
impl core::ops::Deref for TEXC_IE_R {
    type Target = crate::FieldReader<bool, TEXC_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEXC_IE` writer - Trigger Exception Interrupt Enable"]
pub struct TEXC_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEXC_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEXC_IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger exception interrupts are disabled."]
    #[inline(always)]
    pub fn texc_ie_0(self) -> &'a mut W {
        self.variant(TEXC_IE_A::TEXC_IE_0)
    }
    #[doc = "Trigger exception interrupts are enabled."]
    #[inline(always)]
    pub fn texc_ie_1(self) -> &'a mut W {
        self.variant(TEXC_IE_A::TEXC_IE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Trigger Completion Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TCOMP_IE_A {
    #[doc = "0: Trigger completion interrupts are disabled."]
    TCOMP_IE_0 = 0,
    #[doc = "1: Trigger completion interrupts are enabled for trigger source 0 only."]
    TCOMP_IE_1 = 1,
    #[doc = "2: Trigger completion interrupts are enabled for trigger source 1 only."]
    TCOMP_IE_2 = 2,
    #[doc = "3: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_3 = 3,
    #[doc = "4: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_4 = 4,
    #[doc = "5: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_5 = 5,
    #[doc = "6: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_6 = 6,
    #[doc = "7: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_7 = 7,
    #[doc = "8: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_8 = 8,
    #[doc = "9: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_9 = 9,
    #[doc = "65535: Trigger completion interrupts are enabled for every trigger source."]
    TCOMP_IE_65535 = 65535,
}
impl From<TCOMP_IE_A> for u16 {
    #[inline(always)]
    fn from(variant: TCOMP_IE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCOMP_IE` reader - Trigger Completion Interrupt Enable"]
pub struct TCOMP_IE_R(crate::FieldReader<u16, TCOMP_IE_A>);
impl TCOMP_IE_R {
    pub(crate) fn new(bits: u16) -> Self {
        TCOMP_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCOMP_IE_A> {
        match self.bits {
            0 => Some(TCOMP_IE_A::TCOMP_IE_0),
            1 => Some(TCOMP_IE_A::TCOMP_IE_1),
            2 => Some(TCOMP_IE_A::TCOMP_IE_2),
            3 => Some(TCOMP_IE_A::TCOMP_IE_3),
            4 => Some(TCOMP_IE_A::TCOMP_IE_4),
            5 => Some(TCOMP_IE_A::TCOMP_IE_5),
            6 => Some(TCOMP_IE_A::TCOMP_IE_6),
            7 => Some(TCOMP_IE_A::TCOMP_IE_7),
            8 => Some(TCOMP_IE_A::TCOMP_IE_8),
            9 => Some(TCOMP_IE_A::TCOMP_IE_9),
            65535 => Some(TCOMP_IE_A::TCOMP_IE_65535),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_0`"]
    #[inline(always)]
    pub fn is_tcomp_ie_0(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_0
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_1`"]
    #[inline(always)]
    pub fn is_tcomp_ie_1(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_1
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_2`"]
    #[inline(always)]
    pub fn is_tcomp_ie_2(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_2
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_3`"]
    #[inline(always)]
    pub fn is_tcomp_ie_3(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_3
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_4`"]
    #[inline(always)]
    pub fn is_tcomp_ie_4(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_4
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_5`"]
    #[inline(always)]
    pub fn is_tcomp_ie_5(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_5
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_6`"]
    #[inline(always)]
    pub fn is_tcomp_ie_6(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_6
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_7`"]
    #[inline(always)]
    pub fn is_tcomp_ie_7(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_7
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_8`"]
    #[inline(always)]
    pub fn is_tcomp_ie_8(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_8
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_9`"]
    #[inline(always)]
    pub fn is_tcomp_ie_9(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_9
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_65535`"]
    #[inline(always)]
    pub fn is_tcomp_ie_65535(&self) -> bool {
        **self == TCOMP_IE_A::TCOMP_IE_65535
    }
}
impl core::ops::Deref for TCOMP_IE_R {
    type Target = crate::FieldReader<u16, TCOMP_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCOMP_IE` writer - Trigger Completion Interrupt Enable"]
pub struct TCOMP_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCOMP_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCOMP_IE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trigger completion interrupts are disabled."]
    #[inline(always)]
    pub fn tcomp_ie_0(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_0)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    #[inline(always)]
    pub fn tcomp_ie_1(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_1)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    #[inline(always)]
    pub fn tcomp_ie_2(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_2)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_3(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_3)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_4(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_4)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_5(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_5)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_6(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_6)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_7(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_7)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_8(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_8)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_9(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_9)
    }
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    #[inline(always)]
    pub fn tcomp_ie_65535(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_65535)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie0(&self) -> FWMIE0_R {
        FWMIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie0(&self) -> FOFIE0_R {
        FOFIE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie1(&self) -> FWMIE1_R {
        FWMIE1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie1(&self) -> FOFIE1_R {
        FOFIE1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline(always)]
    pub fn texc_ie(&self) -> TEXC_IE_R {
        TEXC_IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline(always)]
    pub fn tcomp_ie(&self) -> TCOMP_IE_R {
        TCOMP_IE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie0(&mut self) -> FWMIE0_W {
        FWMIE0_W { w: self }
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie0(&mut self) -> FOFIE0_W {
        FOFIE0_W { w: self }
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie1(&mut self) -> FWMIE1_W {
        FWMIE1_W { w: self }
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie1(&mut self) -> FOFIE1_W {
        FOFIE1_W { w: self }
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline(always)]
    pub fn texc_ie(&mut self) -> TEXC_IE_W {
        TEXC_IE_W { w: self }
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline(always)]
    pub fn tcomp_ie(&mut self) -> TCOMP_IE_W {
        TCOMP_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
