#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PARAM_SPEC>> for R {
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRIG_NUM` reader - Trigger Number"]
pub struct TRIG_NUM_R(crate::FieldReader<u8, u8>);
impl TRIG_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIG_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIG_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Result FIFO Depth\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIFOSIZE_A {
    #[doc = "1: Result FIFO depth = 1 dataword."]
    FIFOSIZE_1 = 1,
    #[doc = "4: Result FIFO depth = 4 datawords."]
    FIFOSIZE_4 = 4,
    #[doc = "8: Result FIFO depth = 8 datawords."]
    FIFOSIZE_8 = 8,
    #[doc = "16: Result FIFO depth = 16 datawords."]
    FIFOSIZE_16 = 16,
    #[doc = "32: Result FIFO depth = 32 datawords."]
    FIFOSIZE_32 = 32,
    #[doc = "64: Result FIFO depth = 64 datawords."]
    FIFOSIZE_64 = 64,
}
impl From<FIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFOSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FIFOSIZE` reader - Result FIFO Depth"]
pub struct FIFOSIZE_R(crate::FieldReader<u8, FIFOSIZE_A>);
impl FIFOSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FIFOSIZE_A> {
        match self.bits {
            1 => Some(FIFOSIZE_A::FIFOSIZE_1),
            4 => Some(FIFOSIZE_A::FIFOSIZE_4),
            8 => Some(FIFOSIZE_A::FIFOSIZE_8),
            16 => Some(FIFOSIZE_A::FIFOSIZE_16),
            32 => Some(FIFOSIZE_A::FIFOSIZE_32),
            64 => Some(FIFOSIZE_A::FIFOSIZE_64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_1`"]
    #[inline(always)]
    pub fn is_fifosize_1(&self) -> bool {
        **self == FIFOSIZE_A::FIFOSIZE_1
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_4`"]
    #[inline(always)]
    pub fn is_fifosize_4(&self) -> bool {
        **self == FIFOSIZE_A::FIFOSIZE_4
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_8`"]
    #[inline(always)]
    pub fn is_fifosize_8(&self) -> bool {
        **self == FIFOSIZE_A::FIFOSIZE_8
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_16`"]
    #[inline(always)]
    pub fn is_fifosize_16(&self) -> bool {
        **self == FIFOSIZE_A::FIFOSIZE_16
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_32`"]
    #[inline(always)]
    pub fn is_fifosize_32(&self) -> bool {
        **self == FIFOSIZE_A::FIFOSIZE_32
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_64`"]
    #[inline(always)]
    pub fn is_fifosize_64(&self) -> bool {
        **self == FIFOSIZE_A::FIFOSIZE_64
    }
}
impl core::ops::Deref for FIFOSIZE_R {
    type Target = crate::FieldReader<u8, FIFOSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CV_NUM` reader - Compare Value Number"]
pub struct CV_NUM_R(crate::FieldReader<u8, u8>);
impl CV_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_NUM` reader - Command Buffer Number"]
pub struct CMD_NUM_R(crate::FieldReader<u8, u8>);
impl CMD_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0x0f04_1010"]
impl crate::Resettable for PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f04_1010
    }
}
