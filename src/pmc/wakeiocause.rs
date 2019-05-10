#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WAKEIOCAUSE {
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
#[doc = "Possible values of the field `WAKEUP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP0R {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 0."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 0."]
    EVENT,
}
impl WAKEUP0R {
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
            WAKEUP0R::NOEVENT => false,
            WAKEUP0R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP0R {
        match value {
            false => WAKEUP0R::NOEVENT,
            true => WAKEUP0R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP0R::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP0R::EVENT
    }
}
#[doc = "Possible values of the field `WAKEUP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP1R {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    EVENT,
}
impl WAKEUP1R {
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
            WAKEUP1R::NOEVENT => false,
            WAKEUP1R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP1R {
        match value {
            false => WAKEUP1R::NOEVENT,
            true => WAKEUP1R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP1R::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP1R::EVENT
    }
}
#[doc = "Possible values of the field `WAKEUP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP2R {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    EVENT,
}
impl WAKEUP2R {
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
            WAKEUP2R::NOEVENT => false,
            WAKEUP2R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP2R {
        match value {
            false => WAKEUP2R::NOEVENT,
            true => WAKEUP2R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP2R::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP2R::EVENT
    }
}
#[doc = "Possible values of the field `WAKEUP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP3R {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    EVENT,
}
impl WAKEUP3R {
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
            WAKEUP3R::NOEVENT => false,
            WAKEUP3R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUP3R {
        match value {
            false => WAKEUP3R::NOEVENT,
            true => WAKEUP3R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline]
    pub fn is_noevent(&self) -> bool {
        *self == WAKEUP3R::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == WAKEUP3R::EVENT
    }
}
#[doc = "Values that can be written to the field `WAKEUP1`"]
pub enum WAKEUP1W {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    EVENT,
}
impl WAKEUP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP1W::NOEVENT => false,
            WAKEUP1W::EVENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP1W<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 1."]
    #[inline]
    pub fn noevent(self) -> &'a mut W {
        self.variant(WAKEUP1W::NOEVENT)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 1."]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(WAKEUP1W::EVENT)
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
#[doc = "Values that can be written to the field `WAKEUP2`"]
pub enum WAKEUP2W {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    EVENT,
}
impl WAKEUP2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP2W::NOEVENT => false,
            WAKEUP2W::EVENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP2W<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 2."]
    #[inline]
    pub fn noevent(self) -> &'a mut W {
        self.variant(WAKEUP2W::NOEVENT)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 2."]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(WAKEUP2W::EVENT)
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
#[doc = "Values that can be written to the field `WAKEUP3`"]
pub enum WAKEUP3W {
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    NOEVENT,
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    EVENT,
}
impl WAKEUP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUP3W::NOEVENT => false,
            WAKEUP3W::EVENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUP3W<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUP3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Last wake up from Deep Power down mode was NOT triggred by wake up I/O 3."]
    #[inline]
    pub fn noevent(self) -> &'a mut W {
        self.variant(WAKEUP3W::NOEVENT)
    }
    #[doc = "Last wake up from Deep Power down mode was triggred by wake up I/O 3."]
    #[inline]
    pub fn event(self) -> &'a mut W {
        self.variant(WAKEUP3W::EVENT)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Allows to identify Wake up I/O 0 as the wake-up source from Deep Power Down mode."]
    #[inline]
    pub fn wakeup0(&self) -> WAKEUP0R {
        WAKEUP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline]
    pub fn wakeup1(&self) -> WAKEUP1R {
        WAKEUP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline]
    pub fn wakeup2(&self) -> WAKEUP2R {
        WAKEUP2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline]
    pub fn wakeup3(&self) -> WAKEUP3R {
        WAKEUP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 1 - Allows to identify Wake up I/O 1 as the wake-up source from Deep Power Down mode."]
    #[inline]
    pub fn wakeup1(&mut self) -> _WAKEUP1W {
        _WAKEUP1W { w: self }
    }
    #[doc = "Bit 2 - Allows to identify Wake up I/O 2 as the wake-up source from Deep Power Down mode."]
    #[inline]
    pub fn wakeup2(&mut self) -> _WAKEUP2W {
        _WAKEUP2W { w: self }
    }
    #[doc = "Bit 3 - Allows to identify Wake up I/O 3 as the wake-up source from Deep Power Down mode."]
    #[inline]
    pub fn wakeup3(&mut self) -> _WAKEUP3W {
        _WAKEUP3W { w: self }
    }
}
