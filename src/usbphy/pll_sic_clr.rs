#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL_SIC_CLR {
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
#[doc = r" Value of the field"]
pub struct MISC2_CONTROL0R {
    bits: bool,
}
impl MISC2_CONTROL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PLL_EN_USB_CLKSR {
    bits: bool,
}
impl PLL_EN_USB_CLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PLL_POWERR {
    bits: bool,
}
impl PLL_POWERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PLL_ENABLER {
    bits: bool,
}
impl PLL_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PLL_BYPASSR {
    bits: bool,
}
impl PLL_BYPASSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `REFBIAS_PWD_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFBIAS_PWD_SELR {
    #[doc = "Selects PLL_POWER to control the reference bias"]
    VALUE0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    VALUE1,
}
impl REFBIAS_PWD_SELR {
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
            REFBIAS_PWD_SELR::VALUE0 => false,
            REFBIAS_PWD_SELR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFBIAS_PWD_SELR {
        match value {
            false => REFBIAS_PWD_SELR::VALUE0,
            true => REFBIAS_PWD_SELR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == REFBIAS_PWD_SELR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == REFBIAS_PWD_SELR::VALUE1
    }
}
#[doc = r" Value of the field"]
pub struct REFBIAS_PWDR {
    bits: bool,
}
impl REFBIAS_PWDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PLL_REG_ENABLER {
    bits: bool,
}
impl PLL_REG_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `PLL_DIV_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_DIV_SELR {
    #[doc = "Divide by 13"]
    VALUE0,
    #[doc = "Divide by 15"]
    VALUE1,
    #[doc = "Divide by 16"]
    VALUE2,
    #[doc = "Divide by 20"]
    VALUE3,
    #[doc = "Divide by 22"]
    VALUE4,
    #[doc = "Divide by 25"]
    VALUE5,
    #[doc = "Divide by 30"]
    VALUE6,
    #[doc = "Divide by 240"]
    VALUE7,
}
impl PLL_DIV_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLL_DIV_SELR::VALUE0 => 0,
            PLL_DIV_SELR::VALUE1 => 1,
            PLL_DIV_SELR::VALUE2 => 2,
            PLL_DIV_SELR::VALUE3 => 3,
            PLL_DIV_SELR::VALUE4 => 4,
            PLL_DIV_SELR::VALUE5 => 5,
            PLL_DIV_SELR::VALUE6 => 6,
            PLL_DIV_SELR::VALUE7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLL_DIV_SELR {
        match value {
            0 => PLL_DIV_SELR::VALUE0,
            1 => PLL_DIV_SELR::VALUE1,
            2 => PLL_DIV_SELR::VALUE2,
            3 => PLL_DIV_SELR::VALUE3,
            4 => PLL_DIV_SELR::VALUE4,
            5 => PLL_DIV_SELR::VALUE5,
            6 => PLL_DIV_SELR::VALUE6,
            7 => PLL_DIV_SELR::VALUE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == PLL_DIV_SELR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLL_DIV_SELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PLL_DIV_SELR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PLL_DIV_SELR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PLL_DIV_SELR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PLL_DIV_SELR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PLL_DIV_SELR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PLL_DIV_SELR::VALUE7
    }
}
#[doc = "Possible values of the field `PLL_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCKR {
    #[doc = "PLL is not currently locked"]
    VALUE0,
    #[doc = "PLL is currently locked"]
    VALUE1,
}
impl PLL_LOCKR {
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
            PLL_LOCKR::VALUE0 => false,
            PLL_LOCKR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_LOCKR {
        match value {
            false => PLL_LOCKR::VALUE0,
            true => PLL_LOCKR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == PLL_LOCKR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PLL_LOCKR::VALUE1
    }
}
#[doc = r" Proxy"]
pub struct _MISC2_CONTROL0W<'a> {
    w: &'a mut W,
}
impl<'a> _MISC2_CONTROL0W<'a> {
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
#[doc = r" Proxy"]
pub struct _PLL_EN_USB_CLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_EN_USB_CLKSW<'a> {
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
#[doc = r" Proxy"]
pub struct _PLL_POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_POWERW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_ENABLEW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_BYPASSW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFBIAS_PWD_SEL`"]
pub enum REFBIAS_PWD_SELW {
    #[doc = "Selects PLL_POWER to control the reference bias"]
    VALUE0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    VALUE1,
}
impl REFBIAS_PWD_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFBIAS_PWD_SELW::VALUE0 => false,
            REFBIAS_PWD_SELW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFBIAS_PWD_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFBIAS_PWD_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFBIAS_PWD_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects PLL_POWER to control the reference bias"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SELW::VALUE0)
    }
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SELW::VALUE1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REFBIAS_PWDW<'a> {
    w: &'a mut W,
}
impl<'a> _REFBIAS_PWDW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLL_REG_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_REG_ENABLEW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLL_DIV_SEL`"]
pub enum PLL_DIV_SELW {
    #[doc = "Divide by 13"]
    VALUE0,
    #[doc = "Divide by 15"]
    VALUE1,
    #[doc = "Divide by 16"]
    VALUE2,
    #[doc = "Divide by 20"]
    VALUE3,
    #[doc = "Divide by 22"]
    VALUE4,
    #[doc = "Divide by 25"]
    VALUE5,
    #[doc = "Divide by 30"]
    VALUE6,
    #[doc = "Divide by 240"]
    VALUE7,
}
impl PLL_DIV_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLL_DIV_SELW::VALUE0 => 0,
            PLL_DIV_SELW::VALUE1 => 1,
            PLL_DIV_SELW::VALUE2 => 2,
            PLL_DIV_SELW::VALUE3 => 3,
            PLL_DIV_SELW::VALUE4 => 4,
            PLL_DIV_SELW::VALUE5 => 5,
            PLL_DIV_SELW::VALUE6 => 6,
            PLL_DIV_SELW::VALUE7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_DIV_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_DIV_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_DIV_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 13"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::VALUE0)
    }
    #[doc = "Divide by 15"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::VALUE1)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::VALUE2)
    }
    #[doc = "Divide by 20"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::VALUE3)
    }
    #[doc = "Divide by 22"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::VALUE4)
    }
    #[doc = "Divide by 25"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::VALUE5)
    }
    #[doc = "Divide by 30"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::VALUE6)
    }
    #[doc = "Divide by 240"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PLL_DIV_SELW::VALUE7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLL_LOCK`"]
