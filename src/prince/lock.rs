#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCK {
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
#[doc = "Possible values of the field `LOCKREG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKREG0R {
    #[doc = "Disabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are writable.."]
    DISABLED,
    #[doc = "Enabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are not writable.."]
    ENABLED,
}
impl LOCKREG0R {
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
            LOCKREG0R::DISABLED => false,
            LOCKREG0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKREG0R {
        match value {
            false => LOCKREG0R::DISABLED,
            true => LOCKREG0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKREG0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKREG0R::ENABLED
    }
}
#[doc = "Possible values of the field `LOCKREG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKREG1R {
    #[doc = "Disabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are writable.."]
    DISABLED,
    #[doc = "Enabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are not writable.."]
    ENABLED,
}
impl LOCKREG1R {
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
            LOCKREG1R::DISABLED => false,
            LOCKREG1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKREG1R {
        match value {
            false => LOCKREG1R::DISABLED,
            true => LOCKREG1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKREG1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKREG1R::ENABLED
    }
}
#[doc = "Possible values of the field `LOCKREG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKREG2R {
    #[doc = "Disabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are writable.."]
    DISABLED,
    #[doc = "Enabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are not writable.."]
    ENABLED,
}
impl LOCKREG2R {
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
            LOCKREG2R::DISABLED => false,
            LOCKREG2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKREG2R {
        match value {
            false => LOCKREG2R::DISABLED,
            true => LOCKREG2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKREG2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKREG2R::ENABLED
    }
}
#[doc = "Possible values of the field `LOCKMASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKMASKR {
    #[doc = "Disabled. MASK_LSB, and MASK_MSB are writable.."]
    DISABLED,
    #[doc = "Enabled. MASK_LSB, and MASK_MSB are not writable.."]
    ENABLED,
}
impl LOCKMASKR {
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
            LOCKMASKR::DISABLED => false,
            LOCKMASKR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKMASKR {
        match value {
            false => LOCKMASKR::DISABLED,
            true => LOCKMASKR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKMASKR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LOCKMASKR::ENABLED
    }
}
#[doc = "Values that can be written to the field `LOCKREG0`"]
pub enum LOCKREG0W {
    #[doc = "Disabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are writable.."]
    DISABLED,
    #[doc = "Enabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are not writable.."]
    ENABLED,
}
impl LOCKREG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKREG0W::DISABLED => false,
            LOCKREG0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKREG0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKREG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKREG0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are writable.."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKREG0W::DISABLED)
    }
    #[doc = "Enabled. IV_LSB0, IV_MSB0, BASE_ADDR0, and SR_ENABLE0 are not writable.."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCKREG0W::ENABLED)
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
#[doc = "Values that can be written to the field `LOCKREG1`"]
pub enum LOCKREG1W {
    #[doc = "Disabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are writable.."]
    DISABLED,
    #[doc = "Enabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are not writable.."]
    ENABLED,
}
impl LOCKREG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKREG1W::DISABLED => false,
            LOCKREG1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKREG1W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKREG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKREG1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are writable.."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKREG1W::DISABLED)
    }
    #[doc = "Enabled. IV_LSB1, IV_MSB1, BASE_ADDR1, and SR_ENABLE1 are not writable.."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCKREG1W::ENABLED)
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
#[doc = "Values that can be written to the field `LOCKREG2`"]
pub enum LOCKREG2W {
    #[doc = "Disabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are writable.."]
    DISABLED,
    #[doc = "Enabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are not writable.."]
    ENABLED,
}
impl LOCKREG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKREG2W::DISABLED => false,
            LOCKREG2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKREG2W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKREG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKREG2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are writable.."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKREG2W::DISABLED)
    }
    #[doc = "Enabled. IV_LSB2, IV_MSB2, BASE_ADDR2, and SR_ENABLE2 are not writable.."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCKREG2W::ENABLED)
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
#[doc = "Values that can be written to the field `LOCKMASK`"]
pub enum LOCKMASKW {
    #[doc = "Disabled. MASK_LSB, and MASK_MSB are writable.."]
    DISABLED,
    #[doc = "Enabled. MASK_LSB, and MASK_MSB are not writable.."]
    ENABLED,
}
impl LOCKMASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKMASKW::DISABLED => false,
            LOCKMASKW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKMASKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKMASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. MASK_LSB, and MASK_MSB are writable.."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKMASKW::DISABLED)
    }
    #[doc = "Enabled. MASK_LSB, and MASK_MSB are not writable.."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCKMASKW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Lock Region 0 registers."]
    #[inline]
    pub fn lockreg0(&self) -> LOCKREG0R {
        LOCKREG0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Lock Region 1 registers."]
    #[inline]
    pub fn lockreg1(&self) -> LOCKREG1R {
        LOCKREG1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Lock Region 2 registers."]
    #[inline]
    pub fn lockreg2(&self) -> LOCKREG2R {
        LOCKREG2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Lock the Mask registers."]
    #[inline]
    pub fn lockmask(&self) -> LOCKMASKR {
        LOCKMASKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Lock Region 0 registers."]
    #[inline]
    pub fn lockreg0(&mut self) -> _LOCKREG0W {
        _LOCKREG0W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 1 registers."]
    #[inline]
    pub fn lockreg1(&mut self) -> _LOCKREG1W {
        _LOCKREG1W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 2 registers."]
    #[inline]
    pub fn lockreg2(&mut self) -> _LOCKREG2W {
        _LOCKREG2W { w: self }
    }
    #[doc = "Bit 8 - Lock the Mask registers."]
    #[inline]
    pub fn lockmask(&mut self) -> _LOCKMASKW {
        _LOCKMASKW { w: self }
    }
}
