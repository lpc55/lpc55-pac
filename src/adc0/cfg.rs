#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CFG_SPEC>> for R {
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl core::convert::From<crate::W<CFG_SPEC>> for W {
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC trigger priority control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TPRICTRL_A {
    #[doc = "0: If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    TPRICTRL_0 = 0,
    #[doc = "1: If a higher priority trigger is received during command processing, the current command is stopped after after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\]
will be ignored and the higher priority trigger will be serviced."]
    TPRICTRL_1 = 1,
    #[doc = "2: If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    TPRICTRL_2 = 2,
}
impl From<TPRICTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: TPRICTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TPRICTRL` reader - ADC trigger priority control"]
pub struct TPRICTRL_R(crate::FieldReader<u8, TPRICTRL_A>);
impl TPRICTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPRICTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPRICTRL_A> {
        match self.bits {
            0 => Some(TPRICTRL_A::TPRICTRL_0),
            1 => Some(TPRICTRL_A::TPRICTRL_1),
            2 => Some(TPRICTRL_A::TPRICTRL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TPRICTRL_0`"]
    #[inline(always)]
    pub fn is_tprictrl_0(&self) -> bool {
        **self == TPRICTRL_A::TPRICTRL_0
    }
    #[doc = "Checks if the value of the field is `TPRICTRL_1`"]
    #[inline(always)]
    pub fn is_tprictrl_1(&self) -> bool {
        **self == TPRICTRL_A::TPRICTRL_1
    }
    #[doc = "Checks if the value of the field is `TPRICTRL_2`"]
    #[inline(always)]
    pub fn is_tprictrl_2(&self) -> bool {
        **self == TPRICTRL_A::TPRICTRL_2
    }
}
impl core::ops::Deref for TPRICTRL_R {
    type Target = crate::FieldReader<u8, TPRICTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPRICTRL` writer - ADC trigger priority control"]
pub struct TPRICTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPRICTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPRICTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    #[inline(always)]
    pub fn tprictrl_0(self) -> &'a mut W {
        self.variant(TPRICTRL_A::TPRICTRL_0)
    }
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\]
will be ignored and the higher priority trigger will be serviced."]
    #[inline(always)]
    pub fn tprictrl_1(self) -> &'a mut W {
        self.variant(TPRICTRL_A::TPRICTRL_1)
    }
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    #[inline(always)]
    pub fn tprictrl_2(self) -> &'a mut W {
        self.variant(TPRICTRL_A::TPRICTRL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Power Configuration Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRSEL_A {
    #[doc = "0: Lowest power setting."]
    PWRSEL_0 = 0,
    #[doc = "1: Higher power setting than 0b0."]
    PWRSEL_1 = 1,
    #[doc = "2: Higher power setting than 0b1."]
    PWRSEL_2 = 2,
    #[doc = "3: Highest power setting."]
    PWRSEL_3 = 3,
}
impl From<PWRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRSEL` reader - Power Configuration Select"]
pub struct PWRSEL_R(crate::FieldReader<u8, PWRSEL_A>);
impl PWRSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWRSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSEL_A {
        match self.bits {
            0 => PWRSEL_A::PWRSEL_0,
            1 => PWRSEL_A::PWRSEL_1,
            2 => PWRSEL_A::PWRSEL_2,
            3 => PWRSEL_A::PWRSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWRSEL_0`"]
    #[inline(always)]
    pub fn is_pwrsel_0(&self) -> bool {
        **self == PWRSEL_A::PWRSEL_0
    }
    #[doc = "Checks if the value of the field is `PWRSEL_1`"]
    #[inline(always)]
    pub fn is_pwrsel_1(&self) -> bool {
        **self == PWRSEL_A::PWRSEL_1
    }
    #[doc = "Checks if the value of the field is `PWRSEL_2`"]
    #[inline(always)]
    pub fn is_pwrsel_2(&self) -> bool {
        **self == PWRSEL_A::PWRSEL_2
    }
    #[doc = "Checks if the value of the field is `PWRSEL_3`"]
    #[inline(always)]
    pub fn is_pwrsel_3(&self) -> bool {
        **self == PWRSEL_A::PWRSEL_3
    }
}
impl core::ops::Deref for PWRSEL_R {
    type Target = crate::FieldReader<u8, PWRSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRSEL` writer - Power Configuration Select"]
pub struct PWRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Lowest power setting."]
    #[inline(always)]
    pub fn pwrsel_0(self) -> &'a mut W {
        self.variant(PWRSEL_A::PWRSEL_0)
    }
    #[doc = "Higher power setting than 0b0."]
    #[inline(always)]
    pub fn pwrsel_1(self) -> &'a mut W {
        self.variant(PWRSEL_A::PWRSEL_1)
    }
    #[doc = "Higher power setting than 0b1."]
    #[inline(always)]
    pub fn pwrsel_2(self) -> &'a mut W {
        self.variant(PWRSEL_A::PWRSEL_2)
    }
    #[doc = "Highest power setting."]
    #[inline(always)]
    pub fn pwrsel_3(self) -> &'a mut W {
        self.variant(PWRSEL_A::PWRSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: (Default) Option 1 setting."]
    REFSEL_0 = 0,
    #[doc = "1: Option 2 setting."]
    REFSEL_1 = 1,
    #[doc = "2: Option 3 setting."]
    REFSEL_2 = 2,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFSEL` reader - Voltage Reference Selection"]
pub struct REFSEL_R(crate::FieldReader<u8, REFSEL_A>);
impl REFSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::REFSEL_0),
            1 => Some(REFSEL_A::REFSEL_1),
            2 => Some(REFSEL_A::REFSEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REFSEL_0`"]
    #[inline(always)]
    pub fn is_refsel_0(&self) -> bool {
        **self == REFSEL_A::REFSEL_0
    }
    #[doc = "Checks if the value of the field is `REFSEL_1`"]
    #[inline(always)]
    pub fn is_refsel_1(&self) -> bool {
        **self == REFSEL_A::REFSEL_1
    }
    #[doc = "Checks if the value of the field is `REFSEL_2`"]
    #[inline(always)]
    pub fn is_refsel_2(&self) -> bool {
        **self == REFSEL_A::REFSEL_2
    }
}
impl core::ops::Deref for REFSEL_R {
    type Target = crate::FieldReader<u8, REFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFSEL` writer - Voltage Reference Selection"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "(Default) Option 1 setting."]
    #[inline(always)]
    pub fn refsel_0(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_0)
    }
    #[doc = "Option 2 setting."]
    #[inline(always)]
    pub fn refsel_1(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_1)
    }
    #[doc = "Option 3 setting."]
    #[inline(always)]
    pub fn refsel_2(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Trigger Resume Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRES_A {
    #[doc = "0: Trigger sequences interrupted by a high priority trigger exception will not be automatically resumed or restarted."]
    TRES_0 = 0,
    #[doc = "1: Trigger sequences interrupted by a high priority trigger exception will be automatically resumed or restarted."]
    TRES_1 = 1,
}
impl From<TRES_A> for bool {
    #[inline(always)]
    fn from(variant: TRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRES` reader - Trigger Resume Enable"]
pub struct TRES_R(crate::FieldReader<bool, TRES_A>);
impl TRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRES_A {
        match self.bits {
            false => TRES_A::TRES_0,
            true => TRES_A::TRES_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRES_0`"]
    #[inline(always)]
    pub fn is_tres_0(&self) -> bool {
        **self == TRES_A::TRES_0
    }
    #[doc = "Checks if the value of the field is `TRES_1`"]
    #[inline(always)]
    pub fn is_tres_1(&self) -> bool {
        **self == TRES_A::TRES_1
    }
}
impl core::ops::Deref for TRES_R {
    type Target = crate::FieldReader<bool, TRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRES` writer - Trigger Resume Enable"]
pub struct TRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will not be automatically resumed or restarted."]
    #[inline(always)]
    pub fn tres_0(self) -> &'a mut W {
        self.variant(TRES_A::TRES_0)
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically resumed or restarted."]
    #[inline(always)]
    pub fn tres_1(self) -> &'a mut W {
        self.variant(TRES_A::TRES_1)
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
#[doc = "Trigger Command Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCMDRES_A {
    #[doc = "0: Trigger sequences interrupted by a high priority trigger exception will be automatically restarted."]
    TCMDRES_0 = 0,
    #[doc = "1: Trigger sequences interrupted by a high priority trigger exception will be resumed from the command executing before the exception."]
    TCMDRES_1 = 1,
}
impl From<TCMDRES_A> for bool {
    #[inline(always)]
    fn from(variant: TCMDRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCMDRES` reader - Trigger Command Resume"]
pub struct TCMDRES_R(crate::FieldReader<bool, TCMDRES_A>);
impl TCMDRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCMDRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCMDRES_A {
        match self.bits {
            false => TCMDRES_A::TCMDRES_0,
            true => TCMDRES_A::TCMDRES_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCMDRES_0`"]
    #[inline(always)]
    pub fn is_tcmdres_0(&self) -> bool {
        **self == TCMDRES_A::TCMDRES_0
    }
    #[doc = "Checks if the value of the field is `TCMDRES_1`"]
    #[inline(always)]
    pub fn is_tcmdres_1(&self) -> bool {
        **self == TCMDRES_A::TCMDRES_1
    }
}
impl core::ops::Deref for TCMDRES_R {
    type Target = crate::FieldReader<bool, TCMDRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCMDRES` writer - Trigger Command Resume"]
pub struct TCMDRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TCMDRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCMDRES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically restarted."]
    #[inline(always)]
    pub fn tcmdres_0(self) -> &'a mut W {
        self.variant(TCMDRES_A::TCMDRES_0)
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be resumed from the command executing before the exception."]
    #[inline(always)]
    pub fn tcmdres_1(self) -> &'a mut W {
        self.variant(TCMDRES_A::TCMDRES_1)
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
#[doc = "High Priority Trigger Exception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPT_EXDI_A {
    #[doc = "0: High priority trigger exceptions are enabled."]
    HPT_EXDI_0 = 0,
    #[doc = "1: High priority trigger exceptions are disabled."]
    HPT_EXDI_1 = 1,
}
impl From<HPT_EXDI_A> for bool {
    #[inline(always)]
    fn from(variant: HPT_EXDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPT_EXDI` reader - High Priority Trigger Exception Disable"]
pub struct HPT_EXDI_R(crate::FieldReader<bool, HPT_EXDI_A>);
impl HPT_EXDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPT_EXDI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPT_EXDI_A {
        match self.bits {
            false => HPT_EXDI_A::HPT_EXDI_0,
            true => HPT_EXDI_A::HPT_EXDI_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPT_EXDI_0`"]
    #[inline(always)]
    pub fn is_hpt_exdi_0(&self) -> bool {
        **self == HPT_EXDI_A::HPT_EXDI_0
    }
    #[doc = "Checks if the value of the field is `HPT_EXDI_1`"]
    #[inline(always)]
    pub fn is_hpt_exdi_1(&self) -> bool {
        **self == HPT_EXDI_A::HPT_EXDI_1
    }
}
impl core::ops::Deref for HPT_EXDI_R {
    type Target = crate::FieldReader<bool, HPT_EXDI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPT_EXDI` writer - High Priority Trigger Exception Disable"]
pub struct HPT_EXDI_W<'a> {
    w: &'a mut W,
}
impl<'a> HPT_EXDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPT_EXDI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High priority trigger exceptions are enabled."]
    #[inline(always)]
    pub fn hpt_exdi_0(self) -> &'a mut W {
        self.variant(HPT_EXDI_A::HPT_EXDI_0)
    }
    #[doc = "High priority trigger exceptions are disabled."]
    #[inline(always)]
    pub fn hpt_exdi_1(self) -> &'a mut W {
        self.variant(HPT_EXDI_A::HPT_EXDI_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PUDLY` reader - Power Up Delay"]
pub struct PUDLY_R(crate::FieldReader<u8, u8>);
impl PUDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUDLY` writer - Power Up Delay"]
pub struct PUDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PUDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "ADC Analog Pre-Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWREN_A {
    #[doc = "0: ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    PWREN_0 = 0,
    #[doc = "1: ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). A single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog will remain pre-enabled, and no additional delays will be executed."]
    PWREN_1 = 1,
}
impl From<PWREN_A> for bool {
    #[inline(always)]
    fn from(variant: PWREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWREN` reader - ADC Analog Pre-Enable"]
pub struct PWREN_R(crate::FieldReader<bool, PWREN_A>);
impl PWREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWREN_A {
        match self.bits {
            false => PWREN_A::PWREN_0,
            true => PWREN_A::PWREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWREN_0`"]
    #[inline(always)]
    pub fn is_pwren_0(&self) -> bool {
        **self == PWREN_A::PWREN_0
    }
    #[doc = "Checks if the value of the field is `PWREN_1`"]
    #[inline(always)]
    pub fn is_pwren_1(&self) -> bool {
        **self == PWREN_A::PWREN_1
    }
}
impl core::ops::Deref for PWREN_R {
    type Target = crate::FieldReader<bool, PWREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWREN` writer - ADC Analog Pre-Enable"]
pub struct PWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    #[inline(always)]
    pub fn pwren_0(self) -> &'a mut W {
        self.variant(PWREN_A::PWREN_0)
    }
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). A single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog will remain pre-enabled, and no additional delays will be executed."]
    #[inline(always)]
    pub fn pwren_1(self) -> &'a mut W {
        self.variant(PWREN_A::PWREN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC trigger priority control"]
    #[inline(always)]
    pub fn tprictrl(&self) -> TPRICTRL_R {
        TPRICTRL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PWRSEL_R {
        PWRSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Trigger Resume Enable"]
    #[inline(always)]
    pub fn tres(&self) -> TRES_R {
        TRES_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Trigger Command Resume"]
    #[inline(always)]
    pub fn tcmdres(&self) -> TCMDRES_R {
        TCMDRES_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - High Priority Trigger Exception Disable"]
    #[inline(always)]
    pub fn hpt_exdi(&self) -> HPT_EXDI_R {
        HPT_EXDI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Power Up Delay"]
    #[inline(always)]
    pub fn pudly(&self) -> PUDLY_R {
        PUDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC trigger priority control"]
    #[inline(always)]
    pub fn tprictrl(&mut self) -> TPRICTRL_W {
        TPRICTRL_W { w: self }
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline(always)]
    pub fn pwrsel(&mut self) -> PWRSEL_W {
        PWRSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 8 - Trigger Resume Enable"]
    #[inline(always)]
    pub fn tres(&mut self) -> TRES_W {
        TRES_W { w: self }
    }
    #[doc = "Bit 9 - Trigger Command Resume"]
    #[inline(always)]
    pub fn tcmdres(&mut self) -> TCMDRES_W {
        TCMDRES_W { w: self }
    }
    #[doc = "Bit 10 - High Priority Trigger Exception Disable"]
    #[inline(always)]
    pub fn hpt_exdi(&mut self) -> HPT_EXDI_W {
        HPT_EXDI_W { w: self }
    }
    #[doc = "Bits 16:23 - Power Up Delay"]
    #[inline(always)]
    pub fn pudly(&mut self) -> PUDLY_W {
        PUDLY_W { w: self }
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W {
        PWREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x0080_0000"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
