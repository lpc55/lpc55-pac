#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_MASK_LOCK {
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
#[doc = "Possible values of the field `SEC_GPIO_MASK0_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_MASK0_LOCKR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEC_GPIO_MASK0_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEC_GPIO_MASK0_LOCKR::BLOCKED => 1,
            SEC_GPIO_MASK0_LOCKR::WRITABLE => 2,
            SEC_GPIO_MASK0_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEC_GPIO_MASK0_LOCKR {
        match value {
            1 => SEC_GPIO_MASK0_LOCKR::BLOCKED,
            2 => SEC_GPIO_MASK0_LOCKR::WRITABLE,
            i => SEC_GPIO_MASK0_LOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == SEC_GPIO_MASK0_LOCKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == SEC_GPIO_MASK0_LOCKR::WRITABLE
    }
}
#[doc = "Possible values of the field `SEC_GPIO_MASK1_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_MASK1_LOCKR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEC_GPIO_MASK1_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEC_GPIO_MASK1_LOCKR::BLOCKED => 1,
            SEC_GPIO_MASK1_LOCKR::WRITABLE => 2,
            SEC_GPIO_MASK1_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEC_GPIO_MASK1_LOCKR {
        match value {
            1 => SEC_GPIO_MASK1_LOCKR::BLOCKED,
            2 => SEC_GPIO_MASK1_LOCKR::WRITABLE,
            i => SEC_GPIO_MASK1_LOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == SEC_GPIO_MASK1_LOCKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == SEC_GPIO_MASK1_LOCKR::WRITABLE
    }
}
#[doc = "Possible values of the field `SEC_CPU1_INT_MASK0_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_CPU1_INT_MASK0_LOCKR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEC_CPU1_INT_MASK0_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEC_CPU1_INT_MASK0_LOCKR::BLOCKED => 1,
            SEC_CPU1_INT_MASK0_LOCKR::WRITABLE => 2,
            SEC_CPU1_INT_MASK0_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEC_CPU1_INT_MASK0_LOCKR {
        match value {
            1 => SEC_CPU1_INT_MASK0_LOCKR::BLOCKED,
            2 => SEC_CPU1_INT_MASK0_LOCKR::WRITABLE,
            i => SEC_CPU1_INT_MASK0_LOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == SEC_CPU1_INT_MASK0_LOCKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == SEC_CPU1_INT_MASK0_LOCKR::WRITABLE
    }
}
#[doc = "Possible values of the field `SEC_CPU1_INT_MASK1_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_CPU1_INT_MASK1_LOCKR {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEC_CPU1_INT_MASK1_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEC_CPU1_INT_MASK1_LOCKR::BLOCKED => 1,
            SEC_CPU1_INT_MASK1_LOCKR::WRITABLE => 2,
            SEC_CPU1_INT_MASK1_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEC_CPU1_INT_MASK1_LOCKR {
        match value {
            1 => SEC_CPU1_INT_MASK1_LOCKR::BLOCKED,
            2 => SEC_CPU1_INT_MASK1_LOCKR::WRITABLE,
            i => SEC_CPU1_INT_MASK1_LOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline]
    pub fn is_blocked(&self) -> bool {
        *self == SEC_CPU1_INT_MASK1_LOCKR::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline]
    pub fn is_writable(&self) -> bool {
        *self == SEC_CPU1_INT_MASK1_LOCKR::WRITABLE
    }
}
#[doc = "Values that can be written to the field `SEC_GPIO_MASK0_LOCK`"]
pub enum SEC_GPIO_MASK0_LOCKW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl SEC_GPIO_MASK0_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEC_GPIO_MASK0_LOCKW::BLOCKED => 1,
            SEC_GPIO_MASK0_LOCKW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_GPIO_MASK0_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_MASK0_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_GPIO_MASK0_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK0_LOCKW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK0_LOCKW::WRITABLE)
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
#[doc = "Values that can be written to the field `SEC_GPIO_MASK1_LOCK`"]
pub enum SEC_GPIO_MASK1_LOCKW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl SEC_GPIO_MASK1_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEC_GPIO_MASK1_LOCKW::BLOCKED => 1,
            SEC_GPIO_MASK1_LOCKW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_GPIO_MASK1_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_MASK1_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_GPIO_MASK1_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK1_LOCKW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK1_LOCKW::WRITABLE)
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
#[doc = "Values that can be written to the field `SEC_CPU1_INT_MASK0_LOCK`"]
pub enum SEC_CPU1_INT_MASK0_LOCKW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl SEC_CPU1_INT_MASK0_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEC_CPU1_INT_MASK0_LOCKW::BLOCKED => 1,
            SEC_CPU1_INT_MASK0_LOCKW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_CPU1_INT_MASK0_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_CPU1_INT_MASK0_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_CPU1_INT_MASK0_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK0_LOCKW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK0_LOCKW::WRITABLE)
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
#[doc = "Values that can be written to the field `SEC_CPU1_INT_MASK1_LOCK`"]
pub enum SEC_CPU1_INT_MASK1_LOCKW {
    #[doc = "Restricted mode."]
    BLOCKED,
    #[doc = "Writable."]
    WRITABLE,
}
impl SEC_CPU1_INT_MASK1_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SEC_CPU1_INT_MASK1_LOCKW::BLOCKED => 1,
            SEC_CPU1_INT_MASK1_LOCKW::WRITABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_CPU1_INT_MASK1_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_CPU1_INT_MASK1_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_CPU1_INT_MASK1_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK1_LOCKW::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK1_LOCKW::WRITABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline]
    pub fn sec_gpio_mask0_lock(&self) -> SEC_GPIO_MASK0_LOCKR {
        SEC_GPIO_MASK0_LOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline]
    pub fn sec_gpio_mask1_lock(&self) -> SEC_GPIO_MASK1_LOCKR {
        SEC_GPIO_MASK1_LOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - SEC_CPU_INT_MASK0 register write-lock."]
    #[inline]
    pub fn sec_cpu1_int_mask0_lock(&self) -> SEC_CPU1_INT_MASK0_LOCKR {
        SEC_CPU1_INT_MASK0_LOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - SEC_CPU_INT_MASK1 register write-lock."]
    #[inline]
    pub fn sec_cpu1_int_mask1_lock(&self) -> SEC_CPU1_INT_MASK1_LOCKR {
        SEC_CPU1_INT_MASK1_LOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2730 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline]
    pub fn sec_gpio_mask0_lock(&mut self) -> _SEC_GPIO_MASK0_LOCKW {
        _SEC_GPIO_MASK0_LOCKW { w: self }
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline]
    pub fn sec_gpio_mask1_lock(&mut self) -> _SEC_GPIO_MASK1_LOCKW {
        _SEC_GPIO_MASK1_LOCKW { w: self }
    }
    #[doc = "Bits 8:9 - SEC_CPU_INT_MASK0 register write-lock."]
    #[inline]
    pub fn sec_cpu1_int_mask0_lock(&mut self) -> _SEC_CPU1_INT_MASK0_LOCKW {
        _SEC_CPU1_INT_MASK0_LOCKW { w: self }
    }
    #[doc = "Bits 10:11 - SEC_CPU_INT_MASK1 register write-lock."]
    #[inline]
    pub fn sec_cpu1_int_mask1_lock(&mut self) -> _SEC_CPU1_INT_MASK1_LOCKW {
        _SEC_CPU1_INT_MASK1_LOCKW { w: self }
    }
}
