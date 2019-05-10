#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMDH10 {
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
#[doc = "Possible values of the field `WAIT_TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_TRIGR {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1,
}
impl WAIT_TRIGR {
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
            WAIT_TRIGR::WAIT_TRIG_0 => false,
            WAIT_TRIGR::WAIT_TRIG_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAIT_TRIGR {
        match value {
            false => WAIT_TRIGR::WAIT_TRIG_0,
            true => WAIT_TRIGR::WAIT_TRIG_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAIT_TRIG_0`"]
    #[inline]
    pub fn is_wait_trig_0(&self) -> bool {
        *self == WAIT_TRIGR::WAIT_TRIG_0
    }
    #[doc = "Checks if the value of the field is `WAIT_TRIG_1`"]
    #[inline]
    pub fn is_wait_trig_1(&self) -> bool {
        *self == WAIT_TRIGR::WAIT_TRIG_1
    }
}
#[doc = "Possible values of the field `LWI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LWIR {
    #[doc = "Auto channel increment disabled"]
    LWI_0,
    #[doc = "Auto channel increment enabled"]
    LWI_1,
}
impl LWIR {
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
            LWIR::LWI_0 => false,
            LWIR::LWI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LWIR {
        match value {
            false => LWIR::LWI_0,
            true => LWIR::LWI_1,
        }
    }
    #[doc = "Checks if the value of the field is `LWI_0`"]
    #[inline]
    pub fn is_lwi_0(&self) -> bool {
        *self == LWIR::LWI_0
    }
    #[doc = "Checks if the value of the field is `LWI_1`"]
    #[inline]
    pub fn is_lwi_1(&self) -> bool {
        *self == LWIR::LWI_1
    }
}
#[doc = "Possible values of the field `STS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STSR {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7,
}
impl STSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STSR::STS_0 => 0,
            STSR::STS_1 => 1,
            STSR::STS_2 => 2,
            STSR::STS_3 => 3,
            STSR::STS_4 => 4,
            STSR::STS_5 => 5,
            STSR::STS_6 => 6,
            STSR::STS_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STSR {
        match value {
            0 => STSR::STS_0,
            1 => STSR::STS_1,
            2 => STSR::STS_2,
            3 => STSR::STS_3,
            4 => STSR::STS_4,
            5 => STSR::STS_5,
            6 => STSR::STS_6,
            7 => STSR::STS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STS_0`"]
    #[inline]
    pub fn is_sts_0(&self) -> bool {
        *self == STSR::STS_0
    }
    #[doc = "Checks if the value of the field is `STS_1`"]
    #[inline]
    pub fn is_sts_1(&self) -> bool {
        *self == STSR::STS_1
    }
    #[doc = "Checks if the value of the field is `STS_2`"]
    #[inline]
    pub fn is_sts_2(&self) -> bool {
        *self == STSR::STS_2
    }
    #[doc = "Checks if the value of the field is `STS_3`"]
    #[inline]
    pub fn is_sts_3(&self) -> bool {
        *self == STSR::STS_3
    }
    #[doc = "Checks if the value of the field is `STS_4`"]
    #[inline]
    pub fn is_sts_4(&self) -> bool {
        *self == STSR::STS_4
    }
    #[doc = "Checks if the value of the field is `STS_5`"]
    #[inline]
    pub fn is_sts_5(&self) -> bool {
        *self == STSR::STS_5
    }
    #[doc = "Checks if the value of the field is `STS_6`"]
    #[inline]
    pub fn is_sts_6(&self) -> bool {
        *self == STSR::STS_6
    }
    #[doc = "Checks if the value of the field is `STS_7`"]
    #[inline]
    pub fn is_sts_7(&self) -> bool {
        *self == STSR::STS_7
    }
}
#[doc = "Possible values of the field `AVGS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVGSR {
    #[doc = "Single conversion."]
    AVGS_0,
    #[doc = "2 conversions averaged."]
    AVGS_1,
    #[doc = "4 conversions averaged."]
    AVGS_2,
    #[doc = "8 conversions averaged."]
    AVGS_3,
    #[doc = "16 conversions averaged."]
    AVGS_4,
    #[doc = "32 conversions averaged."]
    AVGS_5,
    #[doc = "64 conversions averaged."]
    AVGS_6,
    #[doc = "128 conversions averaged."]
    AVGS_7,
}
impl AVGSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AVGSR::AVGS_0 => 0,
            AVGSR::AVGS_1 => 1,
            AVGSR::AVGS_2 => 2,
            AVGSR::AVGS_3 => 3,
            AVGSR::AVGS_4 => 4,
            AVGSR::AVGS_5 => 5,
            AVGSR::AVGS_6 => 6,
            AVGSR::AVGS_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AVGSR {
        match value {
            0 => AVGSR::AVGS_0,
            1 => AVGSR::AVGS_1,
            2 => AVGSR::AVGS_2,
            3 => AVGSR::AVGS_3,
            4 => AVGSR::AVGS_4,
            5 => AVGSR::AVGS_5,
            6 => AVGSR::AVGS_6,
            7 => AVGSR::AVGS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVGS_0`"]
    #[inline]
    pub fn is_avgs_0(&self) -> bool {
        *self == AVGSR::AVGS_0
    }
    #[doc = "Checks if the value of the field is `AVGS_1`"]
    #[inline]
    pub fn is_avgs_1(&self) -> bool {
        *self == AVGSR::AVGS_1
    }
    #[doc = "Checks if the value of the field is `AVGS_2`"]
    #[inline]
    pub fn is_avgs_2(&self) -> bool {
        *self == AVGSR::AVGS_2
    }
    #[doc = "Checks if the value of the field is `AVGS_3`"]
    #[inline]
    pub fn is_avgs_3(&self) -> bool {
        *self == AVGSR::AVGS_3
    }
    #[doc = "Checks if the value of the field is `AVGS_4`"]
    #[inline]
    pub fn is_avgs_4(&self) -> bool {
        *self == AVGSR::AVGS_4
    }
    #[doc = "Checks if the value of the field is `AVGS_5`"]
    #[inline]
    pub fn is_avgs_5(&self) -> bool {
        *self == AVGSR::AVGS_5
    }
    #[doc = "Checks if the value of the field is `AVGS_6`"]
    #[inline]
    pub fn is_avgs_6(&self) -> bool {
        *self == AVGSR::AVGS_6
    }
    #[doc = "Checks if the value of the field is `AVGS_7`"]
    #[inline]
    pub fn is_avgs_7(&self) -> bool {
        *self == AVGSR::AVGS_7
    }
}
#[doc = "Possible values of the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPR {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOOPR::LOOP_0 => 0,
            LOOPR::LOOP_1 => 1,
            LOOPR::LOOP_2 => 2,
            LOOPR::LOOP_3 => 3,
            LOOPR::LOOP_4 => 4,
            LOOPR::LOOP_5 => 5,
            LOOPR::LOOP_6 => 6,
            LOOPR::LOOP_7 => 7,
            LOOPR::LOOP_8 => 8,
            LOOPR::LOOP_9 => 9,
            LOOPR::LOOP_15 => 15,
            LOOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOOPR {
        match value {
            0 => LOOPR::LOOP_0,
            1 => LOOPR::LOOP_1,
            2 => LOOPR::LOOP_2,
            3 => LOOPR::LOOP_3,
            4 => LOOPR::LOOP_4,
            5 => LOOPR::LOOP_5,
            6 => LOOPR::LOOP_6,
            7 => LOOPR::LOOP_7,
            8 => LOOPR::LOOP_8,
            9 => LOOPR::LOOP_9,
            15 => LOOPR::LOOP_15,
            i => LOOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOOP_0`"]
    #[inline]
    pub fn is_loop_0(&self) -> bool {
        *self == LOOPR::LOOP_0
    }
    #[doc = "Checks if the value of the field is `LOOP_1`"]
    #[inline]
    pub fn is_loop_1(&self) -> bool {
        *self == LOOPR::LOOP_1
    }
    #[doc = "Checks if the value of the field is `LOOP_2`"]
    #[inline]
    pub fn is_loop_2(&self) -> bool {
        *self == LOOPR::LOOP_2
    }
    #[doc = "Checks if the value of the field is `LOOP_3`"]
    #[inline]
    pub fn is_loop_3(&self) -> bool {
        *self == LOOPR::LOOP_3
    }
    #[doc = "Checks if the value of the field is `LOOP_4`"]
    #[inline]
    pub fn is_loop_4(&self) -> bool {
        *self == LOOPR::LOOP_4
    }
    #[doc = "Checks if the value of the field is `LOOP_5`"]
    #[inline]
    pub fn is_loop_5(&self) -> bool {
        *self == LOOPR::LOOP_5
    }
    #[doc = "Checks if the value of the field is `LOOP_6`"]
    #[inline]
    pub fn is_loop_6(&self) -> bool {
        *self == LOOPR::LOOP_6
    }
    #[doc = "Checks if the value of the field is `LOOP_7`"]
    #[inline]
    pub fn is_loop_7(&self) -> bool {
        *self == LOOPR::LOOP_7
    }
    #[doc = "Checks if the value of the field is `LOOP_8`"]
    #[inline]
    pub fn is_loop_8(&self) -> bool {
        *self == LOOPR::LOOP_8
    }
    #[doc = "Checks if the value of the field is `LOOP_9`"]
    #[inline]
    pub fn is_loop_9(&self) -> bool {
        *self == LOOPR::LOOP_9
    }
    #[doc = "Checks if the value of the field is `LOOP_15`"]
    #[inline]
    pub fn is_loop_15(&self) -> bool {
        *self == LOOPR::LOOP_15
    }
}
#[doc = "Possible values of the field `NEXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEXTR {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NEXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NEXTR::NEXT_0 => 0,
            NEXTR::NEXT_1 => 1,
            NEXTR::NEXT_2 => 2,
            NEXTR::NEXT_3 => 3,
            NEXTR::NEXT_4 => 4,
            NEXTR::NEXT_5 => 5,
            NEXTR::NEXT_6 => 6,
            NEXTR::NEXT_7 => 7,
            NEXTR::NEXT_8 => 8,
            NEXTR::NEXT_9 => 9,
            NEXTR::NEXT_15 => 15,
            NEXTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NEXTR {
        match value {
            0 => NEXTR::NEXT_0,
            1 => NEXTR::NEXT_1,
            2 => NEXTR::NEXT_2,
            3 => NEXTR::NEXT_3,
            4 => NEXTR::NEXT_4,
            5 => NEXTR::NEXT_5,
            6 => NEXTR::NEXT_6,
            7 => NEXTR::NEXT_7,
            8 => NEXTR::NEXT_8,
            9 => NEXTR::NEXT_9,
            15 => NEXTR::NEXT_15,
            i => NEXTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NEXT_0`"]
    #[inline]
    pub fn is_next_0(&self) -> bool {
        *self == NEXTR::NEXT_0
    }
    #[doc = "Checks if the value of the field is `NEXT_1`"]
    #[inline]
    pub fn is_next_1(&self) -> bool {
        *self == NEXTR::NEXT_1
    }
    #[doc = "Checks if the value of the field is `NEXT_2`"]
    #[inline]
    pub fn is_next_2(&self) -> bool {
        *self == NEXTR::NEXT_2
    }
    #[doc = "Checks if the value of the field is `NEXT_3`"]
    #[inline]
    pub fn is_next_3(&self) -> bool {
        *self == NEXTR::NEXT_3
    }
    #[doc = "Checks if the value of the field is `NEXT_4`"]
    #[inline]
    pub fn is_next_4(&self) -> bool {
        *self == NEXTR::NEXT_4
    }
    #[doc = "Checks if the value of the field is `NEXT_5`"]
    #[inline]
    pub fn is_next_5(&self) -> bool {
        *self == NEXTR::NEXT_5
    }
    #[doc = "Checks if the value of the field is `NEXT_6`"]
    #[inline]
    pub fn is_next_6(&self) -> bool {
        *self == NEXTR::NEXT_6
    }
    #[doc = "Checks if the value of the field is `NEXT_7`"]
    #[inline]
    pub fn is_next_7(&self) -> bool {
        *self == NEXTR::NEXT_7
    }
    #[doc = "Checks if the value of the field is `NEXT_8`"]
    #[inline]
    pub fn is_next_8(&self) -> bool {
        *self == NEXTR::NEXT_8
    }
    #[doc = "Checks if the value of the field is `NEXT_9`"]
    #[inline]
    pub fn is_next_9(&self) -> bool {
        *self == NEXTR::NEXT_9
    }
    #[doc = "Checks if the value of the field is `NEXT_15`"]
    #[inline]
    pub fn is_next_15(&self) -> bool {
        *self == NEXTR::NEXT_15
    }
}
#[doc = "Values that can be written to the field `WAIT_TRIG`"]
pub enum WAIT_TRIGW {
    #[doc = "This command will be automatically executed."]
    WAIT_TRIG_0,
    #[doc = "The active trigger must be asserted again before executing this command."]
    WAIT_TRIG_1,
}
impl WAIT_TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAIT_TRIGW::WAIT_TRIG_0 => false,
            WAIT_TRIGW::WAIT_TRIG_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAIT_TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _WAIT_TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAIT_TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This command will be automatically executed."]
    #[inline]
    pub fn wait_trig_0(self) -> &'a mut W {
        self.variant(WAIT_TRIGW::WAIT_TRIG_0)
    }
    #[doc = "The active trigger must be asserted again before executing this command."]
    #[inline]
    pub fn wait_trig_1(self) -> &'a mut W {
        self.variant(WAIT_TRIGW::WAIT_TRIG_1)
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
#[doc = "Values that can be written to the field `LWI`"]
pub enum LWIW {
    #[doc = "Auto channel increment disabled"]
    LWI_0,
    #[doc = "Auto channel increment enabled"]
    LWI_1,
}
impl LWIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LWIW::LWI_0 => false,
            LWIW::LWI_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LWIW<'a> {
    w: &'a mut W,
}
impl<'a> _LWIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LWIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto channel increment disabled"]
    #[inline]
    pub fn lwi_0(self) -> &'a mut W {
        self.variant(LWIW::LWI_0)
    }
    #[doc = "Auto channel increment enabled"]
    #[inline]
    pub fn lwi_1(self) -> &'a mut W {
        self.variant(LWIW::LWI_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STS`"]
pub enum STSW {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0,
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1,
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2,
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3,
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4,
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5,
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6,
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7,
}
impl STSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STSW::STS_0 => 0,
            STSW::STS_1 => 1,
            STSW::STS_2 => 2,
            STSW::STS_3 => 3,
            STSW::STS_4 => 4,
            STSW::STS_5 => 5,
            STSW::STS_6 => 6,
            STSW::STS_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STSW<'a> {
    w: &'a mut W,
}
impl<'a> _STSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    #[inline]
    pub fn sts_0(self) -> &'a mut W {
        self.variant(STSW::STS_0)
    }
    #[doc = "3 + 21 ADCK cycles; 5 ADCK cycles total sample time."]
    #[inline]
    pub fn sts_1(self) -> &'a mut W {
        self.variant(STSW::STS_1)
    }
    #[doc = "3 + 22 ADCK cycles; 7 ADCK cycles total sample time."]
    #[inline]
    pub fn sts_2(self) -> &'a mut W {
        self.variant(STSW::STS_2)
    }
    #[doc = "3 + 23 ADCK cycles; 11 ADCK cycles total sample time."]
    #[inline]
    pub fn sts_3(self) -> &'a mut W {
        self.variant(STSW::STS_3)
    }
    #[doc = "3 + 24 ADCK cycles; 19 ADCK cycles total sample time."]
    #[inline]
    pub fn sts_4(self) -> &'a mut W {
        self.variant(STSW::STS_4)
    }
    #[doc = "3 + 25 ADCK cycles; 35 ADCK cycles total sample time."]
    #[inline]
    pub fn sts_5(self) -> &'a mut W {
        self.variant(STSW::STS_5)
    }
    #[doc = "3 + 26 ADCK cycles; 67 ADCK cycles total sample time."]
    #[inline]
    pub fn sts_6(self) -> &'a mut W {
        self.variant(STSW::STS_6)
    }
    #[doc = "3 + 27 ADCK cycles; 131 ADCK cycles total sample time."]
    #[inline]
    pub fn sts_7(self) -> &'a mut W {
        self.variant(STSW::STS_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AVGS`"]
pub enum AVGSW {
    #[doc = "Single conversion."]
    AVGS_0,
    #[doc = "2 conversions averaged."]
    AVGS_1,
    #[doc = "4 conversions averaged."]
    AVGS_2,
    #[doc = "8 conversions averaged."]
    AVGS_3,
    #[doc = "16 conversions averaged."]
    AVGS_4,
    #[doc = "32 conversions averaged."]
    AVGS_5,
    #[doc = "64 conversions averaged."]
    AVGS_6,
    #[doc = "128 conversions averaged."]
    AVGS_7,
}
impl AVGSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AVGSW::AVGS_0 => 0,
            AVGSW::AVGS_1 => 1,
            AVGSW::AVGS_2 => 2,
            AVGSW::AVGS_3 => 3,
            AVGSW::AVGS_4 => 4,
            AVGSW::AVGS_5 => 5,
            AVGSW::AVGS_6 => 6,
            AVGSW::AVGS_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVGSW<'a> {
    w: &'a mut W,
}
impl<'a> _AVGSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVGSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single conversion."]
    #[inline]
    pub fn avgs_0(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_0)
    }
    #[doc = "2 conversions averaged."]
    #[inline]
    pub fn avgs_1(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_1)
    }
    #[doc = "4 conversions averaged."]
    #[inline]
    pub fn avgs_2(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_2)
    }
    #[doc = "8 conversions averaged."]
    #[inline]
    pub fn avgs_3(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_3)
    }
    #[doc = "16 conversions averaged."]
    #[inline]
    pub fn avgs_4(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_4)
    }
    #[doc = "32 conversions averaged."]
    #[inline]
    pub fn avgs_5(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_5)
    }
    #[doc = "64 conversions averaged."]
    #[inline]
    pub fn avgs_6(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_6)
    }
    #[doc = "128 conversions averaged."]
    #[inline]
    pub fn avgs_7(self) -> &'a mut W {
        self.variant(AVGSW::AVGS_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOOP`"]
pub enum LOOPW {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15,
}
impl LOOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOOPW::LOOP_0 => 0,
            LOOPW::LOOP_1 => 1,
            LOOPW::LOOP_2 => 2,
            LOOPW::LOOP_3 => 3,
            LOOPW::LOOP_4 => 4,
            LOOPW::LOOP_5 => 5,
            LOOPW::LOOP_6 => 6,
            LOOPW::LOOP_7 => 7,
            LOOPW::LOOP_8 => 8,
            LOOPW::LOOP_9 => 9,
            LOOPW::LOOP_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Looping not enabled. Command executes 1 time."]
    #[inline]
    pub fn loop_0(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_0)
    }
    #[doc = "Loop 1 time. Command executes 2 times."]
    #[inline]
    pub fn loop_1(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_1)
    }
    #[doc = "Loop 2 times. Command executes 3 times."]
    #[inline]
    pub fn loop_2(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_2)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline]
    pub fn loop_3(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_3)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline]
    pub fn loop_4(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_4)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline]
    pub fn loop_5(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_5)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline]
    pub fn loop_6(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_6)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline]
    pub fn loop_7(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_7)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline]
    pub fn loop_8(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_8)
    }
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    #[inline]
    pub fn loop_9(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_9)
    }
    #[doc = "Loop 15 times. Command executes 16 times."]
    #[inline]
    pub fn loop_15(self) -> &'a mut W {
        self.variant(LOOPW::LOOP_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NEXT`"]
pub enum NEXTW {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15,
}
impl NEXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NEXTW::NEXT_0 => 0,
            NEXTW::NEXT_1 => 1,
            NEXTW::NEXT_2 => 2,
            NEXTW::NEXT_3 => 3,
            NEXTW::NEXT_4 => 4,
            NEXTW::NEXT_5 => 5,
            NEXTW::NEXT_6 => 6,
            NEXTW::NEXT_7 => 7,
            NEXTW::NEXT_8 => 8,
            NEXTW::NEXT_9 => 9,
            NEXTW::NEXT_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _NEXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NEXTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    #[inline]
    pub fn next_0(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_0)
    }
    #[doc = "Select CMD1 command buffer register as next command."]
    #[inline]
    pub fn next_1(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_1)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline]
    pub fn next_2(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_2)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline]
    pub fn next_3(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_3)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline]
    pub fn next_4(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_4)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline]
    pub fn next_5(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_5)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline]
    pub fn next_6(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_6)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline]
    pub fn next_7(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_7)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline]
    pub fn next_8(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_8)
    }
    #[doc = "Select corresponding CMD command buffer register as next command"]
    #[inline]
    pub fn next_9(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_9)
    }
    #[doc = "Select CMD15 command buffer register as next command."]
    #[inline]
    pub fn next_15(self) -> &'a mut W {
        self.variant(NEXTW::NEXT_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 2 - Wait for trigger assertion before execution."]
    #[inline]
    pub fn wait_trig(&self) -> WAIT_TRIGR {
        WAIT_TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Loop with Increment"]
    #[inline]
    pub fn lwi(&self) -> LWIR {
        LWIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Sample Time Select"]
    #[inline]
    pub fn sts(&self) -> STSR {
        STSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Hardware Average Select"]
    #[inline]
    pub fn avgs(&self) -> AVGSR {
        AVGSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Loop Count Select"]
    #[inline]
    pub fn loop_(&self) -> LOOPR {
        LOOPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Next Command Select"]
    #[inline]
    pub fn next(&self) -> NEXTR {
        NEXTR::_from({
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
    #[doc = "Bit 2 - Wait for trigger assertion before execution."]
    #[inline]
    pub fn wait_trig(&mut self) -> _WAIT_TRIGW {
        _WAIT_TRIGW { w: self }
    }
    #[doc = "Bit 7 - Loop with Increment"]
    #[inline]
    pub fn lwi(&mut self) -> _LWIW {
        _LWIW { w: self }
    }
    #[doc = "Bits 8:10 - Sample Time Select"]
    #[inline]
    pub fn sts(&mut self) -> _STSW {
        _STSW { w: self }
    }
    #[doc = "Bits 12:14 - Hardware Average Select"]
    #[inline]
    pub fn avgs(&mut self) -> _AVGSW {
        _AVGSW { w: self }
    }
    #[doc = "Bits 16:19 - Loop Count Select"]
    #[inline]
    pub fn loop_(&mut self) -> _LOOPW {
        _LOOPW { w: self }
    }
    #[doc = "Bits 24:27 - Next Command Select"]
    #[inline]
    pub fn next(&mut self) -> _NEXTW {
        _NEXTW { w: self }
    }
}
