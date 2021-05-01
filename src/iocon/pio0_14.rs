#[doc = "Register `PIO0_14` reader"]
pub struct R(crate::R<PIO0_14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO0_14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PIO0_14_SPEC>> for R {
    fn from(reader: crate::R<PIO0_14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO0_14` writer"]
pub struct W(crate::W<PIO0_14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO0_14_SPEC>;
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
impl core::convert::From<crate::W<PIO0_14_SPEC>> for W {
    fn from(writer: crate::W<PIO0_14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects pin function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: Alternative connection 0."]
    ALT0 = 0,
    #[doc = "1: Alternative connection 1."]
    ALT1 = 1,
    #[doc = "2: Alternative connection 2."]
    ALT2 = 2,
    #[doc = "3: Alternative connection 3."]
    ALT3 = 3,
    #[doc = "4: Alternative connection 4."]
    ALT4 = 4,
    #[doc = "5: Alternative connection 5."]
    ALT5 = 5,
    #[doc = "6: Alternative connection 6."]
    ALT6 = 6,
    #[doc = "7: Alternative connection 7."]
    ALT7 = 7,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC` reader - Selects pin function."]
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::ALT0),
            1 => Some(FUNC_A::ALT1),
            2 => Some(FUNC_A::ALT2),
            3 => Some(FUNC_A::ALT3),
            4 => Some(FUNC_A::ALT4),
            5 => Some(FUNC_A::ALT5),
            6 => Some(FUNC_A::ALT6),
            7 => Some(FUNC_A::ALT7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        **self == FUNC_A::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        **self == FUNC_A::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        **self == FUNC_A::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        **self == FUNC_A::ALT3
    }
    #[doc = "Checks if the value of the field is `ALT4`"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        **self == FUNC_A::ALT4
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        **self == FUNC_A::ALT5
    }
    #[doc = "Checks if the value of the field is `ALT6`"]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        **self == FUNC_A::ALT6
    }
    #[doc = "Checks if the value of the field is `ALT7`"]
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        **self == FUNC_A::ALT7
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function."]
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Alternative connection 0."]
    #[inline(always)]
    pub fn alt0(self) -> &'a mut W {
        self.variant(FUNC_A::ALT0)
    }
    #[doc = "Alternative connection 1."]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(FUNC_A::ALT1)
    }
    #[doc = "Alternative connection 2."]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(FUNC_A::ALT2)
    }
    #[doc = "Alternative connection 3."]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut W {
        self.variant(FUNC_A::ALT3)
    }
    #[doc = "Alternative connection 4."]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut W {
        self.variant(FUNC_A::ALT4)
    }
    #[doc = "Alternative connection 5."]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut W {
        self.variant(FUNC_A::ALT5)
    }
    #[doc = "Alternative connection 6."]
    #[inline(always)]
    pub fn alt6(self) -> &'a mut W {
        self.variant(FUNC_A::ALT6)
    }
    #[doc = "Alternative connection 7."]
    #[inline(always)]
    pub fn alt7(self) -> &'a mut W {
        self.variant(FUNC_A::ALT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater. Repeater mode."]
    REPEATER = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::INACTIVE,
            1 => MODE_A::PULL_DOWN,
            2 => MODE_A::PULL_UP,
            3 => MODE_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == MODE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == MODE_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        **self == MODE_A::REPEATER
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODE_A::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODE_A::REPEATER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Driver slew rate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEW_A {
    #[doc = "0: Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0,
    #[doc = "1: Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 1,
}
impl From<SLEW_A> for bool {
    #[inline(always)]
    fn from(variant: SLEW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEW` reader - Driver slew rate."]
pub struct SLEW_R(crate::FieldReader<bool, SLEW_A>);
impl SLEW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEW_A {
        match self.bits {
            false => SLEW_A::STANDARD,
            true => SLEW_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == SLEW_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        **self == SLEW_A::FAST
    }
}
impl core::ops::Deref for SLEW_R {
    type Target = crate::FieldReader<bool, SLEW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEW` writer - Driver slew rate."]
pub struct SLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SLEW_A::STANDARD)
    }
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(SLEW_A::FAST)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Input polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVERT_A {
    #[doc = "0: Disabled. Input function is not inverted."]
    DISABLED = 0,
    #[doc = "1: Enabled. Input is function inverted."]
    ENABLED = 1,
}
impl From<INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: INVERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVERT` reader - Input polarity."]
pub struct INVERT_R(crate::FieldReader<bool, INVERT_A>);
impl INVERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVERT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVERT_A {
        match self.bits {
            false => INVERT_A::DISABLED,
            true => INVERT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INVERT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INVERT_A::ENABLED
    }
}
impl core::ops::Deref for INVERT_R {
    type Target = crate::FieldReader<bool, INVERT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVERT` writer - Input polarity."]
pub struct INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> INVERT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVERT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. Input function is not inverted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INVERT_A::DISABLED)
    }
    #[doc = "Enabled. Input is function inverted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INVERT_A::ENABLED)
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
#[doc = "Select Digital mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGIMODE_A {
    #[doc = "0: Disable digital mode. Digital input set to 0."]
    ANALOG = 0,
    #[doc = "1: Enable Digital mode. Digital input is enabled."]
    DIGITAL = 1,
}
impl From<DIGIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DIGIMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGIMODE` reader - Select Digital mode."]
pub struct DIGIMODE_R(crate::FieldReader<bool, DIGIMODE_A>);
impl DIGIMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIGIMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIGIMODE_A {
        match self.bits {
            false => DIGIMODE_A::ANALOG,
            true => DIGIMODE_A::DIGITAL,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        **self == DIGIMODE_A::ANALOG
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        **self == DIGIMODE_A::DIGITAL
    }
}
impl core::ops::Deref for DIGIMODE_R {
    type Target = crate::FieldReader<bool, DIGIMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIGIMODE` writer - Select Digital mode."]
