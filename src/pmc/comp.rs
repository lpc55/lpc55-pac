#[doc = "Register `COMP` reader"]
pub struct R(crate::R<COMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP` writer"]
pub struct W(crate::W<COMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_SPEC>;
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
impl From<crate::W<COMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Hysteris when hyst = '1'.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST_A {
    #[doc = "0: Hysteresis is disable."]
    DISABLE = 0,
    #[doc = "1: Hysteresis is enable."]
    ENABLE = 1,
}
impl From<HYST_A> for bool {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYST` reader - Hysteris when hyst = '1'."]
pub struct HYST_R(crate::FieldReader<bool, HYST_A>);
impl HYST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            false => HYST_A::DISABLE,
            true => HYST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == HYST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == HYST_A::ENABLE
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<bool, HYST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST` writer - Hysteris when hyst = '1'."]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Hysteresis is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYST_A::DISABLE)
    }
    #[doc = "Hysteresis is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYST_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFINPUT_A {
    #[doc = "0: Select internal VREF."]
    INTERNALREF = 0,
    #[doc = "1: Select VDDA."]
    VDDA = 1,
}
impl From<VREFINPUT_A> for bool {
    #[inline(always)]
    fn from(variant: VREFINPUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFINPUT` reader - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
pub struct VREFINPUT_R(crate::FieldReader<bool, VREFINPUT_A>);
impl VREFINPUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREFINPUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFINPUT_A {
        match self.bits {
            false => VREFINPUT_A::INTERNALREF,
            true => VREFINPUT_A::VDDA,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNALREF`"]
    #[inline(always)]
    pub fn is_internalref(&self) -> bool {
        **self == VREFINPUT_A::INTERNALREF
    }
    #[doc = "Checks if the value of the field is `VDDA`"]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        **self == VREFINPUT_A::VDDA
    }
}
impl core::ops::Deref for VREFINPUT_R {
    type Target = crate::FieldReader<bool, VREFINPUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFINPUT` writer - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
pub struct VREFINPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFINPUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFINPUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select internal VREF."]
    #[inline(always)]
    pub fn internalref(self) -> &'a mut W {
        self.variant(VREFINPUT_A::INTERNALREF)
    }
    #[doc = "Select VDDA."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut W {
        self.variant(VREFINPUT_A::VDDA)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Low power mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWPOWER_A {
    #[doc = "0: High speed mode."]
    HIGHSPEED = 0,
    #[doc = "1: Low power mode (Low speed)."]
    LOWSPEED = 1,
}
impl From<LOWPOWER_A> for bool {
    #[inline(always)]
    fn from(variant: LOWPOWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWPOWER` reader - Low power mode."]
pub struct LOWPOWER_R(crate::FieldReader<bool, LOWPOWER_A>);
impl LOWPOWER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOWPOWER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOWPOWER_A {
        match self.bits {
            false => LOWPOWER_A::HIGHSPEED,
            true => LOWPOWER_A::LOWSPEED,
        }
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_highspeed(&self) -> bool {
        **self == LOWPOWER_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_lowspeed(&self) -> bool {
        **self == LOWPOWER_A::LOWSPEED
    }
}
impl core::ops::Deref for LOWPOWER_R {
    type Target = crate::FieldReader<bool, LOWPOWER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWPOWER` writer - Low power mode."]
pub struct LOWPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWPOWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOWPOWER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High speed mode."]
    #[inline(always)]
    pub fn highspeed(self) -> &'a mut W {
        self.variant(LOWPOWER_A::HIGHSPEED)
    }
    #[doc = "Low power mode (Low speed)."]
    #[inline(always)]
    pub fn lowspeed(self) -> &'a mut W {
        self.variant(LOWPOWER_A::LOWSPEED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Control word for P multiplexer:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMUX_A {
    #[doc = "0: VREF (See fiedl VREFINPUT)."]
    VREF = 0,
    #[doc = "1: Pin P0_0."]
    CMP0_A = 1,
    #[doc = "2: Pin P0_9."]
    CMP0_B = 2,
    #[doc = "3: Pin P0_18."]
    CMP0_C = 3,
    #[doc = "4: Pin P1_14."]
    CMP0_D = 4,
    #[doc = "5: Pin P2_23."]
    CMP0_E = 5,
}
impl From<PMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: PMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PMUX` reader - Control word for P multiplexer:."]
pub struct PMUX_R(crate::FieldReader<u8, PMUX_A>);
impl PMUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PMUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMUX_A> {
        match self.bits {
            0 => Some(PMUX_A::VREF),
            1 => Some(PMUX_A::CMP0_A),
            2 => Some(PMUX_A::CMP0_B),
            3 => Some(PMUX_A::CMP0_C),
            4 => Some(PMUX_A::CMP0_D),
            5 => Some(PMUX_A::CMP0_E),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        **self == PMUX_A::VREF
    }
    #[doc = "Checks if the value of the field is `CMP0_A`"]
    #[inline(always)]
    pub fn is_cmp0_a(&self) -> bool {
        **self == PMUX_A::CMP0_A
    }
    #[doc = "Checks if the value of the field is `CMP0_B`"]
    #[inline(always)]
    pub fn is_cmp0_b(&self) -> bool {
        **self == PMUX_A::CMP0_B
    }
    #[doc = "Checks if the value of the field is `CMP0_C`"]
    #[inline(always)]
    pub fn is_cmp0_c(&self) -> bool {
        **self == PMUX_A::CMP0_C
    }
    #[doc = "Checks if the value of the field is `CMP0_D`"]
    #[inline(always)]
    pub fn is_cmp0_d(&self) -> bool {
        **self == PMUX_A::CMP0_D
    }
    #[doc = "Checks if the value of the field is `CMP0_E`"]
    #[inline(always)]
    pub fn is_cmp0_e(&self) -> bool {
        **self == PMUX_A::CMP0_E
    }
}
impl core::ops::Deref for PMUX_R {
    type Target = crate::FieldReader<u8, PMUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMUX` writer - Control word for P multiplexer:."]
pub struct PMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMUX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VREF (See fiedl VREFINPUT)."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(PMUX_A::VREF)
    }
    #[doc = "Pin P0_0."]
    #[inline(always)]
    pub fn cmp0_a(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_A)
    }
    #[doc = "Pin P0_9."]
    #[inline(always)]
    pub fn cmp0_b(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_B)
    }
    #[doc = "Pin P0_18."]
    #[inline(always)]
    pub fn cmp0_c(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_C)
    }
    #[doc = "Pin P1_14."]
    #[inline(always)]
    pub fn cmp0_d(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_D)
    }
    #[doc = "Pin P2_23."]
    #[inline(always)]
    pub fn cmp0_e(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_E)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Control word for N multiplexer:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NMUX_A {
    #[doc = "0: VREF (See field VREFINPUT)."]
    VREF = 0,
    #[doc = "1: Pin P0_0."]
    CMP0_A = 1,
    #[doc = "2: Pin P0_9."]
    CMP0_B = 2,
    #[doc = "3: Pin P0_18."]
    CMP0_C = 3,
    #[doc = "4: Pin P1_14."]
    CMP0_D = 4,
    #[doc = "5: Pin P2_23."]
    CMP0_E = 5,
}
impl From<NMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: NMUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NMUX` reader - Control word for N multiplexer:."]
pub struct NMUX_R(crate::FieldReader<u8, NMUX_A>);
impl NMUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NMUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NMUX_A> {
        match self.bits {
            0 => Some(NMUX_A::VREF),
            1 => Some(NMUX_A::CMP0_A),
            2 => Some(NMUX_A::CMP0_B),
            3 => Some(NMUX_A::CMP0_C),
            4 => Some(NMUX_A::CMP0_D),
            5 => Some(NMUX_A::CMP0_E),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        **self == NMUX_A::VREF
    }
    #[doc = "Checks if the value of the field is `CMP0_A`"]
    #[inline(always)]
    pub fn is_cmp0_a(&self) -> bool {
        **self == NMUX_A::CMP0_A
    }
    #[doc = "Checks if the value of the field is `CMP0_B`"]
    #[inline(always)]
    pub fn is_cmp0_b(&self) -> bool {
        **self == NMUX_A::CMP0_B
    }
    #[doc = "Checks if the value of the field is `CMP0_C`"]
    #[inline(always)]
    pub fn is_cmp0_c(&self) -> bool {
        **self == NMUX_A::CMP0_C
    }
    #[doc = "Checks if the value of the field is `CMP0_D`"]
    #[inline(always)]
    pub fn is_cmp0_d(&self) -> bool {
        **self == NMUX_A::CMP0_D
    }
    #[doc = "Checks if the value of the field is `CMP0_E`"]
    #[inline(always)]
    pub fn is_cmp0_e(&self) -> bool {
        **self == NMUX_A::CMP0_E
    }
}
impl core::ops::Deref for NMUX_R {
    type Target = crate::FieldReader<u8, NMUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMUX` writer - Control word for N multiplexer:."]
