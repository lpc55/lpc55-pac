#[doc = "Reader of register MEMORYREMAP"]
pub type R = crate::R<u32, super::MEMORYREMAP>;
#[doc = "Writer for register MEMORYREMAP"]
pub type W = crate::W<u32, super::MEMORYREMAP>;
#[doc = "Register MEMORYREMAP `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMORYREMAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select the location of the vector table :.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAP_A {
    #[doc = "0: Vector Table in ROM."]
    ROM0 = 0,
    #[doc = "1: Vector Table in RAM."]
    RAM1 = 1,
    #[doc = "2: Vector Table in Flash."]
    FLASH0 = 2,
    #[doc = "3: Vector Table in Flash."]
    FLASH1 = 3,
}
impl From<MAP_A> for u8 {
    #[inline(always)]
    fn from(variant: MAP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAP`"]
pub type MAP_R = crate::R<u8, MAP_A>;
impl MAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAP_A {
        match self.bits {
            0 => MAP_A::ROM0,
            1 => MAP_A::RAM1,
            2 => MAP_A::FLASH0,
            3 => MAP_A::FLASH1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ROM0`"]
    #[inline(always)]
    pub fn is_rom0(&self) -> bool {
        *self == MAP_A::ROM0
    }
    #[doc = "Checks if the value of the field is `RAM1`"]
    #[inline(always)]
    pub fn is_ram1(&self) -> bool {
        *self == MAP_A::RAM1
    }
    #[doc = "Checks if the value of the field is `FLASH0`"]
    #[inline(always)]
    pub fn is_flash0(&self) -> bool {
        *self == MAP_A::FLASH0
    }
    #[doc = "Checks if the value of the field is `FLASH1`"]
    #[inline(always)]
    pub fn is_flash1(&self) -> bool {
        *self == MAP_A::FLASH1
    }
}
#[doc = "Write proxy for field `MAP`"]
pub struct MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Vector Table in ROM."]
    #[inline(always)]
    pub fn rom0(self) -> &'a mut W {
        self.variant(MAP_A::ROM0)
    }
    #[doc = "Vector Table in RAM."]
    #[inline(always)]
    pub fn ram1(self) -> &'a mut W {
        self.variant(MAP_A::RAM1)
    }
    #[doc = "Vector Table in Flash."]
    #[inline(always)]
    pub fn flash0(self) -> &'a mut W {
        self.variant(MAP_A::FLASH0)
    }
    #[doc = "Vector Table in Flash."]
    #[inline(always)]
    pub fn flash1(self) -> &'a mut W {
        self.variant(MAP_A::FLASH1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select the location of the vector table :."]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the location of the vector table :."]
    #[inline(always)]
    pub fn map(&mut self) -> MAP_W {
        MAP_W { w: self }
    }
}
