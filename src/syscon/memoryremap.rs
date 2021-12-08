#[doc = "Register `MEMORYREMAP` reader"]
pub struct R(crate::R<MEMORYREMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMORYREMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMORYREMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMORYREMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMORYREMAP` writer"]
pub struct W(crate::W<MEMORYREMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMORYREMAP_SPEC>;
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
impl From<crate::W<MEMORYREMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMORYREMAP_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `MAP` reader - Select the location of the vector table :."]
pub struct MAP_R(crate::FieldReader<u8, MAP_A>);
impl MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAP_R(crate::FieldReader::new(bits))
    }
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
        **self == MAP_A::ROM0
    }
    #[doc = "Checks if the value of the field is `RAM1`"]
    #[inline(always)]
    pub fn is_ram1(&self) -> bool {
        **self == MAP_A::RAM1
    }
    #[doc = "Checks if the value of the field is `FLASH0`"]
    #[inline(always)]
    pub fn is_flash0(&self) -> bool {
        **self == MAP_A::FLASH0
    }
    #[doc = "Checks if the value of the field is `FLASH1`"]
    #[inline(always)]
    pub fn is_flash1(&self) -> bool {
        **self == MAP_A::FLASH1
    }
}
impl core::ops::Deref for MAP_R {
    type Target = crate::FieldReader<u8, MAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAP` writer - Select the location of the vector table :."]
pub struct MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAP_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Remap control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memoryremap](index.html) module"]
pub struct MEMORYREMAP_SPEC;
impl crate::RegisterSpec for MEMORYREMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memoryremap::R](R) reader structure"]
impl crate::Readable for MEMORYREMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memoryremap::W](W) writer structure"]
impl crate::Writable for MEMORYREMAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMORYREMAP to value 0"]
impl crate::Resettable for MEMORYREMAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
