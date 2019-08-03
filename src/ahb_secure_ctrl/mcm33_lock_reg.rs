#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCM33_LOCK_REG {
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
#[doc = "Possible values of the field `LOCK_NS_VTOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_NS_VTORR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_NS_VTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_NS_VTORR::BLOCKED => 1,
            LOCK_NS_VTORR::WRITABLE => 2,
            LOCK_NS_VTORR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_NS_VTORR {
        match value {
            1 => LOCK_NS_VTORR::BLOCKED,
            2 => LOCK_NS_VTORR::WRITABLE,
            i => LOCK_NS_VTORR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_NS_VTORR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_NS_VTORR::WRITABLE
    }
}
#[doc = "Possible values of the field `LOCK_NS_MPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_NS_MPUR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_NS_MPUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_NS_MPUR::BLOCKED => 1,
            LOCK_NS_MPUR::WRITABLE => 2,
            LOCK_NS_MPUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_NS_MPUR {
        match value {
            1 => LOCK_NS_MPUR::BLOCKED,
            2 => LOCK_NS_MPUR::WRITABLE,
            i => LOCK_NS_MPUR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_NS_MPUR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_NS_MPUR::WRITABLE
    }
}
#[doc = "Possible values of the field `MCM33_LOCK_REG_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33_LOCK_REG_LOCKR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MCM33_LOCK_REG_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCM33_LOCK_REG_LOCKR::BLOCKED => 1,
            MCM33_LOCK_REG_LOCKR::WRITABLE => 2,
            MCM33_LOCK_REG_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCM33_LOCK_REG_LOCKR {
        match value {
            1 => MCM33_LOCK_REG_LOCKR::BLOCKED,
            2 => MCM33_LOCK_REG_LOCKR::WRITABLE,
            i => MCM33_LOCK_REG_LOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == MCM33_LOCK_REG_LOCKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == MCM33_LOCK_REG_LOCKR::WRITABLE
    }
}
#[doc = "Values that can be written to the field `LOCK_NS_VTOR`"]
pub enum LOCK_NS_VTORW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl LOCK_NS_VTORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_NS_VTORW::BLOCKED => 1,
            LOCK_NS_VTORW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_NS_VTORW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_NS_VTORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_NS_VTORW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_NS_VTORW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_NS_VTORW::WRITABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK_NS_MPU`"]
pub enum LOCK_NS_MPUW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl LOCK_NS_MPUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_NS_MPUW::BLOCKED => 1,
            LOCK_NS_MPUW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_NS_MPUW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_NS_MPUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_NS_MPUW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_NS_MPUW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_NS_MPUW::WRITABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCM33_LOCK_REG_LOCK`"]
pub enum MCM33_LOCK_REG_LOCKW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl MCM33_LOCK_REG_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCM33_LOCK_REG_LOCKW::BLOCKED => 1,
            MCM33_LOCK_REG_LOCKW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCM33_LOCK_REG_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33_LOCK_REG_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCM33_LOCK_REG_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(MCM33_LOCK_REG_LOCKW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(MCM33_LOCK_REG_LOCKW::WRITABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - micro-CM33 (CPU1) VTOR_NS register write-lock."]
    #[inline]
    pub fn lock_ns_vtor(&self) -> LOCK_NS_VTORR {
        LOCK_NS_VTORR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - micro-CM33 (CPU1) non-secure MPU register write-lock."]
    #[inline]
    pub fn lock_ns_mpu(&self) -> LOCK_NS_MPUR {
        LOCK_NS_MPUR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - MCM33_LOCK_REG write-lock."]
    #[inline]
    pub fn mcm33_lock_reg_lock(&self) -> MCM33_LOCK_REG_LOCKR {
        MCM33_LOCK_REG_LOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483658 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - micro-CM33 (CPU1) VTOR_NS register write-lock."]
    #[inline]
    pub fn lock_ns_vtor(&mut self) -> _LOCK_NS_VTORW {
        _LOCK_NS_VTORW { w: self }
    }
    #[doc = "Bits 2:3 - micro-CM33 (CPU1) non-secure MPU register write-lock."]
    #[inline]
    pub fn lock_ns_mpu(&mut self) -> _LOCK_NS_MPUW {
        _LOCK_NS_MPUW { w: self }
    }
    #[doc = "Bits 30:31 - MCM33_LOCK_REG write-lock."]
    #[inline]
    pub fn mcm33_lock_reg_lock(&mut self) -> _MCM33_LOCK_REG_LOCKW {
        _MCM33_LOCK_REG_LOCKW { w: self }
    }
}
