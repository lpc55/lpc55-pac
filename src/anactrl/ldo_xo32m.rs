#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LDO_XO32M {
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
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSR {
    #[doc = "Disable bypass mode (for normal operations)."]
    DISABLE,
    #[doc = "Activate LDO bypass."]
    ENABLE,
}
impl BYPASSR {
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
            BYPASSR::DISABLE => false,
            BYPASSR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSR {
        match value {
            false => BYPASSR::DISABLE,
            true => BYPASSR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BYPASSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BYPASSR::ENABLE
    }
}
#[doc = "Possible values of the field `HIGHZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIGHZR {
    #[doc = "Output in High normal state."]
    NORMALMPEDANCE,
    #[doc = "Output in High Impedance state."]
    HIGHIMPEDANCE,
}
impl HIGHZR {
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
            HIGHZR::NORMALMPEDANCE => false,
            HIGHZR::HIGHIMPEDANCE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIGHZR {
        match value {
            false => HIGHZR::NORMALMPEDANCE,
            true => HIGHZR::HIGHIMPEDANCE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMALMPEDANCE`"]
    #[inline]
    pub fn is_normalmpedance(&self) -> bool {
        *self == HIGHZR::NORMALMPEDANCE
    }
    #[doc = "Checks if the value of the field is `HIGHIMPEDANCE`"]
    #[inline]
    pub fn is_highimpedance(&self) -> bool {
        *self == HIGHZR::HIGHIMPEDANCE
    }
}
#[doc = "Possible values of the field `VOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOUTR {
    #[doc = "0.750 V."]
    V_0P750,
    #[doc = "0.775 V."]
    V_0P775,
    #[doc = "0.800 V."]
    V_0P800,
    #[doc = "0.825 V."]
    V_0P825,
    #[doc = "0.850 V."]
    V_0P850,
    #[doc = "0.875 V."]
    V_0P875,
    #[doc = "0.900 V."]
    V_0P900,
    #[doc = "0.925 V."]
    V_0P925,
}
impl VOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VOUTR::V_0P750 => 0,
            VOUTR::V_0P775 => 1,
            VOUTR::V_0P800 => 2,
            VOUTR::V_0P825 => 3,
            VOUTR::V_0P850 => 4,
            VOUTR::V_0P875 => 5,
            VOUTR::V_0P900 => 6,
            VOUTR::V_0P925 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VOUTR {
        match value {
            0 => VOUTR::V_0P750,
            1 => VOUTR::V_0P775,
            2 => VOUTR::V_0P800,
            3 => VOUTR::V_0P825,
            4 => VOUTR::V_0P850,
            5 => VOUTR::V_0P875,
            6 => VOUTR::V_0P900,
            7 => VOUTR::V_0P925,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V_0P750`"]
    #[inline]
    pub fn is_v_0p750(&self) -> bool {
        *self == VOUTR::V_0P750
    }
    #[doc = "Checks if the value of the field is `V_0P775`"]
    #[inline]
    pub fn is_v_0p775(&self) -> bool {
        *self == VOUTR::V_0P775
    }
    #[doc = "Checks if the value of the field is `V_0P800`"]
    #[inline]
    pub fn is_v_0p800(&self) -> bool {
        *self == VOUTR::V_0P800
    }
    #[doc = "Checks if the value of the field is `V_0P825`"]
    #[inline]
    pub fn is_v_0p825(&self) -> bool {
        *self == VOUTR::V_0P825
    }
    #[doc = "Checks if the value of the field is `V_0P850`"]
    #[inline]
    pub fn is_v_0p850(&self) -> bool {
        *self == VOUTR::V_0P850
    }
    #[doc = "Checks if the value of the field is `V_0P875`"]
    #[inline]
    pub fn is_v_0p875(&self) -> bool {
        *self == VOUTR::V_0P875
    }
    #[doc = "Checks if the value of the field is `V_0P900`"]
    #[inline]
    pub fn is_v_0p900(&self) -> bool {
        *self == VOUTR::V_0P900
    }
    #[doc = "Checks if the value of the field is `V_0P925`"]
    #[inline]
    pub fn is_v_0p925(&self) -> bool {
        *self == VOUTR::V_0P925
    }
}
#[doc = r" Value of the field"]
pub struct IBIASR {
    bits: u8,
}
impl IBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STABMODER {
    bits: u8,
}
impl STABMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `BYPASS`"]
pub enum BYPASSW {
    #[doc = "Disable bypass mode (for normal operations)."]
    DISABLE,
    #[doc = "Activate LDO bypass."]
    ENABLE,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::DISABLE => false,
            BYPASSW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable bypass mode (for normal operations)."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BYPASSW::DISABLE)
    }
    #[doc = "Activate LDO bypass."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BYPASSW::ENABLE)
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
#[doc = "Values that can be written to the field `HIGHZ`"]
pub enum HIGHZW {
    #[doc = "Output in High normal state."]
    NORMALMPEDANCE,
    #[doc = "Output in High Impedance state."]
    HIGHIMPEDANCE,
}
impl HIGHZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIGHZW::NORMALMPEDANCE => false,
            HIGHZW::HIGHIMPEDANCE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIGHZW<'a> {
    w: &'a mut W,
}
impl<'a> _HIGHZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIGHZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output in High normal state."]
    #[inline]
    pub fn normalmpedance(self) -> &'a mut W {
        self.variant(HIGHZW::NORMALMPEDANCE)
    }
    #[doc = "Output in High Impedance state."]
    #[inline]
    pub fn highimpedance(self) -> &'a mut W {
        self.variant(HIGHZW::HIGHIMPEDANCE)
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
#[doc = "Values that can be written to the field `VOUT`"]
pub enum VOUTW {
    #[doc = "0.750 V."]
    V_0P750,
    #[doc = "0.775 V."]
    V_0P775,
    #[doc = "0.800 V."]
    V_0P800,
    #[doc = "0.825 V."]
    V_0P825,
    #[doc = "0.850 V."]
    V_0P850,
    #[doc = "0.875 V."]
    V_0P875,
    #[doc = "0.900 V."]
    V_0P900,
    #[doc = "0.925 V."]
    V_0P925,
}
impl VOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VOUTW::V_0P750 => 0,
            VOUTW::V_0P775 => 1,
            VOUTW::V_0P800 => 2,
            VOUTW::V_0P825 => 3,
            VOUTW::V_0P850 => 4,
            VOUTW::V_0P875 => 5,
            VOUTW::V_0P900 => 6,
            VOUTW::V_0P925 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _VOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0.750 V."]
    #[inline]
    pub fn v_0p750(self) -> &'a mut W {
        self.variant(VOUTW::V_0P750)
    }
    #[doc = "0.775 V."]
    #[inline]
    pub fn v_0p775(self) -> &'a mut W {
        self.variant(VOUTW::V_0P775)
    }
    #[doc = "0.800 V."]
    #[inline]
    pub fn v_0p800(self) -> &'a mut W {
        self.variant(VOUTW::V_0P800)
    }
    #[doc = "0.825 V."]
    #[inline]
    pub fn v_0p825(self) -> &'a mut W {
        self.variant(VOUTW::V_0P825)
    }
    #[doc = "0.850 V."]
    #[inline]
    pub fn v_0p850(self) -> &'a mut W {
        self.variant(VOUTW::V_0P850)
    }
    #[doc = "0.875 V."]
    #[inline]
    pub fn v_0p875(self) -> &'a mut W {
        self.variant(VOUTW::V_0P875)
    }
    #[doc = "0.900 V."]
    #[inline]
    pub fn v_0p900(self) -> &'a mut W {
        self.variant(VOUTW::V_0P900)
    }
    #[doc = "0.925 V."]
    #[inline]
    pub fn v_0p925(self) -> &'a mut W {
        self.variant(VOUTW::V_0P925)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _IBIASW<'a> {
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
#[doc = r" Proxy"]
pub struct _STABMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _STABMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 1 - Activate LDO bypass."]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - ."]
    #[inline]
    pub fn highz(&self) -> HIGHZR {
        HIGHZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:5 - Sets the LDO output level."]
    #[inline]
    pub fn vout(&self) -> VOUTR {
        VOUTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Adjust the biasing current."]
    #[inline]
    pub fn ibias(&self) -> IBIASR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIASR { bits }
    }
    #[doc = "Bits 8:9 - Stability configuration."]
    #[inline]
    pub fn stabmode(&self) -> STABMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STABMODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 928 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Activate LDO bypass."]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 2 - ."]
    #[inline]
    pub fn highz(&mut self) -> _HIGHZW {
        _HIGHZW { w: self }
    }
    #[doc = "Bits 3:5 - Sets the LDO output level."]
    #[inline]
    pub fn vout(&mut self) -> _VOUTW {
        _VOUTW { w: self }
    }
    #[doc = "Bits 6:7 - Adjust the biasing current."]
    #[inline]
    pub fn ibias(&mut self) -> _IBIASW {
        _IBIASW { w: self }
    }
    #[doc = "Bits 8:9 - Stability configuration."]
    #[inline]
    pub fn stabmode(&mut self) -> _STABMODEW {
        _STABMODEW { w: self }
    }
}
