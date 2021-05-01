#[doc = "Register `TSTAT` reader"]
pub struct R(crate::R<TSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TSTAT_SPEC>> for R {
    fn from(reader: crate::R<TSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSTAT` writer"]
pub struct W(crate::W<TSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSTAT_SPEC>;
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
impl core::convert::From<crate::W<TSTAT_SPEC>> for W {
    fn from(writer: crate::W<TSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger Exception Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TEXC_NUM_A {
    #[doc = "0: No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\]
= 1."]
    TEXC_NUM_0 = 0,
    #[doc = "1: Trigger 0 has been interrupted by a high priority exception."]
    TEXC_NUM_1 = 1,
    #[doc = "2: Trigger 1 has been interrupted by a high priority exception."]
    TEXC_NUM_2 = 2,
    #[doc = "3: Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_3 = 3,
    #[doc = "4: Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_4 = 4,
    #[doc = "5: Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_5 = 5,
    #[doc = "6: Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_6 = 6,
    #[doc = "7: Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_7 = 7,
    #[doc = "8: Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_8 = 8,
    #[doc = "9: Associated trigger sequence has interrupted by a high priority exception."]
    TEXC_NUM_9 = 9,
    #[doc = "65535: Every trigger sequence has been interrupted by a high priority exception."]
    TEXC_NUM_65535 = 65535,
}
impl From<TEXC_NUM_A> for u16 {
    #[inline(always)]
    fn from(variant: TEXC_NUM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TEXC_NUM` reader - Trigger Exception Number"]
pub struct TEXC_NUM_R(crate::FieldReader<u16, TEXC_NUM_A>);
impl TEXC_NUM_R {
    pub(crate) fn new(bits: u16) -> Self {
        TEXC_NUM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TEXC_NUM_A> {
        match self.bits {
            0 => Some(TEXC_NUM_A::TEXC_NUM_0),
            1 => Some(TEXC_NUM_A::TEXC_NUM_1),
            2 => Some(TEXC_NUM_A::TEXC_NUM_2),
            3 => Some(TEXC_NUM_A::TEXC_NUM_3),
            4 => Some(TEXC_NUM_A::TEXC_NUM_4),
            5 => Some(TEXC_NUM_A::TEXC_NUM_5),
            6 => Some(TEXC_NUM_A::TEXC_NUM_6),
            7 => Some(TEXC_NUM_A::TEXC_NUM_7),
            8 => Some(TEXC_NUM_A::TEXC_NUM_8),
            9 => Some(TEXC_NUM_A::TEXC_NUM_9),
            65535 => Some(TEXC_NUM_A::TEXC_NUM_65535),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_0`"]
    #[inline(always)]
    pub fn is_texc_num_0(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_0
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_1`"]
    #[inline(always)]
    pub fn is_texc_num_1(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_1
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_2`"]
    #[inline(always)]
    pub fn is_texc_num_2(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_2
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_3`"]
    #[inline(always)]
    pub fn is_texc_num_3(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_3
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_4`"]
    #[inline(always)]
    pub fn is_texc_num_4(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_4
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_5`"]
    #[inline(always)]
    pub fn is_texc_num_5(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_5
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_6`"]
    #[inline(always)]
    pub fn is_texc_num_6(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_6
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_7`"]
    #[inline(always)]
    pub fn is_texc_num_7(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_7
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_8`"]
    #[inline(always)]
    pub fn is_texc_num_8(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_8
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_9`"]
    #[inline(always)]
    pub fn is_texc_num_9(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_9
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_65535`"]
    #[inline(always)]
    pub fn is_texc_num_65535(&self) -> bool {
        **self == TEXC_NUM_A::TEXC_NUM_65535
    }
}
impl core::ops::Deref for TEXC_NUM_R {
    type Target = crate::FieldReader<u16, TEXC_NUM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEXC_NUM` writer - Trigger Exception Number"]
pub struct TEXC_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TEXC_NUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEXC_NUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No triggers have been interrupted by a high priority exception. Or CFG\\[TRES\\]
= 1."]
    #[inline(always)]
    pub fn texc_num_0(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_0)
    }
    #[doc = "Trigger 0 has been interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_1(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_1)
    }
    #[doc = "Trigger 1 has been interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_2(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_2)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_3(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_3)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_4(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_4)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_5(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_5)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_6(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_6)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_7(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_7)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_8(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_8)
    }
    #[doc = "Associated trigger sequence has interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_9(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_9)
    }
    #[doc = "Every trigger sequence has been interrupted by a high priority exception."]
    #[inline(always)]
    pub fn texc_num_65535(self) -> &'a mut W {
        self.variant(TEXC_NUM_A::TEXC_NUM_65535)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Trigger Completion Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TCOMP_FLAG_A {
    #[doc = "0: No triggers have been completed. Trigger completion interrupts are disabled."]
    TCOMP_FLAG_0 = 0,
    #[doc = "1: Trigger 0 has been completed and triger 0 has enabled completion interrupts."]
    TCOMP_FLAG_1 = 1,
    #[doc = "2: Trigger 1 has been completed and triger 1 has enabled completion interrupts."]
    TCOMP_FLAG_2 = 2,
    #[doc = "3: Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_3 = 3,
    #[doc = "4: Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_4 = 4,
    #[doc = "5: Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_5 = 5,
    #[doc = "6: Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_6 = 6,
    #[doc = "7: Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_7 = 7,
    #[doc = "8: Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_8 = 8,
    #[doc = "9: Associated trigger sequence has completed and has enabled completion interrupts."]
    TCOMP_FLAG_9 = 9,
    #[doc = "65535: Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    TCOMP_FLAG_65535 = 65535,
}
impl From<TCOMP_FLAG_A> for u16 {
    #[inline(always)]
    fn from(variant: TCOMP_FLAG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCOMP_FLAG` reader - Trigger Completion Flag"]
pub struct TCOMP_FLAG_R(crate::FieldReader<u16, TCOMP_FLAG_A>);
impl TCOMP_FLAG_R {
    pub(crate) fn new(bits: u16) -> Self {
        TCOMP_FLAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCOMP_FLAG_A> {
        match self.bits {
            0 => Some(TCOMP_FLAG_A::TCOMP_FLAG_0),
            1 => Some(TCOMP_FLAG_A::TCOMP_FLAG_1),
            2 => Some(TCOMP_FLAG_A::TCOMP_FLAG_2),
            3 => Some(TCOMP_FLAG_A::TCOMP_FLAG_3),
            4 => Some(TCOMP_FLAG_A::TCOMP_FLAG_4),
            5 => Some(TCOMP_FLAG_A::TCOMP_FLAG_5),
            6 => Some(TCOMP_FLAG_A::TCOMP_FLAG_6),
            7 => Some(TCOMP_FLAG_A::TCOMP_FLAG_7),
            8 => Some(TCOMP_FLAG_A::TCOMP_FLAG_8),
            9 => Some(TCOMP_FLAG_A::TCOMP_FLAG_9),
            65535 => Some(TCOMP_FLAG_A::TCOMP_FLAG_65535),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_0`"]
    #[inline(always)]
    pub fn is_tcomp_flag_0(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_0
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_1`"]
    #[inline(always)]
    pub fn is_tcomp_flag_1(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_1
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_2`"]
    #[inline(always)]
    pub fn is_tcomp_flag_2(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_2
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_3`"]
    #[inline(always)]
    pub fn is_tcomp_flag_3(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_3
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_4`"]
    #[inline(always)]
    pub fn is_tcomp_flag_4(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_4
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_5`"]
    #[inline(always)]
    pub fn is_tcomp_flag_5(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_5
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_6`"]
    #[inline(always)]
    pub fn is_tcomp_flag_6(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_6
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_7`"]
    #[inline(always)]
    pub fn is_tcomp_flag_7(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_7
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_8`"]
    #[inline(always)]
    pub fn is_tcomp_flag_8(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_8
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_9`"]
    #[inline(always)]
    pub fn is_tcomp_flag_9(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_9
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_65535`"]
    #[inline(always)]
    pub fn is_tcomp_flag_65535(&self) -> bool {
        **self == TCOMP_FLAG_A::TCOMP_FLAG_65535
    }
}
impl core::ops::Deref for TCOMP_FLAG_R {
    type Target = crate::FieldReader<u16, TCOMP_FLAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCOMP_FLAG` writer - Trigger Completion Flag"]
pub struct TCOMP_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TCOMP_FLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCOMP_FLAG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    #[inline(always)]
    pub fn tcomp_flag_0(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_0)
    }
    #[doc = "Trigger 0 has been completed and triger 0 has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_1(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_1)
    }
    #[doc = "Trigger 1 has been completed and triger 1 has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_2(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_2)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_3(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_3)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_4(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_4)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_5(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_5)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_6(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_6)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_7(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_7)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_8(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_8)
    }
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_9(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_9)
    }
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    #[inline(always)]
    pub fn tcomp_flag_65535(self) -> &'a mut W {
        self.variant(TCOMP_FLAG_A::TCOMP_FLAG_65535)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Trigger Exception Number"]
    #[inline(always)]
    pub fn texc_num(&self) -> TEXC_NUM_R {
        TEXC_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Trigger Completion Flag"]
    #[inline(always)]
    pub fn tcomp_flag(&self) -> TCOMP_FLAG_R {
        TCOMP_FLAG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Trigger Exception Number"]
    #[inline(always)]
    pub fn texc_num(&mut self) -> TEXC_NUM_W {
        TEXC_NUM_W { w: self }
    }
    #[doc = "Bits 16:31 - Trigger Completion Flag"]
    #[inline(always)]
    pub fn tcomp_flag(&mut self) -> TCOMP_FLAG_W {
        TCOMP_FLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstat](index.html) module"]
pub struct TSTAT_SPEC;
impl crate::RegisterSpec for TSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tstat::R](R) reader structure"]
impl crate::Readable for TSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tstat::W](W) writer structure"]
impl crate::Writable for TSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSTAT to value 0"]
impl crate::Resettable for TSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
