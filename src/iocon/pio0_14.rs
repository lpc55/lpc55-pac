#[doc = "Reader of register PIO0_14"]
pub type R = crate::R<u32, super::PIO0_14>;
#[doc = "Writer for register PIO0_14"]
pub type W = crate::W<u32, super::PIO0_14>;
#[doc = "Register PIO0_14 `reset()`'s with value 0x5000"]
impl crate::ResetValue for super::PIO0_14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5000
    }
}
#[doc = "Possible values of the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNC_A {
    #[doc = "Alternative connection 0."]
    ALT0,
    #[doc = "Alternative connection 1."]
    ALT1,
    #[doc = "Alternative connection 2."]
    ALT2,
    #[doc = "Alternative connection 3."]
    ALT3,
    #[doc = "Alternative connection 4."]
    ALT4,
    #[doc = "Alternative connection 5."]
    ALT5,
    #[doc = "Alternative connection 6."]
    ALT6,
    #[doc = "Alternative connection 7."]
    ALT7,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        match variant {
            FUNC_A::ALT0 => 0,
            FUNC_A::ALT1 => 1,
            FUNC_A::ALT2 => 2,
            FUNC_A::ALT3 => 3,
            FUNC_A::ALT4 => 4,
            FUNC_A::ALT5 => 5,
            FUNC_A::ALT6 => 6,
            FUNC_A::ALT7 => 7,
        }
    }
}
#[doc = "Reader of field `FUNC`"]
pub type FUNC_R = crate::R<u8, FUNC_A>;
impl FUNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FUNC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FUNC_A::ALT0),
            1 => Val(FUNC_A::ALT1),
            2 => Val(FUNC_A::ALT2),
            3 => Val(FUNC_A::ALT3),
            4 => Val(FUNC_A::ALT4),
            5 => Val(FUNC_A::ALT5),
            6 => Val(FUNC_A::ALT6),
            7 => Val(FUNC_A::ALT7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        *self == FUNC_A::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == FUNC_A::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == FUNC_A::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == FUNC_A::ALT3
    }
    #[doc = "Checks if the value of the field is `ALT4`"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == FUNC_A::ALT4
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == FUNC_A::ALT5
    }
    #[doc = "Checks if the value of the field is `ALT6`"]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        *self == FUNC_A::ALT6
    }
    #[doc = "Checks if the value of the field is `ALT7`"]
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        *self == FUNC_A::ALT7
    }
}
#[doc = "Write proxy for field `FUNC`"]
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
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. Repeater mode."]
    REPEATER,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::INACTIVE => 0,
            MODE_A::PULL_DOWN => 1,
            MODE_A::PULL_UP => 2,
            MODE_A::REPEATER => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
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
        *self == MODE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == MODE_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == MODE_A::REPEATER
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SLEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEW_A {
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    STANDARD,
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    FAST,
}
impl From<SLEW_A> for bool {
    #[inline(always)]
    fn from(variant: SLEW_A) -> Self {
        match variant {
            SLEW_A::STANDARD => false,
            SLEW_A::FAST => true,
        }
    }
}
#[doc = "Reader of field `SLEW`"]
pub type SLEW_R = crate::R<bool, SLEW_A>;
impl SLEW_R {
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
        *self == SLEW_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == SLEW_A::FAST
    }
}
#[doc = "Write proxy for field `SLEW`"]
pub struct SLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SLEW_A::STANDARD)
    }
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `INVERT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVERT_A {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED,
}
impl From<INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: INVERT_A) -> Self {
        match variant {
            INVERT_A::DISABLED => false,
            INVERT_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `INVERT`"]
pub type INVERT_R = crate::R<bool, INVERT_A>;
impl INVERT_R {
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
        *self == INVERT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INVERT_A::ENABLED
    }
}
#[doc = "Write proxy for field `INVERT`"]
pub struct INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> INVERT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVERT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `DIGIMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGIMODE_A {
    #[doc = "Analog mode, digital input is disabled."]
    ANALOG,
    #[doc = "Digital mode, digital input is enabled."]
    DIGITAL,
}
impl From<DIGIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DIGIMODE_A) -> Self {
        match variant {
            DIGIMODE_A::ANALOG => false,
            DIGIMODE_A::DIGITAL => true,
        }
    }
}
#[doc = "Reader of field `DIGIMODE`"]
pub type DIGIMODE_R = crate::R<bool, DIGIMODE_A>;
impl DIGIMODE_R {
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
        *self == DIGIMODE_A::ANALOG
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == DIGIMODE_A::DIGITAL
    }
}
#[doc = "Write proxy for field `DIGIMODE`"]
pub struct DIGIMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGIMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIGIMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Analog mode, digital input is disabled."]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(DIGIMODE_A::ANALOG)
    }
    #[doc = "Digital mode, digital input is enabled."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_A {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN,
}
impl From<OD_A> for bool {
    #[inline(always)]
    fn from(variant: OD_A) -> Self {
        match variant {
            OD_A::NORMAL => false,
            OD_A::OPEN_DRAIN => true,
        }
    }
}
#[doc = "Reader of field `OD`"]
pub type OD_R = crate::R<bool, OD_A>;
impl OD_R {
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
        *self == OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `OD`"]
pub struct OD_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `SSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSEL_A {
    #[doc = "3V3 Signaling in I2C Mode."]
    SEL3V3,
    #[doc = "1V8 Signaling in I2C Mode."]
    SEL1V8,
}
impl From<SSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SSEL_A) -> Self {
        match variant {
            SSEL_A::SEL3V3 => false,
            SSEL_A::SEL1V8 => true,
        }
    }
}
#[doc = "Reader of field `SSEL`"]
pub type SSEL_R = crate::R<bool, SSEL_A>;
impl SSEL_R {
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
        *self == SSEL_A::SEL3V3
    }
    #[doc = "Checks if the value of the field is `SEL1V8`"]
    #[inline(always)]
    pub fn is_sel1v8(&self) -> bool {
        *self == SSEL_A::SEL1V8
    }
}
#[doc = "Write proxy for field `SSEL`"]
pub struct SSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `FILTEROFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTEROFF_A {
    #[doc = "Filter enabled. Noise pulses below approximately 10 ns are filtered out."]
    ENABLED,
    #[doc = "Filter disabled. No input filtering is done."]
    DISABLED,
}
impl From<FILTEROFF_A> for bool {
    #[inline(always)]
    fn from(variant: FILTEROFF_A) -> Self {
        match variant {
            FILTEROFF_A::ENABLED => false,
            FILTEROFF_A::DISABLED => true,
        }
    }
}
#[doc = "Reader of field `FILTEROFF`"]
pub type FILTEROFF_R = crate::R<bool, FILTEROFF_A>;
impl FILTEROFF_R {
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
        *self == FILTEROFF_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FILTEROFF_A::DISABLED
    }
}
#[doc = "Write proxy for field `FILTEROFF`"]
pub struct FILTEROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEROFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTEROFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Filter enabled. Noise pulses below approximately 10 ns are filtered out."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTEROFF_A::ENABLED)
    }
    #[doc = "Filter disabled. No input filtering is done."]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `ECS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECS_A {
    #[doc = "Disabled. IO is in open drain."]
    DISABLED,
    #[doc = "Enabled. Pull resistor is conencted."]
    ENABLED,
}
impl From<ECS_A> for bool {
    #[inline(always)]
    fn from(variant: ECS_A) -> Self {
        match variant {
            ECS_A::DISABLED => false,
            ECS_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `ECS`"]
pub type ECS_R = crate::R<bool, ECS_A>;
impl ECS_R {
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
        *self == ECS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECS_A::ENABLED
    }
}
#[doc = "Write proxy for field `ECS`"]
pub struct ECS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. IO is in open drain."]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `EGP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGP_A {
    #[doc = "I2C mode."]
    I2C_MODE,
    #[doc = "GPIO mode."]
    GPIO_MODE,
}
impl From<EGP_A> for bool {
    #[inline(always)]
    fn from(variant: EGP_A) -> Self {
        match variant {
            EGP_A::I2C_MODE => false,
            EGP_A::GPIO_MODE => true,
        }
    }
}
#[doc = "Reader of field `EGP`"]
pub type EGP_R = crate::R<bool, EGP_A>;
impl EGP_R {
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
        *self == EGP_A::I2C_MODE
    }
    #[doc = "Checks if the value of the field is `GPIO_MODE`"]
    #[inline(always)]
    pub fn is_gpio_mode(&self) -> bool {
        *self == EGP_A::GPIO_MODE
    }
}
#[doc = "Write proxy for field `EGP`"]
pub struct EGP_W<'a> {
    w: &'a mut W,
}
impl<'a> EGP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EGP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `I2CFILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CFILTER_A {
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Fast-mode and Fast-mode Plus I2C."]
    FAST_MODE,
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for Standard-mode I2C."]
    STANDARD_MODE,
}
impl From<I2CFILTER_A> for bool {
    #[inline(always)]
    fn from(variant: I2CFILTER_A) -> Self {
        match variant {
            I2CFILTER_A::FAST_MODE => false,
            I2CFILTER_A::STANDARD_MODE => true,
        }
    }
}
#[doc = "Reader of field `I2CFILTER`"]
pub type I2CFILTER_R = crate::R<bool, I2CFILTER_A>;
impl I2CFILTER_R {
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
        *self == I2CFILTER_A::FAST_MODE
    }
    #[doc = "Checks if the value of the field is `STANDARD_MODE`"]
    #[inline(always)]
    pub fn is_standard_mode(&self) -> bool {
        *self == I2CFILTER_A::STANDARD_MODE
    }
}
#[doc = "Write proxy for field `I2CFILTER`"]
pub struct I2CFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CFILTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CFILTER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Fast-mode and Fast-mode Plus I2C."]
    #[inline(always)]
    pub fn fast_mode(self) -> &'a mut W {
        self.variant(I2CFILTER_A::FAST_MODE)
    }
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for Standard-mode I2C."]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
    #[doc = "Bit 9 - Controls open-drain mode."]
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
    #[doc = "Bit 13 - Pull-up current source enable in IIC mode."]
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Controls slew rate of I2C pad."]
    #[inline(always)]
    pub fn egp(&self) -> EGP_R {
        EGP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
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
    #[doc = "Bit 9 - Controls open-drain mode."]
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
    #[doc = "Bit 13 - Pull-up current source enable in IIC mode."]
    #[inline(always)]
    pub fn ecs(&mut self) -> ECS_W {
        ECS_W { w: self }
    }
    #[doc = "Bit 14 - Controls slew rate of I2C pad."]
    #[inline(always)]
    pub fn egp(&mut self) -> EGP_W {
        EGP_W { w: self }
    }
    #[doc = "Bit 15 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn i2cfilter(&mut self) -> I2CFILTER_W {
        I2CFILTER_W { w: self }
    }
}
