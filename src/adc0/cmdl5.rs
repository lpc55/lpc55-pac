#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMDL5 {
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
#[doc = "Possible values of the field `ADCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCHR {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCHR::ADCH_0 => 0,
            ADCHR::ADCH_1 => 1,
            ADCHR::ADCH_2 => 2,
            ADCHR::ADCH_3 => 3,
            ADCHR::ADCH_4 => 4,
            ADCHR::ADCH_5 => 5,
            ADCHR::ADCH_6 => 6,
            ADCHR::ADCH_7 => 7,
            ADCHR::ADCH_8 => 8,
            ADCHR::ADCH_9 => 9,
            ADCHR::ADCH_30 => 30,
            ADCHR::ADCH_31 => 31,
            ADCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCHR {
        match value {
            0 => ADCHR::ADCH_0,
            1 => ADCHR::ADCH_1,
            2 => ADCHR::ADCH_2,
            3 => ADCHR::ADCH_3,
            4 => ADCHR::ADCH_4,
            5 => ADCHR::ADCH_5,
            6 => ADCHR::ADCH_6,
            7 => ADCHR::ADCH_7,
            8 => ADCHR::ADCH_8,
            9 => ADCHR::ADCH_9,
            30 => ADCHR::ADCH_30,
            31 => ADCHR::ADCH_31,
            i => ADCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCH_0`"]
    #[inline]
    pub fn is_adch_0(&self) -> bool {
        *self == ADCHR::ADCH_0
    }
    #[doc = "Checks if the value of the field is `ADCH_1`"]
    #[inline]
    pub fn is_adch_1(&self) -> bool {
        *self == ADCHR::ADCH_1
    }
    #[doc = "Checks if the value of the field is `ADCH_2`"]
    #[inline]
    pub fn is_adch_2(&self) -> bool {
        *self == ADCHR::ADCH_2
    }
    #[doc = "Checks if the value of the field is `ADCH_3`"]
    #[inline]
    pub fn is_adch_3(&self) -> bool {
        *self == ADCHR::ADCH_3
    }
    #[doc = "Checks if the value of the field is `ADCH_4`"]
    #[inline]
    pub fn is_adch_4(&self) -> bool {
        *self == ADCHR::ADCH_4
    }
    #[doc = "Checks if the value of the field is `ADCH_5`"]
    #[inline]
    pub fn is_adch_5(&self) -> bool {
        *self == ADCHR::ADCH_5
    }
    #[doc = "Checks if the value of the field is `ADCH_6`"]
    #[inline]
    pub fn is_adch_6(&self) -> bool {
        *self == ADCHR::ADCH_6
    }
    #[doc = "Checks if the value of the field is `ADCH_7`"]
    #[inline]
    pub fn is_adch_7(&self) -> bool {
        *self == ADCHR::ADCH_7
    }
    #[doc = "Checks if the value of the field is `ADCH_8`"]
    #[inline]
    pub fn is_adch_8(&self) -> bool {
        *self == ADCHR::ADCH_8
    }
    #[doc = "Checks if the value of the field is `ADCH_9`"]
    #[inline]
    pub fn is_adch_9(&self) -> bool {
        *self == ADCHR::ADCH_9
    }
    #[doc = "Checks if the value of the field is `ADCH_30`"]
    #[inline]
    pub fn is_adch_30(&self) -> bool {
        *self == ADCHR::ADCH_30
    }
    #[doc = "Checks if the value of the field is `ADCH_31`"]
    #[inline]
    pub fn is_adch_31(&self) -> bool {
        *self == ADCHR::ADCH_31
    }
}
#[doc = "Possible values of the field `CTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTYPER {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3,
}
impl CTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTYPER::CTYPE_0 => 0,
            CTYPER::CTYPE_1 => 1,
            CTYPER::CTYPE_2 => 2,
            CTYPER::CTYPE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTYPER {
        match value {
            0 => CTYPER::CTYPE_0,
            1 => CTYPER::CTYPE_1,
            2 => CTYPER::CTYPE_2,
            3 => CTYPER::CTYPE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTYPE_0`"]
    #[inline]
    pub fn is_ctype_0(&self) -> bool {
        *self == CTYPER::CTYPE_0
    }
    #[doc = "Checks if the value of the field is `CTYPE_1`"]
    #[inline]
    pub fn is_ctype_1(&self) -> bool {
        *self == CTYPER::CTYPE_1
    }
    #[doc = "Checks if the value of the field is `CTYPE_2`"]
    #[inline]
    pub fn is_ctype_2(&self) -> bool {
        *self == CTYPER::CTYPE_2
    }
    #[doc = "Checks if the value of the field is `CTYPE_3`"]
    #[inline]
    pub fn is_ctype_3(&self) -> bool {
        *self == CTYPER::CTYPE_3
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1,
}
impl MODER {
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
            MODER::MODE_0 => false,
            MODER::MODE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::MODE_0,
            true => MODER::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline]
    pub fn is_mode_0(&self) -> bool {
        *self == MODER::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline]
    pub fn is_mode_1(&self) -> bool {
        *self == MODER::MODE_1
    }
}
#[doc = "Values that can be written to the field `ADCH`"]
pub enum ADCHW {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9,
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31,
}
impl ADCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCHW::ADCH_0 => 0,
            ADCHW::ADCH_1 => 1,
            ADCHW::ADCH_2 => 2,
            ADCHW::ADCH_3 => 3,
            ADCHW::ADCH_4 => 4,
            ADCHW::ADCH_5 => 5,
            ADCHW::ADCH_6 => 6,
            ADCHW::ADCH_7 => 7,
            ADCHW::ADCH_8 => 8,
            ADCHW::ADCH_9 => 9,
            ADCHW::ADCH_30 => 30,
            ADCHW::ADCH_31 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    #[inline]
    pub fn adch_0(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_0)
    }
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    #[inline]
    pub fn adch_1(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_1)
    }
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    #[inline]
    pub fn adch_2(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_2)
    }
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    #[inline]
    pub fn adch_3(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_3)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline]
    pub fn adch_4(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_4)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline]
    pub fn adch_5(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_5)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline]
    pub fn adch_6(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_6)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline]
    pub fn adch_7(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_7)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline]
    pub fn adch_8(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_8)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline]
    pub fn adch_9(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_9)
    }
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    #[inline]
    pub fn adch_30(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_30)
    }
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    #[inline]
    pub fn adch_31(self) -> &'a mut W {
        self.variant(ADCHW::ADCH_31)
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
#[doc = "Values that can be written to the field `CTYPE`"]
pub enum CTYPEW {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0,
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1,
    #[doc = "Differential Mode. A-B."]
    CTYPE_2,
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3,
}
impl CTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTYPEW::CTYPE_0 => 0,
            CTYPEW::CTYPE_1 => 1,
            CTYPEW::CTYPE_2 => 2,
            CTYPEW::CTYPE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    #[inline]
    pub fn ctype_0(self) -> &'a mut W {
        self.variant(CTYPEW::CTYPE_0)
    }
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    #[inline]
    pub fn ctype_1(self) -> &'a mut W {
        self.variant(CTYPEW::CTYPE_1)
    }
    #[doc = "Differential Mode. A-B."]
    #[inline]
    pub fn ctype_2(self) -> &'a mut W {
        self.variant(CTYPEW::CTYPE_2)
    }
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    #[inline]
    pub fn ctype_3(self) -> &'a mut W {
        self.variant(CTYPEW::CTYPE_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0,
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::MODE_0 => false,
            MODEW::MODE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    #[inline]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODEW::MODE_0)
    }
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    #[inline]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODEW::MODE_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline]
    pub fn adch(&self) -> ADCHR {
        ADCHR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Conversion Type"]
    #[inline]
    pub fn ctype(&self) -> CTYPER {
        CTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Select resolution of conversions"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline]
    pub fn adch(&mut self) -> _ADCHW {
        _ADCHW { w: self }
    }
    #[doc = "Bits 5:6 - Conversion Type"]
    #[inline]
    pub fn ctype(&mut self) -> _CTYPEW {
        _CTYPEW { w: self }
    }
    #[doc = "Bit 7 - Select resolution of conversions"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
