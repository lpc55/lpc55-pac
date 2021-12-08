#[doc = "Register `USB1_VBUS_DETECT` reader"]
pub struct R(crate::R<USB1_VBUS_DETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_VBUS_DETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_VBUS_DETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_VBUS_DETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_VBUS_DETECT` writer"]
pub struct W(crate::W<USB1_VBUS_DETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_VBUS_DETECT_SPEC>;
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
impl From<crate::W<USB1_VBUS_DETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_VBUS_DETECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sets the threshold for the VBUSVALID comparator\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VBUSVALID_THRESH_A {
    #[doc = "0: 4.0V"]
    VALUE0 = 0,
    #[doc = "1: 4.1V"]
    VALUE1 = 1,
    #[doc = "2: 4.2V"]
    VALUE2 = 2,
    #[doc = "3: 4.3V"]
    VALUE3 = 3,
    #[doc = "4: 4.4V(Default)"]
    VALUE4 = 4,
    #[doc = "5: 4.5V"]
    VALUE5 = 5,
    #[doc = "6: 4.6V"]
    VALUE6 = 6,
    #[doc = "7: 4.7V"]
    VALUE7 = 7,
}
impl From<VBUSVALID_THRESH_A> for u8 {
    #[inline(always)]
    fn from(variant: VBUSVALID_THRESH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VBUSVALID_THRESH` reader - Sets the threshold for the VBUSVALID comparator"]
pub struct VBUSVALID_THRESH_R(crate::FieldReader<u8, VBUSVALID_THRESH_A>);
impl VBUSVALID_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VBUSVALID_THRESH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_THRESH_A {
        match self.bits {
            0 => VBUSVALID_THRESH_A::VALUE0,
            1 => VBUSVALID_THRESH_A::VALUE1,
            2 => VBUSVALID_THRESH_A::VALUE2,
            3 => VBUSVALID_THRESH_A::VALUE3,
            4 => VBUSVALID_THRESH_A::VALUE4,
            5 => VBUSVALID_THRESH_A::VALUE5,
            6 => VBUSVALID_THRESH_A::VALUE6,
            7 => VBUSVALID_THRESH_A::VALUE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == VBUSVALID_THRESH_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VBUSVALID_THRESH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VBUSVALID_THRESH_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == VBUSVALID_THRESH_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == VBUSVALID_THRESH_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == VBUSVALID_THRESH_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == VBUSVALID_THRESH_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == VBUSVALID_THRESH_A::VALUE7
    }
}
impl core::ops::Deref for VBUSVALID_THRESH_R {
    type Target = crate::FieldReader<u8, VBUSVALID_THRESH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSVALID_THRESH` writer - Sets the threshold for the VBUSVALID comparator"]
pub struct VBUSVALID_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_THRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUSVALID_THRESH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "4.0V"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::VALUE0)
    }
    #[doc = "4.1V"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::VALUE1)
    }
    #[doc = "4.2V"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::VALUE2)
    }
    #[doc = "4.3V"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::VALUE3)
    }
    #[doc = "4.4V(Default)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::VALUE4)
    }
    #[doc = "4.5V"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::VALUE5)
    }
    #[doc = "4.6V"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::VALUE6)
    }
    #[doc = "4.7V"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::VALUE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "VBUS detect signal override enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_OVERRIDE_EN_A {
    #[doc = "0: Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    VALUE0 = 0,
    #[doc = "1: Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VALUE1 = 1,
}
impl From<VBUS_OVERRIDE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_OVERRIDE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_OVERRIDE_EN` reader - VBUS detect signal override enable"]
pub struct VBUS_OVERRIDE_EN_R(crate::FieldReader<bool, VBUS_OVERRIDE_EN_A>);
impl VBUS_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_OVERRIDE_EN_A {
        match self.bits {
            false => VBUS_OVERRIDE_EN_A::VALUE0,
            true => VBUS_OVERRIDE_EN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == VBUS_OVERRIDE_EN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VBUS_OVERRIDE_EN_A::VALUE1
    }
}
impl core::ops::Deref for VBUS_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, VBUS_OVERRIDE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS_OVERRIDE_EN` writer - VBUS detect signal override enable"]
pub struct VBUS_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_OVERRIDE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUS_OVERRIDE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_EN_A::VALUE0)
    }
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_EN_A::VALUE1)
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
#[doc = "Field `SESSEND_OVERRIDE` reader - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub struct SESSEND_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl SESSEND_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SESSEND_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESSEND_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SESSEND_OVERRIDE` writer - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub struct SESSEND_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSEND_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `BVALID_OVERRIDE` reader - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub struct BVALID_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl BVALID_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BVALID_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BVALID_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BVALID_OVERRIDE` writer - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub struct BVALID_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> BVALID_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `AVALID_OVERRIDE` reader - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub struct AVALID_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl AVALID_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AVALID_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVALID_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVALID_OVERRIDE` writer - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub struct AVALID_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALID_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `VBUSVALID_OVERRIDE` reader - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\]
is set to 1'b1"]
pub struct VBUSVALID_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl VBUSVALID_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSVALID_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSVALID_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSVALID_OVERRIDE` writer - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\]
is set to 1'b1"]
pub struct VBUSVALID_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_OVERRIDE_W<'a> {
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
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVALID_SEL_A {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0,
    #[doc = "1: Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VALUE1 = 1,
}
impl From<VBUSVALID_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSVALID_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSVALID_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub struct VBUSVALID_SEL_R(crate::FieldReader<bool, VBUSVALID_SEL_A>);
impl VBUSVALID_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSVALID_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_SEL_A {
        match self.bits {
            false => VBUSVALID_SEL_A::VALUE0,
            true => VBUSVALID_SEL_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == VBUSVALID_SEL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VBUSVALID_SEL_A::VALUE1
    }
}
impl core::ops::Deref for VBUSVALID_SEL_R {
    type Target = crate::FieldReader<bool, VBUSVALID_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSVALID_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub struct VBUSVALID_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUSVALID_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUSVALID_SEL_A::VALUE0)
    }
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUSVALID_SEL_A::VALUE1)
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
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VBUS_SOURCE_SEL_A {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0,
    #[doc = "1: Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE1 = 1,
    #[doc = "2: Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE2 = 2,
}
impl From<VBUS_SOURCE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VBUS_SOURCE_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VBUS_SOURCE_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub struct VBUS_SOURCE_SEL_R(crate::FieldReader<u8, VBUS_SOURCE_SEL_A>);
impl VBUS_SOURCE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VBUS_SOURCE_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VBUS_SOURCE_SEL_A> {
        match self.bits {
            0 => Some(VBUS_SOURCE_SEL_A::VALUE0),
            1 => Some(VBUS_SOURCE_SEL_A::VALUE1),
            2 => Some(VBUS_SOURCE_SEL_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == VBUS_SOURCE_SEL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VBUS_SOURCE_SEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VBUS_SOURCE_SEL_A::VALUE2
    }
}
impl core::ops::Deref for VBUS_SOURCE_SEL_R {
    type Target = crate::FieldReader<u8, VBUS_SOURCE_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUS_SOURCE_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub struct VBUS_SOURCE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_SOURCE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUS_SOURCE_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::VALUE0)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::VALUE1)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SEL_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `ID_OVERRIDE_EN` reader - Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
pub struct ID_OVERRIDE_EN_R(crate::FieldReader<bool, bool>);
impl ID_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ID_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID_OVERRIDE_EN` writer - Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
pub struct ID_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_OVERRIDE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ID_OVERRIDE` reader - ID override value."]
pub struct ID_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl ID_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ID_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID_OVERRIDE` writer - ID override value."]
pub struct ID_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Enable ID override using the pinmuxed value:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_ID_OVERRIDE_EN_A {
    #[doc = "0: Select the Muxed value chosen using ID_OVERRIDE_EN."]
    VALUE0 = 0,
    #[doc = "1: Select the external ID value."]
    VALUE1 = 1,
}
impl From<EXT_ID_OVERRIDE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_ID_OVERRIDE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_ID_OVERRIDE_EN` reader - Enable ID override using the pinmuxed value:"]
pub struct EXT_ID_OVERRIDE_EN_R(crate::FieldReader<bool, EXT_ID_OVERRIDE_EN_A>);
impl EXT_ID_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_ID_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXT_ID_OVERRIDE_EN_A {
        match self.bits {
            false => EXT_ID_OVERRIDE_EN_A::VALUE0,
            true => EXT_ID_OVERRIDE_EN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == EXT_ID_OVERRIDE_EN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXT_ID_OVERRIDE_EN_A::VALUE1
    }
}
impl core::ops::Deref for EXT_ID_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, EXT_ID_OVERRIDE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_ID_OVERRIDE_EN` writer - Enable ID override using the pinmuxed value:"]
pub struct EXT_ID_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_ID_OVERRIDE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXT_ID_OVERRIDE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the Muxed value chosen using ID_OVERRIDE_EN."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(EXT_ID_OVERRIDE_EN_A::VALUE0)
    }
    #[doc = "Select the external ID value."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXT_ID_OVERRIDE_EN_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Enable VBUS override using the pinmuxed value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_VBUS_OVERRIDE_EN_A {
    #[doc = "0: Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    VALUE0 = 0,
    #[doc = "1: Select the external VBUS VALID value."]
    VALUE1 = 1,
}
impl From<EXT_VBUS_OVERRIDE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_VBUS_OVERRIDE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_VBUS_OVERRIDE_EN` reader - Enable VBUS override using the pinmuxed value."]
pub struct EXT_VBUS_OVERRIDE_EN_R(crate::FieldReader<bool, EXT_VBUS_OVERRIDE_EN_A>);
impl EXT_VBUS_OVERRIDE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_VBUS_OVERRIDE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXT_VBUS_OVERRIDE_EN_A {
        match self.bits {
            false => EXT_VBUS_OVERRIDE_EN_A::VALUE0,
            true => EXT_VBUS_OVERRIDE_EN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == EXT_VBUS_OVERRIDE_EN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == EXT_VBUS_OVERRIDE_EN_A::VALUE1
    }
}
impl core::ops::Deref for EXT_VBUS_OVERRIDE_EN_R {
    type Target = crate::FieldReader<bool, EXT_VBUS_OVERRIDE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_VBUS_OVERRIDE_EN` writer - Enable VBUS override using the pinmuxed value."]
