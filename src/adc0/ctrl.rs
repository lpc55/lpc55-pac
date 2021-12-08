#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEN_A {
    #[doc = "0: ADC is disabled."]
    ADCEN_0 = 0,
    #[doc = "1: ADC is enabled."]
    ADCEN_1 = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - ADC Enable"]
pub struct ADCEN_R(crate::FieldReader<bool, ADCEN_A>);
impl ADCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::ADCEN_0,
            true => ADCEN_A::ADCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCEN_0`"]
    #[inline(always)]
    pub fn is_adcen_0(&self) -> bool {
        **self == ADCEN_A::ADCEN_0
    }
    #[doc = "Checks if the value of the field is `ADCEN_1`"]
    #[inline(always)]
    pub fn is_adcen_1(&self) -> bool {
        **self == ADCEN_A::ADCEN_1
    }
}
impl core::ops::Deref for ADCEN_R {
    type Target = crate::FieldReader<bool, ADCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCEN` writer - ADC Enable"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC is disabled."]
    #[inline(always)]
    pub fn adcen_0(self) -> &'a mut W {
        self.variant(ADCEN_A::ADCEN_0)
    }
    #[doc = "ADC is enabled."]
    #[inline(always)]
    pub fn adcen_1(self) -> &'a mut W {
        self.variant(ADCEN_A::ADCEN_1)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: ADC logic is not reset."]
    RST_0 = 0,
    #[doc = "1: ADC logic is reset."]
    RST_1 = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Software Reset"]
