#[doc = "Register `FCCTRLSEL%s` reader"]
pub struct R(crate::R<FCCTRLSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCTRLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FCCTRLSEL_SPEC>> for R {
    fn from(reader: crate::R<FCCTRLSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCTRLSEL%s` writer"]
pub struct W(crate::W<FCCTRLSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCTRLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<FCCTRLSEL_SPEC>> for W {
    fn from(writer: crate::W<FCCTRLSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the source for SCK going into this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCKINSEL_A {
    #[doc = "0: Selects the dedicated FCn_SCK function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0,
    #[doc = "1: SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 1,
    #[doc = "2: SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 2,
}
impl From<SCKINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKINSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SCKINSEL` reader - Selects the source for SCK going into this Flexcomm."]
pub struct SCKINSEL_R(crate::FieldReader<u8, SCKINSEL_A>);
impl SCKINSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCKINSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCKINSEL_A> {
        match self.bits {
            0 => Some(SCKINSEL_A::ORIG_FLEX_I2S_SIGNALS),
            1 => Some(SCKINSEL_A::SHARED_SET0_I2S_SIGNALS),
            2 => Some(SCKINSEL_A::SHARED_SET1_I2S_SIGNALS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        **self == SCKINSEL_A::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        **self == SCKINSEL_A::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        **self == SCKINSEL_A::SHARED_SET1_I2S_SIGNALS
    }
}
impl core::ops::Deref for SCKINSEL_R {
    type Target = crate::FieldReader<u8, SCKINSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCKINSEL` writer - Selects the source for SCK going into this Flexcomm."]
pub struct SCKINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCKINSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects the dedicated FCn_SCK function for this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(SCKINSEL_A::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(SCKINSEL_A::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(SCKINSEL_A::SHARED_SET1_I2S_SIGNALS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Selects the source for WS going into this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WSINSEL_A {
    #[doc = "0: Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0,
    #[doc = "1: WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 1,
    #[doc = "2: WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 2,
}
impl From<WSINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WSINSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WSINSEL` reader - Selects the source for WS going into this Flexcomm."]
pub struct WSINSEL_R(crate::FieldReader<u8, WSINSEL_A>);
impl WSINSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WSINSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WSINSEL_A> {
        match self.bits {
            0 => Some(WSINSEL_A::ORIG_FLEX_I2S_SIGNALS),
            1 => Some(WSINSEL_A::SHARED_SET0_I2S_SIGNALS),
            2 => Some(WSINSEL_A::SHARED_SET1_I2S_SIGNALS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        **self == WSINSEL_A::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        **self == WSINSEL_A::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        **self == WSINSEL_A::SHARED_SET1_I2S_SIGNALS
    }
}
impl core::ops::Deref for WSINSEL_R {
    type Target = crate::FieldReader<u8, WSINSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WSINSEL` writer - Selects the source for WS going into this Flexcomm."]
pub struct WSINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WSINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSINSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(WSINSEL_A::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(WSINSEL_A::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(WSINSEL_A::SHARED_SET1_I2S_SIGNALS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Selects the source for DATA input to this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATAINSEL_A {
    #[doc = "0: Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0,
    #[doc = "1: Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 1,
    #[doc = "2: Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 2,
}
impl From<DATAINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAINSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATAINSEL` reader - Selects the source for DATA input to this Flexcomm."]
pub struct DATAINSEL_R(crate::FieldReader<u8, DATAINSEL_A>);
impl DATAINSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAINSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAINSEL_A> {
        match self.bits {
            0 => Some(DATAINSEL_A::ORIG_FLEX_I2S_SIGNALS),
            1 => Some(DATAINSEL_A::SHARED_SET0_I2S_SIGNALS),
            2 => Some(DATAINSEL_A::SHARED_SET1_I2S_SIGNALS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        **self == DATAINSEL_A::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        **self == DATAINSEL_A::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        **self == DATAINSEL_A::SHARED_SET1_I2S_SIGNALS
    }
}
impl core::ops::Deref for DATAINSEL_R {
    type Target = crate::FieldReader<u8, DATAINSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAINSEL` writer - Selects the source for DATA input to this Flexcomm."]
pub struct DATAINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAINSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(DATAINSEL_A::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(DATAINSEL_A::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(DATAINSEL_A::SHARED_SET1_I2S_SIGNALS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Selects the source for DATA output from this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATAOUTSEL_A {
    #[doc = "0: Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0,
    #[doc = "1: Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 1,
    #[doc = "2: Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 2,
}
impl From<DATAOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATAOUTSEL` reader - Selects the source for DATA output from this Flexcomm."]
pub struct DATAOUTSEL_R(crate::FieldReader<u8, DATAOUTSEL_A>);
impl DATAOUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAOUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAOUTSEL_A> {
        match self.bits {
            0 => Some(DATAOUTSEL_A::ORIG_FLEX_I2S_SIGNALS),
            1 => Some(DATAOUTSEL_A::SHARED_SET0_I2S_SIGNALS),
            2 => Some(DATAOUTSEL_A::SHARED_SET1_I2S_SIGNALS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        **self == DATAOUTSEL_A::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        **self == DATAOUTSEL_A::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        **self == DATAOUTSEL_A::SHARED_SET1_I2S_SIGNALS
    }
}
impl core::ops::Deref for DATAOUTSEL_R {
    type Target = crate::FieldReader<u8, DATAOUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAOUTSEL` writer - Selects the source for DATA output from this Flexcomm."]
pub struct DATAOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(DATAOUTSEL_A::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(DATAOUTSEL_A::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(DATAOUTSEL_A::SHARED_SET1_I2S_SIGNALS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects the source for SCK going into this Flexcomm."]
    #[inline(always)]
    pub fn sckinsel(&self) -> SCKINSEL_R {
        SCKINSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Selects the source for WS going into this Flexcomm."]
    #[inline(always)]
    pub fn wsinsel(&self) -> WSINSEL_R {
        WSINSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Selects the source for DATA input to this Flexcomm."]
    #[inline(always)]
    pub fn datainsel(&self) -> DATAINSEL_R {
        DATAINSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Selects the source for DATA output from this Flexcomm."]
    #[inline(always)]
    pub fn dataoutsel(&self) -> DATAOUTSEL_R {
        DATAOUTSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the source for SCK going into this Flexcomm."]
    #[inline(always)]
    pub fn sckinsel(&mut self) -> SCKINSEL_W {
        SCKINSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Selects the source for WS going into this Flexcomm."]
    #[inline(always)]
    pub fn wsinsel(&mut self) -> WSINSEL_W {
        WSINSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Selects the source for DATA input to this Flexcomm."]
    #[inline(always)]
    pub fn datainsel(&mut self) -> DATAINSEL_W {
        DATAINSEL_W { w: self }
    }
    #[doc = "Bits 24:25 - Selects the source for DATA output from this Flexcomm."]
    #[inline(always)]
    pub fn dataoutsel(&mut self) -> DATAOUTSEL_W {
        DATAOUTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects the source for SCK going into Flexcomm index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcctrlsel](index.html) module"]
pub struct FCCTRLSEL_SPEC;
impl crate::RegisterSpec for FCCTRLSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcctrlsel::R](R) reader structure"]
impl crate::Readable for FCCTRLSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcctrlsel::W](W) writer structure"]
impl crate::Writable for FCCTRLSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCCTRLSEL%s to value 0"]
impl crate::Resettable for FCCTRLSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
