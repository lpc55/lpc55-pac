#[doc = "Register `CMDL15` reader"]
pub struct R(crate::R<CMDL15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDL15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CMDL15_SPEC>> for R {
    fn from(reader: crate::R<CMDL15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDL15` writer"]
pub struct W(crate::W<CMDL15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDL15_SPEC>;
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
impl core::convert::From<crate::W<CMDL15_SPEC>> for W {
    fn from(writer: crate::W<CMDL15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCH_A {
    #[doc = "0: Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0,
    #[doc = "1: Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 1,
    #[doc = "2: Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 2,
    #[doc = "3: Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 3,
    #[doc = "4: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 4,
    #[doc = "5: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 5,
    #[doc = "6: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 6,
    #[doc = "7: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 7,
    #[doc = "8: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 8,
    #[doc = "9: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 9,
    #[doc = "30: Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 30,
    #[doc = "31: Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 31,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCH` reader - Input channel select"]
pub struct ADCH_R(crate::FieldReader<u8, ADCH_A>);
impl ADCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCH_A> {
        match self.bits {
            0 => Some(ADCH_A::ADCH_0),
            1 => Some(ADCH_A::ADCH_1),
            2 => Some(ADCH_A::ADCH_2),
            3 => Some(ADCH_A::ADCH_3),
            4 => Some(ADCH_A::ADCH_4),
            5 => Some(ADCH_A::ADCH_5),
            6 => Some(ADCH_A::ADCH_6),
            7 => Some(ADCH_A::ADCH_7),
            8 => Some(ADCH_A::ADCH_8),
            9 => Some(ADCH_A::ADCH_9),
            30 => Some(ADCH_A::ADCH_30),
            31 => Some(ADCH_A::ADCH_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADCH_0`"]
    #[inline(always)]
    pub fn is_adch_0(&self) -> bool {
        **self == ADCH_A::ADCH_0
    }
    #[doc = "Checks if the value of the field is `ADCH_1`"]
    #[inline(always)]
    pub fn is_adch_1(&self) -> bool {
        **self == ADCH_A::ADCH_1
    }
    #[doc = "Checks if the value of the field is `ADCH_2`"]
    #[inline(always)]
    pub fn is_adch_2(&self) -> bool {
        **self == ADCH_A::ADCH_2
    }
    #[doc = "Checks if the value of the field is `ADCH_3`"]
    #[inline(always)]
    pub fn is_adch_3(&self) -> bool {
        **self == ADCH_A::ADCH_3
    }
    #[doc = "Checks if the value of the field is `ADCH_4`"]
    #[inline(always)]
    pub fn is_adch_4(&self) -> bool {
        **self == ADCH_A::ADCH_4
    }
    #[doc = "Checks if the value of the field is `ADCH_5`"]
    #[inline(always)]
    pub fn is_adch_5(&self) -> bool {
        **self == ADCH_A::ADCH_5
    }
    #[doc = "Checks if the value of the field is `ADCH_6`"]
    #[inline(always)]
    pub fn is_adch_6(&self) -> bool {
        **self == ADCH_A::ADCH_6
    }
    #[doc = "Checks if the value of the field is `ADCH_7`"]
    #[inline(always)]
    pub fn is_adch_7(&self) -> bool {
        **self == ADCH_A::ADCH_7
    }
    #[doc = "Checks if the value of the field is `ADCH_8`"]
    #[inline(always)]
    pub fn is_adch_8(&self) -> bool {
        **self == ADCH_A::ADCH_8
    }
    #[doc = "Checks if the value of the field is `ADCH_9`"]
    #[inline(always)]
    pub fn is_adch_9(&self) -> bool {
        **self == ADCH_A::ADCH_9
    }
    #[doc = "Checks if the value of the field is `ADCH_30`"]
    #[inline(always)]
    pub fn is_adch_30(&self) -> bool {
        **self == ADCH_A::ADCH_30
    }
    #[doc = "Checks if the value of the field is `ADCH_31`"]
    #[inline(always)]
    pub fn is_adch_31(&self) -> bool {
        **self == ADCH_A::ADCH_31
    }
}
impl core::ops::Deref for ADCH_R {
    type Target = crate::FieldReader<u8, ADCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCH` writer - Input channel select"]
pub struct ADCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    #[inline(always)]
    pub fn adch_0(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_0)
    }
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    #[inline(always)]
    pub fn adch_1(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_1)
    }
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    #[inline(always)]
    pub fn adch_2(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_2)
    }
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    #[inline(always)]
    pub fn adch_3(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_3)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_4(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_4)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_5(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_5)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_6(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_6)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_7(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_7)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_8(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_8)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_9(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_9)
    }
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    #[inline(always)]
    pub fn adch_30(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_30)
    }
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    #[inline(always)]
    pub fn adch_31(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Conversion Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTYPE_A {
    #[doc = "0: Single-Ended Mode. Only A side channel is converted."]
    CTYPE_0 = 0,
    #[doc = "1: Single-Ended Mode. Only B side channel is converted."]
    CTYPE_1 = 1,
    #[doc = "2: Differential Mode. A-B."]
    CTYPE_2 = 2,
    #[doc = "3: Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    CTYPE_3 = 3,
}
impl From<CTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTYPE` reader - Conversion Type"]
pub struct CTYPE_R(crate::FieldReader<u8, CTYPE_A>);
impl CTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTYPE_A {
        match self.bits {
            0 => CTYPE_A::CTYPE_0,
            1 => CTYPE_A::CTYPE_1,
            2 => CTYPE_A::CTYPE_2,
            3 => CTYPE_A::CTYPE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTYPE_0`"]
    #[inline(always)]
    pub fn is_ctype_0(&self) -> bool {
        **self == CTYPE_A::CTYPE_0
    }
    #[doc = "Checks if the value of the field is `CTYPE_1`"]
    #[inline(always)]
    pub fn is_ctype_1(&self) -> bool {
        **self == CTYPE_A::CTYPE_1
    }
    #[doc = "Checks if the value of the field is `CTYPE_2`"]
    #[inline(always)]
    pub fn is_ctype_2(&self) -> bool {
        **self == CTYPE_A::CTYPE_2
    }
    #[doc = "Checks if the value of the field is `CTYPE_3`"]
    #[inline(always)]
    pub fn is_ctype_3(&self) -> bool {
        **self == CTYPE_A::CTYPE_3
    }
}
impl core::ops::Deref for CTYPE_R {
    type Target = crate::FieldReader<u8, CTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTYPE` writer - Conversion Type"]
pub struct CTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTYPE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    #[inline(always)]
    pub fn ctype_0(self) -> &'a mut W {
        self.variant(CTYPE_A::CTYPE_0)
    }
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    #[inline(always)]
    pub fn ctype_1(self) -> &'a mut W {
        self.variant(CTYPE_A::CTYPE_1)
    }
    #[doc = "Differential Mode. A-B."]
    #[inline(always)]
    pub fn ctype_2(self) -> &'a mut W {
        self.variant(CTYPE_A::CTYPE_2)
    }
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    #[inline(always)]
    pub fn ctype_3(self) -> &'a mut W {
        self.variant(CTYPE_A::CTYPE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Select resolution of conversions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    MODE_0 = 0,
    #[doc = "1: High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    MODE_1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Select resolution of conversions"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0,
            true => MODE_A::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        **self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        **self == MODE_A::MODE_1
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Select resolution of conversions"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Conversion Type"]
    #[inline(always)]
    pub fn ctype(&self) -> CTYPE_R {
        CTYPE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Select resolution of conversions"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&mut self) -> ADCH_W {
        ADCH_W { w: self }
    }
    #[doc = "Bits 5:6 - Conversion Type"]
    #[inline(always)]
    pub fn ctype(&mut self) -> CTYPE_W {
        CTYPE_W { w: self }
    }
    #[doc = "Bit 7 - Select resolution of conversions"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Command Low Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdl15](index.html) module"]
pub struct CMDL15_SPEC;
impl crate::RegisterSpec for CMDL15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdl15::R](R) reader structure"]
impl crate::Readable for CMDL15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdl15::W](W) writer structure"]
impl crate::Writable for CMDL15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDL15 to value 0"]
impl crate::Resettable for CMDL15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
