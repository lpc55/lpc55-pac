#[doc = "Reader of register KEYLOCK"]
pub type R = crate::R<u32, super::KEYLOCK>;
#[doc = "Writer for register KEYLOCK"]
pub type W = crate::W<u32, super::KEYLOCK>;
#[doc = "Register KEYLOCK `reset()`'s with value 0xaa"]
impl crate::ResetValue for super::KEYLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xaa
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
impl R {
    #[doc = "Bits 0:1 - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key0(&mut self) -> KEY0_W {
        KEY0_W { w: self }
    }
    #[doc = "Bits 2:3 - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key1(&mut self) -> KEY1_W {
        KEY1_W { w: self }
    }
    #[doc = "Bits 4:5 - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key2(&mut self) -> KEY2_W {
        KEY2_W { w: self }
    }
    #[doc = "Bits 6:7 - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key3(&mut self) -> KEY3_W {
        KEY3_W { w: self }
    }
}
