#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG_FEATURES {
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
#[doc = "Possible values of the field `CM33_DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM33_DBGENR {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CM33_DBGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM33_DBGENR::DISABLE => 1,
            CM33_DBGENR::ENABLE => 2,
            CM33_DBGENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM33_DBGENR {
        match value {
            1 => CM33_DBGENR::DISABLE,
            2 => CM33_DBGENR::ENABLE,
            i => CM33_DBGENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CM33_DBGENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CM33_DBGENR::ENABLE
    }
}
#[doc = "Possible values of the field `CM33_NIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM33_NIDENR {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CM33_NIDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM33_NIDENR::DISABLE => 1,
            CM33_NIDENR::ENABLE => 2,
            CM33_NIDENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM33_NIDENR {
        match value {
            1 => CM33_NIDENR::DISABLE,
            2 => CM33_NIDENR::ENABLE,
            i => CM33_NIDENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CM33_NIDENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CM33_NIDENR::ENABLE
    }
}
#[doc = "Possible values of the field `CM33_SPIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM33_SPIDENR {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CM33_SPIDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM33_SPIDENR::DISABLE => 1,
            CM33_SPIDENR::ENABLE => 2,
            CM33_SPIDENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM33_SPIDENR {
        match value {
            1 => CM33_SPIDENR::DISABLE,
            2 => CM33_SPIDENR::ENABLE,
            i => CM33_SPIDENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CM33_SPIDENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CM33_SPIDENR::ENABLE
    }
}
#[doc = "Possible values of the field `CM33_SPNIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM33_SPNIDENR {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CM33_SPNIDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM33_SPNIDENR::DISABLE => 1,
            CM33_SPNIDENR::ENABLE => 2,
            CM33_SPNIDENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM33_SPNIDENR {
        match value {
            1 => CM33_SPNIDENR::DISABLE,
            2 => CM33_SPNIDENR::ENABLE,
            i => CM33_SPNIDENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CM33_SPNIDENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CM33_SPNIDENR::ENABLE
    }
}
#[doc = "Possible values of the field `MCM33_DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33_DBGENR {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MCM33_DBGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCM33_DBGENR::DISABLE => 1,
            MCM33_DBGENR::ENABLE => 2,
            MCM33_DBGENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCM33_DBGENR {
        match value {
            1 => MCM33_DBGENR::DISABLE,
            2 => MCM33_DBGENR::ENABLE,
            i => MCM33_DBGENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MCM33_DBGENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MCM33_DBGENR::ENABLE
    }
}
#[doc = "Possible values of the field `MCM33_NIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33_NIDENR {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MCM33_NIDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCM33_NIDENR::DISABLE => 1,
            MCM33_NIDENR::ENABLE => 2,
            MCM33_NIDENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCM33_NIDENR {
        match value {
            1 => MCM33_NIDENR::DISABLE,
            2 => MCM33_NIDENR::ENABLE,
            i => MCM33_NIDENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == MCM33_NIDENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MCM33_NIDENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `CM33_DBGEN`"]
pub enum CM33_DBGENW {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl CM33_DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CM33_DBGENW::DISABLE => 1,
            CM33_DBGENW::ENABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM33_DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _CM33_DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM33_DBGENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CM33_DBGENW::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CM33_DBGENW::ENABLE)
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
#[doc = "Values that can be written to the field `CM33_NIDEN`"]
pub enum CM33_NIDENW {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl CM33_NIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CM33_NIDENW::DISABLE => 1,
            CM33_NIDENW::ENABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM33_NIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _CM33_NIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM33_NIDENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CM33_NIDENW::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CM33_NIDENW::ENABLE)
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
#[doc = "Values that can be written to the field `CM33_SPIDEN`"]
pub enum CM33_SPIDENW {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl CM33_SPIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CM33_SPIDENW::DISABLE => 1,
            CM33_SPIDENW::ENABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM33_SPIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _CM33_SPIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM33_SPIDENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CM33_SPIDENW::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CM33_SPIDENW::ENABLE)
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
#[doc = "Values that can be written to the field `CM33_SPNIDEN`"]
pub enum CM33_SPNIDENW {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl CM33_SPNIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CM33_SPNIDENW::DISABLE => 1,
            CM33_SPNIDENW::ENABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CM33_SPNIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _CM33_SPNIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM33_SPNIDENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CM33_SPNIDENW::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CM33_SPNIDENW::ENABLE)
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
#[doc = "Values that can be written to the field `MCM33_DBGEN`"]
pub enum MCM33_DBGENW {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl MCM33_DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCM33_DBGENW::DISABLE => 1,
            MCM33_DBGENW::ENABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCM33_DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33_DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCM33_DBGENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MCM33_DBGENW::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MCM33_DBGENW::ENABLE)
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
#[doc = "Values that can be written to the field `MCM33_NIDEN`"]
pub enum MCM33_NIDENW {
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE,
}
impl MCM33_NIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCM33_NIDENW::DISABLE => 1,
            MCM33_NIDENW::ENABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCM33_NIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33_NIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCM33_NIDENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(MCM33_NIDENW::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MCM33_NIDENW::ENABLE)
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
    #[doc = "Bits 0:1 - CM33 (CPU0) Invasive debug control:."]
    #[inline]
    pub fn cm33_dbgen(&self) -> CM33_DBGENR {
        CM33_DBGENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - CM33 (CPU0) Non Invasive debug control:."]
    #[inline]
    pub fn cm33_niden(&self) -> CM33_NIDENR {
        CM33_NIDENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - CM33 (CPU0) Secure Invasive debug control:."]
    #[inline]
    pub fn cm33_spiden(&self) -> CM33_SPIDENR {
        CM33_SPIDENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - CM33 (CPU0) Secure Non Invasive debug control:."]
    #[inline]
    pub fn cm33_spniden(&self) -> CM33_SPNIDENR {
        CM33_SPNIDENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Micro-CM33 (CPU1) Invasive debug control:."]
    #[inline]
    pub fn mcm33_dbgen(&self) -> MCM33_DBGENR {
        MCM33_DBGENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Micro-CM33 (CPU1) Non Invasive debug control:."]
    #[inline]
    pub fn mcm33_niden(&self) -> MCM33_NIDENR {
        MCM33_NIDENR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - CM33 (CPU0) Invasive debug control:."]
    #[inline]
    pub fn cm33_dbgen(&mut self) -> _CM33_DBGENW {
        _CM33_DBGENW { w: self }
    }
    #[doc = "Bits 2:3 - CM33 (CPU0) Non Invasive debug control:."]
    #[inline]
    pub fn cm33_niden(&mut self) -> _CM33_NIDENW {
        _CM33_NIDENW { w: self }
    }
    #[doc = "Bits 4:5 - CM33 (CPU0) Secure Invasive debug control:."]
    #[inline]
    pub fn cm33_spiden(&mut self) -> _CM33_SPIDENW {
        _CM33_SPIDENW { w: self }
    }
    #[doc = "Bits 6:7 - CM33 (CPU0) Secure Non Invasive debug control:."]
    #[inline]
    pub fn cm33_spniden(&mut self) -> _CM33_SPNIDENW {
        _CM33_SPNIDENW { w: self }
    }
    #[doc = "Bits 8:9 - Micro-CM33 (CPU1) Invasive debug control:."]
    #[inline]
    pub fn mcm33_dbgen(&mut self) -> _MCM33_DBGENW {
        _MCM33_DBGENW { w: self }
    }
    #[doc = "Bits 10:11 - Micro-CM33 (CPU1) Non Invasive debug control:."]
    #[inline]
    pub fn mcm33_niden(&mut self) -> _MCM33_NIDENW {
        _MCM33_NIDENW { w: self }
    }
}
