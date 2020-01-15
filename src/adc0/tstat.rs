#[doc = "Reader of register TSTAT"]
pub type R = crate::R<u32, super::TSTAT>;
#[doc = "Writer for register TSTAT"]
pub type W = crate::W<u32, super::TSTAT>;
#[doc = "Register TSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::TSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `TEXC_NUM`"]
pub type TEXC_NUM_R = crate::R<u16, TEXC_NUM_A>;
impl TEXC_NUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TEXC_NUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TEXC_NUM_A::TEXC_NUM_0),
            1 => Val(TEXC_NUM_A::TEXC_NUM_1),
            2 => Val(TEXC_NUM_A::TEXC_NUM_2),
            3 => Val(TEXC_NUM_A::TEXC_NUM_3),
            4 => Val(TEXC_NUM_A::TEXC_NUM_4),
            5 => Val(TEXC_NUM_A::TEXC_NUM_5),
            6 => Val(TEXC_NUM_A::TEXC_NUM_6),
            7 => Val(TEXC_NUM_A::TEXC_NUM_7),
            8 => Val(TEXC_NUM_A::TEXC_NUM_8),
            9 => Val(TEXC_NUM_A::TEXC_NUM_9),
            65535 => Val(TEXC_NUM_A::TEXC_NUM_65535),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_0`"]
    #[inline(always)]
    pub fn is_texc_num_0(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_0
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_1`"]
    #[inline(always)]
    pub fn is_texc_num_1(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_1
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_2`"]
    #[inline(always)]
    pub fn is_texc_num_2(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_2
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_3`"]
    #[inline(always)]
    pub fn is_texc_num_3(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_3
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_4`"]
    #[inline(always)]
    pub fn is_texc_num_4(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_4
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_5`"]
    #[inline(always)]
    pub fn is_texc_num_5(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_5
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_6`"]
    #[inline(always)]
    pub fn is_texc_num_6(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_6
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_7`"]
    #[inline(always)]
    pub fn is_texc_num_7(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_7
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_8`"]
    #[inline(always)]
    pub fn is_texc_num_8(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_8
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_9`"]
    #[inline(always)]
    pub fn is_texc_num_9(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_9
    }
    #[doc = "Checks if the value of the field is `TEXC_NUM_65535`"]
    #[inline(always)]
    pub fn is_texc_num_65535(&self) -> bool {
        *self == TEXC_NUM_A::TEXC_NUM_65535
    }
}
#[doc = "Write proxy for field `TEXC_NUM`"]
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
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
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
#[doc = "Reader of field `TCOMP_FLAG`"]
pub type TCOMP_FLAG_R = crate::R<u16, TCOMP_FLAG_A>;
impl TCOMP_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, TCOMP_FLAG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TCOMP_FLAG_A::TCOMP_FLAG_0),
            1 => Val(TCOMP_FLAG_A::TCOMP_FLAG_1),
            2 => Val(TCOMP_FLAG_A::TCOMP_FLAG_2),
            3 => Val(TCOMP_FLAG_A::TCOMP_FLAG_3),
            4 => Val(TCOMP_FLAG_A::TCOMP_FLAG_4),
            5 => Val(TCOMP_FLAG_A::TCOMP_FLAG_5),
            6 => Val(TCOMP_FLAG_A::TCOMP_FLAG_6),
            7 => Val(TCOMP_FLAG_A::TCOMP_FLAG_7),
            8 => Val(TCOMP_FLAG_A::TCOMP_FLAG_8),
            9 => Val(TCOMP_FLAG_A::TCOMP_FLAG_9),
            65535 => Val(TCOMP_FLAG_A::TCOMP_FLAG_65535),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_0`"]
    #[inline(always)]
    pub fn is_tcomp_flag_0(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_0
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_1`"]
    #[inline(always)]
    pub fn is_tcomp_flag_1(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_1
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_2`"]
    #[inline(always)]
    pub fn is_tcomp_flag_2(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_2
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_3`"]
    #[inline(always)]
    pub fn is_tcomp_flag_3(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_3
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_4`"]
    #[inline(always)]
    pub fn is_tcomp_flag_4(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_4
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_5`"]
    #[inline(always)]
    pub fn is_tcomp_flag_5(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_5
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_6`"]
    #[inline(always)]
    pub fn is_tcomp_flag_6(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_6
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_7`"]
    #[inline(always)]
    pub fn is_tcomp_flag_7(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_7
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_8`"]
    #[inline(always)]
    pub fn is_tcomp_flag_8(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_8
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_9`"]
    #[inline(always)]
    pub fn is_tcomp_flag_9(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_9
    }
    #[doc = "Checks if the value of the field is `TCOMP_FLAG_65535`"]
    #[inline(always)]
    pub fn is_tcomp_flag_65535(&self) -> bool {
        *self == TCOMP_FLAG_A::TCOMP_FLAG_65535
    }
}
#[doc = "Write proxy for field `TCOMP_FLAG`"]
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
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
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
}