pub struct DIGIMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGIMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIGIMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable digital mode. Digital input set to 0."]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(DIGIMODE_A::ANALOG)
    }
    #[doc = "Enable Digital mode. Digital input is enabled."]
    #[inline(always)]
    pub fn digital(self) -> &'a mut W {
        self.variant(DIGIMODE_A::DIGITAL)
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
#[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_A {
    #[doc = "0: Normal. Normal push-pull output"]
    NORMAL = 0,
    #[doc = "1: Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 1,
}
impl From<OD_A> for bool {
    #[inline(always)]
    fn from(variant: OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OD` reader - Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
pub struct OD_R(crate::FieldReader<bool, OD_A>);
impl OD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_A {
        match self.bits {
            false => OD_A::NORMAL,
            true => OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == OD_A::OPEN_DRAIN
    }
}
impl core::ops::Deref for OD_R {
    type Target = crate::FieldReader<bool, OD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OD` writer - Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
pub struct OD_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal. Normal push-pull output"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OD_A::NORMAL)
    }
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OD_A::OPEN_DRAIN)
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
#[doc = "Supply Selection bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEL_A {
    #[doc = "0: 3V3 Signaling in I2C Mode."]
    SEL3V3 = 0,
    #[doc = "1: 1V8 Signaling in I2C Mode."]
    SEL1V8 = 1,
}
impl From<SSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEL` reader - Supply Selection bit."]
pub struct SSEL_R(crate::FieldReader<bool, SSEL_A>);
impl SSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSEL_A {
        match self.bits {
            false => SSEL_A::SEL3V3,
            true => SSEL_A::SEL1V8,
        }
    }
    #[doc = "Checks if the value of the field is `SEL3V3`"]
    #[inline(always)]
    pub fn is_sel3v3(&self) -> bool {
        **self == SSEL_A::SEL3V3
    }
    #[doc = "Checks if the value of the field is `SEL1V8`"]
    #[inline(always)]
    pub fn is_sel1v8(&self) -> bool {
        **self == SSEL_A::SEL1V8
    }
}
impl core::ops::Deref for SSEL_R {
    type Target = crate::FieldReader<bool, SSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSEL` writer - Supply Selection bit."]
