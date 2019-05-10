#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCCTRLSEL {
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
#[doc = "Possible values of the field `SCKINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCKINSELR {
    #[doc = "Selects the dedicated FCn_SCK function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS,
    #[doc = "SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS,
    #[doc = "SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCKINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCKINSELR::ORIG_FLEX_I2S_SIGNALS => 0,
            SCKINSELR::SHARED_SET0_I2S_SIGNALS => 1,
            SCKINSELR::SHARED_SET1_I2S_SIGNALS => 2,
            SCKINSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCKINSELR {
        match value {
            0 => SCKINSELR::ORIG_FLEX_I2S_SIGNALS,
            1 => SCKINSELR::SHARED_SET0_I2S_SIGNALS,
            2 => SCKINSELR::SHARED_SET1_I2S_SIGNALS,
            i => SCKINSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == SCKINSELR::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == SCKINSELR::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == SCKINSELR::SHARED_SET1_I2S_SIGNALS
    }
}
#[doc = "Possible values of the field `WSINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSINSELR {
    #[doc = "Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS,
    #[doc = "WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS,
    #[doc = "WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WSINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WSINSELR::ORIG_FLEX_I2S_SIGNALS => 0,
            WSINSELR::SHARED_SET0_I2S_SIGNALS => 1,
            WSINSELR::SHARED_SET1_I2S_SIGNALS => 2,
            WSINSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WSINSELR {
        match value {
            0 => WSINSELR::ORIG_FLEX_I2S_SIGNALS,
            1 => WSINSELR::SHARED_SET0_I2S_SIGNALS,
            2 => WSINSELR::SHARED_SET1_I2S_SIGNALS,
            i => WSINSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == WSINSELR::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == WSINSELR::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == WSINSELR::SHARED_SET1_I2S_SIGNALS
    }
}
#[doc = "Possible values of the field `DATAINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAINSELR {
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS,
    #[doc = "Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS,
    #[doc = "Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATAINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATAINSELR::ORIG_FLEX_I2S_SIGNALS => 0,
            DATAINSELR::SHARED_SET0_I2S_SIGNALS => 1,
            DATAINSELR::SHARED_SET1_I2S_SIGNALS => 2,
            DATAINSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATAINSELR {
        match value {
            0 => DATAINSELR::ORIG_FLEX_I2S_SIGNALS,
            1 => DATAINSELR::SHARED_SET0_I2S_SIGNALS,
            2 => DATAINSELR::SHARED_SET1_I2S_SIGNALS,
            i => DATAINSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == DATAINSELR::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == DATAINSELR::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == DATAINSELR::SHARED_SET1_I2S_SIGNALS
    }
}
#[doc = "Possible values of the field `DATAOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAOUTSELR {
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS,
    #[doc = "Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS,
    #[doc = "Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATAOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATAOUTSELR::ORIG_FLEX_I2S_SIGNALS => 0,
            DATAOUTSELR::SHARED_SET0_I2S_SIGNALS => 1,
            DATAOUTSELR::SHARED_SET1_I2S_SIGNALS => 2,
            DATAOUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATAOUTSELR {
        match value {
            0 => DATAOUTSELR::ORIG_FLEX_I2S_SIGNALS,
            1 => DATAOUTSELR::SHARED_SET0_I2S_SIGNALS,
            2 => DATAOUTSELR::SHARED_SET1_I2S_SIGNALS,
            i => DATAOUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == DATAOUTSELR::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == DATAOUTSELR::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == DATAOUTSELR::SHARED_SET1_I2S_SIGNALS
    }
}
#[doc = "Values that can be written to the field `SCKINSEL`"]
pub enum SCKINSELW {
    #[doc = "Selects the dedicated FCn_SCK function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS,
    #[doc = "SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS,
    #[doc = "SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS,
}
impl SCKINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCKINSELW::ORIG_FLEX_I2S_SIGNALS => 0,
            SCKINSELW::SHARED_SET0_I2S_SIGNALS => 1,
            SCKINSELW::SHARED_SET1_I2S_SIGNALS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCKINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SCKINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCKINSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects the dedicated FCn_SCK function for this Flexcomm."]
    #[inline]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(SCKINSELW::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(SCKINSELW::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(SCKINSELW::SHARED_SET1_I2S_SIGNALS)
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
#[doc = "Values that can be written to the field `WSINSEL`"]
pub enum WSINSELW {
    #[doc = "Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS,
    #[doc = "WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS,
    #[doc = "WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS,
}
impl WSINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WSINSELW::ORIG_FLEX_I2S_SIGNALS => 0,
            WSINSELW::SHARED_SET0_I2S_SIGNALS => 1,
            WSINSELW::SHARED_SET1_I2S_SIGNALS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WSINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSINSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    #[inline]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(WSINSELW::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(WSINSELW::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(WSINSELW::SHARED_SET1_I2S_SIGNALS)
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
#[doc = "Values that can be written to the field `DATAINSEL`"]
pub enum DATAINSELW {
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS,
    #[doc = "Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS,
    #[doc = "Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS,
}
impl DATAINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATAINSELW::ORIG_FLEX_I2S_SIGNALS => 0,
            DATAINSELW::SHARED_SET0_I2S_SIGNALS => 1,
            DATAINSELW::SHARED_SET1_I2S_SIGNALS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATAINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATAINSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    #[inline]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(DATAINSELW::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(DATAINSELW::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(DATAINSELW::SHARED_SET1_I2S_SIGNALS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATAOUTSEL`"]
pub enum DATAOUTSELW {
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS,
    #[doc = "Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS,
    #[doc = "Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS,
}
impl DATAOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATAOUTSELW::ORIG_FLEX_I2S_SIGNALS => 0,
            DATAOUTSELW::SHARED_SET0_I2S_SIGNALS => 1,
            DATAOUTSELW::SHARED_SET1_I2S_SIGNALS => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATAOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATAOUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    #[inline]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(DATAOUTSELW::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(DATAOUTSELW::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(DATAOUTSELW::SHARED_SET1_I2S_SIGNALS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - Selects the source for SCK going into this Flexcomm."]
    #[inline]
    pub fn sckinsel(&self) -> SCKINSELR {
        SCKINSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Selects the source for WS going into this Flexcomm."]
    #[inline]
    pub fn wsinsel(&self) -> WSINSELR {
        WSINSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Selects the source for DATA input to this Flexcomm."]
    #[inline]
    pub fn datainsel(&self) -> DATAINSELR {
        DATAINSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Selects the source for DATA output from this Flexcomm."]
    #[inline]
    pub fn dataoutsel(&self) -> DATAOUTSELR {
        DATAOUTSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - Selects the source for SCK going into this Flexcomm."]
    #[inline]
    pub fn sckinsel(&mut self) -> _SCKINSELW {
        _SCKINSELW { w: self }
    }
    #[doc = "Bits 8:9 - Selects the source for WS going into this Flexcomm."]
    #[inline]
    pub fn wsinsel(&mut self) -> _WSINSELW {
        _WSINSELW { w: self }
    }
    #[doc = "Bits 16:17 - Selects the source for DATA input to this Flexcomm."]
    #[inline]
    pub fn datainsel(&mut self) -> _DATAINSELW {
        _DATAINSELW { w: self }
    }
    #[doc = "Bits 24:25 - Selects the source for DATA output from this Flexcomm."]
    #[inline]
    pub fn dataoutsel(&mut self) -> _DATAOUTSELW {
        _DATAOUTSELW { w: self }
    }
}
