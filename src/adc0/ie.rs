#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IE {
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
#[doc = "Possible values of the field `FWMIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMIE0R {
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    FWMIE0_0,
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    FWMIE0_1,
}
impl FWMIE0R {
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
            FWMIE0R::FWMIE0_0 => false,
            FWMIE0R::FWMIE0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWMIE0R {
        match value {
            false => FWMIE0R::FWMIE0_0,
            true => FWMIE0R::FWMIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMIE0_0`"]
    #[inline]
    pub fn is_fwmie0_0(&self) -> bool {
        *self == FWMIE0R::FWMIE0_0
    }
    #[doc = "Checks if the value of the field is `FWMIE0_1`"]
    #[inline]
    pub fn is_fwmie0_1(&self) -> bool {
        *self == FWMIE0R::FWMIE0_1
    }
}
#[doc = "Possible values of the field `FOFIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFIE0R {
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    FOFIE0_0,
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    FOFIE0_1,
}
impl FOFIE0R {
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
            FOFIE0R::FOFIE0_0 => false,
            FOFIE0R::FOFIE0_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOFIE0R {
        match value {
            false => FOFIE0R::FOFIE0_0,
            true => FOFIE0R::FOFIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFIE0_0`"]
    #[inline]
    pub fn is_fofie0_0(&self) -> bool {
        *self == FOFIE0R::FOFIE0_0
    }
    #[doc = "Checks if the value of the field is `FOFIE0_1`"]
    #[inline]
    pub fn is_fofie0_1(&self) -> bool {
        *self == FOFIE0R::FOFIE0_1
    }
}
#[doc = "Possible values of the field `FWMIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMIE1R {
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    FWMIE1_0,
    #[doc = "FIFO1 watermark interrupts are enabled."]
    FWMIE1_1,
}
impl FWMIE1R {
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
            FWMIE1R::FWMIE1_0 => false,
            FWMIE1R::FWMIE1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWMIE1R {
        match value {
            false => FWMIE1R::FWMIE1_0,
            true => FWMIE1R::FWMIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMIE1_0`"]
    #[inline]
    pub fn is_fwmie1_0(&self) -> bool {
        *self == FWMIE1R::FWMIE1_0
    }
    #[doc = "Checks if the value of the field is `FWMIE1_1`"]
    #[inline]
    pub fn is_fwmie1_1(&self) -> bool {
        *self == FWMIE1R::FWMIE1_1
    }
}
#[doc = "Possible values of the field `FOFIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFIE1R {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_1,
}
impl FOFIE1R {
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
            FOFIE1R::FOFIE1_0 => false,
            FOFIE1R::FOFIE1_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOFIE1R {
        match value {
            false => FOFIE1R::FOFIE1_0,
            true => FOFIE1R::FOFIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFIE1_0`"]
    #[inline]
    pub fn is_fofie1_0(&self) -> bool {
        *self == FOFIE1R::FOFIE1_0
    }
    #[doc = "Checks if the value of the field is `FOFIE1_1`"]
    #[inline]
    pub fn is_fofie1_1(&self) -> bool {
        *self == FOFIE1R::FOFIE1_1
    }
}
#[doc = "Possible values of the field `TEXC_IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEXC_IER {
    #[doc = "Trigger exception interrupts are disabled."]
    TEXC_IE_0,
    #[doc = "Trigger exception interrupts are enabled."]
    TEXC_IE_1,
}
impl TEXC_IER {
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
            TEXC_IER::TEXC_IE_0 => false,
            TEXC_IER::TEXC_IE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEXC_IER {
        match value {
            false => TEXC_IER::TEXC_IE_0,
            true => TEXC_IER::TEXC_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEXC_IE_0`"]
    #[inline]
    pub fn is_texc_ie_0(&self) -> bool {
        *self == TEXC_IER::TEXC_IE_0
    }
    #[doc = "Checks if the value of the field is `TEXC_IE_1`"]
    #[inline]
    pub fn is_texc_ie_1(&self) -> bool {
        *self == TEXC_IER::TEXC_IE_1
    }
}
#[doc = "Possible values of the field `TCOMP_IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCOMP_IER {
    #[doc = "Trigger completion interrupts are disabled."]
    TCOMP_IE_0,
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    TCOMP_IE_1,
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    TCOMP_IE_2,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_3,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_4,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_5,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_6,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_7,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_8,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_9,
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    TCOMP_IE_65535,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TCOMP_IER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TCOMP_IER::TCOMP_IE_0 => 0,
            TCOMP_IER::TCOMP_IE_1 => 1,
            TCOMP_IER::TCOMP_IE_2 => 2,
            TCOMP_IER::TCOMP_IE_3 => 3,
            TCOMP_IER::TCOMP_IE_4 => 4,
            TCOMP_IER::TCOMP_IE_5 => 5,
            TCOMP_IER::TCOMP_IE_6 => 6,
            TCOMP_IER::TCOMP_IE_7 => 7,
            TCOMP_IER::TCOMP_IE_8 => 8,
            TCOMP_IER::TCOMP_IE_9 => 9,
            TCOMP_IER::TCOMP_IE_65535 => 65535,
            TCOMP_IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TCOMP_IER {
        match value {
            0 => TCOMP_IER::TCOMP_IE_0,
            1 => TCOMP_IER::TCOMP_IE_1,
            2 => TCOMP_IER::TCOMP_IE_2,
            3 => TCOMP_IER::TCOMP_IE_3,
            4 => TCOMP_IER::TCOMP_IE_4,
            5 => TCOMP_IER::TCOMP_IE_5,
            6 => TCOMP_IER::TCOMP_IE_6,
            7 => TCOMP_IER::TCOMP_IE_7,
            8 => TCOMP_IER::TCOMP_IE_8,
            9 => TCOMP_IER::TCOMP_IE_9,
            65535 => TCOMP_IER::TCOMP_IE_65535,
            i => TCOMP_IER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_0`"]
    #[inline]
    pub fn is_tcomp_ie_0(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_0
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_1`"]
    #[inline]
    pub fn is_tcomp_ie_1(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_1
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_2`"]
    #[inline]
    pub fn is_tcomp_ie_2(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_2
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_3`"]
    #[inline]
    pub fn is_tcomp_ie_3(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_3
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_4`"]
    #[inline]
    pub fn is_tcomp_ie_4(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_4
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_5`"]
    #[inline]
    pub fn is_tcomp_ie_5(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_5
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_6`"]
    #[inline]
    pub fn is_tcomp_ie_6(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_6
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_7`"]
    #[inline]
    pub fn is_tcomp_ie_7(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_7
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_8`"]
    #[inline]
    pub fn is_tcomp_ie_8(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_8
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_9`"]
    #[inline]
    pub fn is_tcomp_ie_9(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_9
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_65535`"]
    #[inline]
    pub fn is_tcomp_ie_65535(&self) -> bool {
        *self == TCOMP_IER::TCOMP_IE_65535
    }
}
#[doc = "Values that can be written to the field `FWMIE0`"]
pub enum FWMIE0W {
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    FWMIE0_0,
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    FWMIE0_1,
}
impl FWMIE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWMIE0W::FWMIE0_0 => false,
            FWMIE0W::FWMIE0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWMIE0W<'a> {
    w: &'a mut W,
}
impl<'a> _FWMIE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWMIE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    #[inline]
    pub fn fwmie0_0(self) -> &'a mut W {
        self.variant(FWMIE0W::FWMIE0_0)
    }
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    #[inline]
    pub fn fwmie0_1(self) -> &'a mut W {
        self.variant(FWMIE0W::FWMIE0_1)
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
#[doc = "Values that can be written to the field `FOFIE0`"]
pub enum FOFIE0W {
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    FOFIE0_0,
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    FOFIE0_1,
}
impl FOFIE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOFIE0W::FOFIE0_0 => false,
            FOFIE0W::FOFIE0_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOFIE0W<'a> {
    w: &'a mut W,
}
impl<'a> _FOFIE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOFIE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    #[inline]
    pub fn fofie0_0(self) -> &'a mut W {
        self.variant(FOFIE0W::FOFIE0_0)
    }
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    #[inline]
    pub fn fofie0_1(self) -> &'a mut W {
        self.variant(FOFIE0W::FOFIE0_1)
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
#[doc = "Values that can be written to the field `FWMIE1`"]
pub enum FWMIE1W {
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    FWMIE1_0,
    #[doc = "FIFO1 watermark interrupts are enabled."]
    FWMIE1_1,
}
impl FWMIE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWMIE1W::FWMIE1_0 => false,
            FWMIE1W::FWMIE1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWMIE1W<'a> {
    w: &'a mut W,
}
impl<'a> _FWMIE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWMIE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    #[inline]
    pub fn fwmie1_0(self) -> &'a mut W {
        self.variant(FWMIE1W::FWMIE1_0)
    }
    #[doc = "FIFO1 watermark interrupts are enabled."]
    #[inline]
    pub fn fwmie1_1(self) -> &'a mut W {
        self.variant(FWMIE1W::FWMIE1_1)
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
#[doc = "Values that can be written to the field `FOFIE1`"]
pub enum FOFIE1W {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_1,
}
impl FOFIE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOFIE1W::FOFIE1_0 => false,
            FOFIE1W::FOFIE1_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOFIE1W<'a> {
    w: &'a mut W,
}
impl<'a> _FOFIE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOFIE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn fofie1_0(self) -> &'a mut W {
        self.variant(FOFIE1W::FOFIE1_0)
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn fofie1_1(self) -> &'a mut W {
        self.variant(FOFIE1W::FOFIE1_1)
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
#[doc = "Values that can be written to the field `TEXC_IE`"]
pub enum TEXC_IEW {
    #[doc = "Trigger exception interrupts are disabled."]
    TEXC_IE_0,
    #[doc = "Trigger exception interrupts are enabled."]
    TEXC_IE_1,
}
impl TEXC_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEXC_IEW::TEXC_IE_0 => false,
            TEXC_IEW::TEXC_IE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEXC_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEXC_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEXC_IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger exception interrupts are disabled."]
    #[inline]
    pub fn texc_ie_0(self) -> &'a mut W {
        self.variant(TEXC_IEW::TEXC_IE_0)
    }
    #[doc = "Trigger exception interrupts are enabled."]
    #[inline]
    pub fn texc_ie_1(self) -> &'a mut W {
        self.variant(TEXC_IEW::TEXC_IE_1)
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
#[doc = "Values that can be written to the field `TCOMP_IE`"]
pub enum TCOMP_IEW {
    #[doc = "Trigger completion interrupts are disabled."]
    TCOMP_IE_0,
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    TCOMP_IE_1,
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    TCOMP_IE_2,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_3,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_4,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_5,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_6,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_7,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_8,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TCOMP_IE_9,
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    TCOMP_IE_65535,
}
impl TCOMP_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            TCOMP_IEW::TCOMP_IE_0 => 0,
            TCOMP_IEW::TCOMP_IE_1 => 1,
            TCOMP_IEW::TCOMP_IE_2 => 2,
            TCOMP_IEW::TCOMP_IE_3 => 3,
            TCOMP_IEW::TCOMP_IE_4 => 4,
            TCOMP_IEW::TCOMP_IE_5 => 5,
            TCOMP_IEW::TCOMP_IE_6 => 6,
            TCOMP_IEW::TCOMP_IE_7 => 7,
            TCOMP_IEW::TCOMP_IE_8 => 8,
            TCOMP_IEW::TCOMP_IE_9 => 9,
            TCOMP_IEW::TCOMP_IE_65535 => 65535,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCOMP_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCOMP_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCOMP_IEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trigger completion interrupts are disabled."]
    #[inline]
    pub fn tcomp_ie_0(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_0)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    #[inline]
    pub fn tcomp_ie_1(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_1)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    #[inline]
    pub fn tcomp_ie_2(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_2)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline]
    pub fn tcomp_ie_3(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_3)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline]
    pub fn tcomp_ie_4(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_4)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline]
    pub fn tcomp_ie_5(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_5)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline]
    pub fn tcomp_ie_6(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_6)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline]
    pub fn tcomp_ie_7(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_7)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline]
    pub fn tcomp_ie_8(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_8)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline]
    pub fn tcomp_ie_9(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_9)
    }
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    #[inline]
    pub fn tcomp_ie_65535(self) -> &'a mut W {
        self.variant(TCOMP_IEW::TCOMP_IE_65535)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline]
    pub fn fwmie0(&self) -> FWMIE0R {
        FWMIE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline]
    pub fn fofie0(&self) -> FOFIE0R {
        FOFIE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline]
    pub fn fwmie1(&self) -> FWMIE1R {
        FWMIE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline]
    pub fn fofie1(&self) -> FOFIE1R {
        FOFIE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline]
    pub fn texc_ie(&self) -> TEXC_IER {
        TEXC_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline]
    pub fn tcomp_ie(&self) -> TCOMP_IER {
        TCOMP_IER::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline]
    pub fn fwmie0(&mut self) -> _FWMIE0W {
        _FWMIE0W { w: self }
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline]
    pub fn fofie0(&mut self) -> _FOFIE0W {
        _FOFIE0W { w: self }
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline]
    pub fn fwmie1(&mut self) -> _FWMIE1W {
        _FWMIE1W { w: self }
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline]
    pub fn fofie1(&mut self) -> _FOFIE1W {
        _FOFIE1W { w: self }
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline]
    pub fn texc_ie(&mut self) -> _TEXC_IEW {
        _TEXC_IEW { w: self }
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline]
    pub fn tcomp_ie(&mut self) -> _TCOMP_IEW {
        _TCOMP_IEW { w: self }
    }
}