pub struct EXT_VBUS_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_VBUS_OVERRIDE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXT_VBUS_OVERRIDE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(EXT_VBUS_OVERRIDE_EN_A::VALUE0)
    }
    #[doc = "Select the external VBUS VALID value."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXT_VBUS_OVERRIDE_EN_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVALID_TO_SESSVALID_A {
    #[doc = "0: Use the VBUS_VALID comparator for VBUS_VALID results"]
    VALUE0 = 0,
    #[doc = "1: Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VALUE1 = 1,
}
impl From<VBUSVALID_TO_SESSVALID_A> for bool {
    #[inline(always)]
    fn from(variant: VBUSVALID_TO_SESSVALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSVALID_TO_SESSVALID` reader - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator"]
pub struct VBUSVALID_TO_SESSVALID_R(crate::FieldReader<bool, VBUSVALID_TO_SESSVALID_A>);
impl VBUSVALID_TO_SESSVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSVALID_TO_SESSVALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_TO_SESSVALID_A {
        match self.bits {
            false => VBUSVALID_TO_SESSVALID_A::VALUE0,
            true => VBUSVALID_TO_SESSVALID_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == VBUSVALID_TO_SESSVALID_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VBUSVALID_TO_SESSVALID_A::VALUE1
    }
}
impl core::ops::Deref for VBUSVALID_TO_SESSVALID_R {
    type Target = crate::FieldReader<bool, VBUSVALID_TO_SESSVALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSVALID_TO_SESSVALID` writer - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator"]
pub struct VBUSVALID_TO_SESSVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_TO_SESSVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUSVALID_TO_SESSVALID_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALID_A::VALUE0)
    }
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALID_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `VBUSVALID_5VDETECT` reader - no description available"]
pub struct VBUSVALID_5VDETECT_R(crate::FieldReader<bool, bool>);
impl VBUSVALID_5VDETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUSVALID_5VDETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSVALID_5VDETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSVALID_5VDETECT` writer - no description available"]
pub struct VBUSVALID_5VDETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_5VDETECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRUP_CMPS_A {
    #[doc = "0: Powers down the VBUS_VALID comparator"]
    VALUE0 = 0,
    #[doc = "7: Enables the VBUS_VALID comparator (default)"]
    VALUE1 = 7,
}
impl From<PWRUP_CMPS_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRUP_CMPS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRUP_CMPS` reader - Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
pub struct PWRUP_CMPS_R(crate::FieldReader<u8, PWRUP_CMPS_A>);
impl PWRUP_CMPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWRUP_CMPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWRUP_CMPS_A> {
        match self.bits {
            0 => Some(PWRUP_CMPS_A::VALUE0),
            7 => Some(PWRUP_CMPS_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == PWRUP_CMPS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PWRUP_CMPS_A::VALUE1
    }
}
impl core::ops::Deref for PWRUP_CMPS_R {
    type Target = crate::FieldReader<u8, PWRUP_CMPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRUP_CMPS` writer - Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
pub struct PWRUP_CMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUP_CMPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRUP_CMPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Powers down the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(PWRUP_CMPS_A::VALUE0)
    }
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PWRUP_CMPS_A::VALUE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCHARGE_VBUS_A {
    #[doc = "0: VBUS discharge resistor is disabled (Default)"]
    VALUE0 = 0,
    #[doc = "1: VBUS discharge resistor is enabled"]
    VALUE1 = 1,
}
impl From<DISCHARGE_VBUS_A> for bool {
    #[inline(always)]
    fn from(variant: DISCHARGE_VBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCHARGE_VBUS` reader - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
pub struct DISCHARGE_VBUS_R(crate::FieldReader<bool, DISCHARGE_VBUS_A>);
impl DISCHARGE_VBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISCHARGE_VBUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCHARGE_VBUS_A {
        match self.bits {
            false => DISCHARGE_VBUS_A::VALUE0,
            true => DISCHARGE_VBUS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == DISCHARGE_VBUS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DISCHARGE_VBUS_A::VALUE1
    }
}
impl core::ops::Deref for DISCHARGE_VBUS_R {
    type Target = crate::FieldReader<bool, DISCHARGE_VBUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCHARGE_VBUS` writer - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
pub struct DISCHARGE_VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCHARGE_VBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCHARGE_VBUS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DISCHARGE_VBUS_A::VALUE0)
    }
    #[doc = "VBUS discharge resistor is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISCHARGE_VBUS_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&self) -> VBUSVALID_THRESH_R {
        VBUSVALID_THRESH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline(always)]
    pub fn vbus_override_en(&self) -> VBUS_OVERRIDE_EN_R {
        VBUS_OVERRIDE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn sessend_override(&self) -> SESSEND_OVERRIDE_R {
        SESSEND_OVERRIDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn bvalid_override(&self) -> BVALID_OVERRIDE_R {
        BVALID_OVERRIDE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn avalid_override(&self) -> AVALID_OVERRIDE_R {
        AVALID_OVERRIDE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\]
is set to 1'b1"]
    #[inline(always)]
    pub fn vbusvalid_override(&self) -> VBUSVALID_OVERRIDE_R {
        VBUSVALID_OVERRIDE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel(&self) -> VBUSVALID_SEL_R {
        VBUSVALID_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel(&self) -> VBUS_SOURCE_SEL_R {
        VBUS_SOURCE_SEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub fn id_override_en(&self) -> ID_OVERRIDE_EN_R {
        ID_OVERRIDE_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ID override value."]
    #[inline(always)]
    pub fn id_override(&self) -> ID_OVERRIDE_R {
        ID_OVERRIDE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable ID override using the pinmuxed value:"]
    #[inline(always)]
    pub fn ext_id_override_en(&self) -> EXT_ID_OVERRIDE_EN_R {
        EXT_ID_OVERRIDE_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable VBUS override using the pinmuxed value."]
    #[inline(always)]
    pub fn ext_vbus_override_en(&self) -> EXT_VBUS_OVERRIDE_EN_R {
        EXT_VBUS_OVERRIDE_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid(&self) -> VBUSVALID_TO_SESSVALID_R {
        VBUSVALID_TO_SESSVALID_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn vbusvalid_5vdetect(&self) -> VBUSVALID_5VDETECT_R {
        VBUSVALID_5VDETECT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub fn pwrup_cmps(&self) -> PWRUP_CMPS_R {
        PWRUP_CMPS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub fn discharge_vbus(&self) -> DISCHARGE_VBUS_R {
        DISCHARGE_VBUS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&mut self) -> VBUSVALID_THRESH_W {
        VBUSVALID_THRESH_W { w: self }
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline(always)]
    pub fn vbus_override_en(&mut self) -> VBUS_OVERRIDE_EN_W {
        VBUS_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 4 - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn sessend_override(&mut self) -> SESSEND_OVERRIDE_W {
        SESSEND_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn bvalid_override(&mut self) -> BVALID_OVERRIDE_W {
        BVALID_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn avalid_override(&mut self) -> AVALID_OVERRIDE_W {
        AVALID_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\]
is set to 1'b1"]
    #[inline(always)]
    pub fn vbusvalid_override(&mut self) -> VBUSVALID_OVERRIDE_W {
        VBUSVALID_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel(&mut self) -> VBUSVALID_SEL_W {
        VBUSVALID_SEL_W { w: self }
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel(&mut self) -> VBUS_SOURCE_SEL_W {
        VBUS_SOURCE_SEL_W { w: self }
    }
    #[doc = "Bit 11 - Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub fn id_override_en(&mut self) -> ID_OVERRIDE_EN_W {
        ID_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 12 - ID override value."]
    #[inline(always)]
    pub fn id_override(&mut self) -> ID_OVERRIDE_W {
        ID_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 13 - Enable ID override using the pinmuxed value:"]
    #[inline(always)]
    pub fn ext_id_override_en(&mut self) -> EXT_ID_OVERRIDE_EN_W {
        EXT_ID_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 14 - Enable VBUS override using the pinmuxed value."]
    #[inline(always)]
    pub fn ext_vbus_override_en(&mut self) -> EXT_VBUS_OVERRIDE_EN_W {
        EXT_VBUS_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid(&mut self) -> VBUSVALID_TO_SESSVALID_W {
        VBUSVALID_TO_SESSVALID_W { w: self }
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn vbusvalid_5vdetect(&mut self) -> VBUSVALID_5VDETECT_W {
        VBUSVALID_5VDETECT_W { w: self }
    }
    #[doc = "Bits 20:22 - Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub fn pwrup_cmps(&mut self) -> PWRUP_CMPS_W {
        PWRUP_CMPS_W { w: self }
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub fn discharge_vbus(&mut self) -> DISCHARGE_VBUS_W {
        DISCHARGE_VBUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY VBUS Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect](index.html) module"]
pub struct USB1_VBUS_DETECT_SPEC;
impl crate::RegisterSpec for USB1_VBUS_DETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_vbus_detect::R](R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect::W](W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB1_VBUS_DETECT to value 0x0070_0004"]
impl crate::Resettable for USB1_VBUS_DETECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0070_0004
    }
}
