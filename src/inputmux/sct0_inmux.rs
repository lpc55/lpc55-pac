#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCT0_INMUX {
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
#[doc = "Possible values of the field `INP_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INP_NR {
    #[doc = "SCT_GPI0 function selected from IOCON register"]
    VAL0,
    #[doc = "SCT_GPI1 function selected from IOCON register"]
    VAL1,
    #[doc = "SCT_GPI2 function selected from IOCON register"]
    VAL2,
    #[doc = "SCT_GPI3 function selected from IOCON register"]
    VAL3,
    #[doc = "SCT_GPI4 function selected from IOCON register"]
    VAL4,
    #[doc = "SCT_GPI5 function selected from IOCON register"]
    VAL5,
    #[doc = "SCT_GPI6 function selected from IOCON register"]
    VAL6,
    #[doc = "SCT_GPI7 function selected from IOCON register"]
    VAL7,
    #[doc = "T0_OUT0 ctimer 0 match\\[0\\] output"]
    VAL8,
    #[doc = "T1_OUT0 ctimer 1 match\\[0\\] output"]
    VAL9,
    #[doc = "T2_OUT0 ctimer 2 match\\[0\\] output"]
    VAL10,
    #[doc = "T3_OUT0 ctimer 3 match\\[0\\] output"]
    VAL11,
    #[doc = "T4_OUT0 ctimer 4 match\\[0\\] output"]
    VAL12,
    #[doc = "ADC_IRQ interrupt request from ADC"]
    VAL13,
    #[doc = "GPIOINT_BMATCH"]
    VAL14,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL15,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL16,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL17,
    #[doc = "I2S_SHARED_SCK\\[0\\] output from I2S pin sharing"]
    VAL18,
    #[doc = "I2S_SHARED_SCK\\[1\\] output from I2S pin sharing"]
    VAL19,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL20,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL21,
    #[doc = "ARM_TXEV interrupt event from cpu0 or cpu1"]
    VAL22,
    #[doc = "DEBUG_HALTED from cpu0 or cpu1"]
    VAL23,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
}
impl INP_NR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INP_NR::VAL0 => 0,
            INP_NR::VAL1 => 1,
            INP_NR::VAL2 => 2,
            INP_NR::VAL3 => 3,
            INP_NR::VAL4 => 4,
            INP_NR::VAL5 => 5,
            INP_NR::VAL6 => 6,
            INP_NR::VAL7 => 7,
            INP_NR::VAL8 => 8,
            INP_NR::VAL9 => 9,
            INP_NR::VAL10 => 10,
            INP_NR::VAL11 => 11,
            INP_NR::VAL12 => 12,
            INP_NR::VAL13 => 13,
            INP_NR::VAL14 => 14,
            INP_NR::VAL15 => 15,
            INP_NR::VAL16 => 16,
            INP_NR::VAL17 => 17,
            INP_NR::VAL18 => 18,
            INP_NR::VAL19 => 19,
            INP_NR::VAL20 => 20,
            INP_NR::VAL21 => 21,
            INP_NR::VAL22 => 22,
            INP_NR::VAL23 => 23,
            INP_NR::VAL24 => 24,
            INP_NR::VAL24 => 25,
            INP_NR::VAL24 => 26,
            INP_NR::VAL24 => 27,
            INP_NR::VAL24 => 28,
            INP_NR::VAL24 => 29,
            INP_NR::VAL24 => 30,
            INP_NR::VAL24 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INP_NR {
        match value {
            0 => INP_NR::VAL0,
            1 => INP_NR::VAL1,
            2 => INP_NR::VAL2,
            3 => INP_NR::VAL3,
            4 => INP_NR::VAL4,
            5 => INP_NR::VAL5,
            6 => INP_NR::VAL6,
            7 => INP_NR::VAL7,
            8 => INP_NR::VAL8,
            9 => INP_NR::VAL9,
            10 => INP_NR::VAL10,
            11 => INP_NR::VAL11,
            12 => INP_NR::VAL12,
            13 => INP_NR::VAL13,
            14 => INP_NR::VAL14,
            15 => INP_NR::VAL15,
            16 => INP_NR::VAL16,
            17 => INP_NR::VAL17,
            18 => INP_NR::VAL18,
            19 => INP_NR::VAL19,
            20 => INP_NR::VAL20,
            21 => INP_NR::VAL21,
            22 => INP_NR::VAL22,
            23 => INP_NR::VAL23,
            24 => INP_NR::VAL24,
            25 => INP_NR::VAL24,
            26 => INP_NR::VAL24,
            27 => INP_NR::VAL24,
            28 => INP_NR::VAL24,
            29 => INP_NR::VAL24,
            30 => INP_NR::VAL24,
            31 => INP_NR::VAL24,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline]
    pub fn is_val0(&self) -> bool {
        *self == INP_NR::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline]
    pub fn is_val1(&self) -> bool {
        *self == INP_NR::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline]
    pub fn is_val2(&self) -> bool {
        *self == INP_NR::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline]
    pub fn is_val3(&self) -> bool {
        *self == INP_NR::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline]
    pub fn is_val4(&self) -> bool {
        *self == INP_NR::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline]
    pub fn is_val5(&self) -> bool {
        *self == INP_NR::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline]
    pub fn is_val6(&self) -> bool {
        *self == INP_NR::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline]
    pub fn is_val7(&self) -> bool {
        *self == INP_NR::VAL7
    }
    #[doc = "Checks if the value of the field is `VAL8`"]
    #[inline]
    pub fn is_val8(&self) -> bool {
        *self == INP_NR::VAL8
    }
    #[doc = "Checks if the value of the field is `VAL9`"]
    #[inline]
    pub fn is_val9(&self) -> bool {
        *self == INP_NR::VAL9
    }
    #[doc = "Checks if the value of the field is `VAL10`"]
    #[inline]
    pub fn is_val10(&self) -> bool {
        *self == INP_NR::VAL10
    }
    #[doc = "Checks if the value of the field is `VAL11`"]
    #[inline]
    pub fn is_val11(&self) -> bool {
        *self == INP_NR::VAL11
    }
    #[doc = "Checks if the value of the field is `VAL12`"]
    #[inline]
    pub fn is_val12(&self) -> bool {
        *self == INP_NR::VAL12
    }
    #[doc = "Checks if the value of the field is `VAL13`"]
    #[inline]
    pub fn is_val13(&self) -> bool {
        *self == INP_NR::VAL13
    }
    #[doc = "Checks if the value of the field is `VAL14`"]
    #[inline]
    pub fn is_val14(&self) -> bool {
        *self == INP_NR::VAL14
    }
    #[doc = "Checks if the value of the field is `VAL15`"]
    #[inline]
    pub fn is_val15(&self) -> bool {
        *self == INP_NR::VAL15
    }
    #[doc = "Checks if the value of the field is `VAL16`"]
    #[inline]
    pub fn is_val16(&self) -> bool {
        *self == INP_NR::VAL16
    }
    #[doc = "Checks if the value of the field is `VAL17`"]
    #[inline]
    pub fn is_val17(&self) -> bool {
        *self == INP_NR::VAL17
    }
    #[doc = "Checks if the value of the field is `VAL18`"]
    #[inline]
    pub fn is_val18(&self) -> bool {
        *self == INP_NR::VAL18
    }
    #[doc = "Checks if the value of the field is `VAL19`"]
    #[inline]
    pub fn is_val19(&self) -> bool {
        *self == INP_NR::VAL19
    }
    #[doc = "Checks if the value of the field is `VAL20`"]
    #[inline]
    pub fn is_val20(&self) -> bool {
        *self == INP_NR::VAL20
    }
    #[doc = "Checks if the value of the field is `VAL21`"]
    #[inline]
    pub fn is_val21(&self) -> bool {
        *self == INP_NR::VAL21
    }
    #[doc = "Checks if the value of the field is `VAL22`"]
    #[inline]
    pub fn is_val22(&self) -> bool {
        *self == INP_NR::VAL22
    }
    #[doc = "Checks if the value of the field is `VAL23`"]
    #[inline]
    pub fn is_val23(&self) -> bool {
        *self == INP_NR::VAL23
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline]
    pub fn is_val24(&self) -> bool {
        *self == INP_NR::VAL24
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline]
    pub fn is_val24(&self) -> bool {
        *self == INP_NR::VAL24
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline]
    pub fn is_val24(&self) -> bool {
        *self == INP_NR::VAL24
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline]
    pub fn is_val24(&self) -> bool {
        *self == INP_NR::VAL24
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline]
    pub fn is_val24(&self) -> bool {
        *self == INP_NR::VAL24
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline]
    pub fn is_val24(&self) -> bool {
        *self == INP_NR::VAL24
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline]
    pub fn is_val24(&self) -> bool {
        *self == INP_NR::VAL24
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline]
    pub fn is_val24(&self) -> bool {
        *self == INP_NR::VAL24
    }
}
#[doc = "Values that can be written to the field `INP_N`"]
pub enum INP_NW {
    #[doc = "SCT_GPI0 function selected from IOCON register"]
    VAL0,
    #[doc = "SCT_GPI1 function selected from IOCON register"]
    VAL1,
    #[doc = "SCT_GPI2 function selected from IOCON register"]
    VAL2,
    #[doc = "SCT_GPI3 function selected from IOCON register"]
    VAL3,
    #[doc = "SCT_GPI4 function selected from IOCON register"]
    VAL4,
    #[doc = "SCT_GPI5 function selected from IOCON register"]
    VAL5,
    #[doc = "SCT_GPI6 function selected from IOCON register"]
    VAL6,
    #[doc = "SCT_GPI7 function selected from IOCON register"]
    VAL7,
    #[doc = "T0_OUT0 ctimer 0 match\\[0\\] output"]
    VAL8,
    #[doc = "T1_OUT0 ctimer 1 match\\[0\\] output"]
    VAL9,
    #[doc = "T2_OUT0 ctimer 2 match\\[0\\] output"]
    VAL10,
    #[doc = "T3_OUT0 ctimer 3 match\\[0\\] output"]
    VAL11,
    #[doc = "T4_OUT0 ctimer 4 match\\[0\\] output"]
    VAL12,
    #[doc = "ADC_IRQ interrupt request from ADC"]
    VAL13,
    #[doc = "GPIOINT_BMATCH"]
    VAL14,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL15,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL16,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL17,
    #[doc = "I2S_SHARED_SCK\\[0\\] output from I2S pin sharing"]
    VAL18,
    #[doc = "I2S_SHARED_SCK\\[1\\] output from I2S pin sharing"]
    VAL19,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL20,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL21,
    #[doc = "ARM_TXEV interrupt event from cpu0 or cpu1"]
    VAL22,
    #[doc = "DEBUG_HALTED from cpu0 or cpu1"]
    VAL23,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
    #[doc = "None"]
    VAL24,
}
impl INP_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INP_NW::VAL0 => 0,
            INP_NW::VAL1 => 1,
            INP_NW::VAL2 => 2,
            INP_NW::VAL3 => 3,
            INP_NW::VAL4 => 4,
            INP_NW::VAL5 => 5,
            INP_NW::VAL6 => 6,
            INP_NW::VAL7 => 7,
            INP_NW::VAL8 => 8,
            INP_NW::VAL9 => 9,
            INP_NW::VAL10 => 10,
            INP_NW::VAL11 => 11,
            INP_NW::VAL12 => 12,
            INP_NW::VAL13 => 13,
            INP_NW::VAL14 => 14,
            INP_NW::VAL15 => 15,
            INP_NW::VAL16 => 16,
            INP_NW::VAL17 => 17,
            INP_NW::VAL18 => 18,
            INP_NW::VAL19 => 19,
            INP_NW::VAL20 => 20,
            INP_NW::VAL21 => 21,
            INP_NW::VAL22 => 22,
            INP_NW::VAL23 => 23,
            INP_NW::VAL24 => 24,
            INP_NW::VAL24 => 25,
            INP_NW::VAL24 => 26,
            INP_NW::VAL24 => 27,
            INP_NW::VAL24 => 28,
            INP_NW::VAL24 => 29,
            INP_NW::VAL24 => 30,
            INP_NW::VAL24 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INP_NW<'a> {
    w: &'a mut W,
}
impl<'a> _INP_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INP_NW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SCT_GPI0 function selected from IOCON register"]
    #[inline]
    pub fn val0(self) -> &'a mut W {
        self.variant(INP_NW::VAL0)
    }
    #[doc = "SCT_GPI1 function selected from IOCON register"]
    #[inline]
    pub fn val1(self) -> &'a mut W {
        self.variant(INP_NW::VAL1)
    }
    #[doc = "SCT_GPI2 function selected from IOCON register"]
    #[inline]
    pub fn val2(self) -> &'a mut W {
        self.variant(INP_NW::VAL2)
    }
    #[doc = "SCT_GPI3 function selected from IOCON register"]
    #[inline]
    pub fn val3(self) -> &'a mut W {
        self.variant(INP_NW::VAL3)
    }
    #[doc = "SCT_GPI4 function selected from IOCON register"]
    #[inline]
    pub fn val4(self) -> &'a mut W {
        self.variant(INP_NW::VAL4)
    }
    #[doc = "SCT_GPI5 function selected from IOCON register"]
    #[inline]
    pub fn val5(self) -> &'a mut W {
        self.variant(INP_NW::VAL5)
    }
    #[doc = "SCT_GPI6 function selected from IOCON register"]
    #[inline]
    pub fn val6(self) -> &'a mut W {
        self.variant(INP_NW::VAL6)
    }
    #[doc = "SCT_GPI7 function selected from IOCON register"]
    #[inline]
    pub fn val7(self) -> &'a mut W {
        self.variant(INP_NW::VAL7)
    }
    #[doc = "T0_OUT0 ctimer 0 match\\[0\\] output"]
    #[inline]
    pub fn val8(self) -> &'a mut W {
        self.variant(INP_NW::VAL8)
    }
    #[doc = "T1_OUT0 ctimer 1 match\\[0\\] output"]
    #[inline]
    pub fn val9(self) -> &'a mut W {
        self.variant(INP_NW::VAL9)
    }
    #[doc = "T2_OUT0 ctimer 2 match\\[0\\] output"]
    #[inline]
    pub fn val10(self) -> &'a mut W {
        self.variant(INP_NW::VAL10)
    }
    #[doc = "T3_OUT0 ctimer 3 match\\[0\\] output"]
    #[inline]
    pub fn val11(self) -> &'a mut W {
        self.variant(INP_NW::VAL11)
    }
    #[doc = "T4_OUT0 ctimer 4 match\\[0\\] output"]
    #[inline]
    pub fn val12(self) -> &'a mut W {
        self.variant(INP_NW::VAL12)
    }
    #[doc = "ADC_IRQ interrupt request from ADC"]
    #[inline]
    pub fn val13(self) -> &'a mut W {
        self.variant(INP_NW::VAL13)
    }
    #[doc = "GPIOINT_BMATCH"]
    #[inline]
    pub fn val14(self) -> &'a mut W {
        self.variant(INP_NW::VAL14)
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline]
    pub fn val15(self) -> &'a mut W {
        self.variant(INP_NW::VAL15)
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline]
    pub fn val16(self) -> &'a mut W {
        self.variant(INP_NW::VAL16)
    }
    #[doc = "COMP_OUTPUT output from analog comparator"]
    #[inline]
    pub fn val17(self) -> &'a mut W {
        self.variant(INP_NW::VAL17)
    }
    #[doc = "I2S_SHARED_SCK\\[0\\] output from I2S pin sharing"]
    #[inline]
    pub fn val18(self) -> &'a mut W {
        self.variant(INP_NW::VAL18)
    }
    #[doc = "I2S_SHARED_SCK\\[1\\] output from I2S pin sharing"]
    #[inline]
    pub fn val19(self) -> &'a mut W {
        self.variant(INP_NW::VAL19)
    }
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    #[inline]
    pub fn val20(self) -> &'a mut W {
        self.variant(INP_NW::VAL20)
    }
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    #[inline]
    pub fn val21(self) -> &'a mut W {
        self.variant(INP_NW::VAL21)
    }
    #[doc = "ARM_TXEV interrupt event from cpu0 or cpu1"]
    #[inline]
    pub fn val22(self) -> &'a mut W {
        self.variant(INP_NW::VAL22)
    }
    #[doc = "DEBUG_HALTED from cpu0 or cpu1"]
    #[inline]
    pub fn val23(self) -> &'a mut W {
        self.variant(INP_NW::VAL23)
    }
    #[doc = "None"]
    #[inline]
    pub fn val24(self) -> &'a mut W {
        self.variant(INP_NW::VAL24)
    }
    #[doc = "None"]
    #[inline]
    pub fn val24(self) -> &'a mut W {
        self.variant(INP_NW::VAL24)
    }
    #[doc = "None"]
    #[inline]
    pub fn val24(self) -> &'a mut W {
        self.variant(INP_NW::VAL24)
    }
    #[doc = "None"]
    #[inline]
    pub fn val24(self) -> &'a mut W {
        self.variant(INP_NW::VAL24)
    }
    #[doc = "None"]
    #[inline]
    pub fn val24(self) -> &'a mut W {
        self.variant(INP_NW::VAL24)
    }
    #[doc = "None"]
    #[inline]
    pub fn val24(self) -> &'a mut W {
        self.variant(INP_NW::VAL24)
    }
    #[doc = "None"]
    #[inline]
    pub fn val24(self) -> &'a mut W {
        self.variant(INP_NW::VAL24)
    }
    #[doc = "None"]
    #[inline]
    pub fn val24(self) -> &'a mut W {
        self.variant(INP_NW::VAL24)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline]
    pub fn inp_n(&self) -> INP_NR {
        INP_NR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 31 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline]
    pub fn inp_n(&mut self) -> _INP_NW {
        _INP_NW { w: self }
    }
}