pub struct NMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> NMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMUX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VREF (See field VREFINPUT)."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(NMUX_A::VREF)
    }
    #[doc = "Pin P0_0."]
    #[inline(always)]
    pub fn cmp0_a(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_A)
    }
    #[doc = "Pin P0_9."]
    #[inline(always)]
    pub fn cmp0_b(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_B)
    }
    #[doc = "Pin P0_18."]
    #[inline(always)]
    pub fn cmp0_c(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_C)
    }
    #[doc = "Pin P1_14."]
    #[inline(always)]
    pub fn cmp0_d(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_D)
    }
    #[doc = "Pin P2_23."]
    #[inline(always)]
    pub fn cmp0_e(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_E)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
#[doc = "Field `VREF` reader - Control reference voltage step, per steps of (VREFINPUT/31)."]
pub struct VREF_R(crate::FieldReader<u8, u8>);
impl VREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF` writer - Control reference voltage step, per steps of (VREFINPUT/31)."]
pub struct VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `FILTERCGF_SAMPLEMODE` reader - Filter Sample mode."]
pub struct FILTERCGF_SAMPLEMODE_R(crate::FieldReader<u8, u8>);
impl FILTERCGF_SAMPLEMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTERCGF_SAMPLEMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTERCGF_SAMPLEMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTERCGF_SAMPLEMODE` writer - Filter Sample mode."]
pub struct FILTERCGF_SAMPLEMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERCGF_SAMPLEMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FILTERCGF_CLKDIV` reader - Filter Clock div ."]
pub struct FILTERCGF_CLKDIV_R(crate::FieldReader<u8, u8>);
impl FILTERCGF_CLKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILTERCGF_CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTERCGF_CLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTERCGF_CLKDIV` writer - Filter Clock div ."]
pub struct FILTERCGF_CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTERCGF_CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Hysteris when hyst = '1'."]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline(always)]
    pub fn vrefinput(&self) -> VREFINPUT_R {
        VREFINPUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Low power mode."]
    #[inline(always)]
    pub fn lowpower(&self) -> LOWPOWER_R {
        LOWPOWER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Control word for P multiplexer:."]
    #[inline(always)]
    pub fn pmux(&self) -> PMUX_R {
        PMUX_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:9 - Control word for N multiplexer:."]
    #[inline(always)]
    pub fn nmux(&self) -> NMUX_R {
        NMUX_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 10:14 - Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline(always)]
    pub fn vref(&self) -> VREF_R {
        VREF_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Filter Sample mode."]
    #[inline(always)]
    pub fn filtercgf_samplemode(&self) -> FILTERCGF_SAMPLEMODE_R {
        FILTERCGF_SAMPLEMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Filter Clock div ."]
    #[inline(always)]
    pub fn filtercgf_clkdiv(&self) -> FILTERCGF_CLKDIV_R {
        FILTERCGF_CLKDIV_R::new(((self.bits >> 18) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Hysteris when hyst = '1'."]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bit 2 - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline(always)]
    pub fn vrefinput(&mut self) -> VREFINPUT_W {
        VREFINPUT_W { w: self }
    }
    #[doc = "Bit 3 - Low power mode."]
    #[inline(always)]
    pub fn lowpower(&mut self) -> LOWPOWER_W {
        LOWPOWER_W { w: self }
    }
    #[doc = "Bits 4:6 - Control word for P multiplexer:."]
    #[inline(always)]
    pub fn pmux(&mut self) -> PMUX_W {
        PMUX_W { w: self }
    }
    #[doc = "Bits 7:9 - Control word for N multiplexer:."]
    #[inline(always)]
    pub fn nmux(&mut self) -> NMUX_W {
        NMUX_W { w: self }
    }
    #[doc = "Bits 10:14 - Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline(always)]
    pub fn vref(&mut self) -> VREF_W {
        VREF_W { w: self }
    }
    #[doc = "Bits 16:17 - Filter Sample mode."]
    #[inline(always)]
    pub fn filtercgf_samplemode(&mut self) -> FILTERCGF_SAMPLEMODE_W {
        FILTERCGF_SAMPLEMODE_W { w: self }
    }
    #[doc = "Bits 18:20 - Filter Clock div ."]
    #[inline(always)]
    pub fn filtercgf_clkdiv(&mut self) -> FILTERCGF_CLKDIV_W {
        FILTERCGF_CLKDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp](index.html) module"]
pub struct COMP_SPEC;
impl crate::RegisterSpec for COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp::R](R) reader structure"]
impl crate::Readable for COMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp::W](W) writer structure"]
impl crate::Writable for COMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP to value 0x0a"]
impl crate::Resettable for COMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
