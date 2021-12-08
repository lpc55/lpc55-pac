#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
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
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABBPAIR_A {
    #[doc = "0: Bank-pair 0 (1st)"]
    PAIR0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    PAIR1 = 1,
}
impl From<ABBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: ABBPAIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABBPAIR` reader - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
pub struct ABBPAIR_R(crate::FieldReader<bool, ABBPAIR_A>);
impl ABBPAIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABBPAIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABBPAIR_A {
        match self.bits {
            false => ABBPAIR_A::PAIR0,
            true => ABBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        **self == ABBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        **self == ABBPAIR_A::PAIR1
    }
}
impl core::ops::Deref for ABBPAIR_R {
    type Target = crate::FieldReader<bool, ABBPAIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABBPAIR` writer - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
pub struct ABBPAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABBPAIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABBPAIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(ABBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(ABBPAIR_A::PAIR1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ABOFF` reader - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
pub struct ABOFF_R(crate::FieldReader<bool, bool>);
impl ABOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABOFF` writer - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
pub struct ABOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ABOFF_W<'a> {
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
#[doc = "Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDBPAIR_A {
    #[doc = "0: Bank-pair 0 (1st)"]
    PAIR0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    PAIR1 = 1,
}
impl From<CDBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: CDBPAIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDBPAIR` reader - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
pub struct CDBPAIR_R(crate::FieldReader<bool, CDBPAIR_A>);
impl CDBPAIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDBPAIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDBPAIR_A {
        match self.bits {
            false => CDBPAIR_A::PAIR0,
            true => CDBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        **self == CDBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        **self == CDBPAIR_A::PAIR1
    }
}
impl core::ops::Deref for CDBPAIR_R {
    type Target = crate::FieldReader<bool, CDBPAIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDBPAIR` writer - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
pub struct CDBPAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CDBPAIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDBPAIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(CDBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(CDBPAIR_A::PAIR1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CDOFF` reader - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
pub struct CDOFF_R(crate::FieldReader<u16, u16>);
impl CDOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CDOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDOFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDOFF` writer - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
pub struct CDOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CDOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | ((value as u32 & 0x07ff) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn abbpair(&self) -> ABBPAIR_R {
        ABBPAIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline(always)]
    pub fn aboff(&self) -> ABOFF_R {
        ABOFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn cdbpair(&self) -> CDBPAIR_R {
        CDBPAIR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline(always)]
    pub fn cdoff(&self) -> CDOFF_R {
        CDOFF_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn abbpair(&mut self) -> ABBPAIR_W {
        ABBPAIR_W { w: self }
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline(always)]
    pub fn aboff(&mut self) -> ABOFF_W {
        ABOFF_W { w: self }
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn cdbpair(&mut self) -> CDBPAIR_W {
        CDBPAIR_W { w: self }
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline(always)]
    pub fn cdoff(&mut self) -> CDOFF_W {
        CDOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the offsets of AB and CD in the RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
