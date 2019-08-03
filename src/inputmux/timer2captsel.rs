#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMER2CAPTSEL {
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
#[doc = "Possible values of the field `CAPTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTSELR {
    #[doc = "CT_INP0 function selected from IOCON register"]
    VAL0,
    #[doc = "CT_INP1 function selected from IOCON register"]
    VAL1,
    #[doc = "CT_INP2 function selected from IOCON register"]
    VAL2,
    #[doc = "CT_INP3 function selected from IOCON register"]
    VAL3,
    #[doc = "CT_INP4 function selected from IOCON register"]
    VAL4,
    #[doc = "CT_INP5 function selected from IOCON register"]
    VAL5,
    #[doc = "CT_INP6 function selected from IOCON register"]
    VAL6,
    #[doc = "CT_INP7 function selected from IOCON register"]
    VAL7,
    #[doc = "CT_INP8 function selected from IOCON register"]
    VAL8,
    #[doc = "CT_INP9 function selected from IOCON register"]
    VAL9,
    #[doc = "CT_INP10 function selected from IOCON register"]
    VAL10,
    #[doc = "CT_INP11 function selected from IOCON register"]
    VAL11,
    #[doc = "CT_INP12 function selected from IOCON register"]
    VAL12,
    #[doc = "CT_INP13 function selected from IOCON register"]
    VAL13,
    #[doc = "CT_INP14 function selected from IOCON register"]
    VAL14,
    #[doc = "CT_INP15 function selected from IOCON register"]
    VAL15,
    #[doc = "CT_INP16 function selected from IOCON register"]
    VAL16,
    #[doc = "CT_INP17 function selected from IOCON register"]
    VAL17,
    #[doc = "CT_INP18 function selected from IOCON register"]
    VAL18,
    #[doc = "CT_INP19 function selected from IOCON register"]
    VAL19,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL20,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL21,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL22,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL23,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL24,
    #[doc = "None"]
    VAL25,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CAPTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAPTSELR::VAL0 => 0,
            CAPTSELR::VAL1 => 1,
            CAPTSELR::VAL2 => 2,
            CAPTSELR::VAL3 => 3,
            CAPTSELR::VAL4 => 4,
            CAPTSELR::VAL5 => 5,
            CAPTSELR::VAL6 => 6,
            CAPTSELR::VAL7 => 7,
            CAPTSELR::VAL8 => 8,
            CAPTSELR::VAL9 => 9,
            CAPTSELR::VAL10 => 10,
            CAPTSELR::VAL11 => 11,
            CAPTSELR::VAL12 => 12,
            CAPTSELR::VAL13 => 13,
            CAPTSELR::VAL14 => 14,
            CAPTSELR::VAL15 => 15,
            CAPTSELR::VAL16 => 16,
            CAPTSELR::VAL17 => 17,
            CAPTSELR::VAL18 => 18,
            CAPTSELR::VAL19 => 19,
            CAPTSELR::VAL20 => 20,
            CAPTSELR::VAL21 => 21,
            CAPTSELR::VAL22 => 22,
            CAPTSELR::VAL23 => 23,
            CAPTSELR::VAL24 => 24,
            CAPTSELR::VAL25 => 25,
            CAPTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAPTSELR {
        match value {
            0 => CAPTSELR::VAL0,
            1 => CAPTSELR::VAL1,
            2 => CAPTSELR::VAL2,
            3 => CAPTSELR::VAL3,
            4 => CAPTSELR::VAL4,
            5 => CAPTSELR::VAL5,
            6 => CAPTSELR::VAL6,
            7 => CAPTSELR::VAL7,
            8 => CAPTSELR::VAL8,
            9 => CAPTSELR::VAL9,
            10 => CAPTSELR::VAL10,
            11 => CAPTSELR::VAL11,
            12 => CAPTSELR::VAL12,
            13 => CAPTSELR::VAL13,
            14 => CAPTSELR::VAL14,
            15 => CAPTSELR::VAL15,
            16 => CAPTSELR::VAL16,
            17 => CAPTSELR::VAL17,
            18 => CAPTSELR::VAL18,
            19 => CAPTSELR::VAL19,
            20 => CAPTSELR::VAL20,
            21 => CAPTSELR::VAL21,
            22 => CAPTSELR::VAL22,
            23 => CAPTSELR::VAL23,
            24 => CAPTSELR::VAL24,
            25 => CAPTSELR::VAL25,
            i => CAPTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline]
    pub fn is_val0(&self) -> bool {
        *self == CAPTSELR::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline]
    pub fn is_val1(&self) -> bool {
        *self == CAPTSELR::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline]
    pub fn is_val2(&self) -> bool {
        *self == CAPTSELR::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline]
    pub fn is_val3(&self) -> bool {
        *self == CAPTSELR::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline]
    pub fn is_val4(&self) -> bool {
        *self == CAPTSELR::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline]
    pub fn is_val5(&self) -> bool {
        *self == CAPTSELR::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline]
    pub fn is_val6(&self) -> bool {
        *self == CAPTSELR::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline]
    pub fn is_val7(&self) -> bool {
        *self == CAPTSELR::VAL7
    }
    #[doc = "Checks if the value of the field is `VAL8`"]
    #[inline]
    pub fn is_val8(&self) -> bool {
        *self == CAPTSELR::VAL8
    }
    #[doc = "Checks if the value of the field is `VAL9`"]
    #[inline]
    pub fn is_val9(&self) -> bool {
        *self == CAPTSELR::VAL9
    }
    #[doc = "Checks if the value of the field is `VAL10`"]
    #[inline]
    pub fn is_val10(&self) -> bool {
        *self == CAPTSELR::VAL10
    }
    #[doc = "Checks if the value of the field is `VAL11`"]
    #[inline]
    pub fn is_val11(&self) -> bool {
        *self == CAPTSELR::VAL11
    }
    #[doc = "Checks if the value of the field is `VAL12`"]
    #[inline]
    pub fn is_val12(&self) -> bool {
        *self == CAPTSELR::VAL12
    }
    #[doc = "Checks if the value of the field is `VAL13`"]
    #[inline]
    pub fn is_val13(&self) -> bool {
        *self == CAPTSELR::VAL13
    }
    #[doc = "Checks if the value of the field is `VAL14`"]
    #[inline]
    pub fn is_val14(&self) -> bool {
        *self == CAPTSELR::VAL14
    }
    #[doc = "Checks if the value of the field is `VAL15`"]
    #[inline]
    pub fn is_val15(&self) -> bool {
        *self == CAPTSELR::VAL15
    }
    #[doc = "Checks if the value of the field is `VAL16`"]
    #[inline]
    pub fn is_val16(&self) -> bool {
        *self == CAPTSELR::VAL16
    }
    #[doc = "Checks if the value of the field is `VAL17`"]
    #[inline]
    pub fn is_val17(&self) -> bool {
        *self == CAPTSELR::VAL17
    }
    #[doc = "Checks if the value of the field is `VAL18`"]
    #[inline]
    pub fn is_val18(&self) -> bool {
        *self == CAPTSELR::VAL18
    }
    #[doc = "Checks if the value of the field is `VAL19`"]
    #[inline]
    pub fn is_val19(&self) -> bool {
        *self == CAPTSELR::VAL19
    }
    #[doc = "Checks if the value of the field is `VAL20`"]
    #[inline]
    pub fn is_val20(&self) -> bool {
        *self == CAPTSELR::VAL20
    }
    #[doc = "Checks if the value of the field is `VAL21`"]
    #[inline]
    pub fn is_val21(&self) -> bool {
        *self == CAPTSELR::VAL21
    }
    #[doc = "Checks if the value of the field is `VAL22`"]
    #[inline]
    pub fn is_val22(&self) -> bool {
        *self == CAPTSELR::VAL22
    }
    #[doc = "Checks if the value of the field is `VAL23`"]
    #[inline]
    pub fn is_val23(&self) -> bool {
        *self == CAPTSELR::VAL23
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline]
    pub fn is_val24(&self) -> bool {
        *self == CAPTSELR::VAL24
    }
    #[doc = "Checks if the value of the field is `VAL25`"]
    #[inline]
    pub fn is_val25(&self) -> bool {
        *self == CAPTSELR::VAL25
    }
}
#[doc = "Values that can be written to the field `CAPTSEL`"]
pub enum CAPTSELW {
    #[doc = "CT_INP0 function selected from IOCON register"]
    VAL0,
    #[doc = "CT_INP1 function selected from IOCON register"]
    VAL1,
    #[doc = "CT_INP2 function selected from IOCON register"]
    VAL2,
    #[doc = "CT_INP3 function selected from IOCON register"]
    VAL3,
    #[doc = "CT_INP4 function selected from IOCON register"]
    VAL4,
    #[doc = "CT_INP5 function selected from IOCON register"]
    VAL5,
    #[doc = "CT_INP6 function selected from IOCON register"]
    VAL6,
    #[doc = "CT_INP7 function selected from IOCON register"]
    VAL7,
    #[doc = "CT_INP8 function selected from IOCON register"]
    VAL8,
    #[doc = "CT_INP9 function selected from IOCON register"]
    VAL9,
    #[doc = "CT_INP10 function selected from IOCON register"]
    VAL10,
    #[doc = "CT_INP11 function selected from IOCON register"]
    VAL11,
    #[doc = "CT_INP12 function selected from IOCON register"]
    VAL12,
    #[doc = "CT_INP13 function selected from IOCON register"]
    VAL13,
    #[doc = "CT_INP14 function selected from IOCON register"]
    VAL14,
    #[doc = "CT_INP15 function selected from IOCON register"]
    VAL15,
    #[doc = "CT_INP16 function selected from IOCON register"]
    VAL16,
    #[doc = "CT_INP17 function selected from IOCON register"]
    VAL17,
    #[doc = "CT_INP18 function selected from IOCON register"]
    VAL18,
    #[doc = "CT_INP19 function selected from IOCON register"]
    VAL19,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL20,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL21,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL22,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL23,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL24,
    #[doc = "None"]
    VAL25,
}
impl CAPTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAPTSELW::VAL0 => 0,
            CAPTSELW::VAL1 => 1,
            CAPTSELW::VAL2 => 2,
            CAPTSELW::VAL3 => 3,
            CAPTSELW::VAL4 => 4,
            CAPTSELW::VAL5 => 5,
            CAPTSELW::VAL6 => 6,
            CAPTSELW::VAL7 => 7,
            CAPTSELW::VAL8 => 8,
            CAPTSELW::VAL9 => 9,
            CAPTSELW::VAL10 => 10,
            CAPTSELW::VAL11 => 11,
            CAPTSELW::VAL12 => 12,
            CAPTSELW::VAL13 => 13,
            CAPTSELW::VAL14 => 14,
            CAPTSELW::VAL15 => 15,
            CAPTSELW::VAL16 => 16,
            CAPTSELW::VAL17 => 17,
            CAPTSELW::VAL18 => 18,
            CAPTSELW::VAL19 => 19,
            CAPTSELW::VAL20 => 20,
            CAPTSELW::VAL21 => 21,
            CAPTSELW::VAL22 => 22,
            CAPTSELW::VAL23 => 23,
            CAPTSELW::VAL24 => 24,
            CAPTSELW::VAL25 => 25,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CT_INP0 function selected from IOCON register"]
    #[inline]
    pub fn val0(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL0)
    }
    #[doc = "CT_INP1 function selected from IOCON register"]
    #[inline]
    pub fn val1(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL1)
    }
    #[doc = "CT_INP2 function selected from IOCON register"]
    #[inline]
    pub fn val2(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL2)
    }
    #[doc = "CT_INP3 function selected from IOCON register"]
    #[inline]
    pub fn val3(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL3)
    }
    #[doc = "CT_INP4 function selected from IOCON register"]
    #[inline]
    pub fn val4(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL4)
    }
    #[doc = "CT_INP5 function selected from IOCON register"]
    #[inline]
    pub fn val5(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL5)
    }
    #[doc = "CT_INP6 function selected from IOCON register"]
    #[inline]
    pub fn val6(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL6)
    }
    #[doc = "CT_INP7 function selected from IOCON register"]
    #[inline]
    pub fn val7(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL7)
    }
    #[doc = "CT_INP8 function selected from IOCON register"]
    #[inline]
    pub fn val8(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL8)
    }
    #[doc = "CT_INP9 function selected from IOCON register"]
    #[inline]
    pub fn val9(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL9)
    }
    #[doc = "CT_INP10 function selected from IOCON register"]
    #[inline]
    pub fn val10(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL10)
    }
    #[doc = "CT_INP11 function selected from IOCON register"]
    #[inline]
    pub fn val11(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL11)
    }
    #[doc = "CT_INP12 function selected from IOCON register"]
    #[inline]
    pub fn val12(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL12)
    }
    #[doc = "CT_INP13 function selected from IOCON register"]
    #[inline]
    pub fn val13(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL13)
    }
    #[doc = "CT_INP14 function selected from IOCON register"]
    #[inline]
    pub fn val14(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL14)
    }
    #[doc = "CT_INP15 function selected from IOCON register"]
    #[inline]
    pub fn val15(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL15)
    }
    #[doc = "CT_INP16 function selected from IOCON register"]
    #[inline]
    pub fn val16(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL16)
    }
    #[doc = "CT_INP17 function selected from IOCON register"]
    #[inline]
    pub fn val17(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL17)
    }
    #[doc = "CT_INP18 function selected from IOCON register"]
    #[inline]
    pub fn val18(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL18)
    }
    #[doc = "CT_INP19 function selected from IOCON register"]
    #[inline]
    pub fn val19(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL19)
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline]
    pub fn val20(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL20)
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline]
    pub fn val21(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL21)
    }
    #[doc = "COMP_OUTPUT output from analog comparator"]
    #[inline]
    pub fn val22(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL22)
    }
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    #[inline]
    pub fn val23(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL23)
    }
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    #[inline]
    pub fn val24(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL24)
    }
    #[doc = "None"]
    #[inline]
    pub fn val25(self) -> &'a mut W {
        self.variant(CAPTSELW::VAL25)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:4 - Input number to TIMER2 capture inputs 0 to 4"]
    #[inline]
    pub fn captsel(&self) -> CAPTSELR {
        CAPTSELR::_from({
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
    #[doc = "Bits 0:4 - Input number to TIMER2 capture inputs 0 to 4"]
    #[inline]
    pub fn captsel(&mut self) -> _CAPTSELW {
        _CAPTSELW { w: self }
    }
}