pub enum PLL_LOCKW {
    #[doc = "PLL is not currently locked"]
    VALUE0,
    #[doc = "PLL is currently locked"]
    VALUE1,
}
impl PLL_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_LOCKW::VALUE0 => false,
            PLL_LOCKW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL is not currently locked"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(PLL_LOCKW::VALUE0)
    }
    #[doc = "PLL is currently locked"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLL_LOCKW::VALUE1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 5 - Modifies the operation of the pll_sic_power_int signal"]
    #[inline]
    pub fn misc2_control0(&self) -> MISC2_CONTROL0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MISC2_CONTROL0R { bits }
    }
    #[doc = "Bit 6 - Enables the USB clock from PLL to USB PHY"]
    #[inline]
    pub fn pll_en_usb_clks(&self) -> PLL_EN_USB_CLKSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_EN_USB_CLKSR { bits }
    }
    #[doc = "Bit 12 - Power up the USB PLL"]
    #[inline]
    pub fn pll_power(&self) -> PLL_POWERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_POWERR { bits }
    }
    #[doc = "Bit 13 - Enables the clock output from the USB PLL"]
    #[inline]
    pub fn pll_enable(&self) -> PLL_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_ENABLER { bits }
    }
    #[doc = "Bit 16 - Bypass the USB PLL."]
    #[inline]
    pub fn pll_bypass(&self) -> PLL_BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_BYPASSR { bits }
    }
    #[doc = "Bit 19 - Reference bias power down select."]
    #[inline]
    pub fn refbias_pwd_sel(&self) -> REFBIAS_PWD_SELR {
        REFBIAS_PWD_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline]
    pub fn refbias_pwd(&self) -> REFBIAS_PWDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REFBIAS_PWDR { bits }
    }
    #[doc = "Bit 21 - This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline]
    pub fn pll_reg_enable(&self) -> PLL_REG_ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLL_REG_ENABLER { bits }
    }
    #[doc = "Bits 22:24 - This field controls the USB PLL feedback loop divider"]
    #[inline]
    pub fn pll_div_sel(&self) -> PLL_DIV_SELR {
        PLL_DIV_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - USB PLL lock status indicator"]
    #[inline]
    pub fn pll_lock(&self) -> PLL_LOCKR {
        PLL_LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 13705216 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 5 - Modifies the operation of the pll_sic_power_int signal"]
    #[inline]
    pub fn misc2_control0(&mut self) -> _MISC2_CONTROL0W {
        _MISC2_CONTROL0W { w: self }
    }
    #[doc = "Bit 6 - Enables the USB clock from PLL to USB PHY"]
    #[inline]
    pub fn pll_en_usb_clks(&mut self) -> _PLL_EN_USB_CLKSW {
        _PLL_EN_USB_CLKSW { w: self }
    }
    #[doc = "Bit 12 - Power up the USB PLL"]
    #[inline]
    pub fn pll_power(&mut self) -> _PLL_POWERW {
        _PLL_POWERW { w: self }
    }
    #[doc = "Bit 13 - Enables the clock output from the USB PLL"]
    #[inline]
    pub fn pll_enable(&mut self) -> _PLL_ENABLEW {
        _PLL_ENABLEW { w: self }
    }
    #[doc = "Bit 16 - Bypass the USB PLL."]
    #[inline]
    pub fn pll_bypass(&mut self) -> _PLL_BYPASSW {
        _PLL_BYPASSW { w: self }
    }
    #[doc = "Bit 19 - Reference bias power down select."]
    #[inline]
    pub fn refbias_pwd_sel(&mut self) -> _REFBIAS_PWD_SELW {
        _REFBIAS_PWD_SELW { w: self }
    }
    #[doc = "Bit 20 - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline]
    pub fn refbias_pwd(&mut self) -> _REFBIAS_PWDW {
        _REFBIAS_PWDW { w: self }
    }
    #[doc = "Bit 21 - This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline]
    pub fn pll_reg_enable(&mut self) -> _PLL_REG_ENABLEW {
        _PLL_REG_ENABLEW { w: self }
    }
    #[doc = "Bits 22:24 - This field controls the USB PLL feedback loop divider"]
    #[inline]
    pub fn pll_div_sel(&mut self) -> _PLL_DIV_SELW {
        _PLL_DIV_SELW { w: self }
    }
    #[doc = "Bit 31 - USB PLL lock status indicator"]
    #[inline]
    pub fn pll_lock(&mut self) -> _PLL_LOCKW {
        _PLL_LOCKW { w: self }
    }
}
