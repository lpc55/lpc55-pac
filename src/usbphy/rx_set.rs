#[doc = "Register `RX_SET` reader"]
pub struct R(crate::R<RX_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_SET` writer"]
pub struct W(crate::W<RX_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_SET_SPEC>;
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
impl From<crate::W<RX_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The ENVADJ field adjusts the trip point for the envelope detector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENVADJ_A {
    #[doc = "0: Trip-Level Voltage is 0.1000 V"]
    VALUE0 = 0,
    #[doc = "1: Trip-Level Voltage is 0.1125 V"]
    VALUE1 = 1,
    #[doc = "2: Trip-Level Voltage is 0.1250 V"]
    VALUE2 = 2,
    #[doc = "3: Trip-Level Voltage is 0.0875 V"]
    VALUE3 = 3,
}
impl From<ENVADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: ENVADJ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ENVADJ` reader - The ENVADJ field adjusts the trip point for the envelope detector"]
pub struct ENVADJ_R(crate::FieldReader<u8, ENVADJ_A>);
impl ENVADJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ENVADJ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENVADJ_A> {
        match self.bits {
            0 => Some(ENVADJ_A::VALUE0),
            1 => Some(ENVADJ_A::VALUE1),
            2 => Some(ENVADJ_A::VALUE2),
            3 => Some(ENVADJ_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == ENVADJ_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ENVADJ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ENVADJ_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ENVADJ_A::VALUE3
    }
}
impl core::ops::Deref for ENVADJ_R {
    type Target = crate::FieldReader<u8, ENVADJ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENVADJ` writer - The ENVADJ field adjusts the trip point for the envelope detector"]
pub struct ENVADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENVADJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(ENVADJ_A::VALUE0)
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENVADJ_A::VALUE1)
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENVADJ_A::VALUE2)
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ENVADJ_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DISCONADJ_A {
    #[doc = "0: Trip-Level Voltage is 0.56875 V"]
    VALUE0 = 0,
    #[doc = "1: Trip-Level Voltage is 0.55000 V"]
    VALUE1 = 1,
    #[doc = "2: Trip-Level Voltage is 0.58125 V"]
    VALUE2 = 2,
    #[doc = "3: Trip-Level Voltage is 0.60000 V"]
    VALUE3 = 3,
}
impl From<DISCONADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: DISCONADJ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DISCONADJ` reader - The DISCONADJ field adjusts the trip point for the disconnect detector."]
pub struct DISCONADJ_R(crate::FieldReader<u8, DISCONADJ_A>);
impl DISCONADJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DISCONADJ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DISCONADJ_A> {
        match self.bits {
            0 => Some(DISCONADJ_A::VALUE0),
            1 => Some(DISCONADJ_A::VALUE1),
            2 => Some(DISCONADJ_A::VALUE2),
            3 => Some(DISCONADJ_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == DISCONADJ_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DISCONADJ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DISCONADJ_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == DISCONADJ_A::VALUE3
    }
}
impl core::ops::Deref for DISCONADJ_R {
    type Target = crate::FieldReader<u8, DISCONADJ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCONADJ` writer - The DISCONADJ field adjusts the trip point for the disconnect detector."]
pub struct DISCONADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCONADJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DISCONADJ_A::VALUE0)
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISCONADJ_A::VALUE1)
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DISCONADJ_A::VALUE2)
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DISCONADJ_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDBYPASS_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    VALUE1 = 1,
}
impl From<RXDBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: RXDBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDBYPASS` reader - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
pub struct RXDBYPASS_R(crate::FieldReader<bool, RXDBYPASS_A>);
impl RXDBYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDBYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDBYPASS_A {
        match self.bits {
            false => RXDBYPASS_A::VALUE0,
            true => RXDBYPASS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        **self == RXDBYPASS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == RXDBYPASS_A::VALUE1
    }
}
impl core::ops::Deref for RXDBYPASS_R {
    type Target = crate::FieldReader<bool, RXDBYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDBYPASS` writer - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
pub struct RXDBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDBYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::VALUE0)
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXDBYPASS_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&self) -> ENVADJ_R {
        ENVADJ_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&self) -> DISCONADJ_R {
        DISCONADJ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&self) -> RXDBYPASS_R {
        RXDBYPASS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub fn envadj(&mut self) -> ENVADJ_W {
        ENVADJ_W { w: self }
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub fn disconadj(&mut self) -> DISCONADJ_W {
        DISCONADJ_W { w: self }
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub fn rxdbypass(&mut self) -> RXDBYPASS_W {
        RXDBYPASS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Receiver Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_set](index.html) module"]
pub struct RX_SET_SPEC;
impl crate::RegisterSpec for RX_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_set::R](R) reader structure"]
impl crate::Readable for RX_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_set::W](W) writer structure"]
impl crate::Writable for RX_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_SET to value 0"]
impl crate::Resettable for RX_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