pub struct RST_R(crate::FieldReader<bool, RST_A>);
impl RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::RST_0,
            true => RST_A::RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RST_0`"]
    #[inline(always)]
    pub fn is_rst_0(&self) -> bool {
        **self == RST_A::RST_0
    }
    #[doc = "Checks if the value of the field is `RST_1`"]
    #[inline(always)]
    pub fn is_rst_1(&self) -> bool {
        **self == RST_A::RST_1
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST` writer - Software Reset"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC logic is not reset."]
    #[inline(always)]
    pub fn rst_0(self) -> &'a mut W {
        self.variant(RST_A::RST_0)
    }
    #[doc = "ADC logic is reset."]
    #[inline(always)]
    pub fn rst_1(self) -> &'a mut W {
        self.variant(RST_A::RST_1)
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
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEN_A {
    #[doc = "0: ADC is enabled in Doze mode."]
    DOZEN_0 = 0,
    #[doc = "1: ADC is disabled in Doze mode."]
    DOZEN_1 = 1,
}
impl From<DOZEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEN` reader - Doze Enable"]
pub struct DOZEN_R(crate::FieldReader<bool, DOZEN_A>);
impl DOZEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOZEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEN_A {
        match self.bits {
            false => DOZEN_A::DOZEN_0,
            true => DOZEN_A::DOZEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEN_0`"]
    #[inline(always)]
    pub fn is_dozen_0(&self) -> bool {
        **self == DOZEN_A::DOZEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEN_1`"]
    #[inline(always)]
    pub fn is_dozen_1(&self) -> bool {
        **self == DOZEN_A::DOZEN_1
    }
}
impl core::ops::Deref for DOZEN_R {
    type Target = crate::FieldReader<bool, DOZEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOZEN` writer - Doze Enable"]
pub struct DOZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC is enabled in Doze mode."]
    #[inline(always)]
    pub fn dozen_0(self) -> &'a mut W {
        self.variant(DOZEN_A::DOZEN_0)
    }
    #[doc = "ADC is disabled in Doze mode."]
    #[inline(always)]
    pub fn dozen_1(self) -> &'a mut W {
        self.variant(DOZEN_A::DOZEN_1)
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
#[doc = "Auto-Calibration Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_REQ_A {
    #[doc = "0: No request for auto-calibration has been made."]
    CAL_REQ_0 = 0,
    #[doc = "1: A request for auto-calibration has been made"]
    CAL_REQ_1 = 1,
}
impl From<CAL_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL_REQ` reader - Auto-Calibration Request"]
pub struct CAL_REQ_R(crate::FieldReader<bool, CAL_REQ_A>);
impl CAL_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAL_REQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_REQ_A {
        match self.bits {
            false => CAL_REQ_A::CAL_REQ_0,
            true => CAL_REQ_A::CAL_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAL_REQ_0`"]
    #[inline(always)]
    pub fn is_cal_req_0(&self) -> bool {
        **self == CAL_REQ_A::CAL_REQ_0
    }
    #[doc = "Checks if the value of the field is `CAL_REQ_1`"]
    #[inline(always)]
    pub fn is_cal_req_1(&self) -> bool {
        **self == CAL_REQ_A::CAL_REQ_1
    }
}
impl core::ops::Deref for CAL_REQ_R {
    type Target = crate::FieldReader<bool, CAL_REQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL_REQ` writer - Auto-Calibration Request"]
pub struct CAL_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAL_REQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No request for auto-calibration has been made."]
    #[inline(always)]
    pub fn cal_req_0(self) -> &'a mut W {
        self.variant(CAL_REQ_A::CAL_REQ_0)
    }
    #[doc = "A request for auto-calibration has been made"]
    #[inline(always)]
    pub fn cal_req_1(self) -> &'a mut W {
        self.variant(CAL_REQ_A::CAL_REQ_1)
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
#[doc = "Configure for offset calibration function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALOFS_A {
    #[doc = "0: Calibration function disabled"]
    CALOFS_0 = 0,
    #[doc = "1: Request for offset calibration function"]
    CALOFS_1 = 1,
}
impl From<CALOFS_A> for bool {
    #[inline(always)]
    fn from(variant: CALOFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOFS` reader - Configure for offset calibration function"]
pub struct CALOFS_R(crate::FieldReader<bool, CALOFS_A>);
impl CALOFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CALOFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALOFS_A {
        match self.bits {
            false => CALOFS_A::CALOFS_0,
            true => CALOFS_A::CALOFS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALOFS_0`"]
    #[inline(always)]
    pub fn is_calofs_0(&self) -> bool {
        **self == CALOFS_A::CALOFS_0
    }
    #[doc = "Checks if the value of the field is `CALOFS_1`"]
    #[inline(always)]
    pub fn is_calofs_1(&self) -> bool {
        **self == CALOFS_A::CALOFS_1
    }
}
impl core::ops::Deref for CALOFS_R {
    type Target = crate::FieldReader<bool, CALOFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALOFS` writer - Configure for offset calibration function"]
pub struct CALOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CALOFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALOFS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration function disabled"]
    #[inline(always)]
    pub fn calofs_0(self) -> &'a mut W {
        self.variant(CALOFS_A::CALOFS_0)
    }
    #[doc = "Request for offset calibration function"]
    #[inline(always)]
    pub fn calofs_1(self) -> &'a mut W {
        self.variant(CALOFS_A::CALOFS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Reset FIFO 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFIFO0_A {
    #[doc = "0: No effect."]
    RSTFIFO0_0 = 0,
    #[doc = "1: FIFO 0 is reset."]
    RSTFIFO0_1 = 1,
}
impl From<RSTFIFO0_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFIFO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFIFO0` reader - Reset FIFO 0"]
pub struct RSTFIFO0_R(crate::FieldReader<bool, RSTFIFO0_A>);
impl RSTFIFO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTFIFO0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFIFO0_A {
        match self.bits {
            false => RSTFIFO0_A::RSTFIFO0_0,
            true => RSTFIFO0_A::RSTFIFO0_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTFIFO0_0`"]
    #[inline(always)]
    pub fn is_rstfifo0_0(&self) -> bool {
        **self == RSTFIFO0_A::RSTFIFO0_0
    }
    #[doc = "Checks if the value of the field is `RSTFIFO0_1`"]
    #[inline(always)]
    pub fn is_rstfifo0_1(&self) -> bool {
        **self == RSTFIFO0_A::RSTFIFO0_1
    }
}
impl core::ops::Deref for RSTFIFO0_R {
    type Target = crate::FieldReader<bool, RSTFIFO0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTFIFO0` writer - Reset FIFO 0"]
pub struct RSTFIFO0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFIFO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFIFO0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn rstfifo0_0(self) -> &'a mut W {
        self.variant(RSTFIFO0_A::RSTFIFO0_0)
    }
    #[doc = "FIFO 0 is reset."]
    #[inline(always)]
    pub fn rstfifo0_1(self) -> &'a mut W {
        self.variant(RSTFIFO0_A::RSTFIFO0_1)
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
#[doc = "Reset FIFO 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFIFO1_A {
    #[doc = "0: No effect."]
    RSTFIFO1_0 = 0,
    #[doc = "1: FIFO 1 is reset."]
    RSTFIFO1_1 = 1,
}
impl From<RSTFIFO1_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFIFO1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFIFO1` reader - Reset FIFO 1"]
pub struct RSTFIFO1_R(crate::FieldReader<bool, RSTFIFO1_A>);
impl RSTFIFO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTFIFO1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFIFO1_A {
        match self.bits {
            false => RSTFIFO1_A::RSTFIFO1_0,
            true => RSTFIFO1_A::RSTFIFO1_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTFIFO1_0`"]
    #[inline(always)]
    pub fn is_rstfifo1_0(&self) -> bool {
        **self == RSTFIFO1_A::RSTFIFO1_0
    }
    #[doc = "Checks if the value of the field is `RSTFIFO1_1`"]
    #[inline(always)]
    pub fn is_rstfifo1_1(&self) -> bool {
        **self == RSTFIFO1_A::RSTFIFO1_1
    }
}
impl core::ops::Deref for RSTFIFO1_R {
    type Target = crate::FieldReader<bool, RSTFIFO1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTFIFO1` writer - Reset FIFO 1"]
pub struct RSTFIFO1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFIFO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFIFO1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn rstfifo1_0(self) -> &'a mut W {
        self.variant(RSTFIFO1_A::RSTFIFO1_0)
    }
    #[doc = "FIFO 1 is reset."]
    #[inline(always)]
    pub fn rstfifo1_1(self) -> &'a mut W {
        self.variant(RSTFIFO1_A::RSTFIFO1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Auto-Calibration Averages\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAL_AVGS_A {
    #[doc = "0: Single conversion."]
    CAL_AVGS_0 = 0,
    #[doc = "1: 2 conversions averaged."]
    CAL_AVGS_1 = 1,
    #[doc = "2: 4 conversions averaged."]
    CAL_AVGS_2 = 2,
    #[doc = "3: 8 conversions averaged."]
    CAL_AVGS_3 = 3,
    #[doc = "4: 16 conversions averaged."]
    CAL_AVGS_4 = 4,
    #[doc = "5: 32 conversions averaged."]
    CAL_AVGS_5 = 5,
    #[doc = "6: 64 conversions averaged."]
    CAL_AVGS_6 = 6,
    #[doc = "7: 128 conversions averaged."]
    CAL_AVGS_7 = 7,
}
impl From<CAL_AVGS_A> for u8 {
    #[inline(always)]
    fn from(variant: CAL_AVGS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAL_AVGS` reader - Auto-Calibration Averages"]
pub struct CAL_AVGS_R(crate::FieldReader<u8, CAL_AVGS_A>);
impl CAL_AVGS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAL_AVGS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_AVGS_A {
        match self.bits {
            0 => CAL_AVGS_A::CAL_AVGS_0,
            1 => CAL_AVGS_A::CAL_AVGS_1,
            2 => CAL_AVGS_A::CAL_AVGS_2,
            3 => CAL_AVGS_A::CAL_AVGS_3,
            4 => CAL_AVGS_A::CAL_AVGS_4,
            5 => CAL_AVGS_A::CAL_AVGS_5,
            6 => CAL_AVGS_A::CAL_AVGS_6,
            7 => CAL_AVGS_A::CAL_AVGS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_0`"]
    #[inline(always)]
    pub fn is_cal_avgs_0(&self) -> bool {
        **self == CAL_AVGS_A::CAL_AVGS_0
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_1`"]
    #[inline(always)]
    pub fn is_cal_avgs_1(&self) -> bool {
        **self == CAL_AVGS_A::CAL_AVGS_1
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_2`"]
    #[inline(always)]
    pub fn is_cal_avgs_2(&self) -> bool {
        **self == CAL_AVGS_A::CAL_AVGS_2
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_3`"]
    #[inline(always)]
    pub fn is_cal_avgs_3(&self) -> bool {
        **self == CAL_AVGS_A::CAL_AVGS_3
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_4`"]
    #[inline(always)]
    pub fn is_cal_avgs_4(&self) -> bool {
        **self == CAL_AVGS_A::CAL_AVGS_4
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_5`"]
    #[inline(always)]
    pub fn is_cal_avgs_5(&self) -> bool {
        **self == CAL_AVGS_A::CAL_AVGS_5
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_6`"]
    #[inline(always)]
    pub fn is_cal_avgs_6(&self) -> bool {
        **self == CAL_AVGS_A::CAL_AVGS_6
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_7`"]
    #[inline(always)]
    pub fn is_cal_avgs_7(&self) -> bool {
        **self == CAL_AVGS_A::CAL_AVGS_7
    }
}
impl core::ops::Deref for CAL_AVGS_R {
    type Target = crate::FieldReader<u8, CAL_AVGS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL_AVGS` writer - Auto-Calibration Averages"]
pub struct CAL_AVGS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_AVGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAL_AVGS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single conversion."]
    #[inline(always)]
    pub fn cal_avgs_0(self) -> &'a mut W {
        self.variant(CAL_AVGS_A::CAL_AVGS_0)
    }
    #[doc = "2 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_1(self) -> &'a mut W {
        self.variant(CAL_AVGS_A::CAL_AVGS_1)
    }
    #[doc = "4 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_2(self) -> &'a mut W {
        self.variant(CAL_AVGS_A::CAL_AVGS_2)
    }
    #[doc = "8 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_3(self) -> &'a mut W {
        self.variant(CAL_AVGS_A::CAL_AVGS_3)
    }
    #[doc = "16 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_4(self) -> &'a mut W {
        self.variant(CAL_AVGS_A::CAL_AVGS_4)
    }
    #[doc = "32 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_5(self) -> &'a mut W {
        self.variant(CAL_AVGS_A::CAL_AVGS_5)
    }
    #[doc = "64 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_6(self) -> &'a mut W {
        self.variant(CAL_AVGS_A::CAL_AVGS_6)
    }
    #[doc = "128 conversions averaged."]
    #[inline(always)]
    pub fn cal_avgs_7(self) -> &'a mut W {
        self.variant(CAL_AVGS_A::CAL_AVGS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DOZEN_R {
        DOZEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Auto-Calibration Request"]
    #[inline(always)]
    pub fn cal_req(&self) -> CAL_REQ_R {
        CAL_REQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configure for offset calibration function"]
    #[inline(always)]
    pub fn calofs(&self) -> CALOFS_R {
        CALOFS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reset FIFO 0"]
    #[inline(always)]
    pub fn rstfifo0(&self) -> RSTFIFO0_R {
        RSTFIFO0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reset FIFO 1"]
    #[inline(always)]
    pub fn rstfifo1(&self) -> RSTFIFO1_R {
        RSTFIFO1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Auto-Calibration Averages"]
    #[inline(always)]
    pub fn cal_avgs(&self) -> CAL_AVGS_R {
        CAL_AVGS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&mut self) -> DOZEN_W {
        DOZEN_W { w: self }
    }
    #[doc = "Bit 3 - Auto-Calibration Request"]
    #[inline(always)]
    pub fn cal_req(&mut self) -> CAL_REQ_W {
        CAL_REQ_W { w: self }
    }
    #[doc = "Bit 4 - Configure for offset calibration function"]
    #[inline(always)]
    pub fn calofs(&mut self) -> CALOFS_W {
        CALOFS_W { w: self }
    }
    #[doc = "Bit 8 - Reset FIFO 0"]
    #[inline(always)]
    pub fn rstfifo0(&mut self) -> RSTFIFO0_W {
        RSTFIFO0_W { w: self }
    }
    #[doc = "Bit 9 - Reset FIFO 1"]
    #[inline(always)]
    pub fn rstfifo1(&mut self) -> RSTFIFO1_W {
        RSTFIFO1_W { w: self }
    }
    #[doc = "Bits 16:18 - Auto-Calibration Averages"]
    #[inline(always)]
    pub fn cal_avgs(&mut self) -> CAL_AVGS_W {
        CAL_AVGS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
