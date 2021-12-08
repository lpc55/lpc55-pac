#[doc = "Register `TX_CLR` reader"]
pub struct R(crate::R<TX_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CLR` writer"]
pub struct W(crate::W<TX_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CLR_SPEC>;
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
impl From<crate::W<TX_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Decode to trim the nominal 17\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum D_CAL_A {
    #[doc = "0: Maximum current, approximately 19% above nominal."]
    VALUE0 = 0,
    #[doc = "7: Nominal"]
    VALUE7 = 7,
    #[doc = "15: Minimum current, approximately 19% below nominal."]
    VALUE15 = 15,
}
impl From<D_CAL_A> for u8 {
    #[inline(always)]
    fn from(variant: D_CAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `D_CAL` reader - Decode to trim the nominal 17"]
pub struct D_CAL_R(crate::FieldReader<u8, D_CAL_A>);
impl D_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        D_CAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<D_CAL_A> {
        match self.bits {
            0 => Some(D_CAL_A::VALUE0),
            7 => Some(D_CAL_A::VALUE7),
            15 => Some(D_CAL_A::VALUE15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == D_CAL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == D_CAL_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        **self == D_CAL_A::VALUE15
    }
}
impl core::ops::Deref for D_CAL_R {
    type Target = crate::FieldReader<u8, D_CAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_CAL` writer - Decode to trim the nominal 17"]
pub struct D_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> D_CAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: D_CAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Maximum current, approximately 19% above nominal."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(D_CAL_A::VALUE0)
    }
    #[doc = "Nominal"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(D_CAL_A::VALUE7)
    }
    #[doc = "Minimum current, approximately 19% below nominal."]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(D_CAL_A::VALUE15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TXCAL45DM` reader - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
pub struct TXCAL45DM_R(crate::FieldReader<u8, u8>);
impl TXCAL45DM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXCAL45DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCAL45DM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCAL45DM` writer - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
pub struct TXCAL45DM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCAL45DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TXENCAL45DN` reader - Enable resistance calibration on DN."]
pub struct TXENCAL45DN_R(crate::FieldReader<bool, bool>);
impl TXENCAL45DN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXENCAL45DN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXENCAL45DN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXENCAL45DN` writer - Enable resistance calibration on DN."]
pub struct TXENCAL45DN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENCAL45DN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TXCAL45DP` reader - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
pub struct TXCAL45DP_R(crate::FieldReader<u8, u8>);
impl TXCAL45DP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXCAL45DP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCAL45DP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCAL45DP` writer - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
pub struct TXCAL45DP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCAL45DP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TXENCAL45DP` reader - Enable resistance calibration on DP."]
pub struct TXENCAL45DP_R(crate::FieldReader<bool, bool>);
impl TXENCAL45DP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXENCAL45DP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXENCAL45DP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXENCAL45DP` writer - Enable resistance calibration on DP."]
pub struct TXENCAL45DP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENCAL45DP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    pub fn d_cal(&self) -> D_CAL_R {
        D_CAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub fn txcal45dm(&self) -> TXCAL45DM_R {
        TXCAL45DM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    pub fn txencal45dn(&self) -> TXENCAL45DN_R {
        TXENCAL45DN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&self) -> TXCAL45DP_R {
        TXCAL45DP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline(always)]
    pub fn txencal45dp(&self) -> TXENCAL45DP_R {
        TXENCAL45DP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    pub fn d_cal(&mut self) -> D_CAL_W {
        D_CAL_W { w: self }
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub fn txcal45dm(&mut self) -> TXCAL45DM_W {
        TXCAL45DM_W { w: self }
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    pub fn txencal45dn(&mut self) -> TXENCAL45DN_W {
        TXENCAL45DN_W { w: self }
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&mut self) -> TXCAL45DP_W {
        TXCAL45DP_W { w: self }
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline(always)]
    pub fn txencal45dp(&mut self) -> TXENCAL45DP_W {
        TXENCAL45DP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Transmitter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_clr](index.html) module"]
pub struct TX_CLR_SPEC;
impl crate::RegisterSpec for TX_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_clr::R](R) reader structure"]
impl crate::Readable for TX_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_clr::W](W) writer structure"]
impl crate::Writable for TX_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CLR to value 0x0a00_0402"]
impl crate::Resettable for TX_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a00_0402
    }
}
