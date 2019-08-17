#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Valid Key Sore Header : 0x95959595"]
    pub header: HEADER,
    #[doc = "0x04 - puf discharge time in ms."]
    pub puf_discharge_time_in_ms: PUF_DISCHARGE_TIME_IN_MS,
    #[doc = "0x08 - ."]
    pub activation_code: [ACTIVATION_CODE; 298],
    _reserved_3_sbkey: [u8; 4usize],
    _reserved_4_sbkey: [u8; 4usize],
    _reserved_5_sbkey: [u8; 4usize],
    _reserved_6_sbkey: [u8; 4usize],
    _reserved_7_sbkey: [u8; 4usize],
    _reserved_8_sbkey: [u8; 4usize],
    _reserved_9_sbkey: [u8; 4usize],
    _reserved_10_sbkey: [u8; 4usize],
    _reserved_11_sbkey: [u8; 4usize],
    _reserved_12_sbkey: [u8; 4usize],
    _reserved_13_sbkey: [u8; 4usize],
    _reserved_14_sbkey: [u8; 4usize],
    _reserved_15_sbkey: [u8; 4usize],
    _reserved_16_sbkey: [u8; 4usize],
    _reserved_17_user_kek: [u8; 4usize],
    _reserved_18_user_kek: [u8; 4usize],
    _reserved_19_user_kek: [u8; 4usize],
    _reserved_20_user_kek: [u8; 4usize],
    _reserved_21_user_kek: [u8; 4usize],
    _reserved_22_user_kek: [u8; 4usize],
    _reserved_23_user_kek: [u8; 4usize],
    _reserved_24_user_kek: [u8; 4usize],
    _reserved_25_user_kek: [u8; 4usize],
    _reserved_26_user_kek: [u8; 4usize],
    _reserved_27_user_kek: [u8; 4usize],
    _reserved_28_user_kek: [u8; 4usize],
    _reserved_29_user_kek: [u8; 4usize],
    _reserved_30_user_kek: [u8; 4usize],
    _reserved_31_uds: [u8; 4usize],
    _reserved_32_uds: [u8; 4usize],
    _reserved_33_uds: [u8; 4usize],
    _reserved_34_uds: [u8; 4usize],
    _reserved_35_uds: [u8; 4usize],
    _reserved_36_uds: [u8; 4usize],
    _reserved_37_uds: [u8; 4usize],
    _reserved_38_uds: [u8; 4usize],
    _reserved_39_uds: [u8; 4usize],
    _reserved_40_uds: [u8; 4usize],
    _reserved_41_uds: [u8; 4usize],
    _reserved_42_uds: [u8; 4usize],
    _reserved_43_uds: [u8; 4usize],
    _reserved_44_uds: [u8; 4usize],
    _reserved_45_prince_region0: [u8; 4usize],
    _reserved_46_prince_region0: [u8; 4usize],
    _reserved_47_prince_region0: [u8; 4usize],
    _reserved_48_prince_region0: [u8; 4usize],
    _reserved_49_prince_region0: [u8; 4usize],
    _reserved_50_prince_region0: [u8; 4usize],
    _reserved_51_prince_region0: [u8; 4usize],
    _reserved_52_prince_region0: [u8; 4usize],
    _reserved_53_prince_region0: [u8; 4usize],
    _reserved_54_prince_region0: [u8; 4usize],
    _reserved_55_prince_region0: [u8; 4usize],
    _reserved_56_prince_region0: [u8; 4usize],
    _reserved_57_prince_region0: [u8; 4usize],
    _reserved_58_prince_region0: [u8; 4usize],
    _reserved_59_prince_region1: [u8; 4usize],
    _reserved_60_prince_region1: [u8; 4usize],
    _reserved_61_prince_region1: [u8; 4usize],
    _reserved_62_prince_region1: [u8; 4usize],
    _reserved_63_prince_region1: [u8; 4usize],
    _reserved_64_prince_region1: [u8; 4usize],
    _reserved_65_prince_region1: [u8; 4usize],
    _reserved_66_prince_region1: [u8; 4usize],
    _reserved_67_prince_region1: [u8; 4usize],
    _reserved_68_prince_region1: [u8; 4usize],
    _reserved_69_prince_region1: [u8; 4usize],
    _reserved_70_prince_region1: [u8; 4usize],
    _reserved_71_prince_region1: [u8; 4usize],
    _reserved_72_prince_region1: [u8; 4usize],
    _reserved_73_prince_region2: [u8; 4usize],
    _reserved_74_prince_region2: [u8; 4usize],
    _reserved_75_prince_region2: [u8; 4usize],
    _reserved_76_prince_region2: [u8; 4usize],
    _reserved_77_prince_region2: [u8; 4usize],
    _reserved_78_prince_region2: [u8; 4usize],
    _reserved_79_prince_region2: [u8; 4usize],
    _reserved_80_prince_region2: [u8; 4usize],
    _reserved_81_prince_region2: [u8; 4usize],
    _reserved_82_prince_region2: [u8; 4usize],
    _reserved_83_prince_region2: [u8; 4usize],
    _reserved_84_prince_region2: [u8; 4usize],
    _reserved_85_prince_region2: [u8; 4usize],
    _reserved_86_prince_region2: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x4b0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code0(&self) -> &SBKEY_KEY_CODE0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1200usize) as *const SBKEY_KEY_CODE0) }
    }
    #[doc = "0x4b0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code0_mut(&self) -> &mut SBKEY_KEY_CODE0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1200usize) as *mut SBKEY_KEY_CODE0) }
    }
    #[doc = "0x4b0 - ."]
    #[inline(always)]
    pub fn sbkey_header0(&self) -> &SBKEY_HEADER0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1200usize) as *const SBKEY_HEADER0) }
    }
    #[doc = "0x4b0 - ."]
    #[inline(always)]
    pub fn sbkey_header0_mut(&self) -> &mut SBKEY_HEADER0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1200usize) as *mut SBKEY_HEADER0) }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code1(&self) -> &SBKEY_KEY_CODE1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1204usize) as *const SBKEY_KEY_CODE1) }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code1_mut(&self) -> &mut SBKEY_KEY_CODE1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1204usize) as *mut SBKEY_KEY_CODE1) }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub fn sbkey_header1(&self) -> &SBKEY_HEADER1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1204usize) as *const SBKEY_HEADER1) }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub fn sbkey_header1_mut(&self) -> &mut SBKEY_HEADER1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1204usize) as *mut SBKEY_HEADER1) }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub fn sbkey_key_code2(&self) -> &SBKEY_KEY_CODE2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1208usize) as *const SBKEY_KEY_CODE2) }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub fn sbkey_key_code2_mut(&self) -> &mut SBKEY_KEY_CODE2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1208usize) as *mut SBKEY_KEY_CODE2) }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub fn sbkey_body0(&self) -> &SBKEY_BODY0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1208usize) as *const SBKEY_BODY0) }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub fn sbkey_body0_mut(&self) -> &mut SBKEY_BODY0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1208usize) as *mut SBKEY_BODY0) }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub fn sbkey_key_code3(&self) -> &SBKEY_KEY_CODE3 {
        unsafe { &*(((self as *const Self) as *const u8).add(1212usize) as *const SBKEY_KEY_CODE3) }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub fn sbkey_key_code3_mut(&self) -> &mut SBKEY_KEY_CODE3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1212usize) as *mut SBKEY_KEY_CODE3) }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub fn sbkey_body1(&self) -> &SBKEY_BODY1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1212usize) as *const SBKEY_BODY1) }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub fn sbkey_body1_mut(&self) -> &mut SBKEY_BODY1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1212usize) as *mut SBKEY_BODY1) }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code4(&self) -> &SBKEY_KEY_CODE4 {
        unsafe { &*(((self as *const Self) as *const u8).add(1216usize) as *const SBKEY_KEY_CODE4) }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code4_mut(&self) -> &mut SBKEY_KEY_CODE4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1216usize) as *mut SBKEY_KEY_CODE4) }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub fn sbkey_body2(&self) -> &SBKEY_BODY2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1216usize) as *const SBKEY_BODY2) }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub fn sbkey_body2_mut(&self) -> &mut SBKEY_BODY2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1216usize) as *mut SBKEY_BODY2) }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code5(&self) -> &SBKEY_KEY_CODE5 {
        unsafe { &*(((self as *const Self) as *const u8).add(1220usize) as *const SBKEY_KEY_CODE5) }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code5_mut(&self) -> &mut SBKEY_KEY_CODE5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1220usize) as *mut SBKEY_KEY_CODE5) }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub fn sbkey_body3(&self) -> &SBKEY_BODY3 {
        unsafe { &*(((self as *const Self) as *const u8).add(1220usize) as *const SBKEY_BODY3) }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub fn sbkey_body3_mut(&self) -> &mut SBKEY_BODY3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1220usize) as *mut SBKEY_BODY3) }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub fn sbkey_key_code6(&self) -> &SBKEY_KEY_CODE6 {
        unsafe { &*(((self as *const Self) as *const u8).add(1224usize) as *const SBKEY_KEY_CODE6) }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub fn sbkey_key_code6_mut(&self) -> &mut SBKEY_KEY_CODE6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1224usize) as *mut SBKEY_KEY_CODE6) }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub fn sbkey_body4(&self) -> &SBKEY_BODY4 {
        unsafe { &*(((self as *const Self) as *const u8).add(1224usize) as *const SBKEY_BODY4) }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub fn sbkey_body4_mut(&self) -> &mut SBKEY_BODY4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1224usize) as *mut SBKEY_BODY4) }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub fn sbkey_key_code7(&self) -> &SBKEY_KEY_CODE7 {
        unsafe { &*(((self as *const Self) as *const u8).add(1228usize) as *const SBKEY_KEY_CODE7) }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub fn sbkey_key_code7_mut(&self) -> &mut SBKEY_KEY_CODE7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1228usize) as *mut SBKEY_KEY_CODE7) }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub fn sbkey_body5(&self) -> &SBKEY_BODY5 {
        unsafe { &*(((self as *const Self) as *const u8).add(1228usize) as *const SBKEY_BODY5) }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub fn sbkey_body5_mut(&self) -> &mut SBKEY_BODY5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1228usize) as *mut SBKEY_BODY5) }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code8(&self) -> &SBKEY_KEY_CODE8 {
        unsafe { &*(((self as *const Self) as *const u8).add(1232usize) as *const SBKEY_KEY_CODE8) }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code8_mut(&self) -> &mut SBKEY_KEY_CODE8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1232usize) as *mut SBKEY_KEY_CODE8) }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub fn sbkey_body6(&self) -> &SBKEY_BODY6 {
        unsafe { &*(((self as *const Self) as *const u8).add(1232usize) as *const SBKEY_BODY6) }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub fn sbkey_body6_mut(&self) -> &mut SBKEY_BODY6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1232usize) as *mut SBKEY_BODY6) }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code9(&self) -> &SBKEY_KEY_CODE9 {
        unsafe { &*(((self as *const Self) as *const u8).add(1236usize) as *const SBKEY_KEY_CODE9) }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code9_mut(&self) -> &mut SBKEY_KEY_CODE9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1236usize) as *mut SBKEY_KEY_CODE9) }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub fn sbkey_body7(&self) -> &SBKEY_BODY7 {
        unsafe { &*(((self as *const Self) as *const u8).add(1236usize) as *const SBKEY_BODY7) }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub fn sbkey_body7_mut(&self) -> &mut SBKEY_BODY7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1236usize) as *mut SBKEY_BODY7) }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub fn sbkey_key_code10(&self) -> &SBKEY_KEY_CODE10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1240usize) as *const SBKEY_KEY_CODE10)
        }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub fn sbkey_key_code10_mut(&self) -> &mut SBKEY_KEY_CODE10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1240usize) as *mut SBKEY_KEY_CODE10)
        }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub fn sbkey_body8(&self) -> &SBKEY_BODY8 {
        unsafe { &*(((self as *const Self) as *const u8).add(1240usize) as *const SBKEY_BODY8) }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub fn sbkey_body8_mut(&self) -> &mut SBKEY_BODY8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1240usize) as *mut SBKEY_BODY8) }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub fn sbkey_key_code11(&self) -> &SBKEY_KEY_CODE11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1244usize) as *const SBKEY_KEY_CODE11)
        }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub fn sbkey_key_code11_mut(&self) -> &mut SBKEY_KEY_CODE11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1244usize) as *mut SBKEY_KEY_CODE11)
        }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub fn sbkey_body9(&self) -> &SBKEY_BODY9 {
        unsafe { &*(((self as *const Self) as *const u8).add(1244usize) as *const SBKEY_BODY9) }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub fn sbkey_body9_mut(&self) -> &mut SBKEY_BODY9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1244usize) as *mut SBKEY_BODY9) }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code12(&self) -> &SBKEY_KEY_CODE12 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1248usize) as *const SBKEY_KEY_CODE12)
        }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code12_mut(&self) -> &mut SBKEY_KEY_CODE12 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1248usize) as *mut SBKEY_KEY_CODE12)
        }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub fn sbkey_body10(&self) -> &SBKEY_BODY10 {
        unsafe { &*(((self as *const Self) as *const u8).add(1248usize) as *const SBKEY_BODY10) }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub fn sbkey_body10_mut(&self) -> &mut SBKEY_BODY10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1248usize) as *mut SBKEY_BODY10) }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code13(&self) -> &SBKEY_KEY_CODE13 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1252usize) as *const SBKEY_KEY_CODE13)
        }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code13_mut(&self) -> &mut SBKEY_KEY_CODE13 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1252usize) as *mut SBKEY_KEY_CODE13)
        }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub fn sbkey_body11(&self) -> &SBKEY_BODY11 {
        unsafe { &*(((self as *const Self) as *const u8).add(1252usize) as *const SBKEY_BODY11) }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub fn sbkey_body11_mut(&self) -> &mut SBKEY_BODY11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1252usize) as *mut SBKEY_BODY11) }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub fn user_kek_key_code0(&self) -> &USER_KEK_KEY_CODE0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1256usize) as *const USER_KEK_KEY_CODE0)
        }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub fn user_kek_key_code0_mut(&self) -> &mut USER_KEK_KEY_CODE0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1256usize) as *mut USER_KEK_KEY_CODE0)
        }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub fn user_kek_header0(&self) -> &USER_KEK_HEADER0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1256usize) as *const USER_KEK_HEADER0)
        }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub fn user_kek_header0_mut(&self) -> &mut USER_KEK_HEADER0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1256usize) as *mut USER_KEK_HEADER0)
        }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub fn user_kek_key_code1(&self) -> &USER_KEK_KEY_CODE1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1260usize) as *const USER_KEK_KEY_CODE1)
        }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub fn user_kek_key_code1_mut(&self) -> &mut USER_KEK_KEY_CODE1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1260usize) as *mut USER_KEK_KEY_CODE1)
        }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub fn user_kek_header1(&self) -> &USER_KEK_HEADER1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1260usize) as *const USER_KEK_HEADER1)
        }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub fn user_kek_header1_mut(&self) -> &mut USER_KEK_HEADER1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1260usize) as *mut USER_KEK_HEADER1)
        }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub fn user_kek_key_code2(&self) -> &USER_KEK_KEY_CODE2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1264usize) as *const USER_KEK_KEY_CODE2)
        }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub fn user_kek_key_code2_mut(&self) -> &mut USER_KEK_KEY_CODE2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1264usize) as *mut USER_KEK_KEY_CODE2)
        }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub fn user_kek_body0(&self) -> &USER_KEK_BODY0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1264usize) as *const USER_KEK_BODY0) }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub fn user_kek_body0_mut(&self) -> &mut USER_KEK_BODY0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1264usize) as *mut USER_KEK_BODY0) }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub fn user_kek_key_code3(&self) -> &USER_KEK_KEY_CODE3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1268usize) as *const USER_KEK_KEY_CODE3)
        }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub fn user_kek_key_code3_mut(&self) -> &mut USER_KEK_KEY_CODE3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1268usize) as *mut USER_KEK_KEY_CODE3)
        }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub fn user_kek_body1(&self) -> &USER_KEK_BODY1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1268usize) as *const USER_KEK_BODY1) }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub fn user_kek_body1_mut(&self) -> &mut USER_KEK_BODY1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1268usize) as *mut USER_KEK_BODY1) }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub fn user_kek_key_code4(&self) -> &USER_KEK_KEY_CODE4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1272usize) as *const USER_KEK_KEY_CODE4)
        }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub fn user_kek_key_code4_mut(&self) -> &mut USER_KEK_KEY_CODE4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1272usize) as *mut USER_KEK_KEY_CODE4)
        }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub fn user_kek_body2(&self) -> &USER_KEK_BODY2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1272usize) as *const USER_KEK_BODY2) }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub fn user_kek_body2_mut(&self) -> &mut USER_KEK_BODY2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1272usize) as *mut USER_KEK_BODY2) }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub fn user_kek_key_code5(&self) -> &USER_KEK_KEY_CODE5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1276usize) as *const USER_KEK_KEY_CODE5)
        }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub fn user_kek_key_code5_mut(&self) -> &mut USER_KEK_KEY_CODE5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1276usize) as *mut USER_KEK_KEY_CODE5)
        }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub fn user_kek_body3(&self) -> &USER_KEK_BODY3 {
        unsafe { &*(((self as *const Self) as *const u8).add(1276usize) as *const USER_KEK_BODY3) }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub fn user_kek_body3_mut(&self) -> &mut USER_KEK_BODY3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1276usize) as *mut USER_KEK_BODY3) }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub fn user_kek_key_code6(&self) -> &USER_KEK_KEY_CODE6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize) as *const USER_KEK_KEY_CODE6)
        }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub fn user_kek_key_code6_mut(&self) -> &mut USER_KEK_KEY_CODE6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1280usize) as *mut USER_KEK_KEY_CODE6)
        }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub fn user_kek_body4(&self) -> &USER_KEK_BODY4 {
        unsafe { &*(((self as *const Self) as *const u8).add(1280usize) as *const USER_KEK_BODY4) }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub fn user_kek_body4_mut(&self) -> &mut USER_KEK_BODY4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1280usize) as *mut USER_KEK_BODY4) }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub fn user_kek_key_code7(&self) -> &USER_KEK_KEY_CODE7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1284usize) as *const USER_KEK_KEY_CODE7)
        }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub fn user_kek_key_code7_mut(&self) -> &mut USER_KEK_KEY_CODE7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1284usize) as *mut USER_KEK_KEY_CODE7)
        }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub fn user_kek_body5(&self) -> &USER_KEK_BODY5 {
        unsafe { &*(((self as *const Self) as *const u8).add(1284usize) as *const USER_KEK_BODY5) }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub fn user_kek_body5_mut(&self) -> &mut USER_KEK_BODY5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1284usize) as *mut USER_KEK_BODY5) }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub fn user_kek_key_code8(&self) -> &USER_KEK_KEY_CODE8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1288usize) as *const USER_KEK_KEY_CODE8)
        }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub fn user_kek_key_code8_mut(&self) -> &mut USER_KEK_KEY_CODE8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1288usize) as *mut USER_KEK_KEY_CODE8)
        }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub fn user_kek_body6(&self) -> &USER_KEK_BODY6 {
        unsafe { &*(((self as *const Self) as *const u8).add(1288usize) as *const USER_KEK_BODY6) }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub fn user_kek_body6_mut(&self) -> &mut USER_KEK_BODY6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1288usize) as *mut USER_KEK_BODY6) }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub fn user_kek_key_code9(&self) -> &USER_KEK_KEY_CODE9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1292usize) as *const USER_KEK_KEY_CODE9)
        }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub fn user_kek_key_code9_mut(&self) -> &mut USER_KEK_KEY_CODE9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1292usize) as *mut USER_KEK_KEY_CODE9)
        }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub fn user_kek_body7(&self) -> &USER_KEK_BODY7 {
        unsafe { &*(((self as *const Self) as *const u8).add(1292usize) as *const USER_KEK_BODY7) }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub fn user_kek_body7_mut(&self) -> &mut USER_KEK_BODY7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1292usize) as *mut USER_KEK_BODY7) }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub fn user_kek_key_code10(&self) -> &USER_KEK_KEY_CODE10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1296usize) as *const USER_KEK_KEY_CODE10)
        }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub fn user_kek_key_code10_mut(&self) -> &mut USER_KEK_KEY_CODE10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1296usize) as *mut USER_KEK_KEY_CODE10)
        }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub fn user_kek_body8(&self) -> &USER_KEK_BODY8 {
        unsafe { &*(((self as *const Self) as *const u8).add(1296usize) as *const USER_KEK_BODY8) }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub fn user_kek_body8_mut(&self) -> &mut USER_KEK_BODY8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1296usize) as *mut USER_KEK_BODY8) }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub fn user_kek_key_code11(&self) -> &USER_KEK_KEY_CODE11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1300usize) as *const USER_KEK_KEY_CODE11)
        }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub fn user_kek_key_code11_mut(&self) -> &mut USER_KEK_KEY_CODE11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1300usize) as *mut USER_KEK_KEY_CODE11)
        }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub fn user_kek_body9(&self) -> &USER_KEK_BODY9 {
        unsafe { &*(((self as *const Self) as *const u8).add(1300usize) as *const USER_KEK_BODY9) }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub fn user_kek_body9_mut(&self) -> &mut USER_KEK_BODY9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1300usize) as *mut USER_KEK_BODY9) }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub fn user_kek_key_code12(&self) -> &USER_KEK_KEY_CODE12 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1304usize) as *const USER_KEK_KEY_CODE12)
        }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub fn user_kek_key_code12_mut(&self) -> &mut USER_KEK_KEY_CODE12 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1304usize) as *mut USER_KEK_KEY_CODE12)
        }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub fn user_kek_body10(&self) -> &USER_KEK_BODY10 {
        unsafe { &*(((self as *const Self) as *const u8).add(1304usize) as *const USER_KEK_BODY10) }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub fn user_kek_body10_mut(&self) -> &mut USER_KEK_BODY10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1304usize) as *mut USER_KEK_BODY10) }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub fn user_kek_key_code13(&self) -> &USER_KEK_KEY_CODE13 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1308usize) as *const USER_KEK_KEY_CODE13)
        }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub fn user_kek_key_code13_mut(&self) -> &mut USER_KEK_KEY_CODE13 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1308usize) as *mut USER_KEK_KEY_CODE13)
        }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub fn user_kek_body11(&self) -> &USER_KEK_BODY11 {
        unsafe { &*(((self as *const Self) as *const u8).add(1308usize) as *const USER_KEK_BODY11) }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub fn user_kek_body11_mut(&self) -> &mut USER_KEK_BODY11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1308usize) as *mut USER_KEK_BODY11) }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub fn uds_key_code0(&self) -> &UDS_KEY_CODE0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1312usize) as *const UDS_KEY_CODE0) }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub fn uds_key_code0_mut(&self) -> &mut UDS_KEY_CODE0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1312usize) as *mut UDS_KEY_CODE0) }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub fn uds_header0(&self) -> &UDS_HEADER0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1312usize) as *const UDS_HEADER0) }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub fn uds_header0_mut(&self) -> &mut UDS_HEADER0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1312usize) as *mut UDS_HEADER0) }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub fn uds_key_code1(&self) -> &UDS_KEY_CODE1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1316usize) as *const UDS_KEY_CODE1) }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub fn uds_key_code1_mut(&self) -> &mut UDS_KEY_CODE1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1316usize) as *mut UDS_KEY_CODE1) }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub fn uds_header1(&self) -> &UDS_HEADER1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1316usize) as *const UDS_HEADER1) }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub fn uds_header1_mut(&self) -> &mut UDS_HEADER1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1316usize) as *mut UDS_HEADER1) }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub fn uds_key_code2(&self) -> &UDS_KEY_CODE2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1320usize) as *const UDS_KEY_CODE2) }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub fn uds_key_code2_mut(&self) -> &mut UDS_KEY_CODE2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1320usize) as *mut UDS_KEY_CODE2) }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub fn uds_body0(&self) -> &UDS_BODY0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1320usize) as *const UDS_BODY0) }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub fn uds_body0_mut(&self) -> &mut UDS_BODY0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1320usize) as *mut UDS_BODY0) }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub fn uds_key_code3(&self) -> &UDS_KEY_CODE3 {
        unsafe { &*(((self as *const Self) as *const u8).add(1324usize) as *const UDS_KEY_CODE3) }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub fn uds_key_code3_mut(&self) -> &mut UDS_KEY_CODE3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1324usize) as *mut UDS_KEY_CODE3) }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub fn uds_body1(&self) -> &UDS_BODY1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1324usize) as *const UDS_BODY1) }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub fn uds_body1_mut(&self) -> &mut UDS_BODY1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1324usize) as *mut UDS_BODY1) }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub fn uds_key_code4(&self) -> &UDS_KEY_CODE4 {
        unsafe { &*(((self as *const Self) as *const u8).add(1328usize) as *const UDS_KEY_CODE4) }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub fn uds_key_code4_mut(&self) -> &mut UDS_KEY_CODE4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1328usize) as *mut UDS_KEY_CODE4) }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub fn uds_body2(&self) -> &UDS_BODY2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1328usize) as *const UDS_BODY2) }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub fn uds_body2_mut(&self) -> &mut UDS_BODY2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1328usize) as *mut UDS_BODY2) }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub fn uds_key_code5(&self) -> &UDS_KEY_CODE5 {
        unsafe { &*(((self as *const Self) as *const u8).add(1332usize) as *const UDS_KEY_CODE5) }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub fn uds_key_code5_mut(&self) -> &mut UDS_KEY_CODE5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1332usize) as *mut UDS_KEY_CODE5) }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub fn uds_body3(&self) -> &UDS_BODY3 {
        unsafe { &*(((self as *const Self) as *const u8).add(1332usize) as *const UDS_BODY3) }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub fn uds_body3_mut(&self) -> &mut UDS_BODY3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1332usize) as *mut UDS_BODY3) }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub fn uds_key_code6(&self) -> &UDS_KEY_CODE6 {
        unsafe { &*(((self as *const Self) as *const u8).add(1336usize) as *const UDS_KEY_CODE6) }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub fn uds_key_code6_mut(&self) -> &mut UDS_KEY_CODE6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1336usize) as *mut UDS_KEY_CODE6) }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub fn uds_body4(&self) -> &UDS_BODY4 {
        unsafe { &*(((self as *const Self) as *const u8).add(1336usize) as *const UDS_BODY4) }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub fn uds_body4_mut(&self) -> &mut UDS_BODY4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1336usize) as *mut UDS_BODY4) }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub fn uds_key_code7(&self) -> &UDS_KEY_CODE7 {
        unsafe { &*(((self as *const Self) as *const u8).add(1340usize) as *const UDS_KEY_CODE7) }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub fn uds_key_code7_mut(&self) -> &mut UDS_KEY_CODE7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1340usize) as *mut UDS_KEY_CODE7) }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub fn uds_body5(&self) -> &UDS_BODY5 {
        unsafe { &*(((self as *const Self) as *const u8).add(1340usize) as *const UDS_BODY5) }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub fn uds_body5_mut(&self) -> &mut UDS_BODY5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1340usize) as *mut UDS_BODY5) }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub fn uds_key_code8(&self) -> &UDS_KEY_CODE8 {
        unsafe { &*(((self as *const Self) as *const u8).add(1344usize) as *const UDS_KEY_CODE8) }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub fn uds_key_code8_mut(&self) -> &mut UDS_KEY_CODE8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1344usize) as *mut UDS_KEY_CODE8) }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub fn uds_body6(&self) -> &UDS_BODY6 {
        unsafe { &*(((self as *const Self) as *const u8).add(1344usize) as *const UDS_BODY6) }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub fn uds_body6_mut(&self) -> &mut UDS_BODY6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1344usize) as *mut UDS_BODY6) }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub fn uds_key_code9(&self) -> &UDS_KEY_CODE9 {
        unsafe { &*(((self as *const Self) as *const u8).add(1348usize) as *const UDS_KEY_CODE9) }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub fn uds_key_code9_mut(&self) -> &mut UDS_KEY_CODE9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1348usize) as *mut UDS_KEY_CODE9) }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub fn uds_body7(&self) -> &UDS_BODY7 {
        unsafe { &*(((self as *const Self) as *const u8).add(1348usize) as *const UDS_BODY7) }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub fn uds_body7_mut(&self) -> &mut UDS_BODY7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1348usize) as *mut UDS_BODY7) }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub fn uds_key_code10(&self) -> &UDS_KEY_CODE10 {
        unsafe { &*(((self as *const Self) as *const u8).add(1352usize) as *const UDS_KEY_CODE10) }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub fn uds_key_code10_mut(&self) -> &mut UDS_KEY_CODE10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1352usize) as *mut UDS_KEY_CODE10) }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub fn uds_body8(&self) -> &UDS_BODY8 {
        unsafe { &*(((self as *const Self) as *const u8).add(1352usize) as *const UDS_BODY8) }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub fn uds_body8_mut(&self) -> &mut UDS_BODY8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1352usize) as *mut UDS_BODY8) }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub fn uds_key_code11(&self) -> &UDS_KEY_CODE11 {
        unsafe { &*(((self as *const Self) as *const u8).add(1356usize) as *const UDS_KEY_CODE11) }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub fn uds_key_code11_mut(&self) -> &mut UDS_KEY_CODE11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1356usize) as *mut UDS_KEY_CODE11) }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub fn uds_body9(&self) -> &UDS_BODY9 {
        unsafe { &*(((self as *const Self) as *const u8).add(1356usize) as *const UDS_BODY9) }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub fn uds_body9_mut(&self) -> &mut UDS_BODY9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1356usize) as *mut UDS_BODY9) }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub fn uds_key_code12(&self) -> &UDS_KEY_CODE12 {
        unsafe { &*(((self as *const Self) as *const u8).add(1360usize) as *const UDS_KEY_CODE12) }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub fn uds_key_code12_mut(&self) -> &mut UDS_KEY_CODE12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1360usize) as *mut UDS_KEY_CODE12) }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub fn uds_body10(&self) -> &UDS_BODY10 {
        unsafe { &*(((self as *const Self) as *const u8).add(1360usize) as *const UDS_BODY10) }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub fn uds_body10_mut(&self) -> &mut UDS_BODY10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1360usize) as *mut UDS_BODY10) }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub fn uds_key_code13(&self) -> &UDS_KEY_CODE13 {
        unsafe { &*(((self as *const Self) as *const u8).add(1364usize) as *const UDS_KEY_CODE13) }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub fn uds_key_code13_mut(&self) -> &mut UDS_KEY_CODE13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1364usize) as *mut UDS_KEY_CODE13) }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub fn uds_body11(&self) -> &UDS_BODY11 {
        unsafe { &*(((self as *const Self) as *const u8).add(1364usize) as *const UDS_BODY11) }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub fn uds_body11_mut(&self) -> &mut UDS_BODY11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1364usize) as *mut UDS_BODY11) }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code0(&self) -> &PRINCE_REGION0_KEY_CODE0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1368usize)
                as *const PRINCE_REGION0_KEY_CODE0)
        }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code0_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1368usize)
                as *mut PRINCE_REGION0_KEY_CODE0)
        }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub fn prince_region0_header0(&self) -> &PRINCE_REGION0_HEADER0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1368usize) as *const PRINCE_REGION0_HEADER0)
        }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub fn prince_region0_header0_mut(&self) -> &mut PRINCE_REGION0_HEADER0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1368usize) as *mut PRINCE_REGION0_HEADER0)
        }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code1(&self) -> &PRINCE_REGION0_KEY_CODE1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1372usize)
                as *const PRINCE_REGION0_KEY_CODE1)
        }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code1_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1372usize)
                as *mut PRINCE_REGION0_KEY_CODE1)
        }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub fn prince_region0_header1(&self) -> &PRINCE_REGION0_HEADER1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1372usize) as *const PRINCE_REGION0_HEADER1)
        }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub fn prince_region0_header1_mut(&self) -> &mut PRINCE_REGION0_HEADER1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1372usize) as *mut PRINCE_REGION0_HEADER1)
        }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code2(&self) -> &PRINCE_REGION0_KEY_CODE2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const PRINCE_REGION0_KEY_CODE2)
        }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code2_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize)
                as *mut PRINCE_REGION0_KEY_CODE2)
        }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub fn prince_region0_body0(&self) -> &PRINCE_REGION0_BODY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize) as *const PRINCE_REGION0_BODY0)
        }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub fn prince_region0_body0_mut(&self) -> &mut PRINCE_REGION0_BODY0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize) as *mut PRINCE_REGION0_BODY0)
        }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code3(&self) -> &PRINCE_REGION0_KEY_CODE3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1380usize)
                as *const PRINCE_REGION0_KEY_CODE3)
        }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code3_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1380usize)
                as *mut PRINCE_REGION0_KEY_CODE3)
        }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub fn prince_region0_body1(&self) -> &PRINCE_REGION0_BODY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1380usize) as *const PRINCE_REGION0_BODY1)
        }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub fn prince_region0_body1_mut(&self) -> &mut PRINCE_REGION0_BODY1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1380usize) as *mut PRINCE_REGION0_BODY1)
        }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code4(&self) -> &PRINCE_REGION0_KEY_CODE4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1384usize)
                as *const PRINCE_REGION0_KEY_CODE4)
        }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code4_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1384usize)
                as *mut PRINCE_REGION0_KEY_CODE4)
        }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub fn prince_region0_body2(&self) -> &PRINCE_REGION0_BODY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1384usize) as *const PRINCE_REGION0_BODY2)
        }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub fn prince_region0_body2_mut(&self) -> &mut PRINCE_REGION0_BODY2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1384usize) as *mut PRINCE_REGION0_BODY2)
        }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code5(&self) -> &PRINCE_REGION0_KEY_CODE5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1388usize)
                as *const PRINCE_REGION0_KEY_CODE5)
        }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code5_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1388usize)
                as *mut PRINCE_REGION0_KEY_CODE5)
        }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub fn prince_region0_body3(&self) -> &PRINCE_REGION0_BODY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1388usize) as *const PRINCE_REGION0_BODY3)
        }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub fn prince_region0_body3_mut(&self) -> &mut PRINCE_REGION0_BODY3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1388usize) as *mut PRINCE_REGION0_BODY3)
        }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code6(&self) -> &PRINCE_REGION0_KEY_CODE6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1392usize)
                as *const PRINCE_REGION0_KEY_CODE6)
        }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code6_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1392usize)
                as *mut PRINCE_REGION0_KEY_CODE6)
        }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub fn prince_region0_body4(&self) -> &PRINCE_REGION0_BODY4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1392usize) as *const PRINCE_REGION0_BODY4)
        }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub fn prince_region0_body4_mut(&self) -> &mut PRINCE_REGION0_BODY4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1392usize) as *mut PRINCE_REGION0_BODY4)
        }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code7(&self) -> &PRINCE_REGION0_KEY_CODE7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1396usize)
                as *const PRINCE_REGION0_KEY_CODE7)
        }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code7_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1396usize)
                as *mut PRINCE_REGION0_KEY_CODE7)
        }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub fn prince_region0_body5(&self) -> &PRINCE_REGION0_BODY5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1396usize) as *const PRINCE_REGION0_BODY5)
        }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub fn prince_region0_body5_mut(&self) -> &mut PRINCE_REGION0_BODY5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1396usize) as *mut PRINCE_REGION0_BODY5)
        }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code8(&self) -> &PRINCE_REGION0_KEY_CODE8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1400usize)
                as *const PRINCE_REGION0_KEY_CODE8)
        }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code8_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1400usize)
                as *mut PRINCE_REGION0_KEY_CODE8)
        }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub fn prince_region0_body6(&self) -> &PRINCE_REGION0_BODY6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1400usize) as *const PRINCE_REGION0_BODY6)
        }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub fn prince_region0_body6_mut(&self) -> &mut PRINCE_REGION0_BODY6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1400usize) as *mut PRINCE_REGION0_BODY6)
        }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code9(&self) -> &PRINCE_REGION0_KEY_CODE9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1404usize)
                as *const PRINCE_REGION0_KEY_CODE9)
        }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code9_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1404usize)
                as *mut PRINCE_REGION0_KEY_CODE9)
        }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub fn prince_region0_body7(&self) -> &PRINCE_REGION0_BODY7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1404usize) as *const PRINCE_REGION0_BODY7)
        }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub fn prince_region0_body7_mut(&self) -> &mut PRINCE_REGION0_BODY7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1404usize) as *mut PRINCE_REGION0_BODY7)
        }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code10(&self) -> &PRINCE_REGION0_KEY_CODE10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1408usize)
                as *const PRINCE_REGION0_KEY_CODE10)
        }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code10_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1408usize)
                as *mut PRINCE_REGION0_KEY_CODE10)
        }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub fn prince_region0_body8(&self) -> &PRINCE_REGION0_BODY8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1408usize) as *const PRINCE_REGION0_BODY8)
        }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub fn prince_region0_body8_mut(&self) -> &mut PRINCE_REGION0_BODY8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1408usize) as *mut PRINCE_REGION0_BODY8)
        }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code11(&self) -> &PRINCE_REGION0_KEY_CODE11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1412usize)
                as *const PRINCE_REGION0_KEY_CODE11)
        }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code11_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1412usize)
                as *mut PRINCE_REGION0_KEY_CODE11)
        }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub fn prince_region0_body9(&self) -> &PRINCE_REGION0_BODY9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1412usize) as *const PRINCE_REGION0_BODY9)
        }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub fn prince_region0_body9_mut(&self) -> &mut PRINCE_REGION0_BODY9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1412usize) as *mut PRINCE_REGION0_BODY9)
        }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code12(&self) -> &PRINCE_REGION0_KEY_CODE12 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1416usize)
                as *const PRINCE_REGION0_KEY_CODE12)
        }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code12_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE12 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1416usize)
                as *mut PRINCE_REGION0_KEY_CODE12)
        }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub fn prince_region0_body10(&self) -> &PRINCE_REGION0_BODY10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1416usize) as *const PRINCE_REGION0_BODY10)
        }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub fn prince_region0_body10_mut(&self) -> &mut PRINCE_REGION0_BODY10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1416usize) as *mut PRINCE_REGION0_BODY10)
        }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code13(&self) -> &PRINCE_REGION0_KEY_CODE13 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1420usize)
                as *const PRINCE_REGION0_KEY_CODE13)
        }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code13_mut(&self) -> &mut PRINCE_REGION0_KEY_CODE13 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1420usize)
                as *mut PRINCE_REGION0_KEY_CODE13)
        }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub fn prince_region0_body11(&self) -> &PRINCE_REGION0_BODY11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1420usize) as *const PRINCE_REGION0_BODY11)
        }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub fn prince_region0_body11_mut(&self) -> &mut PRINCE_REGION0_BODY11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1420usize) as *mut PRINCE_REGION0_BODY11)
        }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code0(&self) -> &PRINCE_REGION1_KEY_CODE0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const PRINCE_REGION1_KEY_CODE0)
        }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code0_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize)
                as *mut PRINCE_REGION1_KEY_CODE0)
        }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub fn prince_region1_header0(&self) -> &PRINCE_REGION1_HEADER0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize) as *const PRINCE_REGION1_HEADER0)
        }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub fn prince_region1_header0_mut(&self) -> &mut PRINCE_REGION1_HEADER0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize) as *mut PRINCE_REGION1_HEADER0)
        }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code1(&self) -> &PRINCE_REGION1_KEY_CODE1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1428usize)
                as *const PRINCE_REGION1_KEY_CODE1)
        }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code1_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1428usize)
                as *mut PRINCE_REGION1_KEY_CODE1)
        }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub fn prince_region1_header1(&self) -> &PRINCE_REGION1_HEADER1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1428usize) as *const PRINCE_REGION1_HEADER1)
        }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub fn prince_region1_header1_mut(&self) -> &mut PRINCE_REGION1_HEADER1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1428usize) as *mut PRINCE_REGION1_HEADER1)
        }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code2(&self) -> &PRINCE_REGION1_KEY_CODE2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1432usize)
                as *const PRINCE_REGION1_KEY_CODE2)
        }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code2_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1432usize)
                as *mut PRINCE_REGION1_KEY_CODE2)
        }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub fn prince_region1_body0(&self) -> &PRINCE_REGION1_BODY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1432usize) as *const PRINCE_REGION1_BODY0)
        }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub fn prince_region1_body0_mut(&self) -> &mut PRINCE_REGION1_BODY0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1432usize) as *mut PRINCE_REGION1_BODY0)
        }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub fn prince_region1_key_code3(&self) -> &PRINCE_REGION1_KEY_CODE3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1436usize)
                as *const PRINCE_REGION1_KEY_CODE3)
        }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub fn prince_region1_key_code3_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1436usize)
                as *mut PRINCE_REGION1_KEY_CODE3)
        }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub fn prince_region1_body1(&self) -> &PRINCE_REGION1_BODY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1436usize) as *const PRINCE_REGION1_BODY1)
        }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub fn prince_region1_body1_mut(&self) -> &mut PRINCE_REGION1_BODY1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1436usize) as *mut PRINCE_REGION1_BODY1)
        }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code4(&self) -> &PRINCE_REGION1_KEY_CODE4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1440usize)
                as *const PRINCE_REGION1_KEY_CODE4)
        }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code4_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1440usize)
                as *mut PRINCE_REGION1_KEY_CODE4)
        }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub fn prince_region1_body2(&self) -> &PRINCE_REGION1_BODY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1440usize) as *const PRINCE_REGION1_BODY2)
        }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub fn prince_region1_body2_mut(&self) -> &mut PRINCE_REGION1_BODY2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1440usize) as *mut PRINCE_REGION1_BODY2)
        }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code5(&self) -> &PRINCE_REGION1_KEY_CODE5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1444usize)
                as *const PRINCE_REGION1_KEY_CODE5)
        }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code5_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1444usize)
                as *mut PRINCE_REGION1_KEY_CODE5)
        }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub fn prince_region1_body3(&self) -> &PRINCE_REGION1_BODY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1444usize) as *const PRINCE_REGION1_BODY3)
        }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub fn prince_region1_body3_mut(&self) -> &mut PRINCE_REGION1_BODY3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1444usize) as *mut PRINCE_REGION1_BODY3)
        }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code6(&self) -> &PRINCE_REGION1_KEY_CODE6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1448usize)
                as *const PRINCE_REGION1_KEY_CODE6)
        }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code6_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1448usize)
                as *mut PRINCE_REGION1_KEY_CODE6)
        }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub fn prince_region1_body4(&self) -> &PRINCE_REGION1_BODY4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1448usize) as *const PRINCE_REGION1_BODY4)
        }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub fn prince_region1_body4_mut(&self) -> &mut PRINCE_REGION1_BODY4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1448usize) as *mut PRINCE_REGION1_BODY4)
        }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub fn prince_region1_key_code7(&self) -> &PRINCE_REGION1_KEY_CODE7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1452usize)
                as *const PRINCE_REGION1_KEY_CODE7)
        }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub fn prince_region1_key_code7_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1452usize)
                as *mut PRINCE_REGION1_KEY_CODE7)
        }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub fn prince_region1_body5(&self) -> &PRINCE_REGION1_BODY5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1452usize) as *const PRINCE_REGION1_BODY5)
        }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub fn prince_region1_body5_mut(&self) -> &mut PRINCE_REGION1_BODY5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1452usize) as *mut PRINCE_REGION1_BODY5)
        }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code8(&self) -> &PRINCE_REGION1_KEY_CODE8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1456usize)
                as *const PRINCE_REGION1_KEY_CODE8)
        }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code8_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1456usize)
                as *mut PRINCE_REGION1_KEY_CODE8)
        }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub fn prince_region1_body6(&self) -> &PRINCE_REGION1_BODY6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1456usize) as *const PRINCE_REGION1_BODY6)
        }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub fn prince_region1_body6_mut(&self) -> &mut PRINCE_REGION1_BODY6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1456usize) as *mut PRINCE_REGION1_BODY6)
        }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code9(&self) -> &PRINCE_REGION1_KEY_CODE9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1460usize)
                as *const PRINCE_REGION1_KEY_CODE9)
        }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code9_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1460usize)
                as *mut PRINCE_REGION1_KEY_CODE9)
        }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub fn prince_region1_body7(&self) -> &PRINCE_REGION1_BODY7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1460usize) as *const PRINCE_REGION1_BODY7)
        }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub fn prince_region1_body7_mut(&self) -> &mut PRINCE_REGION1_BODY7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1460usize) as *mut PRINCE_REGION1_BODY7)
        }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code10(&self) -> &PRINCE_REGION1_KEY_CODE10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1464usize)
                as *const PRINCE_REGION1_KEY_CODE10)
        }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code10_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1464usize)
                as *mut PRINCE_REGION1_KEY_CODE10)
        }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub fn prince_region1_body8(&self) -> &PRINCE_REGION1_BODY8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1464usize) as *const PRINCE_REGION1_BODY8)
        }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub fn prince_region1_body8_mut(&self) -> &mut PRINCE_REGION1_BODY8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1464usize) as *mut PRINCE_REGION1_BODY8)
        }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub fn prince_region1_key_code11(&self) -> &PRINCE_REGION1_KEY_CODE11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1468usize)
                as *const PRINCE_REGION1_KEY_CODE11)
        }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub fn prince_region1_key_code11_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1468usize)
                as *mut PRINCE_REGION1_KEY_CODE11)
        }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub fn prince_region1_body9(&self) -> &PRINCE_REGION1_BODY9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1468usize) as *const PRINCE_REGION1_BODY9)
        }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub fn prince_region1_body9_mut(&self) -> &mut PRINCE_REGION1_BODY9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1468usize) as *mut PRINCE_REGION1_BODY9)
        }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code12(&self) -> &PRINCE_REGION1_KEY_CODE12 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const PRINCE_REGION1_KEY_CODE12)
        }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code12_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE12 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize)
                as *mut PRINCE_REGION1_KEY_CODE12)
        }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub fn prince_region1_body10(&self) -> &PRINCE_REGION1_BODY10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize) as *const PRINCE_REGION1_BODY10)
        }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub fn prince_region1_body10_mut(&self) -> &mut PRINCE_REGION1_BODY10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize) as *mut PRINCE_REGION1_BODY10)
        }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code13(&self) -> &PRINCE_REGION1_KEY_CODE13 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1476usize)
                as *const PRINCE_REGION1_KEY_CODE13)
        }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code13_mut(&self) -> &mut PRINCE_REGION1_KEY_CODE13 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1476usize)
                as *mut PRINCE_REGION1_KEY_CODE13)
        }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub fn prince_region1_body11(&self) -> &PRINCE_REGION1_BODY11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1476usize) as *const PRINCE_REGION1_BODY11)
        }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub fn prince_region1_body11_mut(&self) -> &mut PRINCE_REGION1_BODY11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1476usize) as *mut PRINCE_REGION1_BODY11)
        }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code0(&self) -> &PRINCE_REGION2_KEY_CODE0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1480usize)
                as *const PRINCE_REGION2_KEY_CODE0)
        }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code0_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1480usize)
                as *mut PRINCE_REGION2_KEY_CODE0)
        }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub fn prince_region2_header0(&self) -> &PRINCE_REGION2_HEADER0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1480usize) as *const PRINCE_REGION2_HEADER0)
        }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub fn prince_region2_header0_mut(&self) -> &mut PRINCE_REGION2_HEADER0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1480usize) as *mut PRINCE_REGION2_HEADER0)
        }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub fn prince_region2_key_code1(&self) -> &PRINCE_REGION2_KEY_CODE1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1484usize)
                as *const PRINCE_REGION2_KEY_CODE1)
        }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub fn prince_region2_key_code1_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1484usize)
                as *mut PRINCE_REGION2_KEY_CODE1)
        }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub fn prince_region2_header1(&self) -> &PRINCE_REGION2_HEADER1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1484usize) as *const PRINCE_REGION2_HEADER1)
        }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub fn prince_region2_header1_mut(&self) -> &mut PRINCE_REGION2_HEADER1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1484usize) as *mut PRINCE_REGION2_HEADER1)
        }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code2(&self) -> &PRINCE_REGION2_KEY_CODE2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1488usize)
                as *const PRINCE_REGION2_KEY_CODE2)
        }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code2_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1488usize)
                as *mut PRINCE_REGION2_KEY_CODE2)
        }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub fn prince_region2_body0(&self) -> &PRINCE_REGION2_BODY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1488usize) as *const PRINCE_REGION2_BODY0)
        }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub fn prince_region2_body0_mut(&self) -> &mut PRINCE_REGION2_BODY0 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1488usize) as *mut PRINCE_REGION2_BODY0)
        }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code3(&self) -> &PRINCE_REGION2_KEY_CODE3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1492usize)
                as *const PRINCE_REGION2_KEY_CODE3)
        }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code3_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1492usize)
                as *mut PRINCE_REGION2_KEY_CODE3)
        }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub fn prince_region2_body1(&self) -> &PRINCE_REGION2_BODY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1492usize) as *const PRINCE_REGION2_BODY1)
        }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub fn prince_region2_body1_mut(&self) -> &mut PRINCE_REGION2_BODY1 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1492usize) as *mut PRINCE_REGION2_BODY1)
        }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code4(&self) -> &PRINCE_REGION2_KEY_CODE4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1496usize)
                as *const PRINCE_REGION2_KEY_CODE4)
        }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code4_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1496usize)
                as *mut PRINCE_REGION2_KEY_CODE4)
        }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub fn prince_region2_body2(&self) -> &PRINCE_REGION2_BODY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1496usize) as *const PRINCE_REGION2_BODY2)
        }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub fn prince_region2_body2_mut(&self) -> &mut PRINCE_REGION2_BODY2 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1496usize) as *mut PRINCE_REGION2_BODY2)
        }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub fn prince_region2_key_code5(&self) -> &PRINCE_REGION2_KEY_CODE5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1500usize)
                as *const PRINCE_REGION2_KEY_CODE5)
        }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub fn prince_region2_key_code5_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1500usize)
                as *mut PRINCE_REGION2_KEY_CODE5)
        }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub fn prince_region2_body3(&self) -> &PRINCE_REGION2_BODY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1500usize) as *const PRINCE_REGION2_BODY3)
        }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub fn prince_region2_body3_mut(&self) -> &mut PRINCE_REGION2_BODY3 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1500usize) as *mut PRINCE_REGION2_BODY3)
        }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code6(&self) -> &PRINCE_REGION2_KEY_CODE6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1504usize)
                as *const PRINCE_REGION2_KEY_CODE6)
        }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code6_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1504usize)
                as *mut PRINCE_REGION2_KEY_CODE6)
        }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub fn prince_region2_body4(&self) -> &PRINCE_REGION2_BODY4 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1504usize) as *const PRINCE_REGION2_BODY4)
        }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub fn prince_region2_body4_mut(&self) -> &mut PRINCE_REGION2_BODY4 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1504usize) as *mut PRINCE_REGION2_BODY4)
        }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code7(&self) -> &PRINCE_REGION2_KEY_CODE7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1508usize)
                as *const PRINCE_REGION2_KEY_CODE7)
        }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code7_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1508usize)
                as *mut PRINCE_REGION2_KEY_CODE7)
        }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub fn prince_region2_body5(&self) -> &PRINCE_REGION2_BODY5 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1508usize) as *const PRINCE_REGION2_BODY5)
        }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub fn prince_region2_body5_mut(&self) -> &mut PRINCE_REGION2_BODY5 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1508usize) as *mut PRINCE_REGION2_BODY5)
        }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code8(&self) -> &PRINCE_REGION2_KEY_CODE8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1512usize)
                as *const PRINCE_REGION2_KEY_CODE8)
        }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code8_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1512usize)
                as *mut PRINCE_REGION2_KEY_CODE8)
        }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub fn prince_region2_body6(&self) -> &PRINCE_REGION2_BODY6 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1512usize) as *const PRINCE_REGION2_BODY6)
        }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub fn prince_region2_body6_mut(&self) -> &mut PRINCE_REGION2_BODY6 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1512usize) as *mut PRINCE_REGION2_BODY6)
        }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub fn prince_region2_key_code9(&self) -> &PRINCE_REGION2_KEY_CODE9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1516usize)
                as *const PRINCE_REGION2_KEY_CODE9)
        }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub fn prince_region2_key_code9_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1516usize)
                as *mut PRINCE_REGION2_KEY_CODE9)
        }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub fn prince_region2_body7(&self) -> &PRINCE_REGION2_BODY7 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1516usize) as *const PRINCE_REGION2_BODY7)
        }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub fn prince_region2_body7_mut(&self) -> &mut PRINCE_REGION2_BODY7 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1516usize) as *mut PRINCE_REGION2_BODY7)
        }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code10(&self) -> &PRINCE_REGION2_KEY_CODE10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const PRINCE_REGION2_KEY_CODE10)
        }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code10_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize)
                as *mut PRINCE_REGION2_KEY_CODE10)
        }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub fn prince_region2_body8(&self) -> &PRINCE_REGION2_BODY8 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize) as *const PRINCE_REGION2_BODY8)
        }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub fn prince_region2_body8_mut(&self) -> &mut PRINCE_REGION2_BODY8 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize) as *mut PRINCE_REGION2_BODY8)
        }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code11(&self) -> &PRINCE_REGION2_KEY_CODE11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1524usize)
                as *const PRINCE_REGION2_KEY_CODE11)
        }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code11_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1524usize)
                as *mut PRINCE_REGION2_KEY_CODE11)
        }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub fn prince_region2_body9(&self) -> &PRINCE_REGION2_BODY9 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1524usize) as *const PRINCE_REGION2_BODY9)
        }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub fn prince_region2_body9_mut(&self) -> &mut PRINCE_REGION2_BODY9 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1524usize) as *mut PRINCE_REGION2_BODY9)
        }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code12(&self) -> &PRINCE_REGION2_KEY_CODE12 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1528usize)
                as *const PRINCE_REGION2_KEY_CODE12)
        }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code12_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE12 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1528usize)
                as *mut PRINCE_REGION2_KEY_CODE12)
        }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub fn prince_region2_body10(&self) -> &PRINCE_REGION2_BODY10 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1528usize) as *const PRINCE_REGION2_BODY10)
        }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub fn prince_region2_body10_mut(&self) -> &mut PRINCE_REGION2_BODY10 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1528usize) as *mut PRINCE_REGION2_BODY10)
        }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub fn prince_region2_key_code13(&self) -> &PRINCE_REGION2_KEY_CODE13 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1532usize)
                as *const PRINCE_REGION2_KEY_CODE13)
        }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub fn prince_region2_key_code13_mut(&self) -> &mut PRINCE_REGION2_KEY_CODE13 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1532usize)
                as *mut PRINCE_REGION2_KEY_CODE13)
        }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub fn prince_region2_body11(&self) -> &PRINCE_REGION2_BODY11 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1532usize) as *const PRINCE_REGION2_BODY11)
        }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub fn prince_region2_body11_mut(&self) -> &mut PRINCE_REGION2_BODY11 {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1532usize) as *mut PRINCE_REGION2_BODY11)
        }
    }
}
#[doc = "Valid Key Sore Header : 0x95959595\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [header](header) module"]
pub type HEADER = crate::Reg<u32, _HEADER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HEADER;
#[doc = "`read()` method returns [header::R](header::R) reader structure"]
impl crate::Readable for HEADER {}
#[doc = "`write(|w| ..)` method takes [header::W](header::W) writer structure"]
impl crate::Writable for HEADER {}
#[doc = "Valid Key Sore Header : 0x95959595"]
pub mod header;
#[doc = "puf discharge time in ms.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [puf_discharge_time_in_ms](puf_discharge_time_in_ms) module"]
pub type PUF_DISCHARGE_TIME_IN_MS = crate::Reg<u32, _PUF_DISCHARGE_TIME_IN_MS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUF_DISCHARGE_TIME_IN_MS;
#[doc = "`read()` method returns [puf_discharge_time_in_ms::R](puf_discharge_time_in_ms::R) reader structure"]
impl crate::Readable for PUF_DISCHARGE_TIME_IN_MS {}
#[doc = "`write(|w| ..)` method takes [puf_discharge_time_in_ms::W](puf_discharge_time_in_ms::W) writer structure"]
impl crate::Writable for PUF_DISCHARGE_TIME_IN_MS {}
#[doc = "puf discharge time in ms."]
pub mod puf_discharge_time_in_ms;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [activation_code](activation_code) module"]
pub type ACTIVATION_CODE = crate::Reg<u32, _ACTIVATION_CODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTIVATION_CODE;
#[doc = "`read()` method returns [activation_code::R](activation_code::R) reader structure"]
impl crate::Readable for ACTIVATION_CODE {}
#[doc = "`write(|w| ..)` method takes [activation_code::W](activation_code::W) writer structure"]
impl crate::Writable for ACTIVATION_CODE {}
#[doc = "."]
pub mod activation_code;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_header0](sbkey_header0) module"]
pub type SBKEY_HEADER0 = crate::Reg<u32, _SBKEY_HEADER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_HEADER0;
#[doc = "`read()` method returns [sbkey_header0::R](sbkey_header0::R) reader structure"]
impl crate::Readable for SBKEY_HEADER0 {}
#[doc = "`write(|w| ..)` method takes [sbkey_header0::W](sbkey_header0::W) writer structure"]
impl crate::Writable for SBKEY_HEADER0 {}
#[doc = "."]
pub mod sbkey_header0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code0](sbkey_key_code0) module"]
pub type SBKEY_KEY_CODE0 = crate::Reg<u32, _SBKEY_KEY_CODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE0;
#[doc = "`read()` method returns [sbkey_key_code0::R](sbkey_key_code0::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE0 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code0::W](sbkey_key_code0::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE0 {}
#[doc = "."]
pub mod sbkey_key_code0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_header1](sbkey_header1) module"]
pub type SBKEY_HEADER1 = crate::Reg<u32, _SBKEY_HEADER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_HEADER1;
#[doc = "`read()` method returns [sbkey_header1::R](sbkey_header1::R) reader structure"]
impl crate::Readable for SBKEY_HEADER1 {}
#[doc = "`write(|w| ..)` method takes [sbkey_header1::W](sbkey_header1::W) writer structure"]
impl crate::Writable for SBKEY_HEADER1 {}
#[doc = "."]
pub mod sbkey_header1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code1](sbkey_key_code1) module"]
pub type SBKEY_KEY_CODE1 = crate::Reg<u32, _SBKEY_KEY_CODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE1;
#[doc = "`read()` method returns [sbkey_key_code1::R](sbkey_key_code1::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE1 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code1::W](sbkey_key_code1::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE1 {}
#[doc = "."]
pub mod sbkey_key_code1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body0](sbkey_body0) module"]
pub type SBKEY_BODY0 = crate::Reg<u32, _SBKEY_BODY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY0;
#[doc = "`read()` method returns [sbkey_body0::R](sbkey_body0::R) reader structure"]
impl crate::Readable for SBKEY_BODY0 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body0::W](sbkey_body0::W) writer structure"]
impl crate::Writable for SBKEY_BODY0 {}
#[doc = "."]
pub mod sbkey_body0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code2](sbkey_key_code2) module"]
pub type SBKEY_KEY_CODE2 = crate::Reg<u32, _SBKEY_KEY_CODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE2;
#[doc = "`read()` method returns [sbkey_key_code2::R](sbkey_key_code2::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE2 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code2::W](sbkey_key_code2::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE2 {}
#[doc = "."]
pub mod sbkey_key_code2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body1](sbkey_body1) module"]
pub type SBKEY_BODY1 = crate::Reg<u32, _SBKEY_BODY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY1;
#[doc = "`read()` method returns [sbkey_body1::R](sbkey_body1::R) reader structure"]
impl crate::Readable for SBKEY_BODY1 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body1::W](sbkey_body1::W) writer structure"]
impl crate::Writable for SBKEY_BODY1 {}
#[doc = "."]
pub mod sbkey_body1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code3](sbkey_key_code3) module"]
pub type SBKEY_KEY_CODE3 = crate::Reg<u32, _SBKEY_KEY_CODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE3;
#[doc = "`read()` method returns [sbkey_key_code3::R](sbkey_key_code3::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE3 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code3::W](sbkey_key_code3::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE3 {}
#[doc = "."]
pub mod sbkey_key_code3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body2](sbkey_body2) module"]
pub type SBKEY_BODY2 = crate::Reg<u32, _SBKEY_BODY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY2;
#[doc = "`read()` method returns [sbkey_body2::R](sbkey_body2::R) reader structure"]
impl crate::Readable for SBKEY_BODY2 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body2::W](sbkey_body2::W) writer structure"]
impl crate::Writable for SBKEY_BODY2 {}
#[doc = "."]
pub mod sbkey_body2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code4](sbkey_key_code4) module"]
pub type SBKEY_KEY_CODE4 = crate::Reg<u32, _SBKEY_KEY_CODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE4;
#[doc = "`read()` method returns [sbkey_key_code4::R](sbkey_key_code4::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE4 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code4::W](sbkey_key_code4::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE4 {}
#[doc = "."]
pub mod sbkey_key_code4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body3](sbkey_body3) module"]
pub type SBKEY_BODY3 = crate::Reg<u32, _SBKEY_BODY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY3;
#[doc = "`read()` method returns [sbkey_body3::R](sbkey_body3::R) reader structure"]
impl crate::Readable for SBKEY_BODY3 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body3::W](sbkey_body3::W) writer structure"]
impl crate::Writable for SBKEY_BODY3 {}
#[doc = "."]
pub mod sbkey_body3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code5](sbkey_key_code5) module"]
pub type SBKEY_KEY_CODE5 = crate::Reg<u32, _SBKEY_KEY_CODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE5;
#[doc = "`read()` method returns [sbkey_key_code5::R](sbkey_key_code5::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE5 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code5::W](sbkey_key_code5::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE5 {}
#[doc = "."]
pub mod sbkey_key_code5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body4](sbkey_body4) module"]
pub type SBKEY_BODY4 = crate::Reg<u32, _SBKEY_BODY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY4;
#[doc = "`read()` method returns [sbkey_body4::R](sbkey_body4::R) reader structure"]
impl crate::Readable for SBKEY_BODY4 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body4::W](sbkey_body4::W) writer structure"]
impl crate::Writable for SBKEY_BODY4 {}
#[doc = "."]
pub mod sbkey_body4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code6](sbkey_key_code6) module"]
pub type SBKEY_KEY_CODE6 = crate::Reg<u32, _SBKEY_KEY_CODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE6;
#[doc = "`read()` method returns [sbkey_key_code6::R](sbkey_key_code6::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE6 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code6::W](sbkey_key_code6::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE6 {}
#[doc = "."]
pub mod sbkey_key_code6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body5](sbkey_body5) module"]
pub type SBKEY_BODY5 = crate::Reg<u32, _SBKEY_BODY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY5;
#[doc = "`read()` method returns [sbkey_body5::R](sbkey_body5::R) reader structure"]
impl crate::Readable for SBKEY_BODY5 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body5::W](sbkey_body5::W) writer structure"]
impl crate::Writable for SBKEY_BODY5 {}
#[doc = "."]
pub mod sbkey_body5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code7](sbkey_key_code7) module"]
pub type SBKEY_KEY_CODE7 = crate::Reg<u32, _SBKEY_KEY_CODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE7;
#[doc = "`read()` method returns [sbkey_key_code7::R](sbkey_key_code7::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE7 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code7::W](sbkey_key_code7::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE7 {}
#[doc = "."]
pub mod sbkey_key_code7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body6](sbkey_body6) module"]
pub type SBKEY_BODY6 = crate::Reg<u32, _SBKEY_BODY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY6;
#[doc = "`read()` method returns [sbkey_body6::R](sbkey_body6::R) reader structure"]
impl crate::Readable for SBKEY_BODY6 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body6::W](sbkey_body6::W) writer structure"]
impl crate::Writable for SBKEY_BODY6 {}
#[doc = "."]
pub mod sbkey_body6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code8](sbkey_key_code8) module"]
pub type SBKEY_KEY_CODE8 = crate::Reg<u32, _SBKEY_KEY_CODE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE8;
#[doc = "`read()` method returns [sbkey_key_code8::R](sbkey_key_code8::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE8 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code8::W](sbkey_key_code8::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE8 {}
#[doc = "."]
pub mod sbkey_key_code8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body7](sbkey_body7) module"]
pub type SBKEY_BODY7 = crate::Reg<u32, _SBKEY_BODY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY7;
#[doc = "`read()` method returns [sbkey_body7::R](sbkey_body7::R) reader structure"]
impl crate::Readable for SBKEY_BODY7 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body7::W](sbkey_body7::W) writer structure"]
impl crate::Writable for SBKEY_BODY7 {}
#[doc = "."]
pub mod sbkey_body7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code9](sbkey_key_code9) module"]
pub type SBKEY_KEY_CODE9 = crate::Reg<u32, _SBKEY_KEY_CODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE9;
#[doc = "`read()` method returns [sbkey_key_code9::R](sbkey_key_code9::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE9 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code9::W](sbkey_key_code9::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE9 {}
#[doc = "."]
pub mod sbkey_key_code9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body8](sbkey_body8) module"]
pub type SBKEY_BODY8 = crate::Reg<u32, _SBKEY_BODY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY8;
#[doc = "`read()` method returns [sbkey_body8::R](sbkey_body8::R) reader structure"]
impl crate::Readable for SBKEY_BODY8 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body8::W](sbkey_body8::W) writer structure"]
impl crate::Writable for SBKEY_BODY8 {}
#[doc = "."]
pub mod sbkey_body8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code10](sbkey_key_code10) module"]
pub type SBKEY_KEY_CODE10 = crate::Reg<u32, _SBKEY_KEY_CODE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE10;
#[doc = "`read()` method returns [sbkey_key_code10::R](sbkey_key_code10::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE10 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code10::W](sbkey_key_code10::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE10 {}
#[doc = "."]
pub mod sbkey_key_code10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body9](sbkey_body9) module"]
pub type SBKEY_BODY9 = crate::Reg<u32, _SBKEY_BODY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY9;
#[doc = "`read()` method returns [sbkey_body9::R](sbkey_body9::R) reader structure"]
impl crate::Readable for SBKEY_BODY9 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body9::W](sbkey_body9::W) writer structure"]
impl crate::Writable for SBKEY_BODY9 {}
#[doc = "."]
pub mod sbkey_body9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code11](sbkey_key_code11) module"]
pub type SBKEY_KEY_CODE11 = crate::Reg<u32, _SBKEY_KEY_CODE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE11;
#[doc = "`read()` method returns [sbkey_key_code11::R](sbkey_key_code11::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE11 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code11::W](sbkey_key_code11::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE11 {}
#[doc = "."]
pub mod sbkey_key_code11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body10](sbkey_body10) module"]
pub type SBKEY_BODY10 = crate::Reg<u32, _SBKEY_BODY10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY10;
#[doc = "`read()` method returns [sbkey_body10::R](sbkey_body10::R) reader structure"]
impl crate::Readable for SBKEY_BODY10 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body10::W](sbkey_body10::W) writer structure"]
impl crate::Writable for SBKEY_BODY10 {}
#[doc = "."]
pub mod sbkey_body10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code12](sbkey_key_code12) module"]
pub type SBKEY_KEY_CODE12 = crate::Reg<u32, _SBKEY_KEY_CODE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE12;
#[doc = "`read()` method returns [sbkey_key_code12::R](sbkey_key_code12::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE12 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code12::W](sbkey_key_code12::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE12 {}
#[doc = "."]
pub mod sbkey_key_code12;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_body11](sbkey_body11) module"]
pub type SBKEY_BODY11 = crate::Reg<u32, _SBKEY_BODY11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_BODY11;
#[doc = "`read()` method returns [sbkey_body11::R](sbkey_body11::R) reader structure"]
impl crate::Readable for SBKEY_BODY11 {}
#[doc = "`write(|w| ..)` method takes [sbkey_body11::W](sbkey_body11::W) writer structure"]
impl crate::Writable for SBKEY_BODY11 {}
#[doc = "."]
pub mod sbkey_body11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbkey_key_code13](sbkey_key_code13) module"]
pub type SBKEY_KEY_CODE13 = crate::Reg<u32, _SBKEY_KEY_CODE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBKEY_KEY_CODE13;
#[doc = "`read()` method returns [sbkey_key_code13::R](sbkey_key_code13::R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE13 {}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code13::W](sbkey_key_code13::W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE13 {}
#[doc = "."]
pub mod sbkey_key_code13;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_header0](user_kek_header0) module"]
pub type USER_KEK_HEADER0 = crate::Reg<u32, _USER_KEK_HEADER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_HEADER0;
#[doc = "`read()` method returns [user_kek_header0::R](user_kek_header0::R) reader structure"]
impl crate::Readable for USER_KEK_HEADER0 {}
#[doc = "`write(|w| ..)` method takes [user_kek_header0::W](user_kek_header0::W) writer structure"]
impl crate::Writable for USER_KEK_HEADER0 {}
#[doc = "."]
pub mod user_kek_header0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code0](user_kek_key_code0) module"]
pub type USER_KEK_KEY_CODE0 = crate::Reg<u32, _USER_KEK_KEY_CODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE0;
#[doc = "`read()` method returns [user_kek_key_code0::R](user_kek_key_code0::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE0 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code0::W](user_kek_key_code0::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE0 {}
#[doc = "."]
pub mod user_kek_key_code0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_header1](user_kek_header1) module"]
pub type USER_KEK_HEADER1 = crate::Reg<u32, _USER_KEK_HEADER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_HEADER1;
#[doc = "`read()` method returns [user_kek_header1::R](user_kek_header1::R) reader structure"]
impl crate::Readable for USER_KEK_HEADER1 {}
#[doc = "`write(|w| ..)` method takes [user_kek_header1::W](user_kek_header1::W) writer structure"]
impl crate::Writable for USER_KEK_HEADER1 {}
#[doc = "."]
pub mod user_kek_header1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code1](user_kek_key_code1) module"]
pub type USER_KEK_KEY_CODE1 = crate::Reg<u32, _USER_KEK_KEY_CODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE1;
#[doc = "`read()` method returns [user_kek_key_code1::R](user_kek_key_code1::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE1 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code1::W](user_kek_key_code1::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE1 {}
#[doc = "."]
pub mod user_kek_key_code1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body0](user_kek_body0) module"]
pub type USER_KEK_BODY0 = crate::Reg<u32, _USER_KEK_BODY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY0;
#[doc = "`read()` method returns [user_kek_body0::R](user_kek_body0::R) reader structure"]
impl crate::Readable for USER_KEK_BODY0 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body0::W](user_kek_body0::W) writer structure"]
impl crate::Writable for USER_KEK_BODY0 {}
#[doc = "."]
pub mod user_kek_body0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code2](user_kek_key_code2) module"]
pub type USER_KEK_KEY_CODE2 = crate::Reg<u32, _USER_KEK_KEY_CODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE2;
#[doc = "`read()` method returns [user_kek_key_code2::R](user_kek_key_code2::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE2 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code2::W](user_kek_key_code2::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE2 {}
#[doc = "."]
pub mod user_kek_key_code2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body1](user_kek_body1) module"]
pub type USER_KEK_BODY1 = crate::Reg<u32, _USER_KEK_BODY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY1;
#[doc = "`read()` method returns [user_kek_body1::R](user_kek_body1::R) reader structure"]
impl crate::Readable for USER_KEK_BODY1 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body1::W](user_kek_body1::W) writer structure"]
impl crate::Writable for USER_KEK_BODY1 {}
#[doc = "."]
pub mod user_kek_body1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code3](user_kek_key_code3) module"]
pub type USER_KEK_KEY_CODE3 = crate::Reg<u32, _USER_KEK_KEY_CODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE3;
#[doc = "`read()` method returns [user_kek_key_code3::R](user_kek_key_code3::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE3 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code3::W](user_kek_key_code3::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE3 {}
#[doc = "."]
pub mod user_kek_key_code3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body2](user_kek_body2) module"]
pub type USER_KEK_BODY2 = crate::Reg<u32, _USER_KEK_BODY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY2;
#[doc = "`read()` method returns [user_kek_body2::R](user_kek_body2::R) reader structure"]
impl crate::Readable for USER_KEK_BODY2 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body2::W](user_kek_body2::W) writer structure"]
impl crate::Writable for USER_KEK_BODY2 {}
#[doc = "."]
pub mod user_kek_body2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code4](user_kek_key_code4) module"]
pub type USER_KEK_KEY_CODE4 = crate::Reg<u32, _USER_KEK_KEY_CODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE4;
#[doc = "`read()` method returns [user_kek_key_code4::R](user_kek_key_code4::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE4 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code4::W](user_kek_key_code4::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE4 {}
#[doc = "."]
pub mod user_kek_key_code4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body3](user_kek_body3) module"]
pub type USER_KEK_BODY3 = crate::Reg<u32, _USER_KEK_BODY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY3;
#[doc = "`read()` method returns [user_kek_body3::R](user_kek_body3::R) reader structure"]
impl crate::Readable for USER_KEK_BODY3 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body3::W](user_kek_body3::W) writer structure"]
impl crate::Writable for USER_KEK_BODY3 {}
#[doc = "."]
pub mod user_kek_body3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code5](user_kek_key_code5) module"]
pub type USER_KEK_KEY_CODE5 = crate::Reg<u32, _USER_KEK_KEY_CODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE5;
#[doc = "`read()` method returns [user_kek_key_code5::R](user_kek_key_code5::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE5 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code5::W](user_kek_key_code5::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE5 {}
#[doc = "."]
pub mod user_kek_key_code5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body4](user_kek_body4) module"]
pub type USER_KEK_BODY4 = crate::Reg<u32, _USER_KEK_BODY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY4;
#[doc = "`read()` method returns [user_kek_body4::R](user_kek_body4::R) reader structure"]
impl crate::Readable for USER_KEK_BODY4 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body4::W](user_kek_body4::W) writer structure"]
impl crate::Writable for USER_KEK_BODY4 {}
#[doc = "."]
pub mod user_kek_body4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code6](user_kek_key_code6) module"]
pub type USER_KEK_KEY_CODE6 = crate::Reg<u32, _USER_KEK_KEY_CODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE6;
#[doc = "`read()` method returns [user_kek_key_code6::R](user_kek_key_code6::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE6 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code6::W](user_kek_key_code6::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE6 {}
#[doc = "."]
pub mod user_kek_key_code6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body5](user_kek_body5) module"]
pub type USER_KEK_BODY5 = crate::Reg<u32, _USER_KEK_BODY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY5;
#[doc = "`read()` method returns [user_kek_body5::R](user_kek_body5::R) reader structure"]
impl crate::Readable for USER_KEK_BODY5 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body5::W](user_kek_body5::W) writer structure"]
impl crate::Writable for USER_KEK_BODY5 {}
#[doc = "."]
pub mod user_kek_body5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code7](user_kek_key_code7) module"]
pub type USER_KEK_KEY_CODE7 = crate::Reg<u32, _USER_KEK_KEY_CODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE7;
#[doc = "`read()` method returns [user_kek_key_code7::R](user_kek_key_code7::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE7 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code7::W](user_kek_key_code7::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE7 {}
#[doc = "."]
pub mod user_kek_key_code7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body6](user_kek_body6) module"]
pub type USER_KEK_BODY6 = crate::Reg<u32, _USER_KEK_BODY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY6;
#[doc = "`read()` method returns [user_kek_body6::R](user_kek_body6::R) reader structure"]
impl crate::Readable for USER_KEK_BODY6 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body6::W](user_kek_body6::W) writer structure"]
impl crate::Writable for USER_KEK_BODY6 {}
#[doc = "."]
pub mod user_kek_body6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code8](user_kek_key_code8) module"]
pub type USER_KEK_KEY_CODE8 = crate::Reg<u32, _USER_KEK_KEY_CODE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE8;
#[doc = "`read()` method returns [user_kek_key_code8::R](user_kek_key_code8::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE8 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code8::W](user_kek_key_code8::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE8 {}
#[doc = "."]
pub mod user_kek_key_code8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body7](user_kek_body7) module"]
pub type USER_KEK_BODY7 = crate::Reg<u32, _USER_KEK_BODY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY7;
#[doc = "`read()` method returns [user_kek_body7::R](user_kek_body7::R) reader structure"]
impl crate::Readable for USER_KEK_BODY7 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body7::W](user_kek_body7::W) writer structure"]
impl crate::Writable for USER_KEK_BODY7 {}
#[doc = "."]
pub mod user_kek_body7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code9](user_kek_key_code9) module"]
pub type USER_KEK_KEY_CODE9 = crate::Reg<u32, _USER_KEK_KEY_CODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE9;
#[doc = "`read()` method returns [user_kek_key_code9::R](user_kek_key_code9::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE9 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code9::W](user_kek_key_code9::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE9 {}
#[doc = "."]
pub mod user_kek_key_code9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body8](user_kek_body8) module"]
pub type USER_KEK_BODY8 = crate::Reg<u32, _USER_KEK_BODY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY8;
#[doc = "`read()` method returns [user_kek_body8::R](user_kek_body8::R) reader structure"]
impl crate::Readable for USER_KEK_BODY8 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body8::W](user_kek_body8::W) writer structure"]
impl crate::Writable for USER_KEK_BODY8 {}
#[doc = "."]
pub mod user_kek_body8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code10](user_kek_key_code10) module"]
pub type USER_KEK_KEY_CODE10 = crate::Reg<u32, _USER_KEK_KEY_CODE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE10;
#[doc = "`read()` method returns [user_kek_key_code10::R](user_kek_key_code10::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE10 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code10::W](user_kek_key_code10::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE10 {}
#[doc = "."]
pub mod user_kek_key_code10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body9](user_kek_body9) module"]
pub type USER_KEK_BODY9 = crate::Reg<u32, _USER_KEK_BODY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY9;
#[doc = "`read()` method returns [user_kek_body9::R](user_kek_body9::R) reader structure"]
impl crate::Readable for USER_KEK_BODY9 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body9::W](user_kek_body9::W) writer structure"]
impl crate::Writable for USER_KEK_BODY9 {}
#[doc = "."]
pub mod user_kek_body9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code11](user_kek_key_code11) module"]
pub type USER_KEK_KEY_CODE11 = crate::Reg<u32, _USER_KEK_KEY_CODE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE11;
#[doc = "`read()` method returns [user_kek_key_code11::R](user_kek_key_code11::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE11 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code11::W](user_kek_key_code11::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE11 {}
#[doc = "."]
pub mod user_kek_key_code11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body10](user_kek_body10) module"]
pub type USER_KEK_BODY10 = crate::Reg<u32, _USER_KEK_BODY10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY10;
#[doc = "`read()` method returns [user_kek_body10::R](user_kek_body10::R) reader structure"]
impl crate::Readable for USER_KEK_BODY10 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body10::W](user_kek_body10::W) writer structure"]
impl crate::Writable for USER_KEK_BODY10 {}
#[doc = "."]
pub mod user_kek_body10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code12](user_kek_key_code12) module"]
pub type USER_KEK_KEY_CODE12 = crate::Reg<u32, _USER_KEK_KEY_CODE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE12;
#[doc = "`read()` method returns [user_kek_key_code12::R](user_kek_key_code12::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE12 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code12::W](user_kek_key_code12::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE12 {}
#[doc = "."]
pub mod user_kek_key_code12;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_body11](user_kek_body11) module"]
pub type USER_KEK_BODY11 = crate::Reg<u32, _USER_KEK_BODY11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_BODY11;
#[doc = "`read()` method returns [user_kek_body11::R](user_kek_body11::R) reader structure"]
impl crate::Readable for USER_KEK_BODY11 {}
#[doc = "`write(|w| ..)` method takes [user_kek_body11::W](user_kek_body11::W) writer structure"]
impl crate::Writable for USER_KEK_BODY11 {}
#[doc = "."]
pub mod user_kek_body11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [user_kek_key_code13](user_kek_key_code13) module"]
pub type USER_KEK_KEY_CODE13 = crate::Reg<u32, _USER_KEK_KEY_CODE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_KEK_KEY_CODE13;
#[doc = "`read()` method returns [user_kek_key_code13::R](user_kek_key_code13::R) reader structure"]
impl crate::Readable for USER_KEK_KEY_CODE13 {}
#[doc = "`write(|w| ..)` method takes [user_kek_key_code13::W](user_kek_key_code13::W) writer structure"]
impl crate::Writable for USER_KEK_KEY_CODE13 {}
#[doc = "."]
pub mod user_kek_key_code13;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_header0](uds_header0) module"]
pub type UDS_HEADER0 = crate::Reg<u32, _UDS_HEADER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_HEADER0;
#[doc = "`read()` method returns [uds_header0::R](uds_header0::R) reader structure"]
impl crate::Readable for UDS_HEADER0 {}
#[doc = "`write(|w| ..)` method takes [uds_header0::W](uds_header0::W) writer structure"]
impl crate::Writable for UDS_HEADER0 {}
#[doc = "."]
pub mod uds_header0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code0](uds_key_code0) module"]
pub type UDS_KEY_CODE0 = crate::Reg<u32, _UDS_KEY_CODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE0;
#[doc = "`read()` method returns [uds_key_code0::R](uds_key_code0::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE0 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code0::W](uds_key_code0::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE0 {}
#[doc = "."]
pub mod uds_key_code0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_header1](uds_header1) module"]
pub type UDS_HEADER1 = crate::Reg<u32, _UDS_HEADER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_HEADER1;
#[doc = "`read()` method returns [uds_header1::R](uds_header1::R) reader structure"]
impl crate::Readable for UDS_HEADER1 {}
#[doc = "`write(|w| ..)` method takes [uds_header1::W](uds_header1::W) writer structure"]
impl crate::Writable for UDS_HEADER1 {}
#[doc = "."]
pub mod uds_header1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code1](uds_key_code1) module"]
pub type UDS_KEY_CODE1 = crate::Reg<u32, _UDS_KEY_CODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE1;
#[doc = "`read()` method returns [uds_key_code1::R](uds_key_code1::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE1 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code1::W](uds_key_code1::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE1 {}
#[doc = "."]
pub mod uds_key_code1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body0](uds_body0) module"]
pub type UDS_BODY0 = crate::Reg<u32, _UDS_BODY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY0;
#[doc = "`read()` method returns [uds_body0::R](uds_body0::R) reader structure"]
impl crate::Readable for UDS_BODY0 {}
#[doc = "`write(|w| ..)` method takes [uds_body0::W](uds_body0::W) writer structure"]
impl crate::Writable for UDS_BODY0 {}
#[doc = "."]
pub mod uds_body0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code2](uds_key_code2) module"]
pub type UDS_KEY_CODE2 = crate::Reg<u32, _UDS_KEY_CODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE2;
#[doc = "`read()` method returns [uds_key_code2::R](uds_key_code2::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE2 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code2::W](uds_key_code2::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE2 {}
#[doc = "."]
pub mod uds_key_code2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body1](uds_body1) module"]
pub type UDS_BODY1 = crate::Reg<u32, _UDS_BODY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY1;
#[doc = "`read()` method returns [uds_body1::R](uds_body1::R) reader structure"]
impl crate::Readable for UDS_BODY1 {}
#[doc = "`write(|w| ..)` method takes [uds_body1::W](uds_body1::W) writer structure"]
impl crate::Writable for UDS_BODY1 {}
#[doc = "."]
pub mod uds_body1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code3](uds_key_code3) module"]
pub type UDS_KEY_CODE3 = crate::Reg<u32, _UDS_KEY_CODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE3;
#[doc = "`read()` method returns [uds_key_code3::R](uds_key_code3::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE3 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code3::W](uds_key_code3::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE3 {}
#[doc = "."]
pub mod uds_key_code3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body2](uds_body2) module"]
pub type UDS_BODY2 = crate::Reg<u32, _UDS_BODY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY2;
#[doc = "`read()` method returns [uds_body2::R](uds_body2::R) reader structure"]
impl crate::Readable for UDS_BODY2 {}
#[doc = "`write(|w| ..)` method takes [uds_body2::W](uds_body2::W) writer structure"]
impl crate::Writable for UDS_BODY2 {}
#[doc = "."]
pub mod uds_body2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code4](uds_key_code4) module"]
pub type UDS_KEY_CODE4 = crate::Reg<u32, _UDS_KEY_CODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE4;
#[doc = "`read()` method returns [uds_key_code4::R](uds_key_code4::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE4 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code4::W](uds_key_code4::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE4 {}
#[doc = "."]
pub mod uds_key_code4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body3](uds_body3) module"]
pub type UDS_BODY3 = crate::Reg<u32, _UDS_BODY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY3;
#[doc = "`read()` method returns [uds_body3::R](uds_body3::R) reader structure"]
impl crate::Readable for UDS_BODY3 {}
#[doc = "`write(|w| ..)` method takes [uds_body3::W](uds_body3::W) writer structure"]
impl crate::Writable for UDS_BODY3 {}
#[doc = "."]
pub mod uds_body3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code5](uds_key_code5) module"]
pub type UDS_KEY_CODE5 = crate::Reg<u32, _UDS_KEY_CODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE5;
#[doc = "`read()` method returns [uds_key_code5::R](uds_key_code5::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE5 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code5::W](uds_key_code5::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE5 {}
#[doc = "."]
pub mod uds_key_code5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body4](uds_body4) module"]
pub type UDS_BODY4 = crate::Reg<u32, _UDS_BODY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY4;
#[doc = "`read()` method returns [uds_body4::R](uds_body4::R) reader structure"]
impl crate::Readable for UDS_BODY4 {}
#[doc = "`write(|w| ..)` method takes [uds_body4::W](uds_body4::W) writer structure"]
impl crate::Writable for UDS_BODY4 {}
#[doc = "."]
pub mod uds_body4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code6](uds_key_code6) module"]
pub type UDS_KEY_CODE6 = crate::Reg<u32, _UDS_KEY_CODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE6;
#[doc = "`read()` method returns [uds_key_code6::R](uds_key_code6::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE6 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code6::W](uds_key_code6::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE6 {}
#[doc = "."]
pub mod uds_key_code6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body5](uds_body5) module"]
pub type UDS_BODY5 = crate::Reg<u32, _UDS_BODY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY5;
#[doc = "`read()` method returns [uds_body5::R](uds_body5::R) reader structure"]
impl crate::Readable for UDS_BODY5 {}
#[doc = "`write(|w| ..)` method takes [uds_body5::W](uds_body5::W) writer structure"]
impl crate::Writable for UDS_BODY5 {}
#[doc = "."]
pub mod uds_body5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code7](uds_key_code7) module"]
pub type UDS_KEY_CODE7 = crate::Reg<u32, _UDS_KEY_CODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE7;
#[doc = "`read()` method returns [uds_key_code7::R](uds_key_code7::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE7 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code7::W](uds_key_code7::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE7 {}
#[doc = "."]
pub mod uds_key_code7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body6](uds_body6) module"]
pub type UDS_BODY6 = crate::Reg<u32, _UDS_BODY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY6;
#[doc = "`read()` method returns [uds_body6::R](uds_body6::R) reader structure"]
impl crate::Readable for UDS_BODY6 {}
#[doc = "`write(|w| ..)` method takes [uds_body6::W](uds_body6::W) writer structure"]
impl crate::Writable for UDS_BODY6 {}
#[doc = "."]
pub mod uds_body6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code8](uds_key_code8) module"]
pub type UDS_KEY_CODE8 = crate::Reg<u32, _UDS_KEY_CODE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE8;
#[doc = "`read()` method returns [uds_key_code8::R](uds_key_code8::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE8 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code8::W](uds_key_code8::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE8 {}
#[doc = "."]
pub mod uds_key_code8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body7](uds_body7) module"]
pub type UDS_BODY7 = crate::Reg<u32, _UDS_BODY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY7;
#[doc = "`read()` method returns [uds_body7::R](uds_body7::R) reader structure"]
impl crate::Readable for UDS_BODY7 {}
#[doc = "`write(|w| ..)` method takes [uds_body7::W](uds_body7::W) writer structure"]
impl crate::Writable for UDS_BODY7 {}
#[doc = "."]
pub mod uds_body7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code9](uds_key_code9) module"]
pub type UDS_KEY_CODE9 = crate::Reg<u32, _UDS_KEY_CODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE9;
#[doc = "`read()` method returns [uds_key_code9::R](uds_key_code9::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE9 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code9::W](uds_key_code9::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE9 {}
#[doc = "."]
pub mod uds_key_code9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body8](uds_body8) module"]
pub type UDS_BODY8 = crate::Reg<u32, _UDS_BODY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY8;
#[doc = "`read()` method returns [uds_body8::R](uds_body8::R) reader structure"]
impl crate::Readable for UDS_BODY8 {}
#[doc = "`write(|w| ..)` method takes [uds_body8::W](uds_body8::W) writer structure"]
impl crate::Writable for UDS_BODY8 {}
#[doc = "."]
pub mod uds_body8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code10](uds_key_code10) module"]
pub type UDS_KEY_CODE10 = crate::Reg<u32, _UDS_KEY_CODE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE10;
#[doc = "`read()` method returns [uds_key_code10::R](uds_key_code10::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE10 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code10::W](uds_key_code10::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE10 {}
#[doc = "."]
pub mod uds_key_code10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body9](uds_body9) module"]
pub type UDS_BODY9 = crate::Reg<u32, _UDS_BODY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY9;
#[doc = "`read()` method returns [uds_body9::R](uds_body9::R) reader structure"]
impl crate::Readable for UDS_BODY9 {}
#[doc = "`write(|w| ..)` method takes [uds_body9::W](uds_body9::W) writer structure"]
impl crate::Writable for UDS_BODY9 {}
#[doc = "."]
pub mod uds_body9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code11](uds_key_code11) module"]
pub type UDS_KEY_CODE11 = crate::Reg<u32, _UDS_KEY_CODE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE11;
#[doc = "`read()` method returns [uds_key_code11::R](uds_key_code11::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE11 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code11::W](uds_key_code11::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE11 {}
#[doc = "."]
pub mod uds_key_code11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body10](uds_body10) module"]
pub type UDS_BODY10 = crate::Reg<u32, _UDS_BODY10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY10;
#[doc = "`read()` method returns [uds_body10::R](uds_body10::R) reader structure"]
impl crate::Readable for UDS_BODY10 {}
#[doc = "`write(|w| ..)` method takes [uds_body10::W](uds_body10::W) writer structure"]
impl crate::Writable for UDS_BODY10 {}
#[doc = "."]
pub mod uds_body10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code12](uds_key_code12) module"]
pub type UDS_KEY_CODE12 = crate::Reg<u32, _UDS_KEY_CODE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE12;
#[doc = "`read()` method returns [uds_key_code12::R](uds_key_code12::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE12 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code12::W](uds_key_code12::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE12 {}
#[doc = "."]
pub mod uds_key_code12;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_body11](uds_body11) module"]
pub type UDS_BODY11 = crate::Reg<u32, _UDS_BODY11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_BODY11;
#[doc = "`read()` method returns [uds_body11::R](uds_body11::R) reader structure"]
impl crate::Readable for UDS_BODY11 {}
#[doc = "`write(|w| ..)` method takes [uds_body11::W](uds_body11::W) writer structure"]
impl crate::Writable for UDS_BODY11 {}
#[doc = "."]
pub mod uds_body11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uds_key_code13](uds_key_code13) module"]
pub type UDS_KEY_CODE13 = crate::Reg<u32, _UDS_KEY_CODE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDS_KEY_CODE13;
#[doc = "`read()` method returns [uds_key_code13::R](uds_key_code13::R) reader structure"]
impl crate::Readable for UDS_KEY_CODE13 {}
#[doc = "`write(|w| ..)` method takes [uds_key_code13::W](uds_key_code13::W) writer structure"]
impl crate::Writable for UDS_KEY_CODE13 {}
#[doc = "."]
pub mod uds_key_code13;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_header0](prince_region0_header0) module"]
pub type PRINCE_REGION0_HEADER0 = crate::Reg<u32, _PRINCE_REGION0_HEADER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_HEADER0;
#[doc = "`read()` method returns [prince_region0_header0::R](prince_region0_header0::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_HEADER0 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_header0::W](prince_region0_header0::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_HEADER0 {}
#[doc = "."]
pub mod prince_region0_header0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code0](prince_region0_key_code0) module"]
pub type PRINCE_REGION0_KEY_CODE0 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE0;
#[doc = "`read()` method returns [prince_region0_key_code0::R](prince_region0_key_code0::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE0 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code0::W](prince_region0_key_code0::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE0 {}
#[doc = "."]
pub mod prince_region0_key_code0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_header1](prince_region0_header1) module"]
pub type PRINCE_REGION0_HEADER1 = crate::Reg<u32, _PRINCE_REGION0_HEADER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_HEADER1;
#[doc = "`read()` method returns [prince_region0_header1::R](prince_region0_header1::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_HEADER1 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_header1::W](prince_region0_header1::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_HEADER1 {}
#[doc = "."]
pub mod prince_region0_header1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code1](prince_region0_key_code1) module"]
pub type PRINCE_REGION0_KEY_CODE1 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE1;
#[doc = "`read()` method returns [prince_region0_key_code1::R](prince_region0_key_code1::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE1 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code1::W](prince_region0_key_code1::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE1 {}
#[doc = "."]
pub mod prince_region0_key_code1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body0](prince_region0_body0) module"]
pub type PRINCE_REGION0_BODY0 = crate::Reg<u32, _PRINCE_REGION0_BODY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY0;
#[doc = "`read()` method returns [prince_region0_body0::R](prince_region0_body0::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY0 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body0::W](prince_region0_body0::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY0 {}
#[doc = "."]
pub mod prince_region0_body0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code2](prince_region0_key_code2) module"]
pub type PRINCE_REGION0_KEY_CODE2 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE2;
#[doc = "`read()` method returns [prince_region0_key_code2::R](prince_region0_key_code2::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE2 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code2::W](prince_region0_key_code2::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE2 {}
#[doc = "."]
pub mod prince_region0_key_code2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body1](prince_region0_body1) module"]
pub type PRINCE_REGION0_BODY1 = crate::Reg<u32, _PRINCE_REGION0_BODY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY1;
#[doc = "`read()` method returns [prince_region0_body1::R](prince_region0_body1::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY1 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body1::W](prince_region0_body1::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY1 {}
#[doc = "."]
pub mod prince_region0_body1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code3](prince_region0_key_code3) module"]
pub type PRINCE_REGION0_KEY_CODE3 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE3;
#[doc = "`read()` method returns [prince_region0_key_code3::R](prince_region0_key_code3::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE3 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code3::W](prince_region0_key_code3::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE3 {}
#[doc = "."]
pub mod prince_region0_key_code3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body2](prince_region0_body2) module"]
pub type PRINCE_REGION0_BODY2 = crate::Reg<u32, _PRINCE_REGION0_BODY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY2;
#[doc = "`read()` method returns [prince_region0_body2::R](prince_region0_body2::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY2 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body2::W](prince_region0_body2::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY2 {}
#[doc = "."]
pub mod prince_region0_body2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code4](prince_region0_key_code4) module"]
pub type PRINCE_REGION0_KEY_CODE4 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE4;
#[doc = "`read()` method returns [prince_region0_key_code4::R](prince_region0_key_code4::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE4 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code4::W](prince_region0_key_code4::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE4 {}
#[doc = "."]
pub mod prince_region0_key_code4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body3](prince_region0_body3) module"]
pub type PRINCE_REGION0_BODY3 = crate::Reg<u32, _PRINCE_REGION0_BODY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY3;
#[doc = "`read()` method returns [prince_region0_body3::R](prince_region0_body3::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY3 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body3::W](prince_region0_body3::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY3 {}
#[doc = "."]
pub mod prince_region0_body3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code5](prince_region0_key_code5) module"]
pub type PRINCE_REGION0_KEY_CODE5 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE5;
#[doc = "`read()` method returns [prince_region0_key_code5::R](prince_region0_key_code5::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE5 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code5::W](prince_region0_key_code5::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE5 {}
#[doc = "."]
pub mod prince_region0_key_code5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body4](prince_region0_body4) module"]
pub type PRINCE_REGION0_BODY4 = crate::Reg<u32, _PRINCE_REGION0_BODY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY4;
#[doc = "`read()` method returns [prince_region0_body4::R](prince_region0_body4::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY4 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body4::W](prince_region0_body4::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY4 {}
#[doc = "."]
pub mod prince_region0_body4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code6](prince_region0_key_code6) module"]
pub type PRINCE_REGION0_KEY_CODE6 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE6;
#[doc = "`read()` method returns [prince_region0_key_code6::R](prince_region0_key_code6::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE6 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code6::W](prince_region0_key_code6::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE6 {}
#[doc = "."]
pub mod prince_region0_key_code6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body5](prince_region0_body5) module"]
pub type PRINCE_REGION0_BODY5 = crate::Reg<u32, _PRINCE_REGION0_BODY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY5;
#[doc = "`read()` method returns [prince_region0_body5::R](prince_region0_body5::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY5 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body5::W](prince_region0_body5::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY5 {}
#[doc = "."]
pub mod prince_region0_body5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code7](prince_region0_key_code7) module"]
pub type PRINCE_REGION0_KEY_CODE7 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE7;
#[doc = "`read()` method returns [prince_region0_key_code7::R](prince_region0_key_code7::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE7 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code7::W](prince_region0_key_code7::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE7 {}
#[doc = "."]
pub mod prince_region0_key_code7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body6](prince_region0_body6) module"]
pub type PRINCE_REGION0_BODY6 = crate::Reg<u32, _PRINCE_REGION0_BODY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY6;
#[doc = "`read()` method returns [prince_region0_body6::R](prince_region0_body6::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY6 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body6::W](prince_region0_body6::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY6 {}
#[doc = "."]
pub mod prince_region0_body6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code8](prince_region0_key_code8) module"]
pub type PRINCE_REGION0_KEY_CODE8 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE8;
#[doc = "`read()` method returns [prince_region0_key_code8::R](prince_region0_key_code8::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE8 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code8::W](prince_region0_key_code8::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE8 {}
#[doc = "."]
pub mod prince_region0_key_code8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body7](prince_region0_body7) module"]
pub type PRINCE_REGION0_BODY7 = crate::Reg<u32, _PRINCE_REGION0_BODY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY7;
#[doc = "`read()` method returns [prince_region0_body7::R](prince_region0_body7::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY7 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body7::W](prince_region0_body7::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY7 {}
#[doc = "."]
pub mod prince_region0_body7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code9](prince_region0_key_code9) module"]
pub type PRINCE_REGION0_KEY_CODE9 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE9;
#[doc = "`read()` method returns [prince_region0_key_code9::R](prince_region0_key_code9::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE9 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code9::W](prince_region0_key_code9::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE9 {}
#[doc = "."]
pub mod prince_region0_key_code9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body8](prince_region0_body8) module"]
pub type PRINCE_REGION0_BODY8 = crate::Reg<u32, _PRINCE_REGION0_BODY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY8;
#[doc = "`read()` method returns [prince_region0_body8::R](prince_region0_body8::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY8 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body8::W](prince_region0_body8::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY8 {}
#[doc = "."]
pub mod prince_region0_body8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code10](prince_region0_key_code10) module"]
pub type PRINCE_REGION0_KEY_CODE10 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE10;
#[doc = "`read()` method returns [prince_region0_key_code10::R](prince_region0_key_code10::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE10 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code10::W](prince_region0_key_code10::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE10 {}
#[doc = "."]
pub mod prince_region0_key_code10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body9](prince_region0_body9) module"]
pub type PRINCE_REGION0_BODY9 = crate::Reg<u32, _PRINCE_REGION0_BODY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY9;
#[doc = "`read()` method returns [prince_region0_body9::R](prince_region0_body9::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY9 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body9::W](prince_region0_body9::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY9 {}
#[doc = "."]
pub mod prince_region0_body9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code11](prince_region0_key_code11) module"]
pub type PRINCE_REGION0_KEY_CODE11 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE11;
#[doc = "`read()` method returns [prince_region0_key_code11::R](prince_region0_key_code11::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE11 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code11::W](prince_region0_key_code11::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE11 {}
#[doc = "."]
pub mod prince_region0_key_code11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body10](prince_region0_body10) module"]
pub type PRINCE_REGION0_BODY10 = crate::Reg<u32, _PRINCE_REGION0_BODY10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY10;
#[doc = "`read()` method returns [prince_region0_body10::R](prince_region0_body10::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY10 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body10::W](prince_region0_body10::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY10 {}
#[doc = "."]
pub mod prince_region0_body10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code12](prince_region0_key_code12) module"]
pub type PRINCE_REGION0_KEY_CODE12 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE12;
#[doc = "`read()` method returns [prince_region0_key_code12::R](prince_region0_key_code12::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE12 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code12::W](prince_region0_key_code12::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE12 {}
#[doc = "."]
pub mod prince_region0_key_code12;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_body11](prince_region0_body11) module"]
pub type PRINCE_REGION0_BODY11 = crate::Reg<u32, _PRINCE_REGION0_BODY11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_BODY11;
#[doc = "`read()` method returns [prince_region0_body11::R](prince_region0_body11::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_BODY11 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_body11::W](prince_region0_body11::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_BODY11 {}
#[doc = "."]
pub mod prince_region0_body11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region0_key_code13](prince_region0_key_code13) module"]
pub type PRINCE_REGION0_KEY_CODE13 = crate::Reg<u32, _PRINCE_REGION0_KEY_CODE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION0_KEY_CODE13;
#[doc = "`read()` method returns [prince_region0_key_code13::R](prince_region0_key_code13::R) reader structure"]
impl crate::Readable for PRINCE_REGION0_KEY_CODE13 {}
#[doc = "`write(|w| ..)` method takes [prince_region0_key_code13::W](prince_region0_key_code13::W) writer structure"]
impl crate::Writable for PRINCE_REGION0_KEY_CODE13 {}
#[doc = "."]
pub mod prince_region0_key_code13;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_header0](prince_region1_header0) module"]
pub type PRINCE_REGION1_HEADER0 = crate::Reg<u32, _PRINCE_REGION1_HEADER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_HEADER0;
#[doc = "`read()` method returns [prince_region1_header0::R](prince_region1_header0::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_HEADER0 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_header0::W](prince_region1_header0::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_HEADER0 {}
#[doc = "."]
pub mod prince_region1_header0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code0](prince_region1_key_code0) module"]
pub type PRINCE_REGION1_KEY_CODE0 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE0;
#[doc = "`read()` method returns [prince_region1_key_code0::R](prince_region1_key_code0::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE0 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code0::W](prince_region1_key_code0::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE0 {}
#[doc = "."]
pub mod prince_region1_key_code0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_header1](prince_region1_header1) module"]
pub type PRINCE_REGION1_HEADER1 = crate::Reg<u32, _PRINCE_REGION1_HEADER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_HEADER1;
#[doc = "`read()` method returns [prince_region1_header1::R](prince_region1_header1::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_HEADER1 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_header1::W](prince_region1_header1::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_HEADER1 {}
#[doc = "."]
pub mod prince_region1_header1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code1](prince_region1_key_code1) module"]
pub type PRINCE_REGION1_KEY_CODE1 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE1;
#[doc = "`read()` method returns [prince_region1_key_code1::R](prince_region1_key_code1::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE1 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code1::W](prince_region1_key_code1::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE1 {}
#[doc = "."]
pub mod prince_region1_key_code1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body0](prince_region1_body0) module"]
pub type PRINCE_REGION1_BODY0 = crate::Reg<u32, _PRINCE_REGION1_BODY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY0;
#[doc = "`read()` method returns [prince_region1_body0::R](prince_region1_body0::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY0 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body0::W](prince_region1_body0::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY0 {}
#[doc = "."]
pub mod prince_region1_body0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code2](prince_region1_key_code2) module"]
pub type PRINCE_REGION1_KEY_CODE2 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE2;
#[doc = "`read()` method returns [prince_region1_key_code2::R](prince_region1_key_code2::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE2 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code2::W](prince_region1_key_code2::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE2 {}
#[doc = "."]
pub mod prince_region1_key_code2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body1](prince_region1_body1) module"]
pub type PRINCE_REGION1_BODY1 = crate::Reg<u32, _PRINCE_REGION1_BODY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY1;
#[doc = "`read()` method returns [prince_region1_body1::R](prince_region1_body1::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY1 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body1::W](prince_region1_body1::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY1 {}
#[doc = "."]
pub mod prince_region1_body1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code3](prince_region1_key_code3) module"]
pub type PRINCE_REGION1_KEY_CODE3 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE3;
#[doc = "`read()` method returns [prince_region1_key_code3::R](prince_region1_key_code3::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE3 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code3::W](prince_region1_key_code3::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE3 {}
#[doc = "."]
pub mod prince_region1_key_code3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body2](prince_region1_body2) module"]
pub type PRINCE_REGION1_BODY2 = crate::Reg<u32, _PRINCE_REGION1_BODY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY2;
#[doc = "`read()` method returns [prince_region1_body2::R](prince_region1_body2::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY2 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body2::W](prince_region1_body2::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY2 {}
#[doc = "."]
pub mod prince_region1_body2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code4](prince_region1_key_code4) module"]
pub type PRINCE_REGION1_KEY_CODE4 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE4;
#[doc = "`read()` method returns [prince_region1_key_code4::R](prince_region1_key_code4::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE4 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code4::W](prince_region1_key_code4::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE4 {}
#[doc = "."]
pub mod prince_region1_key_code4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body3](prince_region1_body3) module"]
pub type PRINCE_REGION1_BODY3 = crate::Reg<u32, _PRINCE_REGION1_BODY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY3;
#[doc = "`read()` method returns [prince_region1_body3::R](prince_region1_body3::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY3 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body3::W](prince_region1_body3::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY3 {}
#[doc = "."]
pub mod prince_region1_body3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code5](prince_region1_key_code5) module"]
pub type PRINCE_REGION1_KEY_CODE5 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE5;
#[doc = "`read()` method returns [prince_region1_key_code5::R](prince_region1_key_code5::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE5 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code5::W](prince_region1_key_code5::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE5 {}
#[doc = "."]
pub mod prince_region1_key_code5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body4](prince_region1_body4) module"]
pub type PRINCE_REGION1_BODY4 = crate::Reg<u32, _PRINCE_REGION1_BODY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY4;
#[doc = "`read()` method returns [prince_region1_body4::R](prince_region1_body4::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY4 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body4::W](prince_region1_body4::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY4 {}
#[doc = "."]
pub mod prince_region1_body4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code6](prince_region1_key_code6) module"]
pub type PRINCE_REGION1_KEY_CODE6 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE6;
#[doc = "`read()` method returns [prince_region1_key_code6::R](prince_region1_key_code6::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE6 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code6::W](prince_region1_key_code6::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE6 {}
#[doc = "."]
pub mod prince_region1_key_code6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body5](prince_region1_body5) module"]
pub type PRINCE_REGION1_BODY5 = crate::Reg<u32, _PRINCE_REGION1_BODY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY5;
#[doc = "`read()` method returns [prince_region1_body5::R](prince_region1_body5::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY5 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body5::W](prince_region1_body5::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY5 {}
#[doc = "."]
pub mod prince_region1_body5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code7](prince_region1_key_code7) module"]
pub type PRINCE_REGION1_KEY_CODE7 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE7;
#[doc = "`read()` method returns [prince_region1_key_code7::R](prince_region1_key_code7::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE7 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code7::W](prince_region1_key_code7::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE7 {}
#[doc = "."]
pub mod prince_region1_key_code7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body6](prince_region1_body6) module"]
pub type PRINCE_REGION1_BODY6 = crate::Reg<u32, _PRINCE_REGION1_BODY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY6;
#[doc = "`read()` method returns [prince_region1_body6::R](prince_region1_body6::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY6 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body6::W](prince_region1_body6::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY6 {}
#[doc = "."]
pub mod prince_region1_body6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code8](prince_region1_key_code8) module"]
pub type PRINCE_REGION1_KEY_CODE8 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE8;
#[doc = "`read()` method returns [prince_region1_key_code8::R](prince_region1_key_code8::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE8 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code8::W](prince_region1_key_code8::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE8 {}
#[doc = "."]
pub mod prince_region1_key_code8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body7](prince_region1_body7) module"]
pub type PRINCE_REGION1_BODY7 = crate::Reg<u32, _PRINCE_REGION1_BODY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY7;
#[doc = "`read()` method returns [prince_region1_body7::R](prince_region1_body7::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY7 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body7::W](prince_region1_body7::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY7 {}
#[doc = "."]
pub mod prince_region1_body7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code9](prince_region1_key_code9) module"]
pub type PRINCE_REGION1_KEY_CODE9 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE9;
#[doc = "`read()` method returns [prince_region1_key_code9::R](prince_region1_key_code9::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE9 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code9::W](prince_region1_key_code9::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE9 {}
#[doc = "."]
pub mod prince_region1_key_code9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body8](prince_region1_body8) module"]
pub type PRINCE_REGION1_BODY8 = crate::Reg<u32, _PRINCE_REGION1_BODY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY8;
#[doc = "`read()` method returns [prince_region1_body8::R](prince_region1_body8::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY8 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body8::W](prince_region1_body8::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY8 {}
#[doc = "."]
pub mod prince_region1_body8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code10](prince_region1_key_code10) module"]
pub type PRINCE_REGION1_KEY_CODE10 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE10;
#[doc = "`read()` method returns [prince_region1_key_code10::R](prince_region1_key_code10::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE10 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code10::W](prince_region1_key_code10::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE10 {}
#[doc = "."]
pub mod prince_region1_key_code10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body9](prince_region1_body9) module"]
pub type PRINCE_REGION1_BODY9 = crate::Reg<u32, _PRINCE_REGION1_BODY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY9;
#[doc = "`read()` method returns [prince_region1_body9::R](prince_region1_body9::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY9 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body9::W](prince_region1_body9::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY9 {}
#[doc = "."]
pub mod prince_region1_body9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code11](prince_region1_key_code11) module"]
pub type PRINCE_REGION1_KEY_CODE11 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE11;
#[doc = "`read()` method returns [prince_region1_key_code11::R](prince_region1_key_code11::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE11 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code11::W](prince_region1_key_code11::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE11 {}
#[doc = "."]
pub mod prince_region1_key_code11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body10](prince_region1_body10) module"]
pub type PRINCE_REGION1_BODY10 = crate::Reg<u32, _PRINCE_REGION1_BODY10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY10;
#[doc = "`read()` method returns [prince_region1_body10::R](prince_region1_body10::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY10 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body10::W](prince_region1_body10::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY10 {}
#[doc = "."]
pub mod prince_region1_body10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code12](prince_region1_key_code12) module"]
pub type PRINCE_REGION1_KEY_CODE12 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE12;
#[doc = "`read()` method returns [prince_region1_key_code12::R](prince_region1_key_code12::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE12 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code12::W](prince_region1_key_code12::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE12 {}
#[doc = "."]
pub mod prince_region1_key_code12;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_body11](prince_region1_body11) module"]
pub type PRINCE_REGION1_BODY11 = crate::Reg<u32, _PRINCE_REGION1_BODY11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_BODY11;
#[doc = "`read()` method returns [prince_region1_body11::R](prince_region1_body11::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_BODY11 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_body11::W](prince_region1_body11::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_BODY11 {}
#[doc = "."]
pub mod prince_region1_body11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region1_key_code13](prince_region1_key_code13) module"]
pub type PRINCE_REGION1_KEY_CODE13 = crate::Reg<u32, _PRINCE_REGION1_KEY_CODE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION1_KEY_CODE13;
#[doc = "`read()` method returns [prince_region1_key_code13::R](prince_region1_key_code13::R) reader structure"]
impl crate::Readable for PRINCE_REGION1_KEY_CODE13 {}
#[doc = "`write(|w| ..)` method takes [prince_region1_key_code13::W](prince_region1_key_code13::W) writer structure"]
impl crate::Writable for PRINCE_REGION1_KEY_CODE13 {}
#[doc = "."]
pub mod prince_region1_key_code13;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_header0](prince_region2_header0) module"]
pub type PRINCE_REGION2_HEADER0 = crate::Reg<u32, _PRINCE_REGION2_HEADER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_HEADER0;
#[doc = "`read()` method returns [prince_region2_header0::R](prince_region2_header0::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_HEADER0 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_header0::W](prince_region2_header0::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_HEADER0 {}
#[doc = "."]
pub mod prince_region2_header0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code0](prince_region2_key_code0) module"]
pub type PRINCE_REGION2_KEY_CODE0 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE0;
#[doc = "`read()` method returns [prince_region2_key_code0::R](prince_region2_key_code0::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE0 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code0::W](prince_region2_key_code0::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE0 {}
#[doc = "."]
pub mod prince_region2_key_code0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_header1](prince_region2_header1) module"]
pub type PRINCE_REGION2_HEADER1 = crate::Reg<u32, _PRINCE_REGION2_HEADER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_HEADER1;
#[doc = "`read()` method returns [prince_region2_header1::R](prince_region2_header1::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_HEADER1 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_header1::W](prince_region2_header1::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_HEADER1 {}
#[doc = "."]
pub mod prince_region2_header1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code1](prince_region2_key_code1) module"]
pub type PRINCE_REGION2_KEY_CODE1 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE1;
#[doc = "`read()` method returns [prince_region2_key_code1::R](prince_region2_key_code1::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE1 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code1::W](prince_region2_key_code1::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE1 {}
#[doc = "."]
pub mod prince_region2_key_code1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body0](prince_region2_body0) module"]
pub type PRINCE_REGION2_BODY0 = crate::Reg<u32, _PRINCE_REGION2_BODY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY0;
#[doc = "`read()` method returns [prince_region2_body0::R](prince_region2_body0::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY0 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body0::W](prince_region2_body0::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY0 {}
#[doc = "."]
pub mod prince_region2_body0;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code2](prince_region2_key_code2) module"]
pub type PRINCE_REGION2_KEY_CODE2 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE2;
#[doc = "`read()` method returns [prince_region2_key_code2::R](prince_region2_key_code2::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE2 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code2::W](prince_region2_key_code2::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE2 {}
#[doc = "."]
pub mod prince_region2_key_code2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body1](prince_region2_body1) module"]
pub type PRINCE_REGION2_BODY1 = crate::Reg<u32, _PRINCE_REGION2_BODY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY1;
#[doc = "`read()` method returns [prince_region2_body1::R](prince_region2_body1::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY1 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body1::W](prince_region2_body1::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY1 {}
#[doc = "."]
pub mod prince_region2_body1;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code3](prince_region2_key_code3) module"]
pub type PRINCE_REGION2_KEY_CODE3 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE3;
#[doc = "`read()` method returns [prince_region2_key_code3::R](prince_region2_key_code3::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE3 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code3::W](prince_region2_key_code3::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE3 {}
#[doc = "."]
pub mod prince_region2_key_code3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body2](prince_region2_body2) module"]
pub type PRINCE_REGION2_BODY2 = crate::Reg<u32, _PRINCE_REGION2_BODY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY2;
#[doc = "`read()` method returns [prince_region2_body2::R](prince_region2_body2::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY2 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body2::W](prince_region2_body2::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY2 {}
#[doc = "."]
pub mod prince_region2_body2;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code4](prince_region2_key_code4) module"]
pub type PRINCE_REGION2_KEY_CODE4 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE4;
#[doc = "`read()` method returns [prince_region2_key_code4::R](prince_region2_key_code4::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE4 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code4::W](prince_region2_key_code4::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE4 {}
#[doc = "."]
pub mod prince_region2_key_code4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body3](prince_region2_body3) module"]
pub type PRINCE_REGION2_BODY3 = crate::Reg<u32, _PRINCE_REGION2_BODY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY3;
#[doc = "`read()` method returns [prince_region2_body3::R](prince_region2_body3::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY3 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body3::W](prince_region2_body3::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY3 {}
#[doc = "."]
pub mod prince_region2_body3;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code5](prince_region2_key_code5) module"]
pub type PRINCE_REGION2_KEY_CODE5 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE5;
#[doc = "`read()` method returns [prince_region2_key_code5::R](prince_region2_key_code5::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE5 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code5::W](prince_region2_key_code5::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE5 {}
#[doc = "."]
pub mod prince_region2_key_code5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body4](prince_region2_body4) module"]
pub type PRINCE_REGION2_BODY4 = crate::Reg<u32, _PRINCE_REGION2_BODY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY4;
#[doc = "`read()` method returns [prince_region2_body4::R](prince_region2_body4::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY4 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body4::W](prince_region2_body4::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY4 {}
#[doc = "."]
pub mod prince_region2_body4;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code6](prince_region2_key_code6) module"]
pub type PRINCE_REGION2_KEY_CODE6 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE6;
#[doc = "`read()` method returns [prince_region2_key_code6::R](prince_region2_key_code6::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE6 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code6::W](prince_region2_key_code6::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE6 {}
#[doc = "."]
pub mod prince_region2_key_code6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body5](prince_region2_body5) module"]
pub type PRINCE_REGION2_BODY5 = crate::Reg<u32, _PRINCE_REGION2_BODY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY5;
#[doc = "`read()` method returns [prince_region2_body5::R](prince_region2_body5::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY5 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body5::W](prince_region2_body5::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY5 {}
#[doc = "."]
pub mod prince_region2_body5;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code7](prince_region2_key_code7) module"]
pub type PRINCE_REGION2_KEY_CODE7 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE7;
#[doc = "`read()` method returns [prince_region2_key_code7::R](prince_region2_key_code7::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE7 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code7::W](prince_region2_key_code7::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE7 {}
#[doc = "."]
pub mod prince_region2_key_code7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body6](prince_region2_body6) module"]
pub type PRINCE_REGION2_BODY6 = crate::Reg<u32, _PRINCE_REGION2_BODY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY6;
#[doc = "`read()` method returns [prince_region2_body6::R](prince_region2_body6::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY6 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body6::W](prince_region2_body6::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY6 {}
#[doc = "."]
pub mod prince_region2_body6;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code8](prince_region2_key_code8) module"]
pub type PRINCE_REGION2_KEY_CODE8 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE8;
#[doc = "`read()` method returns [prince_region2_key_code8::R](prince_region2_key_code8::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE8 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code8::W](prince_region2_key_code8::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE8 {}
#[doc = "."]
pub mod prince_region2_key_code8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body7](prince_region2_body7) module"]
pub type PRINCE_REGION2_BODY7 = crate::Reg<u32, _PRINCE_REGION2_BODY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY7;
#[doc = "`read()` method returns [prince_region2_body7::R](prince_region2_body7::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY7 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body7::W](prince_region2_body7::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY7 {}
#[doc = "."]
pub mod prince_region2_body7;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code9](prince_region2_key_code9) module"]
pub type PRINCE_REGION2_KEY_CODE9 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE9;
#[doc = "`read()` method returns [prince_region2_key_code9::R](prince_region2_key_code9::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE9 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code9::W](prince_region2_key_code9::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE9 {}
#[doc = "."]
pub mod prince_region2_key_code9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body8](prince_region2_body8) module"]
pub type PRINCE_REGION2_BODY8 = crate::Reg<u32, _PRINCE_REGION2_BODY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY8;
#[doc = "`read()` method returns [prince_region2_body8::R](prince_region2_body8::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY8 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body8::W](prince_region2_body8::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY8 {}
#[doc = "."]
pub mod prince_region2_body8;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code10](prince_region2_key_code10) module"]
pub type PRINCE_REGION2_KEY_CODE10 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE10;
#[doc = "`read()` method returns [prince_region2_key_code10::R](prince_region2_key_code10::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE10 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code10::W](prince_region2_key_code10::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE10 {}
#[doc = "."]
pub mod prince_region2_key_code10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body9](prince_region2_body9) module"]
pub type PRINCE_REGION2_BODY9 = crate::Reg<u32, _PRINCE_REGION2_BODY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY9;
#[doc = "`read()` method returns [prince_region2_body9::R](prince_region2_body9::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY9 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body9::W](prince_region2_body9::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY9 {}
#[doc = "."]
pub mod prince_region2_body9;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code11](prince_region2_key_code11) module"]
pub type PRINCE_REGION2_KEY_CODE11 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE11;
#[doc = "`read()` method returns [prince_region2_key_code11::R](prince_region2_key_code11::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE11 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code11::W](prince_region2_key_code11::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE11 {}
#[doc = "."]
pub mod prince_region2_key_code11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body10](prince_region2_body10) module"]
pub type PRINCE_REGION2_BODY10 = crate::Reg<u32, _PRINCE_REGION2_BODY10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY10;
#[doc = "`read()` method returns [prince_region2_body10::R](prince_region2_body10::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY10 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body10::W](prince_region2_body10::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY10 {}
#[doc = "."]
pub mod prince_region2_body10;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code12](prince_region2_key_code12) module"]
pub type PRINCE_REGION2_KEY_CODE12 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE12;
#[doc = "`read()` method returns [prince_region2_key_code12::R](prince_region2_key_code12::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE12 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code12::W](prince_region2_key_code12::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE12 {}
#[doc = "."]
pub mod prince_region2_key_code12;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_body11](prince_region2_body11) module"]
pub type PRINCE_REGION2_BODY11 = crate::Reg<u32, _PRINCE_REGION2_BODY11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_BODY11;
#[doc = "`read()` method returns [prince_region2_body11::R](prince_region2_body11::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_BODY11 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_body11::W](prince_region2_body11::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_BODY11 {}
#[doc = "."]
pub mod prince_region2_body11;
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prince_region2_key_code13](prince_region2_key_code13) module"]
pub type PRINCE_REGION2_KEY_CODE13 = crate::Reg<u32, _PRINCE_REGION2_KEY_CODE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRINCE_REGION2_KEY_CODE13;
#[doc = "`read()` method returns [prince_region2_key_code13::R](prince_region2_key_code13::R) reader structure"]
impl crate::Readable for PRINCE_REGION2_KEY_CODE13 {}
#[doc = "`write(|w| ..)` method takes [prince_region2_key_code13::W](prince_region2_key_code13::W) writer structure"]
impl crate::Writable for PRINCE_REGION2_KEY_CODE13 {}
#[doc = "."]
pub mod prince_region2_key_code13;
