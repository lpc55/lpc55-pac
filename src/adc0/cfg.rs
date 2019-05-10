#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TPRICTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRICTRLR {
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    TPRICTRL_0,
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\] will be ignored and the higher priority trigger will be serviced."]
    TPRICTRL_1,
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    TPRICTRL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TPRICTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPRICTRLR::TPRICTRL_0 => 0,
            TPRICTRLR::TPRICTRL_1 => 1,
            TPRICTRLR::TPRICTRL_2 => 2,
            TPRICTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPRICTRLR {
        match value {
            0 => TPRICTRLR::TPRICTRL_0,
            1 => TPRICTRLR::TPRICTRL_1,
            2 => TPRICTRLR::TPRICTRL_2,
            i => TPRICTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TPRICTRL_0`"]
    #[inline]
    pub fn is_tprictrl_0(&self) -> bool {
        *self == TPRICTRLR::TPRICTRL_0
    }
    #[doc = "Checks if the value of the field is `TPRICTRL_1`"]
    #[inline]
    pub fn is_tprictrl_1(&self) -> bool {
        *self == TPRICTRLR::TPRICTRL_1
    }
    #[doc = "Checks if the value of the field is `TPRICTRL_2`"]
    #[inline]
    pub fn is_tprictrl_2(&self) -> bool {
        *self == TPRICTRLR::TPRICTRL_2
    }
}
#[doc = "Possible values of the field `PWRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSELR {
    #[doc = "Lowest power setting."]
    PWRSEL_0,
    #[doc = "Higher power setting than 0b0."]
    PWRSEL_1,
    #[doc = "Higher power setting than 0b1."]
    PWRSEL_2,
    #[doc = "Highest power setting."]
    PWRSEL_3,
}
impl PWRSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRSELR::PWRSEL_0 => 0,
            PWRSELR::PWRSEL_1 => 1,
            PWRSELR::PWRSEL_2 => 2,
            PWRSELR::PWRSEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRSELR {
        match value {
            0 => PWRSELR::PWRSEL_0,
            1 => PWRSELR::PWRSEL_1,
            2 => PWRSELR::PWRSEL_2,
            3 => PWRSELR::PWRSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWRSEL_0`"]
    #[inline]
    pub fn is_pwrsel_0(&self) -> bool {
        *self == PWRSELR::PWRSEL_0
    }
    #[doc = "Checks if the value of the field is `PWRSEL_1`"]
    #[inline]
    pub fn is_pwrsel_1(&self) -> bool {
        *self == PWRSELR::PWRSEL_1
    }
    #[doc = "Checks if the value of the field is `PWRSEL_2`"]
    #[inline]
    pub fn is_pwrsel_2(&self) -> bool {
        *self == PWRSELR::PWRSEL_2
    }
    #[doc = "Checks if the value of the field is `PWRSEL_3`"]
    #[inline]
    pub fn is_pwrsel_3(&self) -> bool {
        *self == PWRSELR::PWRSEL_3
    }
}
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "(Default) Option 1 setting."]
    REFSEL_0,
    #[doc = "Option 2 setting."]
    REFSEL_1,
    #[doc = "Option 3 setting."]
    REFSEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::REFSEL_0 => 0,
            REFSELR::REFSEL_1 => 1,
            REFSELR::REFSEL_2 => 2,
            REFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::REFSEL_0,
            1 => REFSELR::REFSEL_1,
            2 => REFSELR::REFSEL_2,
            i => REFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REFSEL_0`"]
    #[inline]
    pub fn is_refsel_0(&self) -> bool {
        *self == REFSELR::REFSEL_0
    }
    #[doc = "Checks if the value of the field is `REFSEL_1`"]
    #[inline]
    pub fn is_refsel_1(&self) -> bool {
        *self == REFSELR::REFSEL_1
    }
    #[doc = "Checks if the value of the field is `REFSEL_2`"]
    #[inline]
    pub fn is_refsel_2(&self) -> bool {
        *self == REFSELR::REFSEL_2
    }
}
#[doc = "Possible values of the field `TRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRESR {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will not be automatically resumed or restarted."]
    TRES_0,
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically resumed or restarted."]
    TRES_1,
}
impl TRESR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TRESR::TRES_0 => false,
            TRESR::TRES_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRESR {
        match value {
            false => TRESR::TRES_0,
            true => TRESR::TRES_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRES_0`"]
    #[inline]
    pub fn is_tres_0(&self) -> bool {
        *self == TRESR::TRES_0
    }
    #[doc = "Checks if the value of the field is `TRES_1`"]
    #[inline]
    pub fn is_tres_1(&self) -> bool {
        *self == TRESR::TRES_1
    }
}
#[doc = "Possible values of the field `TCMDRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCMDRESR {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically restarted."]
    TCMDRES_0,
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be resumed from the command executing before the exception."]
    TCMDRES_1,
}
impl TCMDRESR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TCMDRESR::TCMDRES_0 => false,
            TCMDRESR::TCMDRES_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCMDRESR {
        match value {
            false => TCMDRESR::TCMDRES_0,
            true => TCMDRESR::TCMDRES_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCMDRES_0`"]
    #[inline]
    pub fn is_tcmdres_0(&self) -> bool {
        *self == TCMDRESR::TCMDRES_0
    }
    #[doc = "Checks if the value of the field is `TCMDRES_1`"]
    #[inline]
    pub fn is_tcmdres_1(&self) -> bool {
        *self == TCMDRESR::TCMDRES_1
    }
}
#[doc = "Possible values of the field `HPT_EXDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPT_EXDIR {
    #[doc = "High priority trigger exceptions are enabled."]
    HPT_EXDI_0,
    #[doc = "High priority trigger exceptions are disabled."]
    HPT_EXDI_1,
}
impl HPT_EXDIR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HPT_EXDIR::HPT_EXDI_0 => false,
            HPT_EXDIR::HPT_EXDI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPT_EXDIR {
        match value {
            false => HPT_EXDIR::HPT_EXDI_0,
            true => HPT_EXDIR::HPT_EXDI_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPT_EXDI_0`"]
    #[inline]
    pub fn is_hpt_exdi_0(&self) -> bool {
        *self == HPT_EXDIR::HPT_EXDI_0
    }
    #[doc = "Checks if the value of the field is `HPT_EXDI_1`"]
    #[inline]
    pub fn is_hpt_exdi_1(&self) -> bool {
        *self == HPT_EXDIR::HPT_EXDI_1
    }
}
#[doc = r" Value of the field"]
pub struct PUDLYR {
    bits: u8,
}
impl PUDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PWREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRENR {
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    PWREN_0,
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). A single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog will remain pre-enabled, and no additional delays will be executed."]
    PWREN_1,
}
impl PWRENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PWRENR::PWREN_0 => false,
            PWRENR::PWREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRENR {
        match value {
            false => PWRENR::PWREN_0,
            true => PWRENR::PWREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWREN_0`"]
    #[inline]
    pub fn is_pwren_0(&self) -> bool {
        *self == PWRENR::PWREN_0
    }
    #[doc = "Checks if the value of the field is `PWREN_1`"]
    #[inline]
    pub fn is_pwren_1(&self) -> bool {
        *self == PWRENR::PWREN_1
    }
}
#[doc = "Values that can be written to the field `TPRICTRL`"]
pub enum TPRICTRLW {
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    TPRICTRL_0,
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\] will be ignored and the higher priority trigger will be serviced."]
    TPRICTRL_1,
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    TPRICTRL_2,
}
impl TPRICTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPRICTRLW::TPRICTRL_0 => 0,
            TPRICTRLW::TPRICTRL_1 => 1,
            TPRICTRLW::TPRICTRL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPRICTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _TPRICTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPRICTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    #[inline]
    pub fn tprictrl_0(self) -> &'a mut W {
        self.variant(TPRICTRLW::TPRICTRL_0)
    }
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\] will be ignored and the higher priority trigger will be serviced."]
    #[inline]
    pub fn tprictrl_1(self) -> &'a mut W {
        self.variant(TPRICTRLW::TPRICTRL_1)
    }
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    #[inline]
    pub fn tprictrl_2(self) -> &'a mut W {
        self.variant(TPRICTRLW::TPRICTRL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWRSEL`"]
pub enum PWRSELW {
    #[doc = "Lowest power setting."]
    PWRSEL_0,
    #[doc = "Higher power setting than 0b0."]
    PWRSEL_1,
    #[doc = "Higher power setting than 0b1."]
    PWRSEL_2,
    #[doc = "Highest power setting."]
    PWRSEL_3,
}
impl PWRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWRSELW::PWRSEL_0 => 0,
            PWRSELW::PWRSEL_1 => 1,
            PWRSELW::PWRSEL_2 => 2,
            PWRSELW::PWRSEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Lowest power setting."]
    #[inline]
    pub fn pwrsel_0(self) -> &'a mut W {
        self.variant(PWRSELW::PWRSEL_0)
    }
    #[doc = "Higher power setting than 0b0."]
    #[inline]
    pub fn pwrsel_1(self) -> &'a mut W {
        self.variant(PWRSELW::PWRSEL_1)
    }
    #[doc = "Higher power setting than 0b1."]
    #[inline]
    pub fn pwrsel_2(self) -> &'a mut W {
        self.variant(PWRSELW::PWRSEL_2)
    }
    #[doc = "Highest power setting."]
    #[inline]
    pub fn pwrsel_3(self) -> &'a mut W {
        self.variant(PWRSELW::PWRSEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "(Default) Option 1 setting."]
    REFSEL_0,
    #[doc = "Option 2 setting."]
    REFSEL_1,
    #[doc = "Option 3 setting."]
    REFSEL_2,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::REFSEL_0 => 0,
            REFSELW::REFSEL_1 => 1,
            REFSELW::REFSEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "(Default) Option 1 setting."]
    #[inline]
    pub fn refsel_0(self) -> &'a mut W {
        self.variant(REFSELW::REFSEL_0)
    }
    #[doc = "Option 2 setting."]
    #[inline]
    pub fn refsel_1(self) -> &'a mut W {
        self.variant(REFSELW::REFSEL_1)
    }
    #[doc = "Option 3 setting."]
    #[inline]
    pub fn refsel_2(self) -> &'a mut W {
        self.variant(REFSELW::REFSEL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRES`"]
pub enum TRESW {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will not be automatically resumed or restarted."]
    TRES_0,
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically resumed or restarted."]
    TRES_1,
}
impl TRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRESW::TRES_0 => false,
            TRESW::TRES_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRESW<'a> {
    w: &'a mut W,
}
impl<'a> _TRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will not be automatically resumed or restarted."]
    #[inline]
    pub fn tres_0(self) -> &'a mut W {
        self.variant(TRESW::TRES_0)
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically resumed or restarted."]
    #[inline]
    pub fn tres_1(self) -> &'a mut W {
        self.variant(TRESW::TRES_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCMDRES`"]
pub enum TCMDRESW {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically restarted."]
    TCMDRES_0,
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be resumed from the command executing before the exception."]
    TCMDRES_1,
}
impl TCMDRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCMDRESW::TCMDRES_0 => false,
            TCMDRESW::TCMDRES_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCMDRESW<'a> {
    w: &'a mut W,
}
impl<'a> _TCMDRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCMDRESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically restarted."]
    #[inline]
    pub fn tcmdres_0(self) -> &'a mut W {
        self.variant(TCMDRESW::TCMDRES_0)
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be resumed from the command executing before the exception."]
    #[inline]
    pub fn tcmdres_1(self) -> &'a mut W {
        self.variant(TCMDRESW::TCMDRES_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPT_EXDI`"]
pub enum HPT_EXDIW {
    #[doc = "High priority trigger exceptions are enabled."]
    HPT_EXDI_0,
    #[doc = "High priority trigger exceptions are disabled."]
    HPT_EXDI_1,
}
impl HPT_EXDIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPT_EXDIW::HPT_EXDI_0 => false,
            HPT_EXDIW::HPT_EXDI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPT_EXDIW<'a> {
    w: &'a mut W,
}
impl<'a> _HPT_EXDIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPT_EXDIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High priority trigger exceptions are enabled."]
    #[inline]
    pub fn hpt_exdi_0(self) -> &'a mut W {
        self.variant(HPT_EXDIW::HPT_EXDI_0)
    }
    #[doc = "High priority trigger exceptions are disabled."]
    #[inline]
    pub fn hpt_exdi_1(self) -> &'a mut W {
        self.variant(HPT_EXDIW::HPT_EXDI_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PUDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _PUDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWREN`"]
pub enum PWRENW {
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    PWREN_0,
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). A single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog will remain pre-enabled, and no additional delays will be executed."]
    PWREN_1,
}
impl PWRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRENW::PWREN_0 => false,
            PWRENW::PWREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    #[inline]
    pub fn pwren_0(self) -> &'a mut W {
        self.variant(PWRENW::PWREN_0)
    }
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). A single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog will remain pre-enabled, and no additional delays will be executed."]
    #[inline]
    pub fn pwren_1(self) -> &'a mut W {
        self.variant(PWRENW::PWREN_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - ADC trigger priority control"]
    #[inline]
    pub fn tprictrl(&self) -> TPRICTRLR {
        TPRICTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline]
    pub fn pwrsel(&self) -> PWRSELR {
        PWRSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Trigger Resume Enable"]
    #[inline]
    pub fn tres(&self) -> TRESR {
        TRESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Trigger Command Resume"]
    #[inline]
    pub fn tcmdres(&self) -> TCMDRESR {
        TCMDRESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - High Priority Trigger Exception Disable"]
    #[inline]
    pub fn hpt_exdi(&self) -> HPT_EXDIR {
        HPT_EXDIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Power Up Delay"]
    #[inline]
    pub fn pudly(&self) -> PUDLYR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PUDLYR { bits }
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline]
    pub fn pwren(&self) -> PWRENR {
        PWRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8388608 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - ADC trigger priority control"]
    #[inline]
    pub fn tprictrl(&mut self) -> _TPRICTRLW {
        _TPRICTRLW { w: self }
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline]
    pub fn pwrsel(&mut self) -> _PWRSELW {
        _PWRSELW { w: self }
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bit 8 - Trigger Resume Enable"]
    #[inline]
    pub fn tres(&mut self) -> _TRESW {
        _TRESW { w: self }
    }
    #[doc = "Bit 9 - Trigger Command Resume"]
    #[inline]
    pub fn tcmdres(&mut self) -> _TCMDRESW {
        _TCMDRESW { w: self }
    }
    #[doc = "Bit 10 - High Priority Trigger Exception Disable"]
    #[inline]
    pub fn hpt_exdi(&mut self) -> _HPT_EXDIW {
        _HPT_EXDIW { w: self }
    }
    #[doc = "Bits 16:23 - Power Up Delay"]
    #[inline]
    pub fn pudly(&mut self) -> _PUDLYW {
        _PUDLYW { w: self }
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline]
    pub fn pwren(&mut self) -> _PWRENW {
        _PWRENW { w: self }
    }
}
