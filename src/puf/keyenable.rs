#[doc = "Reader of register KEYENABLE"]
pub type R = crate::R<u32, super::KEYENABLE>;
#[doc = "Writer for register KEYENABLE"]
pub type W = crate::W<u32, super::KEYENABLE>;
#[doc = "Register KEYENABLE `reset()`'s with value 0x55"]
impl crate::ResetValue for super::KEYENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x55
    }
}
#[doc = "Reader of field `KEY0`"]
pub type KEY0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY0`"]
pub struct KEY0_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `KEY1`"]
pub type KEY1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY1`"]
pub struct KEY1_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `KEY2`"]
pub type KEY2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY2`"]
pub struct KEY2_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `KEY3`"]
pub type KEY3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY3`"]
pub struct KEY3_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Key destination for PUF key.\n\nValue on reset: 85"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "86: Send key to AES engine."]
    AES = 86,
    #[doc = "89: Send key to PRINCE engine for memory layout 0."]
    PRINCE0 = 89,
    #[doc = "101: Send key to PRINCE engine for memory layout 1."]
    PRINCE1 = 101,
    #[doc = "149: Send key to PRINCE engine for memory layout 2."]
    PRINCE2 = 149,
    #[doc = "85: Do not send key to any hardware engine."]
    NONE = 85,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u8, KEY_A>;
impl KEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KEY_A> {
        use crate::Variant::*;
        match self.bits {
            86 => Val(KEY_A::AES),
            89 => Val(KEY_A::PRINCE0),
            101 => Val(KEY_A::PRINCE1),
            149 => Val(KEY_A::PRINCE2),
            85 => Val(KEY_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES`"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == KEY_A::AES
    }
    #[doc = "Checks if the value of the field is `PRINCE0`"]
    #[inline(always)]
    pub fn is_prince0(&self) -> bool {
        *self == KEY_A::PRINCE0
    }
    #[doc = "Checks if the value of the field is `PRINCE1`"]
    #[inline(always)]
    pub fn is_prince1(&self) -> bool {
        *self == KEY_A::PRINCE1
    }
    #[doc = "Checks if the value of the field is `PRINCE2`"]
    #[inline(always)]
    pub fn is_prince2(&self) -> bool {
        *self == KEY_A::PRINCE2
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == KEY_A::NONE
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Send key to AES engine."]
    #[inline(always)]
    pub fn aes(self) -> &'a mut W {
        self.variant(KEY_A::AES)
    }
    #[doc = "Send key to PRINCE engine for memory layout 0."]
    #[inline(always)]
    pub fn prince0(self) -> &'a mut W {
        self.variant(KEY_A::PRINCE0)
    }
    #[doc = "Send key to PRINCE engine for memory layout 1."]
    #[inline(always)]
    pub fn prince1(self) -> &'a mut W {
        self.variant(KEY_A::PRINCE1)
    }
    #[doc = "Send key to PRINCE engine for memory layout 2."]
    #[inline(always)]
    pub fn prince2(self) -> &'a mut W {
        self.variant(KEY_A::PRINCE2)
    }
    #[doc = "Do not send key to any hardware engine."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(KEY_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:7 - Key destination for PUF key."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    pub fn key0(&mut self) -> KEY0_W {
        KEY0_W { w: self }
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    pub fn key1(&mut self) -> KEY1_W {
        KEY1_W { w: self }
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    pub fn key2(&mut self) -> KEY2_W {
        KEY2_W { w: self }
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    pub fn key3(&mut self) -> KEY3_W {
        KEY3_W { w: self }
    }
    #[doc = "Bits 0:7 - Key destination for PUF key."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
