#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `TRIG_NUM`"]
pub type TRIG_NUM_R = crate::R<u8, u8>;
#[doc = "Possible values of the field `FIFOSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOSIZE_A {
    #[doc = "Result FIFO depth = 1 dataword."]
    FIFOSIZE_1,
    #[doc = "Result FIFO depth = 4 datawords."]
    FIFOSIZE_4,
    #[doc = "Result FIFO depth = 8 datawords."]
    FIFOSIZE_8,
    #[doc = "Result FIFO depth = 16 datawords."]
    FIFOSIZE_16,
    #[doc = "Result FIFO depth = 32 datawords."]
    FIFOSIZE_32,
    #[doc = "Result FIFO depth = 64 datawords."]
    FIFOSIZE_64,
}
impl From<FIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFOSIZE_A) -> Self {
        match variant {
            FIFOSIZE_A::FIFOSIZE_1 => 1,
            FIFOSIZE_A::FIFOSIZE_4 => 4,
            FIFOSIZE_A::FIFOSIZE_8 => 8,
            FIFOSIZE_A::FIFOSIZE_16 => 16,
            FIFOSIZE_A::FIFOSIZE_32 => 32,
            FIFOSIZE_A::FIFOSIZE_64 => 64,
        }
    }
}
#[doc = "Reader of field `FIFOSIZE`"]
pub type FIFOSIZE_R = crate::R<u8, FIFOSIZE_A>;
impl FIFOSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FIFOSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(FIFOSIZE_A::FIFOSIZE_1),
            4 => Val(FIFOSIZE_A::FIFOSIZE_4),
            8 => Val(FIFOSIZE_A::FIFOSIZE_8),
            16 => Val(FIFOSIZE_A::FIFOSIZE_16),
            32 => Val(FIFOSIZE_A::FIFOSIZE_32),
            64 => Val(FIFOSIZE_A::FIFOSIZE_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_1`"]
    #[inline(always)]
    pub fn is_fifosize_1(&self) -> bool {
        *self == FIFOSIZE_A::FIFOSIZE_1
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_4`"]
    #[inline(always)]
    pub fn is_fifosize_4(&self) -> bool {
        *self == FIFOSIZE_A::FIFOSIZE_4
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_8`"]
    #[inline(always)]
    pub fn is_fifosize_8(&self) -> bool {
        *self == FIFOSIZE_A::FIFOSIZE_8
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_16`"]
    #[inline(always)]
    pub fn is_fifosize_16(&self) -> bool {
        *self == FIFOSIZE_A::FIFOSIZE_16
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_32`"]
    #[inline(always)]
    pub fn is_fifosize_32(&self) -> bool {
        *self == FIFOSIZE_A::FIFOSIZE_32
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_64`"]
    #[inline(always)]
    pub fn is_fifosize_64(&self) -> bool {
        *self == FIFOSIZE_A::FIFOSIZE_64
    }
}
#[doc = "Reader of field `CV_NUM`"]
pub type CV_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `CMD_NUM`"]
pub type CMD_NUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Trigger Number"]
    #[inline(always)]
    pub fn trig_num(&self) -> TRIG_NUM_R {
        TRIG_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Result FIFO Depth"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Compare Value Number"]
    #[inline(always)]
    pub fn cv_num(&self) -> CV_NUM_R {
        CV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Command Buffer Number"]
    #[inline(always)]
    pub fn cmd_num(&self) -> CMD_NUM_R {
        CMD_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
