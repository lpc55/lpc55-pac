#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLVADR {
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
#[doc = "Possible values of the field `SADISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SADISABLER {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED,
}
impl SADISABLER {
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
            SADISABLER::ENABLED => false,
            SADISABLER::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SADISABLER {
        match value {
            false => SADISABLER::ENABLED,
            true => SADISABLER::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SADISABLER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SADISABLER::DISABLED
    }
}
#[doc = r" Value of the field"]
pub struct SLVADRR {
    bits: u8,
}
impl SLVADRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `AUTONACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTONACKR {
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    NORMAL,
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    AUTOMATIC,
}
impl AUTONACKR {
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
            AUTONACKR::NORMAL => false,
            AUTONACKR::AUTOMATIC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTONACKR {
        match value {
            false => AUTONACKR::NORMAL,
            true => AUTONACKR::AUTOMATIC,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == AUTONACKR::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline]
    pub fn is_automatic(&self) -> bool {
        *self == AUTONACKR::AUTOMATIC
    }
}
#[doc = "Values that can be written to the field `SADISABLE`"]
pub enum SADISABLEW {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED,
}
impl SADISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SADISABLEW::ENABLED => false,
            SADISABLEW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SADISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SADISABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SADISABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SADISABLEW::ENABLED)
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SADISABLEW::DISABLED)
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
#[doc = r" Proxy"]
pub struct _SLVADRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVADRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUTONACK`"]
pub enum AUTONACKW {
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    NORMAL,
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    AUTOMATIC,
}
impl AUTONACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTONACKW::NORMAL => false,
            AUTONACKW::AUTOMATIC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTONACKW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTONACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTONACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(AUTONACKW::NORMAL)
    }
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    #[inline]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AUTONACKW::AUTOMATIC)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline]
    pub fn sadisable(&self) -> SADISABLER {
        SADISABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline]
    pub fn slvadr(&self) -> SLVADRR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLVADRR { bits }
    }
    #[doc = "Bit 15 - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline]
    pub fn autonack(&self) -> AUTONACKR {
        AUTONACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline]
    pub fn sadisable(&mut self) -> _SADISABLEW {
        _SADISABLEW { w: self }
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline]
    pub fn slvadr(&mut self) -> _SLVADRW {
        _SLVADRW { w: self }
    }
    #[doc = "Bit 15 - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline]
    pub fn autonack(&mut self) -> _AUTONACKW {
        _AUTONACKW { w: self }
    }
}
