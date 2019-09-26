#[doc = "Reader of register DIEID"]
pub type R = crate::R<u32, super::DIEID>;
#[doc = "Reader of field `REV_ID`"]
pub type REV_ID_R = crate::R<u8, u8>;
#[doc = "Reader of field `MCO_NUM_IN_DIE_ID`"]
pub type MCO_NUM_IN_DIE_ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:3 - Chip Metal Revision ID."]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Chip Number 0x426B."]
    #[inline(always)]
    pub fn mco_num_in_die_id(&self) -> MCO_NUM_IN_DIE_ID_R {
        MCO_NUM_IN_DIE_ID_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
}
