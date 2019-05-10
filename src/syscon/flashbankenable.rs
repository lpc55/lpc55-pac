#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASHBANKENABLE {
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
#[doc = "Possible values of the field `BANK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK0R {
    #[doc = "Flash BANK0 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    ENABLE,
    #[doc = "1010: Flash BANK0 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BANK0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BANK0R::ENABLE => 0,
            BANK0R::DISABLE => 10,
            BANK0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BANK0R {
        match value {
            0 => BANK0R::ENABLE,
            10 => BANK0R::DISABLE,
            i => BANK0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BANK0R::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BANK0R::DISABLE
    }
}
#[doc = "Possible values of the field `BANK1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK1R {
    #[doc = "Flash BANK1 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    ENABLE,
    #[doc = "1010: Flash BANK1 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BANK1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BANK1R::ENABLE => 0,
            BANK1R::DISABLE => 10,
            BANK1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BANK1R {
        match value {
            0 => BANK1R::ENABLE,
            10 => BANK1R::DISABLE,
            i => BANK1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BANK1R::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BANK1R::DISABLE
    }
}
#[doc = "Possible values of the field `BANK2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK2R {
    #[doc = "Flash BANK2 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    ENABLE,
    #[doc = "1010: Flash BANK2 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BANK2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BANK2R::ENABLE => 0,
            BANK2R::DISABLE => 10,
            BANK2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BANK2R {
        match value {
            0 => BANK2R::ENABLE,
            10 => BANK2R::DISABLE,
            i => BANK2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BANK2R::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BANK2R::DISABLE
    }
}
#[doc = "Values that can be written to the field `BANK0`"]
pub enum BANK0W {
    #[doc = "Flash BANK0 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    ENABLE,
    #[doc = "1010: Flash BANK0 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    DISABLE,
}
impl BANK0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BANK0W::ENABLE => 0,
            BANK0W::DISABLE => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BANK0W<'a> {
    w: &'a mut W,
}
impl<'a> _BANK0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BANK0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flash BANK0 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BANK0W::ENABLE)
    }
    #[doc = "1010: Flash BANK0 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BANK0W::DISABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BANK1`"]
pub enum BANK1W {
    #[doc = "Flash BANK1 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    ENABLE,
    #[doc = "1010: Flash BANK1 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    DISABLE,
}
impl BANK1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BANK1W::ENABLE => 0,
            BANK1W::DISABLE => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BANK1W<'a> {
    w: &'a mut W,
}
impl<'a> _BANK1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BANK1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flash BANK1 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BANK1W::ENABLE)
    }
    #[doc = "1010: Flash BANK1 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BANK1W::DISABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BANK2`"]
pub enum BANK2W {
    #[doc = "Flash BANK2 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    ENABLE,
    #[doc = "1010: Flash BANK2 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    DISABLE,
}
impl BANK2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BANK2W::ENABLE => 0,
            BANK2W::DISABLE => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BANK2W<'a> {
    w: &'a mut W,
}
impl<'a> _BANK2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BANK2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flash BANK2 checker is enabled (all Flash pages inside this bank cannot be erased nor programmed)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BANK2W::ENABLE)
    }
    #[doc = "1010: Flash BANK2 checker is disabled (all Flash pages inside this bank can be erased and programmed)."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BANK2W::DISABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Flash Bank0 control."]
    #[inline]
    pub fn bank0(&self) -> BANK0R {
        BANK0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Flash Bank1 control."]
    #[inline]
    pub fn bank1(&self) -> BANK1R {
        BANK1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Flash Bank2 control."]
    #[inline]
    pub fn bank2(&self) -> BANK2R {
        BANK2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:3 - Flash Bank0 control."]
    #[inline]
    pub fn bank0(&mut self) -> _BANK0W {
        _BANK0W { w: self }
    }
    #[doc = "Bits 4:7 - Flash Bank1 control."]
    #[inline]
    pub fn bank1(&mut self) -> _BANK1W {
        _BANK1W { w: self }
    }
    #[doc = "Bits 8:11 - Flash Bank2 control."]
    #[inline]
    pub fn bank2(&mut self) -> _BANK2W {
        _BANK2W { w: self }
    }
}