pub struct SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "3V3 Signaling in I2C Mode."]
    #[inline(always)]
    pub fn sel3v3(self) -> &'a mut W {
        self.variant(SSEL_A::SEL3V3)
    }
    #[doc = "1V8 Signaling in I2C Mode."]
    #[inline(always)]
    pub fn sel1v8(self) -> &'a mut W {
        self.variant(SSEL_A::SEL1V8)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Controls input glitch filter.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTEROFF_A {
    #[doc = "0: Filter enabled."]
    ENABLED = 0,
    #[doc = "1: Filter disabled."]
    DISABLED = 1,
}
impl From<FILTEROFF_A> for bool {
    #[inline(always)]
    fn from(variant: FILTEROFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTEROFF` reader - Controls input glitch filter."]
pub struct FILTEROFF_R(crate::FieldReader<bool, FILTEROFF_A>);
impl FILTEROFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FILTEROFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTEROFF_A {
        match self.bits {
            false => FILTEROFF_A::ENABLED,
            true => FILTEROFF_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FILTEROFF_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FILTEROFF_A::DISABLED
    }
}
impl core::ops::Deref for FILTEROFF_R {
    type Target = crate::FieldReader<bool, FILTEROFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEROFF` writer - Controls input glitch filter."]
pub struct FILTEROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEROFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTEROFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTEROFF_A::ENABLED)
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FILTEROFF_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Pull-up current source enable in I2C mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECS_A {
    #[doc = "0: Disabled. IO is in open drain cell."]
    DISABLED = 0,
    #[doc = "1: Enabled. Pull resistor is conencted."]
    ENABLED = 1,
}
impl From<ECS_A> for bool {
    #[inline(always)]
    fn from(variant: ECS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECS` reader - Pull-up current source enable in I2C mode."]
pub struct ECS_R(crate::FieldReader<bool, ECS_A>);
impl ECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECS_A {
        match self.bits {
            false => ECS_A::DISABLED,
            true => ECS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ECS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ECS_A::ENABLED
    }
}
impl core::ops::Deref for ECS_R {
    type Target = crate::FieldReader<bool, ECS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECS` writer - Pull-up current source enable in I2C mode."]
pub struct ECS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. IO is in open drain cell."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECS_A::DISABLED)
    }
    #[doc = "Enabled. Pull resistor is conencted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECS_A::ENABLED)
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
#[doc = "Switch between GPIO mode and I2C mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGP_A {
    #[doc = "0: I2C mode."]
    I2C_MODE = 0,
    #[doc = "1: GPIO mode."]
    GPIO_MODE = 1,
}
impl From<EGP_A> for bool {
    #[inline(always)]
    fn from(variant: EGP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EGP` reader - Switch between GPIO mode and I2C mode."]
pub struct EGP_R(crate::FieldReader<bool, EGP_A>);
impl EGP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EGP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EGP_A {
        match self.bits {
            false => EGP_A::I2C_MODE,
            true => EGP_A::GPIO_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_MODE`"]
    #[inline(always)]
    pub fn is_i2c_mode(&self) -> bool {
        **self == EGP_A::I2C_MODE
    }
    #[doc = "Checks if the value of the field is `GPIO_MODE`"]
    #[inline(always)]
    pub fn is_gpio_mode(&self) -> bool {
        **self == EGP_A::GPIO_MODE
    }
}
impl core::ops::Deref for EGP_R {
    type Target = crate::FieldReader<bool, EGP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EGP` writer - Switch between GPIO mode and I2C mode."]
pub struct EGP_W<'a> {
    w: &'a mut W,
}
impl<'a> EGP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EGP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C mode."]
    #[inline(always)]
    pub fn i2c_mode(self) -> &'a mut W {
        self.variant(EGP_A::I2C_MODE)
    }
    #[doc = "GPIO mode."]
    #[inline(always)]
    pub fn gpio_mode(self) -> &'a mut W {
        self.variant(EGP_A::GPIO_MODE)
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
#[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CFILTER_A {
    #[doc = "0: I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    FAST_MODE = 0,
    #[doc = "1: I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    STANDARD_MODE = 1,
}
impl From<I2CFILTER_A> for bool {
    #[inline(always)]
    fn from(variant: I2CFILTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CFILTER` reader - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
pub struct I2CFILTER_R(crate::FieldReader<bool, I2CFILTER_A>);
impl I2CFILTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2CFILTER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CFILTER_A {
        match self.bits {
            false => I2CFILTER_A::FAST_MODE,
            true => I2CFILTER_A::STANDARD_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `FAST_MODE`"]
    #[inline(always)]
    pub fn is_fast_mode(&self) -> bool {
        **self == I2CFILTER_A::FAST_MODE
    }
    #[doc = "Checks if the value of the field is `STANDARD_MODE`"]
    #[inline(always)]
    pub fn is_standard_mode(&self) -> bool {
        **self == I2CFILTER_A::STANDARD_MODE
    }
}
impl core::ops::Deref for I2CFILTER_R {
    type Target = crate::FieldReader<bool, I2CFILTER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CFILTER` writer - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
pub struct I2CFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CFILTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CFILTER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    #[inline(always)]
    pub fn fast_mode(self) -> &'a mut W {
        self.variant(I2CFILTER_A::FAST_MODE)
    }
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    #[inline(always)]
    pub fn standard_mode(self) -> &'a mut W {
        self.variant(I2CFILTER_A::STANDARD_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Driver slew rate."]
    #[inline(always)]
    pub fn slew(&self) -> SLEW_R {
        SLEW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Select Digital mode."]
    #[inline(always)]
    pub fn digimode(&self) -> DIGIMODE_R {
        DIGIMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Supply Selection bit."]
    #[inline(always)]
    pub fn ssel(&self) -> SSEL_R {
        SSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Controls input glitch filter."]
    #[inline(always)]
    pub fn filteroff(&self) -> FILTEROFF_R {
        FILTEROFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub fn egp(&self) -> EGP_R {
        EGP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub fn i2cfilter(&self) -> I2CFILTER_R {
        I2CFILTER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 6 - Driver slew rate."]
    #[inline(always)]
    pub fn slew(&mut self) -> SLEW_W {
        SLEW_W { w: self }
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    pub fn invert(&mut self) -> INVERT_W {
        INVERT_W { w: self }
    }
    #[doc = "Bit 8 - Select Digital mode."]
    #[inline(always)]
    pub fn digimode(&mut self) -> DIGIMODE_W {
        DIGIMODE_W { w: self }
    }
    #[doc = "Bit 9 - Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub fn od(&mut self) -> OD_W {
        OD_W { w: self }
    }
    #[doc = "Bit 11 - Supply Selection bit."]
    #[inline(always)]
    pub fn ssel(&mut self) -> SSEL_W {
        SSEL_W { w: self }
    }
    #[doc = "Bit 12 - Controls input glitch filter."]
    #[inline(always)]
    pub fn filteroff(&mut self) -> FILTEROFF_W {
        FILTEROFF_W { w: self }
    }
    #[doc = "Bit 13 - Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub fn ecs(&mut self) -> ECS_W {
        ECS_W { w: self }
    }
    #[doc = "Bit 14 - Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub fn egp(&mut self) -> EGP_W {
        EGP_W { w: self }
    }
    #[doc = "Bit 15 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub fn i2cfilter(&mut self) -> I2CFILTER_W {
        I2CFILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_14](index.html) module"]
pub struct PIO0_14_SPEC;
impl crate::RegisterSpec for PIO0_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio0_14::R](R) reader structure"]
impl crate::Readable for PIO0_14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio0_14::W](W) writer structure"]
impl crate::Writable for PIO0_14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO0_14 to value 0x5000"]
impl crate::Resettable for PIO0_14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5000
    }
}
