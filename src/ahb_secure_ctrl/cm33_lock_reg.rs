#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CM33_LOCK_REG {
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
#[doc = "Possible values of the field `LOCK_S_VTAIRCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_S_VTAIRCRR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_S_VTAIRCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_S_VTAIRCRR::BLOCKED => 1,
            LOCK_S_VTAIRCRR::WRITABLE => 2,
            LOCK_S_VTAIRCRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_S_VTAIRCRR {
        match value {
            1 => LOCK_S_VTAIRCRR::BLOCKED,
            2 => LOCK_S_VTAIRCRR::WRITABLE,
            i => LOCK_S_VTAIRCRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_S_VTAIRCRR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_S_VTAIRCRR::WRITABLE
    }
}
#[doc = "Possible values of the field `LOCK_S_MPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_S_MPUR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_S_MPUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_S_MPUR::BLOCKED => 1,
            LOCK_S_MPUR::WRITABLE => 2,
            LOCK_S_MPUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_S_MPUR {
        match value {
            1 => LOCK_S_MPUR::BLOCKED,
            2 => LOCK_S_MPUR::WRITABLE,
            i => LOCK_S_MPUR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_S_MPUR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_S_MPUR::WRITABLE
    }
}
#[doc = "Possible values of the field `LOCK_SAU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_SAUR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOCK_SAUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOCK_SAUR::BLOCKED => 1,
            LOCK_SAUR::WRITABLE => 2,
            LOCK_SAUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOCK_SAUR {
        match value {
            1 => LOCK_SAUR::BLOCKED,
            2 => LOCK_SAUR::WRITABLE,
            i => LOCK_SAUR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == LOCK_SAUR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == LOCK_SAUR::WRITABLE
    }
}
#[doc = "Possible values of the field `CM33_LOCK_REG_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM33_LOCK_REG_LOCKR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CM33_LOCK_REG_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM33_LOCK_REG_LOCKR::BLOCKED => 1,
            CM33_LOCK_REG_LOCKR::WRITABLE => 2,
            CM33_LOCK_REG_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM33_LOCK_REG_LOCKR {
        match value {
            1 => CM33_LOCK_REG_LOCKR::BLOCKED,
            2 => CM33_LOCK_REG_LOCKR::WRITABLE,
            i => CM33_LOCK_REG_LOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == CM33_LOCK_REG_LOCKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == CM33_LOCK_REG_LOCKR::WRITABLE
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
#[doc = "Values that can be written to the field `LOCK_S_VTAIRCR`"]
pub enum LOCK_S_VTAIRCRW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl LOCK_S_VTAIRCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_S_VTAIRCRW::BLOCKED => 1,
            LOCK_S_VTAIRCRW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_S_VTAIRCRW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_S_VTAIRCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_S_VTAIRCRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_S_VTAIRCRW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_S_VTAIRCRW::WRITABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK_S_MPU`"]
pub enum LOCK_S_MPUW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl LOCK_S_MPUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_S_MPUW::BLOCKED => 1,
            LOCK_S_MPUW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_S_MPUW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_S_MPUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_S_MPUW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_S_MPUW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_S_MPUW::WRITABLE)
    }
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
#[doc = "Values that can be written to the field `LOCK_SAU`"]
pub enum LOCK_SAUW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl LOCK_SAUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LOCK_SAUW::BLOCKED => 1,
            LOCK_SAUW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_SAUW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_SAUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_SAUW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(LOCK_SAUW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(LOCK_SAUW::WRITABLE)
    }
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
#[doc = "Values that can be written to the field `CM33_LOCK_REG_LOCK`"]
pub enum CM33_LOCK_REG_LOCKW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl CM33_LOCK_REG_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CM33_LOCK_REG_LOCKW::BLOCKED => 1,
            CM33_LOCK_REG_LOCKW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM33_LOCK_REG_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _CM33_LOCK_REG_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM33_LOCK_REG_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(CM33_LOCK_REG_LOCKW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(CM33_LOCK_REG_LOCKW::WRITABLE)
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
    #[doc = "Bits 0:1 - CM33 (CPU0) VTOR_NS register write-lock."]
    #[inline]
    pub fn lock_ns_vtor(&self) -> LOCK_NS_VTORR {
        LOCK_NS_VTORR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - CM33 (CPU0) non-secure MPU register write-lock."]
    #[inline]
    pub fn lock_ns_mpu(&self) -> LOCK_NS_MPUR {
        LOCK_NS_MPUR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - CM33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[inline]
    pub fn lock_s_vtaircr(&self) -> LOCK_S_VTAIRCRR {
        LOCK_S_VTAIRCRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - CM33 (CPU0) Secure MPU registers write-lock."]
    #[inline]
    pub fn lock_s_mpu(&self) -> LOCK_S_MPUR {
        LOCK_S_MPUR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - CM33 (CPU0) SAU registers write-lock."]
    #[inline]
    pub fn lock_sau(&self) -> LOCK_SAUR {
        LOCK_SAUR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - CM33_LOCK_REG write-lock."]
    #[inline]
    pub fn cm33_lock_reg_lock(&self) -> CM33_LOCK_REG_LOCKR {
        CM33_LOCK_REG_LOCKR::_from({
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
        W { bits: 2147484330 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - CM33 (CPU0) VTOR_NS register write-lock."]
    #[inline]
    pub fn lock_ns_vtor(&mut self) -> _LOCK_NS_VTORW {
        _LOCK_NS_VTORW { w: self }
    }
    #[doc = "Bits 2:3 - CM33 (CPU0) non-secure MPU register write-lock."]
    #[inline]
    pub fn lock_ns_mpu(&mut self) -> _LOCK_NS_MPUW {
        _LOCK_NS_MPUW { w: self }
    }
    #[doc = "Bits 4:5 - CM33 (CPU0) VTOR_S, AIRCR.PRIS, IRCR.BFHFNMINS registers write-lock."]
    #[inline]
    pub fn lock_s_vtaircr(&mut self) -> _LOCK_S_VTAIRCRW {
        _LOCK_S_VTAIRCRW { w: self }
    }
    #[doc = "Bits 6:7 - CM33 (CPU0) Secure MPU registers write-lock."]
    #[inline]
    pub fn lock_s_mpu(&mut self) -> _LOCK_S_MPUW {
        _LOCK_S_MPUW { w: self }
    }
    #[doc = "Bits 8:9 - CM33 (CPU0) SAU registers write-lock."]
    #[inline]
    pub fn lock_sau(&mut self) -> _LOCK_SAUW {
        _LOCK_SAUW { w: self }
    }
    #[doc = "Bits 30:31 - CM33_LOCK_REG write-lock."]
    #[inline]
    pub fn cm33_lock_reg_lock(&mut self) -> _CM33_LOCK_REG_LOCKW {
        _CM33_LOCK_REG_LOCKW { w: self }
    }
}
