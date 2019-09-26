#[doc = "Reader of register FMCCR"]
pub type R = crate::R<u32, super::FMCCR>;
#[doc = "Writer for register FMCCR"]
pub type W = crate::W<u32, super::FMCCR>;
#[doc = "Register FMCCR `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::FMCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "Possible values of the field `FLASHTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHTIM_A {
    #[doc = "1 system clock flash access time (for system clock rates up to 11 MHz)."]
    FLASHTIM0,
    #[doc = "2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    FLASHTIM1,
    #[doc = "3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    FLASHTIM2,
    #[doc = "4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    FLASHTIM3,
    #[doc = "5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    FLASHTIM4,
    #[doc = "6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    FLASHTIM5,
    #[doc = "7 system clocks flash access time (for system clock rates up to 77 MHz)."]
    FLASHTIM6,
    #[doc = "8 system clocks flash access time (for system clock rates up to 88 MHz)."]
    FLASHTIM7,
    #[doc = "9 system clocks flash access time (for system clock rates up to 100 MHz)."]
    FLASHTIM8,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        match variant {
            FLASHTIM_A::FLASHTIM0 => 0,
            FLASHTIM_A::FLASHTIM1 => 1,
            FLASHTIM_A::FLASHTIM2 => 2,
            FLASHTIM_A::FLASHTIM3 => 3,
            FLASHTIM_A::FLASHTIM4 => 4,
            FLASHTIM_A::FLASHTIM5 => 5,
            FLASHTIM_A::FLASHTIM6 => 6,
            FLASHTIM_A::FLASHTIM7 => 7,
            FLASHTIM_A::FLASHTIM8 => 8,
        }
    }
}
#[doc = "Reader of field `FLASHTIM`"]
pub type FLASHTIM_R = crate::R<u8, FLASHTIM_A>;
impl FLASHTIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLASHTIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLASHTIM_A::FLASHTIM0),
            1 => Val(FLASHTIM_A::FLASHTIM1),
            2 => Val(FLASHTIM_A::FLASHTIM2),
            3 => Val(FLASHTIM_A::FLASHTIM3),
            4 => Val(FLASHTIM_A::FLASHTIM4),
            5 => Val(FLASHTIM_A::FLASHTIM5),
            6 => Val(FLASHTIM_A::FLASHTIM6),
            7 => Val(FLASHTIM_A::FLASHTIM7),
            8 => Val(FLASHTIM_A::FLASHTIM8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASHTIM0`"]
    #[inline(always)]
    pub fn is_flashtim0(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM0
    }
    #[doc = "Checks if the value of the field is `FLASHTIM1`"]
    #[inline(always)]
    pub fn is_flashtim1(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM1
    }
    #[doc = "Checks if the value of the field is `FLASHTIM2`"]
    #[inline(always)]
    pub fn is_flashtim2(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM2
    }
    #[doc = "Checks if the value of the field is `FLASHTIM3`"]
    #[inline(always)]
    pub fn is_flashtim3(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM3
    }
    #[doc = "Checks if the value of the field is `FLASHTIM4`"]
    #[inline(always)]
    pub fn is_flashtim4(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM4
    }
    #[doc = "Checks if the value of the field is `FLASHTIM5`"]
    #[inline(always)]
    pub fn is_flashtim5(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM5
    }
    #[doc = "Checks if the value of the field is `FLASHTIM6`"]
    #[inline(always)]
    pub fn is_flashtim6(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM6
    }
    #[doc = "Checks if the value of the field is `FLASHTIM7`"]
    #[inline(always)]
    pub fn is_flashtim7(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM7
    }
    #[doc = "Checks if the value of the field is `FLASHTIM8`"]
    #[inline(always)]
    pub fn is_flashtim8(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM8
    }
}
#[doc = "Write proxy for field `FLASHTIM`"]
pub struct FLASHTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 system clock flash access time (for system clock rates up to 11 MHz)."]
    #[inline(always)]
    pub fn flashtim0(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM0)
    }
    #[doc = "2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    #[inline(always)]
    pub fn flashtim1(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM1)
    }
    #[doc = "3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    #[inline(always)]
    pub fn flashtim2(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM2)
    }
    #[doc = "4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    #[inline(always)]
    pub fn flashtim3(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM3)
    }
    #[doc = "5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    #[inline(always)]
    pub fn flashtim4(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM4)
    }
    #[doc = "6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    #[inline(always)]
    pub fn flashtim5(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM5)
    }
    #[doc = "7 system clocks flash access time (for system clock rates up to 77 MHz)."]
    #[inline(always)]
    pub fn flashtim6(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM6)
    }
    #[doc = "8 system clocks flash access time (for system clock rates up to 88 MHz)."]
    #[inline(always)]
    pub fn flashtim7(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM7)
    }
    #[doc = "9 system clocks flash access time (for system clock rates up to 100 MHz)."]
    #[inline(always)]
    pub fn flashtim8(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Flash memory access time."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Flash memory access time."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W {
        FLASHTIM_W { w: self }
    }
}
