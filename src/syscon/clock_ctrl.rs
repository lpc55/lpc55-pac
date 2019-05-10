#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLOCK_CTRL {
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
#[doc = "Possible values of the field `FLASH48MHZ_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH48MHZ_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FLASH48MHZ_ENAR {
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
            FLASH48MHZ_ENAR::DISABLE => false,
            FLASH48MHZ_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH48MHZ_ENAR {
        match value {
            false => FLASH48MHZ_ENAR::DISABLE,
            true => FLASH48MHZ_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FLASH48MHZ_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FLASH48MHZ_ENAR::ENABLE
    }
}
#[doc = "Possible values of the field `XTAL32MHZ_FREQM_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL32MHZ_FREQM_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl XTAL32MHZ_FREQM_ENAR {
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
            XTAL32MHZ_FREQM_ENAR::DISABLE => false,
            XTAL32MHZ_FREQM_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTAL32MHZ_FREQM_ENAR {
        match value {
            false => XTAL32MHZ_FREQM_ENAR::DISABLE,
            true => XTAL32MHZ_FREQM_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == XTAL32MHZ_FREQM_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == XTAL32MHZ_FREQM_ENAR::ENABLE
    }
}
#[doc = "Possible values of the field `FRO1MHZ_UTICK_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO1MHZ_UTICK_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FRO1MHZ_UTICK_ENAR {
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
            FRO1MHZ_UTICK_ENAR::DISABLE => false,
            FRO1MHZ_UTICK_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRO1MHZ_UTICK_ENAR {
        match value {
            false => FRO1MHZ_UTICK_ENAR::DISABLE,
            true => FRO1MHZ_UTICK_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FRO1MHZ_UTICK_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FRO1MHZ_UTICK_ENAR::ENABLE
    }
}
#[doc = "Possible values of the field `FRO12MHZ_FREQM_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO12MHZ_FREQM_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FRO12MHZ_FREQM_ENAR {
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
            FRO12MHZ_FREQM_ENAR::DISABLE => false,
            FRO12MHZ_FREQM_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRO12MHZ_FREQM_ENAR {
        match value {
            false => FRO12MHZ_FREQM_ENAR::DISABLE,
            true => FRO12MHZ_FREQM_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FRO12MHZ_FREQM_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FRO12MHZ_FREQM_ENAR::ENABLE
    }
}
#[doc = "Possible values of the field `FRO_HF_FREQM_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO_HF_FREQM_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FRO_HF_FREQM_ENAR {
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
            FRO_HF_FREQM_ENAR::DISABLE => false,
            FRO_HF_FREQM_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRO_HF_FREQM_ENAR {
        match value {
            false => FRO_HF_FREQM_ENAR::DISABLE,
            true => FRO_HF_FREQM_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FRO_HF_FREQM_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FRO_HF_FREQM_ENAR::ENABLE
    }
}
#[doc = "Possible values of the field `CLKIN_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKIN_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl CLKIN_ENAR {
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
            CLKIN_ENAR::DISABLE => false,
            CLKIN_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKIN_ENAR {
        match value {
            false => CLKIN_ENAR::DISABLE,
            true => CLKIN_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CLKIN_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CLKIN_ENAR::ENABLE
    }
}
#[doc = "Possible values of the field `FRO1MHZ_CLK_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO1MHZ_CLK_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FRO1MHZ_CLK_ENAR {
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
            FRO1MHZ_CLK_ENAR::DISABLE => false,
            FRO1MHZ_CLK_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRO1MHZ_CLK_ENAR {
        match value {
            false => FRO1MHZ_CLK_ENAR::DISABLE,
            true => FRO1MHZ_CLK_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == FRO1MHZ_CLK_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FRO1MHZ_CLK_ENAR::ENABLE
    }
}
#[doc = "Possible values of the field `ANA_FRO12M_CLK_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANA_FRO12M_CLK_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl ANA_FRO12M_CLK_ENAR {
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
            ANA_FRO12M_CLK_ENAR::DISABLE => false,
            ANA_FRO12M_CLK_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANA_FRO12M_CLK_ENAR {
        match value {
            false => ANA_FRO12M_CLK_ENAR::DISABLE,
            true => ANA_FRO12M_CLK_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ANA_FRO12M_CLK_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ANA_FRO12M_CLK_ENAR::ENABLE
    }
}
#[doc = "Possible values of the field `XO_CAL_CLK_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XO_CAL_CLK_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl XO_CAL_CLK_ENAR {
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
            XO_CAL_CLK_ENAR::DISABLE => false,
            XO_CAL_CLK_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XO_CAL_CLK_ENAR {
        match value {
            false => XO_CAL_CLK_ENAR::DISABLE,
            true => XO_CAL_CLK_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == XO_CAL_CLK_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == XO_CAL_CLK_ENAR::ENABLE
    }
}
#[doc = "Possible values of the field `PLU_DEGLITCH_CLK_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLU_DEGLITCH_CLK_ENAR {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl PLU_DEGLITCH_CLK_ENAR {
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
            PLU_DEGLITCH_CLK_ENAR::DISABLE => false,
            PLU_DEGLITCH_CLK_ENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLU_DEGLITCH_CLK_ENAR {
        match value {
            false => PLU_DEGLITCH_CLK_ENAR::DISABLE,
            true => PLU_DEGLITCH_CLK_ENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PLU_DEGLITCH_CLK_ENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PLU_DEGLITCH_CLK_ENAR::ENABLE
    }
}
#[doc = "Values that can be written to the field `FLASH48MHZ_ENA`"]
pub enum FLASH48MHZ_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FLASH48MHZ_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH48MHZ_ENAW::DISABLE => false,
            FLASH48MHZ_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH48MHZ_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH48MHZ_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH48MHZ_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLASH48MHZ_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLASH48MHZ_ENAW::ENABLE)
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
#[doc = "Values that can be written to the field `XTAL32MHZ_FREQM_ENA`"]
pub enum XTAL32MHZ_FREQM_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl XTAL32MHZ_FREQM_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTAL32MHZ_FREQM_ENAW::DISABLE => false,
            XTAL32MHZ_FREQM_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTAL32MHZ_FREQM_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _XTAL32MHZ_FREQM_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTAL32MHZ_FREQM_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(XTAL32MHZ_FREQM_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(XTAL32MHZ_FREQM_ENAW::ENABLE)
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
#[doc = "Values that can be written to the field `FRO1MHZ_UTICK_ENA`"]
pub enum FRO1MHZ_UTICK_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FRO1MHZ_UTICK_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRO1MHZ_UTICK_ENAW::DISABLE => false,
            FRO1MHZ_UTICK_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRO1MHZ_UTICK_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _FRO1MHZ_UTICK_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRO1MHZ_UTICK_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO1MHZ_UTICK_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO1MHZ_UTICK_ENAW::ENABLE)
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
#[doc = "Values that can be written to the field `FRO12MHZ_FREQM_ENA`"]
pub enum FRO12MHZ_FREQM_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FRO12MHZ_FREQM_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRO12MHZ_FREQM_ENAW::DISABLE => false,
            FRO12MHZ_FREQM_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRO12MHZ_FREQM_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _FRO12MHZ_FREQM_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRO12MHZ_FREQM_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO12MHZ_FREQM_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO12MHZ_FREQM_ENAW::ENABLE)
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
#[doc = "Values that can be written to the field `FRO_HF_FREQM_ENA`"]
pub enum FRO_HF_FREQM_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FRO_HF_FREQM_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRO_HF_FREQM_ENAW::DISABLE => false,
            FRO_HF_FREQM_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRO_HF_FREQM_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _FRO_HF_FREQM_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRO_HF_FREQM_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO_HF_FREQM_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO_HF_FREQM_ENAW::ENABLE)
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
#[doc = "Values that can be written to the field `CLKIN_ENA`"]
pub enum CLKIN_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl CLKIN_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKIN_ENAW::DISABLE => false,
            CLKIN_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKIN_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKIN_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKIN_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLKIN_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLKIN_ENAW::ENABLE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRO1MHZ_CLK_ENA`"]
pub enum FRO1MHZ_CLK_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl FRO1MHZ_CLK_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRO1MHZ_CLK_ENAW::DISABLE => false,
            FRO1MHZ_CLK_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRO1MHZ_CLK_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _FRO1MHZ_CLK_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRO1MHZ_CLK_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO1MHZ_CLK_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO1MHZ_CLK_ENAW::ENABLE)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ANA_FRO12M_CLK_ENA`"]
