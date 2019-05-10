#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
#[doc = "Possible values of the field `RDY0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY0R {
    #[doc = "Result FIFO 0 data level not above watermark level."]
    RDY0_0,
    #[doc = "Result FIFO 0 holding data above watermark level."]
    RDY0_1,
}
impl RDY0R {
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
            RDY0R::RDY0_0 => false,
            RDY0R::RDY0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDY0R {
        match value {
            false => RDY0R::RDY0_0,
            true => RDY0R::RDY0_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDY0_0`"]
    #[inline]
    pub fn is_rdy0_0(&self) -> bool {
        *self == RDY0R::RDY0_0
    }
    #[doc = "Checks if the value of the field is `RDY0_1`"]
    #[inline]
    pub fn is_rdy0_1(&self) -> bool {
        *self == RDY0R::RDY0_1
    }
}
#[doc = "Possible values of the field `FOF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOF0R {
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_1,
}
impl FOF0R {
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
            FOF0R::FOF0_0 => false,
            FOF0R::FOF0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOF0R {
        match value {
            false => FOF0R::FOF0_0,
            true => FOF0R::FOF0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOF0_0`"]
    #[inline]
    pub fn is_fof0_0(&self) -> bool {
        *self == FOF0R::FOF0_0
    }
    #[doc = "Checks if the value of the field is `FOF0_1`"]
    #[inline]
    pub fn is_fof0_1(&self) -> bool {
        *self == FOF0R::FOF0_1
    }
}
#[doc = "Possible values of the field `RDY1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY1R {
    #[doc = "Result FIFO1 data level not above watermark level."]
    RDY1_0,
    #[doc = "Result FIFO1 holding data above watermark level."]
    RDY1_1,
}
impl RDY1R {
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
            RDY1R::RDY1_0 => false,
            RDY1R::RDY1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDY1R {
        match value {
            false => RDY1R::RDY1_0,
            true => RDY1R::RDY1_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDY1_0`"]
    #[inline]
    pub fn is_rdy1_0(&self) -> bool {
        *self == RDY1R::RDY1_0
    }
    #[doc = "Checks if the value of the field is `RDY1_1`"]
    #[inline]
    pub fn is_rdy1_1(&self) -> bool {
        *self == RDY1R::RDY1_1
    }
}
#[doc = "Possible values of the field `FOF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOF1R {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_1,
}
impl FOF1R {
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
            FOF1R::FOF1_0 => false,
            FOF1R::FOF1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOF1R {
        match value {
            false => FOF1R::FOF1_0,
            true => FOF1R::FOF1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOF1_0`"]
    #[inline]
    pub fn is_fof1_0(&self) -> bool {
        *self == FOF1R::FOF1_0
    }
    #[doc = "Checks if the value of the field is `FOF1_1`"]
    #[inline]
    pub fn is_fof1_1(&self) -> bool {
        *self == FOF1R::FOF1_1
    }
}
#[doc = "Possible values of the field `TEXC_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEXC_INTR {
    #[doc = "No trigger exceptions have occurred."]
    TEXC_INT_0,
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    TEXC_INT_1,
}
impl TEXC_INTR {
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
            TEXC_INTR::TEXC_INT_0 => false,
            TEXC_INTR::TEXC_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEXC_INTR {
        match value {
            false => TEXC_INTR::TEXC_INT_0,
            true => TEXC_INTR::TEXC_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEXC_INT_0`"]
    #[inline]
    pub fn is_texc_int_0(&self) -> bool {
        *self == TEXC_INTR::TEXC_INT_0
    }
    #[doc = "Checks if the value of the field is `TEXC_INT_1`"]
    #[inline]
    pub fn is_texc_int_1(&self) -> bool {
        *self == TEXC_INTR::TEXC_INT_1
    }
}
#[doc = "Possible values of the field `TCOMP_INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCOMP_INTR {
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    TCOMP_INT_0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    TCOMP_INT_1,
}
impl TCOMP_INTR {
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
            TCOMP_INTR::TCOMP_INT_0 => false,
            TCOMP_INTR::TCOMP_INT_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCOMP_INTR {
        match value {
            false => TCOMP_INTR::TCOMP_INT_0,
            true => TCOMP_INTR::TCOMP_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCOMP_INT_0`"]
    #[inline]
    pub fn is_tcomp_int_0(&self) -> bool {
        *self == TCOMP_INTR::TCOMP_INT_0
    }
    #[doc = "Checks if the value of the field is `TCOMP_INT_1`"]
    #[inline]
    pub fn is_tcomp_int_1(&self) -> bool {
        *self == TCOMP_INTR::TCOMP_INT_1
    }
}
#[doc = "Possible values of the field `CAL_RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_RDYR {
    #[doc = "Calibration is incomplete or hasn't been ran."]
    CAL_RDY_0,
    #[doc = "The ADC is calibrated."]
    CAL_RDY_1,
}
impl CAL_RDYR {
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
            CAL_RDYR::CAL_RDY_0 => false,
            CAL_RDYR::CAL_RDY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAL_RDYR {
        match value {
            false => CAL_RDYR::CAL_RDY_0,
            true => CAL_RDYR::CAL_RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAL_RDY_0`"]
    #[inline]
    pub fn is_cal_rdy_0(&self) -> bool {
        *self == CAL_RDYR::CAL_RDY_0
    }
    #[doc = "Checks if the value of the field is `CAL_RDY_1`"]
    #[inline]
    pub fn is_cal_rdy_1(&self) -> bool {
        *self == CAL_RDYR::CAL_RDY_1
    }
}
#[doc = "Possible values of the field `ADC_ACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_ACTIVER {
    #[doc = "The ADC is IDLE. There are no pending triggers to service and no active commands are being processed."]
    ADC_ACTIVE_0,
    #[doc = "The ADC is processing a conversion, running through the power up delay, or servicing a trigger."]
    ADC_ACTIVE_1,
}
impl ADC_ACTIVER {
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
            ADC_ACTIVER::ADC_ACTIVE_0 => false,
            ADC_ACTIVER::ADC_ACTIVE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_ACTIVER {
        match value {
            false => ADC_ACTIVER::ADC_ACTIVE_0,
            true => ADC_ACTIVER::ADC_ACTIVE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_ACTIVE_0`"]
    #[inline]
    pub fn is_adc_active_0(&self) -> bool {
        *self == ADC_ACTIVER::ADC_ACTIVE_0
    }
    #[doc = "Checks if the value of the field is `ADC_ACTIVE_1`"]
    #[inline]
    pub fn is_adc_active_1(&self) -> bool {
        *self == ADC_ACTIVER::ADC_ACTIVE_1
    }
}
#[doc = "Possible values of the field `TRGACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGACTR {
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    TRGACT_0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    TRGACT_1,
    #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
    TRGACT_2,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_3,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_4,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_5,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_6,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_7,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_8,
    #[doc = "Command (sequence) from the associated Trigger number is currently being executed."]
    TRGACT_9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRGACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGACTR::TRGACT_0 => 0,
            TRGACTR::TRGACT_1 => 1,
            TRGACTR::TRGACT_2 => 2,
            TRGACTR::TRGACT_3 => 3,
            TRGACTR::TRGACT_4 => 4,
            TRGACTR::TRGACT_5 => 5,
            TRGACTR::TRGACT_6 => 6,
            TRGACTR::TRGACT_7 => 7,
            TRGACTR::TRGACT_8 => 8,
            TRGACTR::TRGACT_9 => 9,
            TRGACTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGACTR {
        match value {
            0 => TRGACTR::TRGACT_0,
            1 => TRGACTR::TRGACT_1,
            2 => TRGACTR::TRGACT_2,
            3 => TRGACTR::TRGACT_3,
            4 => TRGACTR::TRGACT_4,
            5 => TRGACTR::TRGACT_5,
            6 => TRGACTR::TRGACT_6,
            7 => TRGACTR::TRGACT_7,
            8 => TRGACTR::TRGACT_8,
            9 => TRGACTR::TRGACT_9,
            i => TRGACTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TRGACT_0`"]
    #[inline]
    pub fn is_trgact_0(&self) -> bool {
        *self == TRGACTR::TRGACT_0
    }
    #[doc = "Checks if the value of the field is `TRGACT_1`"]
    #[inline]
    pub fn is_trgact_1(&self) -> bool {
        *self == TRGACTR::TRGACT_1
    }
    #[doc = "Checks if the value of the field is `TRGACT_2`"]
    #[inline]
    pub fn is_trgact_2(&self) -> bool {
        *self == TRGACTR::TRGACT_2
    }
    #[doc = "Checks if the value of the field is `TRGACT_3`"]
    #[inline]
    pub fn is_trgact_3(&self) -> bool {
        *self == TRGACTR::TRGACT_3
    }
    #[doc = "Checks if the value of the field is `TRGACT_4`"]
    #[inline]
    pub fn is_trgact_4(&self) -> bool {
        *self == TRGACTR::TRGACT_4
    }
    #[doc = "Checks if the value of the field is `TRGACT_5`"]
    #[inline]
    pub fn is_trgact_5(&self) -> bool {
        *self == TRGACTR::TRGACT_5
    }
    #[doc = "Checks if the value of the field is `TRGACT_6`"]
    #[inline]
    pub fn is_trgact_6(&self) -> bool {
        *self == TRGACTR::TRGACT_6
    }
    #[doc = "Checks if the value of the field is `TRGACT_7`"]
    #[inline]
    pub fn is_trgact_7(&self) -> bool {
        *self == TRGACTR::TRGACT_7
    }
    #[doc = "Checks if the value of the field is `TRGACT_8`"]
    #[inline]
    pub fn is_trgact_8(&self) -> bool {
        *self == TRGACTR::TRGACT_8
    }
    #[doc = "Checks if the value of the field is `TRGACT_9`"]
    #[inline]
    pub fn is_trgact_9(&self) -> bool {
        *self == TRGACTR::TRGACT_9
    }
}
#[doc = "Possible values of the field `CMDACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDACTR {
    #[doc = "No command is currently in progress."]
    CMDACT_0,
    #[doc = "Command 1 currently being executed."]
    CMDACT_1,
    #[doc = "Command 2 currently being executed."]
    CMDACT_2,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_3,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_4,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_5,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_6,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_7,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_8,
    #[doc = "Associated command number is currently being executed."]
    CMDACT_9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDACTR::CMDACT_0 => 0,
            CMDACTR::CMDACT_1 => 1,
            CMDACTR::CMDACT_2 => 2,
            CMDACTR::CMDACT_3 => 3,
            CMDACTR::CMDACT_4 => 4,
            CMDACTR::CMDACT_5 => 5,
            CMDACTR::CMDACT_6 => 6,
            CMDACTR::CMDACT_7 => 7,
            CMDACTR::CMDACT_8 => 8,
            CMDACTR::CMDACT_9 => 9,
            CMDACTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDACTR {
        match value {
            0 => CMDACTR::CMDACT_0,
            1 => CMDACTR::CMDACT_1,
            2 => CMDACTR::CMDACT_2,
            3 => CMDACTR::CMDACT_3,
            4 => CMDACTR::CMDACT_4,
            5 => CMDACTR::CMDACT_5,
            6 => CMDACTR::CMDACT_6,
            7 => CMDACTR::CMDACT_7,
            8 => CMDACTR::CMDACT_8,
            9 => CMDACTR::CMDACT_9,
            i => CMDACTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMDACT_0`"]
    #[inline]
    pub fn is_cmdact_0(&self) -> bool {
        *self == CMDACTR::CMDACT_0
    }
    #[doc = "Checks if the value of the field is `CMDACT_1`"]
    #[inline]
    pub fn is_cmdact_1(&self) -> bool {
        *self == CMDACTR::CMDACT_1
    }
    #[doc = "Checks if the value of the field is `CMDACT_2`"]
    #[inline]
    pub fn is_cmdact_2(&self) -> bool {
        *self == CMDACTR::CMDACT_2
    }
    #[doc = "Checks if the value of the field is `CMDACT_3`"]
    #[inline]
    pub fn is_cmdact_3(&self) -> bool {
        *self == CMDACTR::CMDACT_3
    }
    #[doc = "Checks if the value of the field is `CMDACT_4`"]
    #[inline]
    pub fn is_cmdact_4(&self) -> bool {
        *self == CMDACTR::CMDACT_4
    }
    #[doc = "Checks if the value of the field is `CMDACT_5`"]
    #[inline]
    pub fn is_cmdact_5(&self) -> bool {
        *self == CMDACTR::CMDACT_5
    }
    #[doc = "Checks if the value of the field is `CMDACT_6`"]
    #[inline]
    pub fn is_cmdact_6(&self) -> bool {
        *self == CMDACTR::CMDACT_6
    }
    #[doc = "Checks if the value of the field is `CMDACT_7`"]
    #[inline]
    pub fn is_cmdact_7(&self) -> bool {
        *self == CMDACTR::CMDACT_7
    }
    #[doc = "Checks if the value of the field is `CMDACT_8`"]
    #[inline]
    pub fn is_cmdact_8(&self) -> bool {
        *self == CMDACTR::CMDACT_8
    }
    #[doc = "Checks if the value of the field is `CMDACT_9`"]
    #[inline]
    pub fn is_cmdact_9(&self) -> bool {
        *self == CMDACTR::CMDACT_9
    }
}
#[doc = "Values that can be written to the field `FOF0`"]
pub enum FOF0W {
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    FOF0_1,
}
impl FOF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOF0W::FOF0_0 => false,
            FOF0W::FOF0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOF0W<'a> {
    w: &'a mut W,
}
impl<'a> _FOF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn fof0_0(self) -> &'a mut W {
        self.variant(FOF0W::FOF0_0)
    }
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn fof0_1(self) -> &'a mut W {
        self.variant(FOF0W::FOF0_1)
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
#[doc = "Values that can be written to the field `FOF1`"]
pub enum FOF1W {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOF1_1,
}
impl FOF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOF1W::FOF1_0 => false,
            FOF1W::FOF1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOF1W<'a> {
    w: &'a mut W,
}
impl<'a> _FOF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn fof1_0(self) -> &'a mut W {
        self.variant(FOF1W::FOF1_0)
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn fof1_1(self) -> &'a mut W {
        self.variant(FOF1W::FOF1_1)
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
#[doc = "Values that can be written to the field `TEXC_INT`"]
pub enum TEXC_INTW {
    #[doc = "No trigger exceptions have occurred."]
    TEXC_INT_0,
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    TEXC_INT_1,
}
impl TEXC_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEXC_INTW::TEXC_INT_0 => false,
            TEXC_INTW::TEXC_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEXC_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _TEXC_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEXC_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No trigger exceptions have occurred."]
    #[inline]
    pub fn texc_int_0(self) -> &'a mut W {
        self.variant(TEXC_INTW::TEXC_INT_0)
    }
    #[doc = "A trigger exception has occurred and is pending acknowledgement."]
    #[inline]
    pub fn texc_int_1(self) -> &'a mut W {
        self.variant(TEXC_INTW::TEXC_INT_1)
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
#[doc = "Values that can be written to the field `TCOMP_INT`"]
pub enum TCOMP_INTW {
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    TCOMP_INT_0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    TCOMP_INT_1,
}
impl TCOMP_INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCOMP_INTW::TCOMP_INT_0 => false,
            TCOMP_INTW::TCOMP_INT_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCOMP_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _TCOMP_INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCOMP_INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Either IE\\[TCOMP_IE\\] is set to 0, or no trigger sequences have run to completion."]
    #[inline]
    pub fn tcomp_int_0(self) -> &'a mut W {
        self.variant(TCOMP_INTW::TCOMP_INT_0)
    }
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    #[inline]
    pub fn tcomp_int_1(self) -> &'a mut W {
        self.variant(TCOMP_INTW::TCOMP_INT_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Result FIFO 0 Ready Flag"]
    #[inline]
    pub fn rdy0(&self) -> RDY0R {
        RDY0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Flag"]
    #[inline]
    pub fn fof0(&self) -> FOF0R {
        FOF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Result FIFO1 Ready Flag"]
    #[inline]
    pub fn rdy1(&self) -> RDY1R {
        RDY1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Flag"]
    #[inline]
    pub fn fof1(&self) -> FOF1R {
        FOF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Interrupt Flag For High Priority Trigger Exception"]
    #[inline]
    pub fn texc_int(&self) -> TEXC_INTR {
        TEXC_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Interrupt Flag For Trigger Completion"]
    #[inline]
    pub fn tcomp_int(&self) -> TCOMP_INTR {
        TCOMP_INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Calibration Ready"]
    #[inline]
    pub fn cal_rdy(&self) -> CAL_RDYR {
        CAL_RDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - ADC Active"]
    #[inline]
    pub fn adc_active(&self) -> ADC_ACTIVER {
        ADC_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Trigger Active"]
    #[inline]
    pub fn trgact(&self) -> TRGACTR {
        TRGACTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Command Active"]
    #[inline]
    pub fn cmdact(&self) -> CMDACTR {
        CMDACTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 1 - Result FIFO 0 Overflow Flag"]
    #[inline]
    pub fn fof0(&mut self) -> _FOF0W {
        _FOF0W { w: self }
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Flag"]
    #[inline]
    pub fn fof1(&mut self) -> _FOF1W {
        _FOF1W { w: self }
    }
    #[doc = "Bit 8 - Interrupt Flag For High Priority Trigger Exception"]
    #[inline]
    pub fn texc_int(&mut self) -> _TEXC_INTW {
        _TEXC_INTW { w: self }
    }
    #[doc = "Bit 9 - Interrupt Flag For Trigger Completion"]
    #[inline]
    pub fn tcomp_int(&mut self) -> _TCOMP_INTW {
        _TCOMP_INTW { w: self }
    }
}
