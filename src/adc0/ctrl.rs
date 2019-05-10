#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `ADCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCENR {
    #[doc = "ADC is disabled."]
    ADCEN_0,
    #[doc = "ADC is enabled."]
    ADCEN_1,
}
impl ADCENR {
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
            ADCENR::ADCEN_0 => false,
            ADCENR::ADCEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCENR {
        match value {
            false => ADCENR::ADCEN_0,
            true => ADCENR::ADCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCEN_0`"]
    #[inline]
    pub fn is_adcen_0(&self) -> bool {
        *self == ADCENR::ADCEN_0
    }
    #[doc = "Checks if the value of the field is `ADCEN_1`"]
    #[inline]
    pub fn is_adcen_1(&self) -> bool {
        *self == ADCENR::ADCEN_1
    }
}
#[doc = "Possible values of the field `RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTR {
    #[doc = "ADC logic is not reset."]
    RST_0,
    #[doc = "ADC logic is reset."]
    RST_1,
}
impl RSTR {
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
            RSTR::RST_0 => false,
            RSTR::RST_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTR {
        match value {
            false => RSTR::RST_0,
            true => RSTR::RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RST_0`"]
    #[inline]
    pub fn is_rst_0(&self) -> bool {
        *self == RSTR::RST_0
    }
    #[doc = "Checks if the value of the field is `RST_1`"]
    #[inline]
    pub fn is_rst_1(&self) -> bool {
        *self == RSTR::RST_1
    }
}
#[doc = "Possible values of the field `DOZEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZENR {
    #[doc = "ADC is enabled in Doze mode."]
    DOZEN_0,
    #[doc = "ADC is disabled in Doze mode."]
    DOZEN_1,
}
impl DOZENR {
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
            DOZENR::DOZEN_0 => false,
            DOZENR::DOZEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZENR {
        match value {
            false => DOZENR::DOZEN_0,
            true => DOZENR::DOZEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEN_0`"]
    #[inline]
    pub fn is_dozen_0(&self) -> bool {
        *self == DOZENR::DOZEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEN_1`"]
    #[inline]
    pub fn is_dozen_1(&self) -> bool {
        *self == DOZENR::DOZEN_1
    }
}
#[doc = "Possible values of the field `CAL_REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_REQR {
    #[doc = "No request for auto-calibration has been made."]
    CAL_REQ_0,
    #[doc = "A request for auto-calibration has been made"]
    CAL_REQ_1,
}
impl CAL_REQR {
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
            CAL_REQR::CAL_REQ_0 => false,
            CAL_REQR::CAL_REQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAL_REQR {
        match value {
            false => CAL_REQR::CAL_REQ_0,
            true => CAL_REQR::CAL_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAL_REQ_0`"]
    #[inline]
    pub fn is_cal_req_0(&self) -> bool {
        *self == CAL_REQR::CAL_REQ_0
    }
    #[doc = "Checks if the value of the field is `CAL_REQ_1`"]
    #[inline]
    pub fn is_cal_req_1(&self) -> bool {
        *self == CAL_REQR::CAL_REQ_1
    }
}
#[doc = "Possible values of the field `CALOFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALOFSR {
    #[doc = "Calibration function disabled"]
    CALOFS_0,
    #[doc = "Request for offset calibration function"]
    CALOFS_1,
}
impl CALOFSR {
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
            CALOFSR::CALOFS_0 => false,
            CALOFSR::CALOFS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALOFSR {
        match value {
            false => CALOFSR::CALOFS_0,
            true => CALOFSR::CALOFS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALOFS_0`"]
    #[inline]
    pub fn is_calofs_0(&self) -> bool {
        *self == CALOFSR::CALOFS_0
    }
    #[doc = "Checks if the value of the field is `CALOFS_1`"]
    #[inline]
    pub fn is_calofs_1(&self) -> bool {
        *self == CALOFSR::CALOFS_1
    }
}
#[doc = "Possible values of the field `RSTFIFO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFIFO0R {
    #[doc = "No effect."]
    RSTFIFO0_0,
    #[doc = "FIFO 0 is reset."]
    RSTFIFO0_1,
}
impl RSTFIFO0R {
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
            RSTFIFO0R::RSTFIFO0_0 => false,
            RSTFIFO0R::RSTFIFO0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTFIFO0R {
        match value {
            false => RSTFIFO0R::RSTFIFO0_0,
            true => RSTFIFO0R::RSTFIFO0_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTFIFO0_0`"]
    #[inline]
    pub fn is_rstfifo0_0(&self) -> bool {
        *self == RSTFIFO0R::RSTFIFO0_0
    }
    #[doc = "Checks if the value of the field is `RSTFIFO0_1`"]
    #[inline]
    pub fn is_rstfifo0_1(&self) -> bool {
        *self == RSTFIFO0R::RSTFIFO0_1
    }
}
#[doc = "Possible values of the field `RSTFIFO1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFIFO1R {
    #[doc = "No effect."]
    RSTFIFO1_0,
    #[doc = "FIFO 1 is reset."]
    RSTFIFO1_1,
}
impl RSTFIFO1R {
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
            RSTFIFO1R::RSTFIFO1_0 => false,
            RSTFIFO1R::RSTFIFO1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTFIFO1R {
        match value {
            false => RSTFIFO1R::RSTFIFO1_0,
            true => RSTFIFO1R::RSTFIFO1_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSTFIFO1_0`"]
    #[inline]
    pub fn is_rstfifo1_0(&self) -> bool {
        *self == RSTFIFO1R::RSTFIFO1_0
    }
    #[doc = "Checks if the value of the field is `RSTFIFO1_1`"]
    #[inline]
    pub fn is_rstfifo1_1(&self) -> bool {
        *self == RSTFIFO1R::RSTFIFO1_1
    }
}
#[doc = "Possible values of the field `CAL_AVGS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_AVGSR {
    #[doc = "Single conversion."]
    CAL_AVGS_0,
    #[doc = "2 conversions averaged."]
    CAL_AVGS_1,
    #[doc = "4 conversions averaged."]
    CAL_AVGS_2,
    #[doc = "8 conversions averaged."]
    CAL_AVGS_3,
    #[doc = "16 conversions averaged."]
    CAL_AVGS_4,
    #[doc = "32 conversions averaged."]
    CAL_AVGS_5,
    #[doc = "64 conversions averaged."]
    CAL_AVGS_6,
    #[doc = "128 conversions averaged."]
    CAL_AVGS_7,
}
impl CAL_AVGSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAL_AVGSR::CAL_AVGS_0 => 0,
            CAL_AVGSR::CAL_AVGS_1 => 1,
            CAL_AVGSR::CAL_AVGS_2 => 2,
            CAL_AVGSR::CAL_AVGS_3 => 3,
            CAL_AVGSR::CAL_AVGS_4 => 4,
            CAL_AVGSR::CAL_AVGS_5 => 5,
            CAL_AVGSR::CAL_AVGS_6 => 6,
            CAL_AVGSR::CAL_AVGS_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAL_AVGSR {
        match value {
            0 => CAL_AVGSR::CAL_AVGS_0,
            1 => CAL_AVGSR::CAL_AVGS_1,
            2 => CAL_AVGSR::CAL_AVGS_2,
            3 => CAL_AVGSR::CAL_AVGS_3,
            4 => CAL_AVGSR::CAL_AVGS_4,
            5 => CAL_AVGSR::CAL_AVGS_5,
            6 => CAL_AVGSR::CAL_AVGS_6,
            7 => CAL_AVGSR::CAL_AVGS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_0`"]
    #[inline]
    pub fn is_cal_avgs_0(&self) -> bool {
        *self == CAL_AVGSR::CAL_AVGS_0
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_1`"]
    #[inline]
    pub fn is_cal_avgs_1(&self) -> bool {
        *self == CAL_AVGSR::CAL_AVGS_1
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_2`"]
    #[inline]
    pub fn is_cal_avgs_2(&self) -> bool {
        *self == CAL_AVGSR::CAL_AVGS_2
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_3`"]
    #[inline]
    pub fn is_cal_avgs_3(&self) -> bool {
        *self == CAL_AVGSR::CAL_AVGS_3
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_4`"]
    #[inline]
    pub fn is_cal_avgs_4(&self) -> bool {
        *self == CAL_AVGSR::CAL_AVGS_4
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_5`"]
    #[inline]
    pub fn is_cal_avgs_5(&self) -> bool {
        *self == CAL_AVGSR::CAL_AVGS_5
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_6`"]
    #[inline]
    pub fn is_cal_avgs_6(&self) -> bool {
        *self == CAL_AVGSR::CAL_AVGS_6
    }
    #[doc = "Checks if the value of the field is `CAL_AVGS_7`"]
    #[inline]
    pub fn is_cal_avgs_7(&self) -> bool {
        *self == CAL_AVGSR::CAL_AVGS_7
    }
}
#[doc = "Values that can be written to the field `ADCEN`"]
pub enum ADCENW {
    #[doc = "ADC is disabled."]
    ADCEN_0,
    #[doc = "ADC is enabled."]
    ADCEN_1,
}
impl ADCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCENW::ADCEN_0 => false,
            ADCENW::ADCEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC is disabled."]
    #[inline]
    pub fn adcen_0(self) -> &'a mut W {
        self.variant(ADCENW::ADCEN_0)
    }
    #[doc = "ADC is enabled."]
    #[inline]
    pub fn adcen_1(self) -> &'a mut W {
        self.variant(ADCENW::ADCEN_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RST`"]
pub enum RSTW {
    #[doc = "ADC logic is not reset."]
    RST_0,
    #[doc = "ADC logic is reset."]
    RST_1,
}
impl RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTW::RST_0 => false,
            RSTW::RST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC logic is not reset."]
    #[inline]
    pub fn rst_0(self) -> &'a mut W {
        self.variant(RSTW::RST_0)
    }
    #[doc = "ADC logic is reset."]
    #[inline]
    pub fn rst_1(self) -> &'a mut W {
        self.variant(RSTW::RST_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DOZEN`"]
pub enum DOZENW {
    #[doc = "ADC is enabled in Doze mode."]
    DOZEN_0,
    #[doc = "ADC is disabled in Doze mode."]
    DOZEN_1,
}
impl DOZENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZENW::DOZEN_0 => false,
            DOZENW::DOZEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADC is enabled in Doze mode."]
    #[inline]
    pub fn dozen_0(self) -> &'a mut W {
        self.variant(DOZENW::DOZEN_0)
    }
    #[doc = "ADC is disabled in Doze mode."]
    #[inline]
    pub fn dozen_1(self) -> &'a mut W {
        self.variant(DOZENW::DOZEN_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAL_REQ`"]
pub enum CAL_REQW {
    #[doc = "No request for auto-calibration has been made."]
    CAL_REQ_0,
    #[doc = "A request for auto-calibration has been made"]
    CAL_REQ_1,
}
impl CAL_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAL_REQW::CAL_REQ_0 => false,
            CAL_REQW::CAL_REQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAL_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _CAL_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAL_REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No request for auto-calibration has been made."]
    #[inline]
    pub fn cal_req_0(self) -> &'a mut W {
        self.variant(CAL_REQW::CAL_REQ_0)
    }
    #[doc = "A request for auto-calibration has been made"]
    #[inline]
    pub fn cal_req_1(self) -> &'a mut W {
        self.variant(CAL_REQW::CAL_REQ_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CALOFS`"]
pub enum CALOFSW {
    #[doc = "Calibration function disabled"]
    CALOFS_0,
    #[doc = "Request for offset calibration function"]
    CALOFS_1,
}
impl CALOFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CALOFSW::CALOFS_0 => false,
            CALOFSW::CALOFS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CALOFSW<'a> {
    w: &'a mut W,
}
impl<'a> _CALOFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CALOFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Calibration function disabled"]
    #[inline]
    pub fn calofs_0(self) -> &'a mut W {
        self.variant(CALOFSW::CALOFS_0)
    }
    #[doc = "Request for offset calibration function"]
    #[inline]
    pub fn calofs_1(self) -> &'a mut W {
        self.variant(CALOFSW::CALOFS_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTFIFO0`"]
pub enum RSTFIFO0W {
    #[doc = "No effect."]
    RSTFIFO0_0,
    #[doc = "FIFO 0 is reset."]
    RSTFIFO0_1,
}
impl RSTFIFO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTFIFO0W::RSTFIFO0_0 => false,
            RSTFIFO0W::RSTFIFO0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTFIFO0W<'a> {
    w: &'a mut W,
}
impl<'a> _RSTFIFO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTFIFO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn rstfifo0_0(self) -> &'a mut W {
        self.variant(RSTFIFO0W::RSTFIFO0_0)
    }
    #[doc = "FIFO 0 is reset."]
    #[inline]
    pub fn rstfifo0_1(self) -> &'a mut W {
        self.variant(RSTFIFO0W::RSTFIFO0_1)
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
#[doc = "Values that can be written to the field `RSTFIFO1`"]
pub enum RSTFIFO1W {
    #[doc = "No effect."]
    RSTFIFO1_0,
    #[doc = "FIFO 1 is reset."]
    RSTFIFO1_1,
}
impl RSTFIFO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTFIFO1W::RSTFIFO1_0 => false,
            RSTFIFO1W::RSTFIFO1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTFIFO1W<'a> {
    w: &'a mut W,
}
impl<'a> _RSTFIFO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTFIFO1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn rstfifo1_0(self) -> &'a mut W {
        self.variant(RSTFIFO1W::RSTFIFO1_0)
    }
    #[doc = "FIFO 1 is reset."]
    #[inline]
    pub fn rstfifo1_1(self) -> &'a mut W {
        self.variant(RSTFIFO1W::RSTFIFO1_1)
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
#[doc = "Values that can be written to the field `CAL_AVGS`"]
pub enum CAL_AVGSW {
    #[doc = "Single conversion."]
    CAL_AVGS_0,
    #[doc = "2 conversions averaged."]
    CAL_AVGS_1,
    #[doc = "4 conversions averaged."]
    CAL_AVGS_2,
    #[doc = "8 conversions averaged."]
    CAL_AVGS_3,
    #[doc = "16 conversions averaged."]
    CAL_AVGS_4,
    #[doc = "32 conversions averaged."]
    CAL_AVGS_5,
    #[doc = "64 conversions averaged."]
    CAL_AVGS_6,
    #[doc = "128 conversions averaged."]
    CAL_AVGS_7,
}
impl CAL_AVGSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAL_AVGSW::CAL_AVGS_0 => 0,
            CAL_AVGSW::CAL_AVGS_1 => 1,
            CAL_AVGSW::CAL_AVGS_2 => 2,
            CAL_AVGSW::CAL_AVGS_3 => 3,
            CAL_AVGSW::CAL_AVGS_4 => 4,
            CAL_AVGSW::CAL_AVGS_5 => 5,
            CAL_AVGSW::CAL_AVGS_6 => 6,
            CAL_AVGSW::CAL_AVGS_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAL_AVGSW<'a> {
    w: &'a mut W,
}
impl<'a> _CAL_AVGSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAL_AVGSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single conversion."]
    #[inline]
    pub fn cal_avgs_0(self) -> &'a mut W {
        self.variant(CAL_AVGSW::CAL_AVGS_0)
    }
    #[doc = "2 conversions averaged."]
    #[inline]
    pub fn cal_avgs_1(self) -> &'a mut W {
        self.variant(CAL_AVGSW::CAL_AVGS_1)
    }
    #[doc = "4 conversions averaged."]
    #[inline]
    pub fn cal_avgs_2(self) -> &'a mut W {
        self.variant(CAL_AVGSW::CAL_AVGS_2)
    }
    #[doc = "8 conversions averaged."]
    #[inline]
    pub fn cal_avgs_3(self) -> &'a mut W {
        self.variant(CAL_AVGSW::CAL_AVGS_3)
    }
    #[doc = "16 conversions averaged."]
    #[inline]
    pub fn cal_avgs_4(self) -> &'a mut W {
        self.variant(CAL_AVGSW::CAL_AVGS_4)
    }
    #[doc = "32 conversions averaged."]
    #[inline]
    pub fn cal_avgs_5(self) -> &'a mut W {
        self.variant(CAL_AVGSW::CAL_AVGS_5)
    }
    #[doc = "64 conversions averaged."]
    #[inline]
    pub fn cal_avgs_6(self) -> &'a mut W {
        self.variant(CAL_AVGSW::CAL_AVGS_6)
    }
    #[doc = "128 conversions averaged."]
    #[inline]
    pub fn cal_avgs_7(self) -> &'a mut W {
        self.variant(CAL_AVGSW::CAL_AVGS_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - ADC Enable"]
    #[inline]
    pub fn adcen(&self) -> ADCENR {
        ADCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&self) -> RSTR {
        RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline]
    pub fn dozen(&self) -> DOZENR {
        DOZENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Auto-Calibration Request"]
    #[inline]
    pub fn cal_req(&self) -> CAL_REQR {
        CAL_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Configure for offset calibration function"]
    #[inline]
    pub fn calofs(&self) -> CALOFSR {
        CALOFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Reset FIFO 0"]
    #[inline]
    pub fn rstfifo0(&self) -> RSTFIFO0R {
        RSTFIFO0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Reset FIFO 1"]
    #[inline]
    pub fn rstfifo1(&self) -> RSTFIFO1R {
        RSTFIFO1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Auto-Calibration Averages"]
    #[inline]
    pub fn cal_avgs(&self) -> CAL_AVGSR {
        CAL_AVGSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - ADC Enable"]
    #[inline]
    pub fn adcen(&mut self) -> _ADCENW {
        _ADCENW { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bit 2 - Doze Enable"]
    #[inline]
    pub fn dozen(&mut self) -> _DOZENW {
        _DOZENW { w: self }
    }
    #[doc = "Bit 3 - Auto-Calibration Request"]
    #[inline]
    pub fn cal_req(&mut self) -> _CAL_REQW {
        _CAL_REQW { w: self }
    }
    #[doc = "Bit 4 - Configure for offset calibration function"]
    #[inline]
    pub fn calofs(&mut self) -> _CALOFSW {
        _CALOFSW { w: self }
    }
    #[doc = "Bit 8 - Reset FIFO 0"]
    #[inline]
    pub fn rstfifo0(&mut self) -> _RSTFIFO0W {
        _RSTFIFO0W { w: self }
    }
    #[doc = "Bit 9 - Reset FIFO 1"]
    #[inline]
    pub fn rstfifo1(&mut self) -> _RSTFIFO1W {
        _RSTFIFO1W { w: self }
    }
    #[doc = "Bits 16:18 - Auto-Calibration Averages"]
    #[inline]
    pub fn cal_avgs(&mut self) -> _CAL_AVGSW {
        _CAL_AVGSW { w: self }
    }
}