pub enum ANA_FRO12M_CLK_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl ANA_FRO12M_CLK_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANA_FRO12M_CLK_ENAW::DISABLE => false,
            ANA_FRO12M_CLK_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANA_FRO12M_CLK_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _ANA_FRO12M_CLK_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANA_FRO12M_CLK_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ANA_FRO12M_CLK_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ANA_FRO12M_CLK_ENAW::ENABLE)
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
#[doc = "Values that can be written to the field `XO_CAL_CLK_ENA`"]
pub enum XO_CAL_CLK_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl XO_CAL_CLK_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XO_CAL_CLK_ENAW::DISABLE => false,
            XO_CAL_CLK_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XO_CAL_CLK_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _XO_CAL_CLK_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XO_CAL_CLK_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(XO_CAL_CLK_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(XO_CAL_CLK_ENAW::ENABLE)
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
#[doc = "Values that can be written to the field `PLU_DEGLITCH_CLK_ENA`"]
pub enum PLU_DEGLITCH_CLK_ENAW {
    #[doc = "The clock is not enabled."]
    DISABLE,
    #[doc = "The clock is enabled."]
    ENABLE,
}
impl PLU_DEGLITCH_CLK_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLU_DEGLITCH_CLK_ENAW::DISABLE => false,
            PLU_DEGLITCH_CLK_ENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLU_DEGLITCH_CLK_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _PLU_DEGLITCH_CLK_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLU_DEGLITCH_CLK_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLU_DEGLITCH_CLK_ENAW::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLU_DEGLITCH_CLK_ENAW::ENABLE)
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
    #[doc = "Bit 0 - Enable Flash 48 MHz clock."]
    #[inline]
    pub fn flash48mhz_ena(&self) -> FLASH48MHZ_ENAR {
        FLASH48MHZ_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline]
    pub fn xtal32mhz_freqm_ena(&self) -> XTAL32MHZ_FREQM_ENAR {
        XTAL32MHZ_FREQM_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline]
    pub fn fro1mhz_utick_ena(&self) -> FRO1MHZ_UTICK_ENAR {
        FRO1MHZ_UTICK_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline]
    pub fn fro12mhz_freqm_ena(&self) -> FRO12MHZ_FREQM_ENAR {
        FRO12MHZ_FREQM_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline]
    pub fn fro_hf_freqm_ena(&self) -> FRO_HF_FREQM_ENAR {
        FRO_HF_FREQM_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable clock_in clock for clock module."]
    #[inline]
    pub fn clkin_ena(&self) -> CLKIN_ENAR {
        CLKIN_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline]
    pub fn fro1mhz_clk_ena(&self) -> FRO1MHZ_CLK_ENAR {
        FRO1MHZ_CLK_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline]
    pub fn ana_fro12m_clk_ena(&self) -> ANA_FRO12M_CLK_ENAR {
        ANA_FRO12M_CLK_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable clock for cristal oscilator calibration."]
    #[inline]
    pub fn xo_cal_clk_ena(&self) -> XO_CAL_CLK_ENAR {
        XO_CAL_CLK_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline]
    pub fn plu_deglitch_clk_ena(&self) -> PLU_DEGLITCH_CLK_ENAR {
        PLU_DEGLITCH_CLK_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable Flash 48 MHz clock."]
    #[inline]
    pub fn flash48mhz_ena(&mut self) -> _FLASH48MHZ_ENAW {
        _FLASH48MHZ_ENAW { w: self }
    }
    #[doc = "Bit 1 - Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline]
    pub fn xtal32mhz_freqm_ena(&mut self) -> _XTAL32MHZ_FREQM_ENAW {
        _XTAL32MHZ_FREQM_ENAW { w: self }
    }
    #[doc = "Bit 2 - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline]
    pub fn fro1mhz_utick_ena(&mut self) -> _FRO1MHZ_UTICK_ENAW {
        _FRO1MHZ_UTICK_ENAW { w: self }
    }
    #[doc = "Bit 3 - Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline]
    pub fn fro12mhz_freqm_ena(&mut self) -> _FRO12MHZ_FREQM_ENAW {
        _FRO12MHZ_FREQM_ENAW { w: self }
    }
    #[doc = "Bit 4 - Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline]
    pub fn fro_hf_freqm_ena(&mut self) -> _FRO_HF_FREQM_ENAW {
        _FRO_HF_FREQM_ENAW { w: self }
    }
    #[doc = "Bit 5 - Enable clock_in clock for clock module."]
    #[inline]
    pub fn clkin_ena(&mut self) -> _CLKIN_ENAW {
        _CLKIN_ENAW { w: self }
    }
    #[doc = "Bit 6 - Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline]
    pub fn fro1mhz_clk_ena(&mut self) -> _FRO1MHZ_CLK_ENAW {
        _FRO1MHZ_CLK_ENAW { w: self }
    }
    #[doc = "Bit 7 - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline]
    pub fn ana_fro12m_clk_ena(&mut self) -> _ANA_FRO12M_CLK_ENAW {
        _ANA_FRO12M_CLK_ENAW { w: self }
    }
    #[doc = "Bit 8 - Enable clock for cristal oscilator calibration."]
    #[inline]
    pub fn xo_cal_clk_ena(&mut self) -> _XO_CAL_CLK_ENAW {
        _XO_CAL_CLK_ENAW { w: self }
    }
    #[doc = "Bit 9 - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline]
    pub fn plu_deglitch_clk_ena(&mut self) -> _PLU_DEGLITCH_CLK_ENAW {
        _PLU_DEGLITCH_CLK_ENAW { w: self }
    }
}
