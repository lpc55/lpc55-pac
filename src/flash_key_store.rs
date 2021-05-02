#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Valid Key Sore Header : 0x95959595"]
    pub header: crate::Reg<header::HEADER_SPEC>,
    #[doc = "0x04 - puf discharge time in ms."]
    pub puf_discharge_time_in_ms:
        crate::Reg<puf_discharge_time_in_ms::PUF_DISCHARGE_TIME_IN_MS_SPEC>,
    #[doc = "0x08 - ."]
    pub activation_code: [crate::Reg<activation_code::ACTIVATION_CODE_SPEC>; 298],
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
    pub fn sbkey_key_code0(&self) -> &crate::Reg<sbkey_key_code0::SBKEY_KEY_CODE0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1200usize)
                as *const crate::Reg<sbkey_key_code0::SBKEY_KEY_CODE0_SPEC>)
        }
    }
    #[doc = "0x4b0 - ."]
    #[inline(always)]
    pub fn sbkey_header0(&self) -> &crate::Reg<sbkey_header0::SBKEY_HEADER0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1200usize)
                as *const crate::Reg<sbkey_header0::SBKEY_HEADER0_SPEC>)
        }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code1(&self) -> &crate::Reg<sbkey_key_code1::SBKEY_KEY_CODE1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1204usize)
                as *const crate::Reg<sbkey_key_code1::SBKEY_KEY_CODE1_SPEC>)
        }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub fn sbkey_header1(&self) -> &crate::Reg<sbkey_header1::SBKEY_HEADER1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1204usize)
                as *const crate::Reg<sbkey_header1::SBKEY_HEADER1_SPEC>)
        }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub fn sbkey_key_code2(&self) -> &crate::Reg<sbkey_key_code2::SBKEY_KEY_CODE2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1208usize)
                as *const crate::Reg<sbkey_key_code2::SBKEY_KEY_CODE2_SPEC>)
        }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub fn sbkey_body0(&self) -> &crate::Reg<sbkey_body0::SBKEY_BODY0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1208usize)
                as *const crate::Reg<sbkey_body0::SBKEY_BODY0_SPEC>)
        }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub fn sbkey_key_code3(&self) -> &crate::Reg<sbkey_key_code3::SBKEY_KEY_CODE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1212usize)
                as *const crate::Reg<sbkey_key_code3::SBKEY_KEY_CODE3_SPEC>)
        }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub fn sbkey_body1(&self) -> &crate::Reg<sbkey_body1::SBKEY_BODY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1212usize)
                as *const crate::Reg<sbkey_body1::SBKEY_BODY1_SPEC>)
        }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code4(&self) -> &crate::Reg<sbkey_key_code4::SBKEY_KEY_CODE4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1216usize)
                as *const crate::Reg<sbkey_key_code4::SBKEY_KEY_CODE4_SPEC>)
        }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub fn sbkey_body2(&self) -> &crate::Reg<sbkey_body2::SBKEY_BODY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1216usize)
                as *const crate::Reg<sbkey_body2::SBKEY_BODY2_SPEC>)
        }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code5(&self) -> &crate::Reg<sbkey_key_code5::SBKEY_KEY_CODE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1220usize)
                as *const crate::Reg<sbkey_key_code5::SBKEY_KEY_CODE5_SPEC>)
        }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub fn sbkey_body3(&self) -> &crate::Reg<sbkey_body3::SBKEY_BODY3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1220usize)
                as *const crate::Reg<sbkey_body3::SBKEY_BODY3_SPEC>)
        }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub fn sbkey_key_code6(&self) -> &crate::Reg<sbkey_key_code6::SBKEY_KEY_CODE6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1224usize)
                as *const crate::Reg<sbkey_key_code6::SBKEY_KEY_CODE6_SPEC>)
        }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub fn sbkey_body4(&self) -> &crate::Reg<sbkey_body4::SBKEY_BODY4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1224usize)
                as *const crate::Reg<sbkey_body4::SBKEY_BODY4_SPEC>)
        }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub fn sbkey_key_code7(&self) -> &crate::Reg<sbkey_key_code7::SBKEY_KEY_CODE7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1228usize)
                as *const crate::Reg<sbkey_key_code7::SBKEY_KEY_CODE7_SPEC>)
        }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub fn sbkey_body5(&self) -> &crate::Reg<sbkey_body5::SBKEY_BODY5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1228usize)
                as *const crate::Reg<sbkey_body5::SBKEY_BODY5_SPEC>)
        }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code8(&self) -> &crate::Reg<sbkey_key_code8::SBKEY_KEY_CODE8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1232usize)
                as *const crate::Reg<sbkey_key_code8::SBKEY_KEY_CODE8_SPEC>)
        }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub fn sbkey_body6(&self) -> &crate::Reg<sbkey_body6::SBKEY_BODY6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1232usize)
                as *const crate::Reg<sbkey_body6::SBKEY_BODY6_SPEC>)
        }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code9(&self) -> &crate::Reg<sbkey_key_code9::SBKEY_KEY_CODE9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1236usize)
                as *const crate::Reg<sbkey_key_code9::SBKEY_KEY_CODE9_SPEC>)
        }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub fn sbkey_body7(&self) -> &crate::Reg<sbkey_body7::SBKEY_BODY7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1236usize)
                as *const crate::Reg<sbkey_body7::SBKEY_BODY7_SPEC>)
        }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub fn sbkey_key_code10(&self) -> &crate::Reg<sbkey_key_code10::SBKEY_KEY_CODE10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1240usize)
                as *const crate::Reg<sbkey_key_code10::SBKEY_KEY_CODE10_SPEC>)
        }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub fn sbkey_body8(&self) -> &crate::Reg<sbkey_body8::SBKEY_BODY8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1240usize)
                as *const crate::Reg<sbkey_body8::SBKEY_BODY8_SPEC>)
        }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub fn sbkey_key_code11(&self) -> &crate::Reg<sbkey_key_code11::SBKEY_KEY_CODE11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1244usize)
                as *const crate::Reg<sbkey_key_code11::SBKEY_KEY_CODE11_SPEC>)
        }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub fn sbkey_body9(&self) -> &crate::Reg<sbkey_body9::SBKEY_BODY9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1244usize)
                as *const crate::Reg<sbkey_body9::SBKEY_BODY9_SPEC>)
        }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub fn sbkey_key_code12(&self) -> &crate::Reg<sbkey_key_code12::SBKEY_KEY_CODE12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1248usize)
                as *const crate::Reg<sbkey_key_code12::SBKEY_KEY_CODE12_SPEC>)
        }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub fn sbkey_body10(&self) -> &crate::Reg<sbkey_body10::SBKEY_BODY10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1248usize)
                as *const crate::Reg<sbkey_body10::SBKEY_BODY10_SPEC>)
        }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub fn sbkey_key_code13(&self) -> &crate::Reg<sbkey_key_code13::SBKEY_KEY_CODE13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1252usize)
                as *const crate::Reg<sbkey_key_code13::SBKEY_KEY_CODE13_SPEC>)
        }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub fn sbkey_body11(&self) -> &crate::Reg<sbkey_body11::SBKEY_BODY11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1252usize)
                as *const crate::Reg<sbkey_body11::SBKEY_BODY11_SPEC>)
        }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub fn user_kek_key_code0(&self) -> &crate::Reg<user_kek_key_code0::USER_KEK_KEY_CODE0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1256usize)
                as *const crate::Reg<user_kek_key_code0::USER_KEK_KEY_CODE0_SPEC>)
        }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub fn user_kek_header0(&self) -> &crate::Reg<user_kek_header0::USER_KEK_HEADER0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1256usize)
                as *const crate::Reg<user_kek_header0::USER_KEK_HEADER0_SPEC>)
        }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub fn user_kek_key_code1(&self) -> &crate::Reg<user_kek_key_code1::USER_KEK_KEY_CODE1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1260usize)
                as *const crate::Reg<user_kek_key_code1::USER_KEK_KEY_CODE1_SPEC>)
        }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub fn user_kek_header1(&self) -> &crate::Reg<user_kek_header1::USER_KEK_HEADER1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1260usize)
                as *const crate::Reg<user_kek_header1::USER_KEK_HEADER1_SPEC>)
        }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub fn user_kek_key_code2(&self) -> &crate::Reg<user_kek_key_code2::USER_KEK_KEY_CODE2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1264usize)
                as *const crate::Reg<user_kek_key_code2::USER_KEK_KEY_CODE2_SPEC>)
        }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub fn user_kek_body0(&self) -> &crate::Reg<user_kek_body0::USER_KEK_BODY0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1264usize)
                as *const crate::Reg<user_kek_body0::USER_KEK_BODY0_SPEC>)
        }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub fn user_kek_key_code3(&self) -> &crate::Reg<user_kek_key_code3::USER_KEK_KEY_CODE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1268usize)
                as *const crate::Reg<user_kek_key_code3::USER_KEK_KEY_CODE3_SPEC>)
        }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub fn user_kek_body1(&self) -> &crate::Reg<user_kek_body1::USER_KEK_BODY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1268usize)
                as *const crate::Reg<user_kek_body1::USER_KEK_BODY1_SPEC>)
        }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub fn user_kek_key_code4(&self) -> &crate::Reg<user_kek_key_code4::USER_KEK_KEY_CODE4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1272usize)
                as *const crate::Reg<user_kek_key_code4::USER_KEK_KEY_CODE4_SPEC>)
        }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub fn user_kek_body2(&self) -> &crate::Reg<user_kek_body2::USER_KEK_BODY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1272usize)
                as *const crate::Reg<user_kek_body2::USER_KEK_BODY2_SPEC>)
        }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub fn user_kek_key_code5(&self) -> &crate::Reg<user_kek_key_code5::USER_KEK_KEY_CODE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1276usize)
                as *const crate::Reg<user_kek_key_code5::USER_KEK_KEY_CODE5_SPEC>)
        }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub fn user_kek_body3(&self) -> &crate::Reg<user_kek_body3::USER_KEK_BODY3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1276usize)
                as *const crate::Reg<user_kek_body3::USER_KEK_BODY3_SPEC>)
        }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub fn user_kek_key_code6(&self) -> &crate::Reg<user_kek_key_code6::USER_KEK_KEY_CODE6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize)
                as *const crate::Reg<user_kek_key_code6::USER_KEK_KEY_CODE6_SPEC>)
        }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub fn user_kek_body4(&self) -> &crate::Reg<user_kek_body4::USER_KEK_BODY4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize)
                as *const crate::Reg<user_kek_body4::USER_KEK_BODY4_SPEC>)
        }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub fn user_kek_key_code7(&self) -> &crate::Reg<user_kek_key_code7::USER_KEK_KEY_CODE7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1284usize)
                as *const crate::Reg<user_kek_key_code7::USER_KEK_KEY_CODE7_SPEC>)
        }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub fn user_kek_body5(&self) -> &crate::Reg<user_kek_body5::USER_KEK_BODY5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1284usize)
                as *const crate::Reg<user_kek_body5::USER_KEK_BODY5_SPEC>)
        }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub fn user_kek_key_code8(&self) -> &crate::Reg<user_kek_key_code8::USER_KEK_KEY_CODE8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1288usize)
                as *const crate::Reg<user_kek_key_code8::USER_KEK_KEY_CODE8_SPEC>)
        }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub fn user_kek_body6(&self) -> &crate::Reg<user_kek_body6::USER_KEK_BODY6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1288usize)
                as *const crate::Reg<user_kek_body6::USER_KEK_BODY6_SPEC>)
        }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub fn user_kek_key_code9(&self) -> &crate::Reg<user_kek_key_code9::USER_KEK_KEY_CODE9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1292usize)
                as *const crate::Reg<user_kek_key_code9::USER_KEK_KEY_CODE9_SPEC>)
        }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub fn user_kek_body7(&self) -> &crate::Reg<user_kek_body7::USER_KEK_BODY7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1292usize)
                as *const crate::Reg<user_kek_body7::USER_KEK_BODY7_SPEC>)
        }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub fn user_kek_key_code10(
        &self,
    ) -> &crate::Reg<user_kek_key_code10::USER_KEK_KEY_CODE10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1296usize)
                as *const crate::Reg<user_kek_key_code10::USER_KEK_KEY_CODE10_SPEC>)
        }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub fn user_kek_body8(&self) -> &crate::Reg<user_kek_body8::USER_KEK_BODY8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1296usize)
                as *const crate::Reg<user_kek_body8::USER_KEK_BODY8_SPEC>)
        }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub fn user_kek_key_code11(
        &self,
    ) -> &crate::Reg<user_kek_key_code11::USER_KEK_KEY_CODE11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1300usize)
                as *const crate::Reg<user_kek_key_code11::USER_KEK_KEY_CODE11_SPEC>)
        }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub fn user_kek_body9(&self) -> &crate::Reg<user_kek_body9::USER_KEK_BODY9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1300usize)
                as *const crate::Reg<user_kek_body9::USER_KEK_BODY9_SPEC>)
        }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub fn user_kek_key_code12(
        &self,
    ) -> &crate::Reg<user_kek_key_code12::USER_KEK_KEY_CODE12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1304usize)
                as *const crate::Reg<user_kek_key_code12::USER_KEK_KEY_CODE12_SPEC>)
        }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub fn user_kek_body10(&self) -> &crate::Reg<user_kek_body10::USER_KEK_BODY10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1304usize)
                as *const crate::Reg<user_kek_body10::USER_KEK_BODY10_SPEC>)
        }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub fn user_kek_key_code13(
        &self,
    ) -> &crate::Reg<user_kek_key_code13::USER_KEK_KEY_CODE13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1308usize)
                as *const crate::Reg<user_kek_key_code13::USER_KEK_KEY_CODE13_SPEC>)
        }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub fn user_kek_body11(&self) -> &crate::Reg<user_kek_body11::USER_KEK_BODY11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1308usize)
                as *const crate::Reg<user_kek_body11::USER_KEK_BODY11_SPEC>)
        }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub fn uds_key_code0(&self) -> &crate::Reg<uds_key_code0::UDS_KEY_CODE0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1312usize)
                as *const crate::Reg<uds_key_code0::UDS_KEY_CODE0_SPEC>)
        }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub fn uds_header0(&self) -> &crate::Reg<uds_header0::UDS_HEADER0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1312usize)
                as *const crate::Reg<uds_header0::UDS_HEADER0_SPEC>)
        }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub fn uds_key_code1(&self) -> &crate::Reg<uds_key_code1::UDS_KEY_CODE1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1316usize)
                as *const crate::Reg<uds_key_code1::UDS_KEY_CODE1_SPEC>)
        }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub fn uds_header1(&self) -> &crate::Reg<uds_header1::UDS_HEADER1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1316usize)
                as *const crate::Reg<uds_header1::UDS_HEADER1_SPEC>)
        }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub fn uds_key_code2(&self) -> &crate::Reg<uds_key_code2::UDS_KEY_CODE2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1320usize)
                as *const crate::Reg<uds_key_code2::UDS_KEY_CODE2_SPEC>)
        }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub fn uds_body0(&self) -> &crate::Reg<uds_body0::UDS_BODY0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1320usize)
                as *const crate::Reg<uds_body0::UDS_BODY0_SPEC>)
        }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub fn uds_key_code3(&self) -> &crate::Reg<uds_key_code3::UDS_KEY_CODE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1324usize)
                as *const crate::Reg<uds_key_code3::UDS_KEY_CODE3_SPEC>)
        }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub fn uds_body1(&self) -> &crate::Reg<uds_body1::UDS_BODY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1324usize)
                as *const crate::Reg<uds_body1::UDS_BODY1_SPEC>)
        }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub fn uds_key_code4(&self) -> &crate::Reg<uds_key_code4::UDS_KEY_CODE4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const crate::Reg<uds_key_code4::UDS_KEY_CODE4_SPEC>)
        }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub fn uds_body2(&self) -> &crate::Reg<uds_body2::UDS_BODY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize)
                as *const crate::Reg<uds_body2::UDS_BODY2_SPEC>)
        }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub fn uds_key_code5(&self) -> &crate::Reg<uds_key_code5::UDS_KEY_CODE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1332usize)
                as *const crate::Reg<uds_key_code5::UDS_KEY_CODE5_SPEC>)
        }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub fn uds_body3(&self) -> &crate::Reg<uds_body3::UDS_BODY3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1332usize)
                as *const crate::Reg<uds_body3::UDS_BODY3_SPEC>)
        }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub fn uds_key_code6(&self) -> &crate::Reg<uds_key_code6::UDS_KEY_CODE6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1336usize)
                as *const crate::Reg<uds_key_code6::UDS_KEY_CODE6_SPEC>)
        }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub fn uds_body4(&self) -> &crate::Reg<uds_body4::UDS_BODY4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1336usize)
                as *const crate::Reg<uds_body4::UDS_BODY4_SPEC>)
        }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub fn uds_key_code7(&self) -> &crate::Reg<uds_key_code7::UDS_KEY_CODE7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1340usize)
                as *const crate::Reg<uds_key_code7::UDS_KEY_CODE7_SPEC>)
        }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub fn uds_body5(&self) -> &crate::Reg<uds_body5::UDS_BODY5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1340usize)
                as *const crate::Reg<uds_body5::UDS_BODY5_SPEC>)
        }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub fn uds_key_code8(&self) -> &crate::Reg<uds_key_code8::UDS_KEY_CODE8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1344usize)
                as *const crate::Reg<uds_key_code8::UDS_KEY_CODE8_SPEC>)
        }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub fn uds_body6(&self) -> &crate::Reg<uds_body6::UDS_BODY6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1344usize)
                as *const crate::Reg<uds_body6::UDS_BODY6_SPEC>)
        }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub fn uds_key_code9(&self) -> &crate::Reg<uds_key_code9::UDS_KEY_CODE9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1348usize)
                as *const crate::Reg<uds_key_code9::UDS_KEY_CODE9_SPEC>)
        }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub fn uds_body7(&self) -> &crate::Reg<uds_body7::UDS_BODY7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1348usize)
                as *const crate::Reg<uds_body7::UDS_BODY7_SPEC>)
        }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub fn uds_key_code10(&self) -> &crate::Reg<uds_key_code10::UDS_KEY_CODE10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1352usize)
                as *const crate::Reg<uds_key_code10::UDS_KEY_CODE10_SPEC>)
        }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub fn uds_body8(&self) -> &crate::Reg<uds_body8::UDS_BODY8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1352usize)
                as *const crate::Reg<uds_body8::UDS_BODY8_SPEC>)
        }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub fn uds_key_code11(&self) -> &crate::Reg<uds_key_code11::UDS_KEY_CODE11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1356usize)
                as *const crate::Reg<uds_key_code11::UDS_KEY_CODE11_SPEC>)
        }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub fn uds_body9(&self) -> &crate::Reg<uds_body9::UDS_BODY9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1356usize)
                as *const crate::Reg<uds_body9::UDS_BODY9_SPEC>)
        }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub fn uds_key_code12(&self) -> &crate::Reg<uds_key_code12::UDS_KEY_CODE12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1360usize)
                as *const crate::Reg<uds_key_code12::UDS_KEY_CODE12_SPEC>)
        }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub fn uds_body10(&self) -> &crate::Reg<uds_body10::UDS_BODY10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1360usize)
                as *const crate::Reg<uds_body10::UDS_BODY10_SPEC>)
        }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub fn uds_key_code13(&self) -> &crate::Reg<uds_key_code13::UDS_KEY_CODE13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1364usize)
                as *const crate::Reg<uds_key_code13::UDS_KEY_CODE13_SPEC>)
        }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub fn uds_body11(&self) -> &crate::Reg<uds_body11::UDS_BODY11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1364usize)
                as *const crate::Reg<uds_body11::UDS_BODY11_SPEC>)
        }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code0(
        &self,
    ) -> &crate::Reg<prince_region0_key_code0::PRINCE_REGION0_KEY_CODE0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1368usize)
                as *const crate::Reg<prince_region0_key_code0::PRINCE_REGION0_KEY_CODE0_SPEC>)
        }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub fn prince_region0_header0(
        &self,
    ) -> &crate::Reg<prince_region0_header0::PRINCE_REGION0_HEADER0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1368usize)
                as *const crate::Reg<prince_region0_header0::PRINCE_REGION0_HEADER0_SPEC>)
        }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code1(
        &self,
    ) -> &crate::Reg<prince_region0_key_code1::PRINCE_REGION0_KEY_CODE1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1372usize)
                as *const crate::Reg<prince_region0_key_code1::PRINCE_REGION0_KEY_CODE1_SPEC>)
        }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub fn prince_region0_header1(
        &self,
    ) -> &crate::Reg<prince_region0_header1::PRINCE_REGION0_HEADER1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1372usize)
                as *const crate::Reg<prince_region0_header1::PRINCE_REGION0_HEADER1_SPEC>)
        }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code2(
        &self,
    ) -> &crate::Reg<prince_region0_key_code2::PRINCE_REGION0_KEY_CODE2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const crate::Reg<prince_region0_key_code2::PRINCE_REGION0_KEY_CODE2_SPEC>)
        }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub fn prince_region0_body0(
        &self,
    ) -> &crate::Reg<prince_region0_body0::PRINCE_REGION0_BODY0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize)
                as *const crate::Reg<prince_region0_body0::PRINCE_REGION0_BODY0_SPEC>)
        }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code3(
        &self,
    ) -> &crate::Reg<prince_region0_key_code3::PRINCE_REGION0_KEY_CODE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1380usize)
                as *const crate::Reg<prince_region0_key_code3::PRINCE_REGION0_KEY_CODE3_SPEC>)
        }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub fn prince_region0_body1(
        &self,
    ) -> &crate::Reg<prince_region0_body1::PRINCE_REGION0_BODY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1380usize)
                as *const crate::Reg<prince_region0_body1::PRINCE_REGION0_BODY1_SPEC>)
        }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code4(
        &self,
    ) -> &crate::Reg<prince_region0_key_code4::PRINCE_REGION0_KEY_CODE4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1384usize)
                as *const crate::Reg<prince_region0_key_code4::PRINCE_REGION0_KEY_CODE4_SPEC>)
        }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub fn prince_region0_body2(
        &self,
    ) -> &crate::Reg<prince_region0_body2::PRINCE_REGION0_BODY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1384usize)
                as *const crate::Reg<prince_region0_body2::PRINCE_REGION0_BODY2_SPEC>)
        }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code5(
        &self,
    ) -> &crate::Reg<prince_region0_key_code5::PRINCE_REGION0_KEY_CODE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1388usize)
                as *const crate::Reg<prince_region0_key_code5::PRINCE_REGION0_KEY_CODE5_SPEC>)
        }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub fn prince_region0_body3(
        &self,
    ) -> &crate::Reg<prince_region0_body3::PRINCE_REGION0_BODY3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1388usize)
                as *const crate::Reg<prince_region0_body3::PRINCE_REGION0_BODY3_SPEC>)
        }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code6(
        &self,
    ) -> &crate::Reg<prince_region0_key_code6::PRINCE_REGION0_KEY_CODE6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1392usize)
                as *const crate::Reg<prince_region0_key_code6::PRINCE_REGION0_KEY_CODE6_SPEC>)
        }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub fn prince_region0_body4(
        &self,
    ) -> &crate::Reg<prince_region0_body4::PRINCE_REGION0_BODY4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1392usize)
                as *const crate::Reg<prince_region0_body4::PRINCE_REGION0_BODY4_SPEC>)
        }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code7(
        &self,
    ) -> &crate::Reg<prince_region0_key_code7::PRINCE_REGION0_KEY_CODE7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1396usize)
                as *const crate::Reg<prince_region0_key_code7::PRINCE_REGION0_KEY_CODE7_SPEC>)
        }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub fn prince_region0_body5(
        &self,
    ) -> &crate::Reg<prince_region0_body5::PRINCE_REGION0_BODY5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1396usize)
                as *const crate::Reg<prince_region0_body5::PRINCE_REGION0_BODY5_SPEC>)
        }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code8(
        &self,
    ) -> &crate::Reg<prince_region0_key_code8::PRINCE_REGION0_KEY_CODE8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1400usize)
                as *const crate::Reg<prince_region0_key_code8::PRINCE_REGION0_KEY_CODE8_SPEC>)
        }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub fn prince_region0_body6(
        &self,
    ) -> &crate::Reg<prince_region0_body6::PRINCE_REGION0_BODY6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1400usize)
                as *const crate::Reg<prince_region0_body6::PRINCE_REGION0_BODY6_SPEC>)
        }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code9(
        &self,
    ) -> &crate::Reg<prince_region0_key_code9::PRINCE_REGION0_KEY_CODE9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1404usize)
                as *const crate::Reg<prince_region0_key_code9::PRINCE_REGION0_KEY_CODE9_SPEC>)
        }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub fn prince_region0_body7(
        &self,
    ) -> &crate::Reg<prince_region0_body7::PRINCE_REGION0_BODY7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1404usize)
                as *const crate::Reg<prince_region0_body7::PRINCE_REGION0_BODY7_SPEC>)
        }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code10(
        &self,
    ) -> &crate::Reg<prince_region0_key_code10::PRINCE_REGION0_KEY_CODE10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1408usize)
                as *const crate::Reg<prince_region0_key_code10::PRINCE_REGION0_KEY_CODE10_SPEC>)
        }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub fn prince_region0_body8(
        &self,
    ) -> &crate::Reg<prince_region0_body8::PRINCE_REGION0_BODY8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1408usize)
                as *const crate::Reg<prince_region0_body8::PRINCE_REGION0_BODY8_SPEC>)
        }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code11(
        &self,
    ) -> &crate::Reg<prince_region0_key_code11::PRINCE_REGION0_KEY_CODE11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1412usize)
                as *const crate::Reg<prince_region0_key_code11::PRINCE_REGION0_KEY_CODE11_SPEC>)
        }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub fn prince_region0_body9(
        &self,
    ) -> &crate::Reg<prince_region0_body9::PRINCE_REGION0_BODY9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1412usize)
                as *const crate::Reg<prince_region0_body9::PRINCE_REGION0_BODY9_SPEC>)
        }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub fn prince_region0_key_code12(
        &self,
    ) -> &crate::Reg<prince_region0_key_code12::PRINCE_REGION0_KEY_CODE12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1416usize)
                as *const crate::Reg<prince_region0_key_code12::PRINCE_REGION0_KEY_CODE12_SPEC>)
        }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub fn prince_region0_body10(
        &self,
    ) -> &crate::Reg<prince_region0_body10::PRINCE_REGION0_BODY10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1416usize)
                as *const crate::Reg<prince_region0_body10::PRINCE_REGION0_BODY10_SPEC>)
        }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub fn prince_region0_key_code13(
        &self,
    ) -> &crate::Reg<prince_region0_key_code13::PRINCE_REGION0_KEY_CODE13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1420usize)
                as *const crate::Reg<prince_region0_key_code13::PRINCE_REGION0_KEY_CODE13_SPEC>)
        }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub fn prince_region0_body11(
        &self,
    ) -> &crate::Reg<prince_region0_body11::PRINCE_REGION0_BODY11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1420usize)
                as *const crate::Reg<prince_region0_body11::PRINCE_REGION0_BODY11_SPEC>)
        }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code0(
        &self,
    ) -> &crate::Reg<prince_region1_key_code0::PRINCE_REGION1_KEY_CODE0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const crate::Reg<prince_region1_key_code0::PRINCE_REGION1_KEY_CODE0_SPEC>)
        }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub fn prince_region1_header0(
        &self,
    ) -> &crate::Reg<prince_region1_header0::PRINCE_REGION1_HEADER0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize)
                as *const crate::Reg<prince_region1_header0::PRINCE_REGION1_HEADER0_SPEC>)
        }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code1(
        &self,
    ) -> &crate::Reg<prince_region1_key_code1::PRINCE_REGION1_KEY_CODE1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1428usize)
                as *const crate::Reg<prince_region1_key_code1::PRINCE_REGION1_KEY_CODE1_SPEC>)
        }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub fn prince_region1_header1(
        &self,
    ) -> &crate::Reg<prince_region1_header1::PRINCE_REGION1_HEADER1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1428usize)
                as *const crate::Reg<prince_region1_header1::PRINCE_REGION1_HEADER1_SPEC>)
        }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code2(
        &self,
    ) -> &crate::Reg<prince_region1_key_code2::PRINCE_REGION1_KEY_CODE2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1432usize)
                as *const crate::Reg<prince_region1_key_code2::PRINCE_REGION1_KEY_CODE2_SPEC>)
        }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub fn prince_region1_body0(
        &self,
    ) -> &crate::Reg<prince_region1_body0::PRINCE_REGION1_BODY0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1432usize)
                as *const crate::Reg<prince_region1_body0::PRINCE_REGION1_BODY0_SPEC>)
        }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub fn prince_region1_key_code3(
        &self,
    ) -> &crate::Reg<prince_region1_key_code3::PRINCE_REGION1_KEY_CODE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1436usize)
                as *const crate::Reg<prince_region1_key_code3::PRINCE_REGION1_KEY_CODE3_SPEC>)
        }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub fn prince_region1_body1(
        &self,
    ) -> &crate::Reg<prince_region1_body1::PRINCE_REGION1_BODY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1436usize)
                as *const crate::Reg<prince_region1_body1::PRINCE_REGION1_BODY1_SPEC>)
        }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code4(
        &self,
    ) -> &crate::Reg<prince_region1_key_code4::PRINCE_REGION1_KEY_CODE4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1440usize)
                as *const crate::Reg<prince_region1_key_code4::PRINCE_REGION1_KEY_CODE4_SPEC>)
        }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub fn prince_region1_body2(
        &self,
    ) -> &crate::Reg<prince_region1_body2::PRINCE_REGION1_BODY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1440usize)
                as *const crate::Reg<prince_region1_body2::PRINCE_REGION1_BODY2_SPEC>)
        }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code5(
        &self,
    ) -> &crate::Reg<prince_region1_key_code5::PRINCE_REGION1_KEY_CODE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1444usize)
                as *const crate::Reg<prince_region1_key_code5::PRINCE_REGION1_KEY_CODE5_SPEC>)
        }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub fn prince_region1_body3(
        &self,
    ) -> &crate::Reg<prince_region1_body3::PRINCE_REGION1_BODY3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1444usize)
                as *const crate::Reg<prince_region1_body3::PRINCE_REGION1_BODY3_SPEC>)
        }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code6(
        &self,
    ) -> &crate::Reg<prince_region1_key_code6::PRINCE_REGION1_KEY_CODE6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1448usize)
                as *const crate::Reg<prince_region1_key_code6::PRINCE_REGION1_KEY_CODE6_SPEC>)
        }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub fn prince_region1_body4(
        &self,
    ) -> &crate::Reg<prince_region1_body4::PRINCE_REGION1_BODY4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1448usize)
                as *const crate::Reg<prince_region1_body4::PRINCE_REGION1_BODY4_SPEC>)
        }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub fn prince_region1_key_code7(
        &self,
    ) -> &crate::Reg<prince_region1_key_code7::PRINCE_REGION1_KEY_CODE7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1452usize)
                as *const crate::Reg<prince_region1_key_code7::PRINCE_REGION1_KEY_CODE7_SPEC>)
        }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub fn prince_region1_body5(
        &self,
    ) -> &crate::Reg<prince_region1_body5::PRINCE_REGION1_BODY5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1452usize)
                as *const crate::Reg<prince_region1_body5::PRINCE_REGION1_BODY5_SPEC>)
        }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code8(
        &self,
    ) -> &crate::Reg<prince_region1_key_code8::PRINCE_REGION1_KEY_CODE8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1456usize)
                as *const crate::Reg<prince_region1_key_code8::PRINCE_REGION1_KEY_CODE8_SPEC>)
        }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub fn prince_region1_body6(
        &self,
    ) -> &crate::Reg<prince_region1_body6::PRINCE_REGION1_BODY6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1456usize)
                as *const crate::Reg<prince_region1_body6::PRINCE_REGION1_BODY6_SPEC>)
        }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code9(
        &self,
    ) -> &crate::Reg<prince_region1_key_code9::PRINCE_REGION1_KEY_CODE9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1460usize)
                as *const crate::Reg<prince_region1_key_code9::PRINCE_REGION1_KEY_CODE9_SPEC>)
        }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub fn prince_region1_body7(
        &self,
    ) -> &crate::Reg<prince_region1_body7::PRINCE_REGION1_BODY7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1460usize)
                as *const crate::Reg<prince_region1_body7::PRINCE_REGION1_BODY7_SPEC>)
        }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code10(
        &self,
    ) -> &crate::Reg<prince_region1_key_code10::PRINCE_REGION1_KEY_CODE10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1464usize)
                as *const crate::Reg<prince_region1_key_code10::PRINCE_REGION1_KEY_CODE10_SPEC>)
        }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub fn prince_region1_body8(
        &self,
    ) -> &crate::Reg<prince_region1_body8::PRINCE_REGION1_BODY8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1464usize)
                as *const crate::Reg<prince_region1_body8::PRINCE_REGION1_BODY8_SPEC>)
        }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub fn prince_region1_key_code11(
        &self,
    ) -> &crate::Reg<prince_region1_key_code11::PRINCE_REGION1_KEY_CODE11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1468usize)
                as *const crate::Reg<prince_region1_key_code11::PRINCE_REGION1_KEY_CODE11_SPEC>)
        }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub fn prince_region1_body9(
        &self,
    ) -> &crate::Reg<prince_region1_body9::PRINCE_REGION1_BODY9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1468usize)
                as *const crate::Reg<prince_region1_body9::PRINCE_REGION1_BODY9_SPEC>)
        }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code12(
        &self,
    ) -> &crate::Reg<prince_region1_key_code12::PRINCE_REGION1_KEY_CODE12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const crate::Reg<prince_region1_key_code12::PRINCE_REGION1_KEY_CODE12_SPEC>)
        }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub fn prince_region1_body10(
        &self,
    ) -> &crate::Reg<prince_region1_body10::PRINCE_REGION1_BODY10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize)
                as *const crate::Reg<prince_region1_body10::PRINCE_REGION1_BODY10_SPEC>)
        }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub fn prince_region1_key_code13(
        &self,
    ) -> &crate::Reg<prince_region1_key_code13::PRINCE_REGION1_KEY_CODE13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1476usize)
                as *const crate::Reg<prince_region1_key_code13::PRINCE_REGION1_KEY_CODE13_SPEC>)
        }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub fn prince_region1_body11(
        &self,
    ) -> &crate::Reg<prince_region1_body11::PRINCE_REGION1_BODY11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1476usize)
                as *const crate::Reg<prince_region1_body11::PRINCE_REGION1_BODY11_SPEC>)
        }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code0(
        &self,
    ) -> &crate::Reg<prince_region2_key_code0::PRINCE_REGION2_KEY_CODE0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1480usize)
                as *const crate::Reg<prince_region2_key_code0::PRINCE_REGION2_KEY_CODE0_SPEC>)
        }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub fn prince_region2_header0(
        &self,
    ) -> &crate::Reg<prince_region2_header0::PRINCE_REGION2_HEADER0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1480usize)
                as *const crate::Reg<prince_region2_header0::PRINCE_REGION2_HEADER0_SPEC>)
        }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub fn prince_region2_key_code1(
        &self,
    ) -> &crate::Reg<prince_region2_key_code1::PRINCE_REGION2_KEY_CODE1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1484usize)
                as *const crate::Reg<prince_region2_key_code1::PRINCE_REGION2_KEY_CODE1_SPEC>)
        }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub fn prince_region2_header1(
        &self,
    ) -> &crate::Reg<prince_region2_header1::PRINCE_REGION2_HEADER1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1484usize)
                as *const crate::Reg<prince_region2_header1::PRINCE_REGION2_HEADER1_SPEC>)
        }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code2(
        &self,
    ) -> &crate::Reg<prince_region2_key_code2::PRINCE_REGION2_KEY_CODE2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1488usize)
                as *const crate::Reg<prince_region2_key_code2::PRINCE_REGION2_KEY_CODE2_SPEC>)
        }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub fn prince_region2_body0(
        &self,
    ) -> &crate::Reg<prince_region2_body0::PRINCE_REGION2_BODY0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1488usize)
                as *const crate::Reg<prince_region2_body0::PRINCE_REGION2_BODY0_SPEC>)
        }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code3(
        &self,
    ) -> &crate::Reg<prince_region2_key_code3::PRINCE_REGION2_KEY_CODE3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1492usize)
                as *const crate::Reg<prince_region2_key_code3::PRINCE_REGION2_KEY_CODE3_SPEC>)
        }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub fn prince_region2_body1(
        &self,
    ) -> &crate::Reg<prince_region2_body1::PRINCE_REGION2_BODY1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1492usize)
                as *const crate::Reg<prince_region2_body1::PRINCE_REGION2_BODY1_SPEC>)
        }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code4(
        &self,
    ) -> &crate::Reg<prince_region2_key_code4::PRINCE_REGION2_KEY_CODE4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1496usize)
                as *const crate::Reg<prince_region2_key_code4::PRINCE_REGION2_KEY_CODE4_SPEC>)
        }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub fn prince_region2_body2(
        &self,
    ) -> &crate::Reg<prince_region2_body2::PRINCE_REGION2_BODY2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1496usize)
                as *const crate::Reg<prince_region2_body2::PRINCE_REGION2_BODY2_SPEC>)
        }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub fn prince_region2_key_code5(
        &self,
    ) -> &crate::Reg<prince_region2_key_code5::PRINCE_REGION2_KEY_CODE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1500usize)
                as *const crate::Reg<prince_region2_key_code5::PRINCE_REGION2_KEY_CODE5_SPEC>)
        }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub fn prince_region2_body3(
        &self,
    ) -> &crate::Reg<prince_region2_body3::PRINCE_REGION2_BODY3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1500usize)
                as *const crate::Reg<prince_region2_body3::PRINCE_REGION2_BODY3_SPEC>)
        }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code6(
        &self,
    ) -> &crate::Reg<prince_region2_key_code6::PRINCE_REGION2_KEY_CODE6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1504usize)
                as *const crate::Reg<prince_region2_key_code6::PRINCE_REGION2_KEY_CODE6_SPEC>)
        }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub fn prince_region2_body4(
        &self,
    ) -> &crate::Reg<prince_region2_body4::PRINCE_REGION2_BODY4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1504usize)
                as *const crate::Reg<prince_region2_body4::PRINCE_REGION2_BODY4_SPEC>)
        }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code7(
        &self,
    ) -> &crate::Reg<prince_region2_key_code7::PRINCE_REGION2_KEY_CODE7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1508usize)
                as *const crate::Reg<prince_region2_key_code7::PRINCE_REGION2_KEY_CODE7_SPEC>)
        }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub fn prince_region2_body5(
        &self,
    ) -> &crate::Reg<prince_region2_body5::PRINCE_REGION2_BODY5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1508usize)
                as *const crate::Reg<prince_region2_body5::PRINCE_REGION2_BODY5_SPEC>)
        }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code8(
        &self,
    ) -> &crate::Reg<prince_region2_key_code8::PRINCE_REGION2_KEY_CODE8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1512usize)
                as *const crate::Reg<prince_region2_key_code8::PRINCE_REGION2_KEY_CODE8_SPEC>)
        }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub fn prince_region2_body6(
        &self,
    ) -> &crate::Reg<prince_region2_body6::PRINCE_REGION2_BODY6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1512usize)
                as *const crate::Reg<prince_region2_body6::PRINCE_REGION2_BODY6_SPEC>)
        }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub fn prince_region2_key_code9(
        &self,
    ) -> &crate::Reg<prince_region2_key_code9::PRINCE_REGION2_KEY_CODE9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1516usize)
                as *const crate::Reg<prince_region2_key_code9::PRINCE_REGION2_KEY_CODE9_SPEC>)
        }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub fn prince_region2_body7(
        &self,
    ) -> &crate::Reg<prince_region2_body7::PRINCE_REGION2_BODY7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1516usize)
                as *const crate::Reg<prince_region2_body7::PRINCE_REGION2_BODY7_SPEC>)
        }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code10(
        &self,
    ) -> &crate::Reg<prince_region2_key_code10::PRINCE_REGION2_KEY_CODE10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const crate::Reg<prince_region2_key_code10::PRINCE_REGION2_KEY_CODE10_SPEC>)
        }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub fn prince_region2_body8(
        &self,
    ) -> &crate::Reg<prince_region2_body8::PRINCE_REGION2_BODY8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize)
                as *const crate::Reg<prince_region2_body8::PRINCE_REGION2_BODY8_SPEC>)
        }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code11(
        &self,
    ) -> &crate::Reg<prince_region2_key_code11::PRINCE_REGION2_KEY_CODE11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1524usize)
                as *const crate::Reg<prince_region2_key_code11::PRINCE_REGION2_KEY_CODE11_SPEC>)
        }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub fn prince_region2_body9(
        &self,
    ) -> &crate::Reg<prince_region2_body9::PRINCE_REGION2_BODY9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1524usize)
                as *const crate::Reg<prince_region2_body9::PRINCE_REGION2_BODY9_SPEC>)
        }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub fn prince_region2_key_code12(
        &self,
    ) -> &crate::Reg<prince_region2_key_code12::PRINCE_REGION2_KEY_CODE12_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1528usize)
                as *const crate::Reg<prince_region2_key_code12::PRINCE_REGION2_KEY_CODE12_SPEC>)
        }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub fn prince_region2_body10(
        &self,
    ) -> &crate::Reg<prince_region2_body10::PRINCE_REGION2_BODY10_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1528usize)
                as *const crate::Reg<prince_region2_body10::PRINCE_REGION2_BODY10_SPEC>)
        }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub fn prince_region2_key_code13(
        &self,
    ) -> &crate::Reg<prince_region2_key_code13::PRINCE_REGION2_KEY_CODE13_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1532usize)
                as *const crate::Reg<prince_region2_key_code13::PRINCE_REGION2_KEY_CODE13_SPEC>)
        }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub fn prince_region2_body11(
        &self,
    ) -> &crate::Reg<prince_region2_body11::PRINCE_REGION2_BODY11_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1532usize)
                as *const crate::Reg<prince_region2_body11::PRINCE_REGION2_BODY11_SPEC>)
        }
    }
}
#[doc = "HEADER register accessor: an alias for `Reg<HEADER_SPEC>`"]
pub type HEADER = crate::Reg<header::HEADER_SPEC>;
#[doc = "Valid Key Sore Header : 0x95959595"]
pub mod header;
#[doc = "puf_discharge_time_in_ms register accessor: an alias for `Reg<PUF_DISCHARGE_TIME_IN_MS_SPEC>`"]
pub type PUF_DISCHARGE_TIME_IN_MS =
    crate::Reg<puf_discharge_time_in_ms::PUF_DISCHARGE_TIME_IN_MS_SPEC>;
#[doc = "puf discharge time in ms."]
pub mod puf_discharge_time_in_ms;
#[doc = "ACTIVATION_CODE register accessor: an alias for `Reg<ACTIVATION_CODE_SPEC>`"]
pub type ACTIVATION_CODE = crate::Reg<activation_code::ACTIVATION_CODE_SPEC>;
#[doc = "."]
pub mod activation_code;
#[doc = "SBKEY_HEADER0 register accessor: an alias for `Reg<SBKEY_HEADER0_SPEC>`"]
pub type SBKEY_HEADER0 = crate::Reg<sbkey_header0::SBKEY_HEADER0_SPEC>;
#[doc = "."]
pub mod sbkey_header0;
#[doc = "SBKEY_KEY_CODE0 register accessor: an alias for `Reg<SBKEY_KEY_CODE0_SPEC>`"]
pub type SBKEY_KEY_CODE0 = crate::Reg<sbkey_key_code0::SBKEY_KEY_CODE0_SPEC>;
#[doc = "."]
pub mod sbkey_key_code0;
#[doc = "SBKEY_HEADER1 register accessor: an alias for `Reg<SBKEY_HEADER1_SPEC>`"]
pub type SBKEY_HEADER1 = crate::Reg<sbkey_header1::SBKEY_HEADER1_SPEC>;
#[doc = "."]
pub mod sbkey_header1;
#[doc = "SBKEY_KEY_CODE1 register accessor: an alias for `Reg<SBKEY_KEY_CODE1_SPEC>`"]
pub type SBKEY_KEY_CODE1 = crate::Reg<sbkey_key_code1::SBKEY_KEY_CODE1_SPEC>;
#[doc = "."]
pub mod sbkey_key_code1;
#[doc = "SBKEY_BODY0 register accessor: an alias for `Reg<SBKEY_BODY0_SPEC>`"]
pub type SBKEY_BODY0 = crate::Reg<sbkey_body0::SBKEY_BODY0_SPEC>;
#[doc = "."]
pub mod sbkey_body0;
#[doc = "SBKEY_KEY_CODE2 register accessor: an alias for `Reg<SBKEY_KEY_CODE2_SPEC>`"]
pub type SBKEY_KEY_CODE2 = crate::Reg<sbkey_key_code2::SBKEY_KEY_CODE2_SPEC>;
#[doc = "."]
pub mod sbkey_key_code2;
#[doc = "SBKEY_BODY1 register accessor: an alias for `Reg<SBKEY_BODY1_SPEC>`"]
pub type SBKEY_BODY1 = crate::Reg<sbkey_body1::SBKEY_BODY1_SPEC>;
#[doc = "."]
pub mod sbkey_body1;
#[doc = "SBKEY_KEY_CODE3 register accessor: an alias for `Reg<SBKEY_KEY_CODE3_SPEC>`"]
pub type SBKEY_KEY_CODE3 = crate::Reg<sbkey_key_code3::SBKEY_KEY_CODE3_SPEC>;
#[doc = "."]
pub mod sbkey_key_code3;
#[doc = "SBKEY_BODY2 register accessor: an alias for `Reg<SBKEY_BODY2_SPEC>`"]
pub type SBKEY_BODY2 = crate::Reg<sbkey_body2::SBKEY_BODY2_SPEC>;
#[doc = "."]
pub mod sbkey_body2;
#[doc = "SBKEY_KEY_CODE4 register accessor: an alias for `Reg<SBKEY_KEY_CODE4_SPEC>`"]
pub type SBKEY_KEY_CODE4 = crate::Reg<sbkey_key_code4::SBKEY_KEY_CODE4_SPEC>;
#[doc = "."]
pub mod sbkey_key_code4;
#[doc = "SBKEY_BODY3 register accessor: an alias for `Reg<SBKEY_BODY3_SPEC>`"]
pub type SBKEY_BODY3 = crate::Reg<sbkey_body3::SBKEY_BODY3_SPEC>;
#[doc = "."]
pub mod sbkey_body3;
#[doc = "SBKEY_KEY_CODE5 register accessor: an alias for `Reg<SBKEY_KEY_CODE5_SPEC>`"]
pub type SBKEY_KEY_CODE5 = crate::Reg<sbkey_key_code5::SBKEY_KEY_CODE5_SPEC>;
#[doc = "."]
pub mod sbkey_key_code5;
#[doc = "SBKEY_BODY4 register accessor: an alias for `Reg<SBKEY_BODY4_SPEC>`"]
pub type SBKEY_BODY4 = crate::Reg<sbkey_body4::SBKEY_BODY4_SPEC>;
#[doc = "."]
pub mod sbkey_body4;
#[doc = "SBKEY_KEY_CODE6 register accessor: an alias for `Reg<SBKEY_KEY_CODE6_SPEC>`"]
pub type SBKEY_KEY_CODE6 = crate::Reg<sbkey_key_code6::SBKEY_KEY_CODE6_SPEC>;
#[doc = "."]
pub mod sbkey_key_code6;
#[doc = "SBKEY_BODY5 register accessor: an alias for `Reg<SBKEY_BODY5_SPEC>`"]
pub type SBKEY_BODY5 = crate::Reg<sbkey_body5::SBKEY_BODY5_SPEC>;
#[doc = "."]
pub mod sbkey_body5;
#[doc = "SBKEY_KEY_CODE7 register accessor: an alias for `Reg<SBKEY_KEY_CODE7_SPEC>`"]
pub type SBKEY_KEY_CODE7 = crate::Reg<sbkey_key_code7::SBKEY_KEY_CODE7_SPEC>;
#[doc = "."]
pub mod sbkey_key_code7;
#[doc = "SBKEY_BODY6 register accessor: an alias for `Reg<SBKEY_BODY6_SPEC>`"]
pub type SBKEY_BODY6 = crate::Reg<sbkey_body6::SBKEY_BODY6_SPEC>;
#[doc = "."]
pub mod sbkey_body6;
#[doc = "SBKEY_KEY_CODE8 register accessor: an alias for `Reg<SBKEY_KEY_CODE8_SPEC>`"]
pub type SBKEY_KEY_CODE8 = crate::Reg<sbkey_key_code8::SBKEY_KEY_CODE8_SPEC>;
#[doc = "."]
pub mod sbkey_key_code8;
#[doc = "SBKEY_BODY7 register accessor: an alias for `Reg<SBKEY_BODY7_SPEC>`"]
pub type SBKEY_BODY7 = crate::Reg<sbkey_body7::SBKEY_BODY7_SPEC>;
#[doc = "."]
pub mod sbkey_body7;
#[doc = "SBKEY_KEY_CODE9 register accessor: an alias for `Reg<SBKEY_KEY_CODE9_SPEC>`"]
pub type SBKEY_KEY_CODE9 = crate::Reg<sbkey_key_code9::SBKEY_KEY_CODE9_SPEC>;
#[doc = "."]
pub mod sbkey_key_code9;
#[doc = "SBKEY_BODY8 register accessor: an alias for `Reg<SBKEY_BODY8_SPEC>`"]
pub type SBKEY_BODY8 = crate::Reg<sbkey_body8::SBKEY_BODY8_SPEC>;
#[doc = "."]
pub mod sbkey_body8;
#[doc = "SBKEY_KEY_CODE10 register accessor: an alias for `Reg<SBKEY_KEY_CODE10_SPEC>`"]
pub type SBKEY_KEY_CODE10 = crate::Reg<sbkey_key_code10::SBKEY_KEY_CODE10_SPEC>;
#[doc = "."]
pub mod sbkey_key_code10;
#[doc = "SBKEY_BODY9 register accessor: an alias for `Reg<SBKEY_BODY9_SPEC>`"]
pub type SBKEY_BODY9 = crate::Reg<sbkey_body9::SBKEY_BODY9_SPEC>;
#[doc = "."]
pub mod sbkey_body9;
#[doc = "SBKEY_KEY_CODE11 register accessor: an alias for `Reg<SBKEY_KEY_CODE11_SPEC>`"]
pub type SBKEY_KEY_CODE11 = crate::Reg<sbkey_key_code11::SBKEY_KEY_CODE11_SPEC>;
#[doc = "."]
pub mod sbkey_key_code11;
#[doc = "SBKEY_BODY10 register accessor: an alias for `Reg<SBKEY_BODY10_SPEC>`"]
pub type SBKEY_BODY10 = crate::Reg<sbkey_body10::SBKEY_BODY10_SPEC>;
#[doc = "."]
pub mod sbkey_body10;
#[doc = "SBKEY_KEY_CODE12 register accessor: an alias for `Reg<SBKEY_KEY_CODE12_SPEC>`"]
pub type SBKEY_KEY_CODE12 = crate::Reg<sbkey_key_code12::SBKEY_KEY_CODE12_SPEC>;
#[doc = "."]
pub mod sbkey_key_code12;
#[doc = "SBKEY_BODY11 register accessor: an alias for `Reg<SBKEY_BODY11_SPEC>`"]
pub type SBKEY_BODY11 = crate::Reg<sbkey_body11::SBKEY_BODY11_SPEC>;
#[doc = "."]
pub mod sbkey_body11;
#[doc = "SBKEY_KEY_CODE13 register accessor: an alias for `Reg<SBKEY_KEY_CODE13_SPEC>`"]
pub type SBKEY_KEY_CODE13 = crate::Reg<sbkey_key_code13::SBKEY_KEY_CODE13_SPEC>;
#[doc = "."]
pub mod sbkey_key_code13;
#[doc = "USER_KEK_HEADER0 register accessor: an alias for `Reg<USER_KEK_HEADER0_SPEC>`"]
pub type USER_KEK_HEADER0 = crate::Reg<user_kek_header0::USER_KEK_HEADER0_SPEC>;
#[doc = "."]
pub mod user_kek_header0;
#[doc = "USER_KEK_KEY_CODE0 register accessor: an alias for `Reg<USER_KEK_KEY_CODE0_SPEC>`"]
pub type USER_KEK_KEY_CODE0 = crate::Reg<user_kek_key_code0::USER_KEK_KEY_CODE0_SPEC>;
#[doc = "."]
pub mod user_kek_key_code0;
#[doc = "USER_KEK_HEADER1 register accessor: an alias for `Reg<USER_KEK_HEADER1_SPEC>`"]
pub type USER_KEK_HEADER1 = crate::Reg<user_kek_header1::USER_KEK_HEADER1_SPEC>;
#[doc = "."]
pub mod user_kek_header1;
#[doc = "USER_KEK_KEY_CODE1 register accessor: an alias for `Reg<USER_KEK_KEY_CODE1_SPEC>`"]
pub type USER_KEK_KEY_CODE1 = crate::Reg<user_kek_key_code1::USER_KEK_KEY_CODE1_SPEC>;
#[doc = "."]
pub mod user_kek_key_code1;
#[doc = "USER_KEK_BODY0 register accessor: an alias for `Reg<USER_KEK_BODY0_SPEC>`"]
pub type USER_KEK_BODY0 = crate::Reg<user_kek_body0::USER_KEK_BODY0_SPEC>;
#[doc = "."]
pub mod user_kek_body0;
#[doc = "USER_KEK_KEY_CODE2 register accessor: an alias for `Reg<USER_KEK_KEY_CODE2_SPEC>`"]
pub type USER_KEK_KEY_CODE2 = crate::Reg<user_kek_key_code2::USER_KEK_KEY_CODE2_SPEC>;
#[doc = "."]
pub mod user_kek_key_code2;
#[doc = "USER_KEK_BODY1 register accessor: an alias for `Reg<USER_KEK_BODY1_SPEC>`"]
pub type USER_KEK_BODY1 = crate::Reg<user_kek_body1::USER_KEK_BODY1_SPEC>;
#[doc = "."]
pub mod user_kek_body1;
#[doc = "USER_KEK_KEY_CODE3 register accessor: an alias for `Reg<USER_KEK_KEY_CODE3_SPEC>`"]
pub type USER_KEK_KEY_CODE3 = crate::Reg<user_kek_key_code3::USER_KEK_KEY_CODE3_SPEC>;
#[doc = "."]
pub mod user_kek_key_code3;
#[doc = "USER_KEK_BODY2 register accessor: an alias for `Reg<USER_KEK_BODY2_SPEC>`"]
pub type USER_KEK_BODY2 = crate::Reg<user_kek_body2::USER_KEK_BODY2_SPEC>;
#[doc = "."]
pub mod user_kek_body2;
#[doc = "USER_KEK_KEY_CODE4 register accessor: an alias for `Reg<USER_KEK_KEY_CODE4_SPEC>`"]
pub type USER_KEK_KEY_CODE4 = crate::Reg<user_kek_key_code4::USER_KEK_KEY_CODE4_SPEC>;
#[doc = "."]
pub mod user_kek_key_code4;
#[doc = "USER_KEK_BODY3 register accessor: an alias for `Reg<USER_KEK_BODY3_SPEC>`"]
pub type USER_KEK_BODY3 = crate::Reg<user_kek_body3::USER_KEK_BODY3_SPEC>;
#[doc = "."]
pub mod user_kek_body3;
#[doc = "USER_KEK_KEY_CODE5 register accessor: an alias for `Reg<USER_KEK_KEY_CODE5_SPEC>`"]
pub type USER_KEK_KEY_CODE5 = crate::Reg<user_kek_key_code5::USER_KEK_KEY_CODE5_SPEC>;
#[doc = "."]
pub mod user_kek_key_code5;
#[doc = "USER_KEK_BODY4 register accessor: an alias for `Reg<USER_KEK_BODY4_SPEC>`"]
pub type USER_KEK_BODY4 = crate::Reg<user_kek_body4::USER_KEK_BODY4_SPEC>;
#[doc = "."]
pub mod user_kek_body4;
#[doc = "USER_KEK_KEY_CODE6 register accessor: an alias for `Reg<USER_KEK_KEY_CODE6_SPEC>`"]
pub type USER_KEK_KEY_CODE6 = crate::Reg<user_kek_key_code6::USER_KEK_KEY_CODE6_SPEC>;
#[doc = "."]
pub mod user_kek_key_code6;
#[doc = "USER_KEK_BODY5 register accessor: an alias for `Reg<USER_KEK_BODY5_SPEC>`"]
pub type USER_KEK_BODY5 = crate::Reg<user_kek_body5::USER_KEK_BODY5_SPEC>;
#[doc = "."]
pub mod user_kek_body5;
#[doc = "USER_KEK_KEY_CODE7 register accessor: an alias for `Reg<USER_KEK_KEY_CODE7_SPEC>`"]
pub type USER_KEK_KEY_CODE7 = crate::Reg<user_kek_key_code7::USER_KEK_KEY_CODE7_SPEC>;
#[doc = "."]
pub mod user_kek_key_code7;
#[doc = "USER_KEK_BODY6 register accessor: an alias for `Reg<USER_KEK_BODY6_SPEC>`"]
pub type USER_KEK_BODY6 = crate::Reg<user_kek_body6::USER_KEK_BODY6_SPEC>;
#[doc = "."]
pub mod user_kek_body6;
#[doc = "USER_KEK_KEY_CODE8 register accessor: an alias for `Reg<USER_KEK_KEY_CODE8_SPEC>`"]
pub type USER_KEK_KEY_CODE8 = crate::Reg<user_kek_key_code8::USER_KEK_KEY_CODE8_SPEC>;
#[doc = "."]
pub mod user_kek_key_code8;
#[doc = "USER_KEK_BODY7 register accessor: an alias for `Reg<USER_KEK_BODY7_SPEC>`"]
pub type USER_KEK_BODY7 = crate::Reg<user_kek_body7::USER_KEK_BODY7_SPEC>;
#[doc = "."]
pub mod user_kek_body7;
#[doc = "USER_KEK_KEY_CODE9 register accessor: an alias for `Reg<USER_KEK_KEY_CODE9_SPEC>`"]
pub type USER_KEK_KEY_CODE9 = crate::Reg<user_kek_key_code9::USER_KEK_KEY_CODE9_SPEC>;
#[doc = "."]
pub mod user_kek_key_code9;
#[doc = "USER_KEK_BODY8 register accessor: an alias for `Reg<USER_KEK_BODY8_SPEC>`"]
pub type USER_KEK_BODY8 = crate::Reg<user_kek_body8::USER_KEK_BODY8_SPEC>;
#[doc = "."]
pub mod user_kek_body8;
#[doc = "USER_KEK_KEY_CODE10 register accessor: an alias for `Reg<USER_KEK_KEY_CODE10_SPEC>`"]
pub type USER_KEK_KEY_CODE10 = crate::Reg<user_kek_key_code10::USER_KEK_KEY_CODE10_SPEC>;
#[doc = "."]
pub mod user_kek_key_code10;
#[doc = "USER_KEK_BODY9 register accessor: an alias for `Reg<USER_KEK_BODY9_SPEC>`"]
pub type USER_KEK_BODY9 = crate::Reg<user_kek_body9::USER_KEK_BODY9_SPEC>;
#[doc = "."]
pub mod user_kek_body9;
#[doc = "USER_KEK_KEY_CODE11 register accessor: an alias for `Reg<USER_KEK_KEY_CODE11_SPEC>`"]
pub type USER_KEK_KEY_CODE11 = crate::Reg<user_kek_key_code11::USER_KEK_KEY_CODE11_SPEC>;
#[doc = "."]
pub mod user_kek_key_code11;
#[doc = "USER_KEK_BODY10 register accessor: an alias for `Reg<USER_KEK_BODY10_SPEC>`"]
pub type USER_KEK_BODY10 = crate::Reg<user_kek_body10::USER_KEK_BODY10_SPEC>;
#[doc = "."]
pub mod user_kek_body10;
#[doc = "USER_KEK_KEY_CODE12 register accessor: an alias for `Reg<USER_KEK_KEY_CODE12_SPEC>`"]
pub type USER_KEK_KEY_CODE12 = crate::Reg<user_kek_key_code12::USER_KEK_KEY_CODE12_SPEC>;
#[doc = "."]
pub mod user_kek_key_code12;
#[doc = "USER_KEK_BODY11 register accessor: an alias for `Reg<USER_KEK_BODY11_SPEC>`"]
pub type USER_KEK_BODY11 = crate::Reg<user_kek_body11::USER_KEK_BODY11_SPEC>;
#[doc = "."]
pub mod user_kek_body11;
#[doc = "USER_KEK_KEY_CODE13 register accessor: an alias for `Reg<USER_KEK_KEY_CODE13_SPEC>`"]
pub type USER_KEK_KEY_CODE13 = crate::Reg<user_kek_key_code13::USER_KEK_KEY_CODE13_SPEC>;
#[doc = "."]
pub mod user_kek_key_code13;
#[doc = "UDS_HEADER0 register accessor: an alias for `Reg<UDS_HEADER0_SPEC>`"]
pub type UDS_HEADER0 = crate::Reg<uds_header0::UDS_HEADER0_SPEC>;
#[doc = "."]
pub mod uds_header0;
#[doc = "UDS_KEY_CODE0 register accessor: an alias for `Reg<UDS_KEY_CODE0_SPEC>`"]
pub type UDS_KEY_CODE0 = crate::Reg<uds_key_code0::UDS_KEY_CODE0_SPEC>;
#[doc = "."]
pub mod uds_key_code0;
#[doc = "UDS_HEADER1 register accessor: an alias for `Reg<UDS_HEADER1_SPEC>`"]
pub type UDS_HEADER1 = crate::Reg<uds_header1::UDS_HEADER1_SPEC>;
#[doc = "."]
pub mod uds_header1;
#[doc = "UDS_KEY_CODE1 register accessor: an alias for `Reg<UDS_KEY_CODE1_SPEC>`"]
pub type UDS_KEY_CODE1 = crate::Reg<uds_key_code1::UDS_KEY_CODE1_SPEC>;
#[doc = "."]
pub mod uds_key_code1;
#[doc = "UDS_BODY0 register accessor: an alias for `Reg<UDS_BODY0_SPEC>`"]
pub type UDS_BODY0 = crate::Reg<uds_body0::UDS_BODY0_SPEC>;
#[doc = "."]
pub mod uds_body0;
#[doc = "UDS_KEY_CODE2 register accessor: an alias for `Reg<UDS_KEY_CODE2_SPEC>`"]
pub type UDS_KEY_CODE2 = crate::Reg<uds_key_code2::UDS_KEY_CODE2_SPEC>;
#[doc = "."]
pub mod uds_key_code2;
#[doc = "UDS_BODY1 register accessor: an alias for `Reg<UDS_BODY1_SPEC>`"]
pub type UDS_BODY1 = crate::Reg<uds_body1::UDS_BODY1_SPEC>;
#[doc = "."]
pub mod uds_body1;
#[doc = "UDS_KEY_CODE3 register accessor: an alias for `Reg<UDS_KEY_CODE3_SPEC>`"]
pub type UDS_KEY_CODE3 = crate::Reg<uds_key_code3::UDS_KEY_CODE3_SPEC>;
#[doc = "."]
pub mod uds_key_code3;
#[doc = "UDS_BODY2 register accessor: an alias for `Reg<UDS_BODY2_SPEC>`"]
pub type UDS_BODY2 = crate::Reg<uds_body2::UDS_BODY2_SPEC>;
#[doc = "."]
pub mod uds_body2;
#[doc = "UDS_KEY_CODE4 register accessor: an alias for `Reg<UDS_KEY_CODE4_SPEC>`"]
pub type UDS_KEY_CODE4 = crate::Reg<uds_key_code4::UDS_KEY_CODE4_SPEC>;
#[doc = "."]
pub mod uds_key_code4;
#[doc = "UDS_BODY3 register accessor: an alias for `Reg<UDS_BODY3_SPEC>`"]
pub type UDS_BODY3 = crate::Reg<uds_body3::UDS_BODY3_SPEC>;
#[doc = "."]
pub mod uds_body3;
#[doc = "UDS_KEY_CODE5 register accessor: an alias for `Reg<UDS_KEY_CODE5_SPEC>`"]
pub type UDS_KEY_CODE5 = crate::Reg<uds_key_code5::UDS_KEY_CODE5_SPEC>;
#[doc = "."]
pub mod uds_key_code5;
#[doc = "UDS_BODY4 register accessor: an alias for `Reg<UDS_BODY4_SPEC>`"]
pub type UDS_BODY4 = crate::Reg<uds_body4::UDS_BODY4_SPEC>;
#[doc = "."]
pub mod uds_body4;
#[doc = "UDS_KEY_CODE6 register accessor: an alias for `Reg<UDS_KEY_CODE6_SPEC>`"]
pub type UDS_KEY_CODE6 = crate::Reg<uds_key_code6::UDS_KEY_CODE6_SPEC>;
#[doc = "."]
pub mod uds_key_code6;
#[doc = "UDS_BODY5 register accessor: an alias for `Reg<UDS_BODY5_SPEC>`"]
pub type UDS_BODY5 = crate::Reg<uds_body5::UDS_BODY5_SPEC>;
#[doc = "."]
pub mod uds_body5;
#[doc = "UDS_KEY_CODE7 register accessor: an alias for `Reg<UDS_KEY_CODE7_SPEC>`"]
pub type UDS_KEY_CODE7 = crate::Reg<uds_key_code7::UDS_KEY_CODE7_SPEC>;
#[doc = "."]
pub mod uds_key_code7;
#[doc = "UDS_BODY6 register accessor: an alias for `Reg<UDS_BODY6_SPEC>`"]
pub type UDS_BODY6 = crate::Reg<uds_body6::UDS_BODY6_SPEC>;
#[doc = "."]
pub mod uds_body6;
#[doc = "UDS_KEY_CODE8 register accessor: an alias for `Reg<UDS_KEY_CODE8_SPEC>`"]
pub type UDS_KEY_CODE8 = crate::Reg<uds_key_code8::UDS_KEY_CODE8_SPEC>;
#[doc = "."]
pub mod uds_key_code8;
#[doc = "UDS_BODY7 register accessor: an alias for `Reg<UDS_BODY7_SPEC>`"]
pub type UDS_BODY7 = crate::Reg<uds_body7::UDS_BODY7_SPEC>;
#[doc = "."]
pub mod uds_body7;
#[doc = "UDS_KEY_CODE9 register accessor: an alias for `Reg<UDS_KEY_CODE9_SPEC>`"]
pub type UDS_KEY_CODE9 = crate::Reg<uds_key_code9::UDS_KEY_CODE9_SPEC>;
#[doc = "."]
pub mod uds_key_code9;
#[doc = "UDS_BODY8 register accessor: an alias for `Reg<UDS_BODY8_SPEC>`"]
pub type UDS_BODY8 = crate::Reg<uds_body8::UDS_BODY8_SPEC>;
#[doc = "."]
pub mod uds_body8;
#[doc = "UDS_KEY_CODE10 register accessor: an alias for `Reg<UDS_KEY_CODE10_SPEC>`"]
pub type UDS_KEY_CODE10 = crate::Reg<uds_key_code10::UDS_KEY_CODE10_SPEC>;
#[doc = "."]
pub mod uds_key_code10;
#[doc = "UDS_BODY9 register accessor: an alias for `Reg<UDS_BODY9_SPEC>`"]
pub type UDS_BODY9 = crate::Reg<uds_body9::UDS_BODY9_SPEC>;
#[doc = "."]
pub mod uds_body9;
#[doc = "UDS_KEY_CODE11 register accessor: an alias for `Reg<UDS_KEY_CODE11_SPEC>`"]
pub type UDS_KEY_CODE11 = crate::Reg<uds_key_code11::UDS_KEY_CODE11_SPEC>;
#[doc = "."]
pub mod uds_key_code11;
#[doc = "UDS_BODY10 register accessor: an alias for `Reg<UDS_BODY10_SPEC>`"]
pub type UDS_BODY10 = crate::Reg<uds_body10::UDS_BODY10_SPEC>;
#[doc = "."]
pub mod uds_body10;
#[doc = "UDS_KEY_CODE12 register accessor: an alias for `Reg<UDS_KEY_CODE12_SPEC>`"]
pub type UDS_KEY_CODE12 = crate::Reg<uds_key_code12::UDS_KEY_CODE12_SPEC>;
#[doc = "."]
pub mod uds_key_code12;
#[doc = "UDS_BODY11 register accessor: an alias for `Reg<UDS_BODY11_SPEC>`"]
pub type UDS_BODY11 = crate::Reg<uds_body11::UDS_BODY11_SPEC>;
#[doc = "."]
pub mod uds_body11;
#[doc = "UDS_KEY_CODE13 register accessor: an alias for `Reg<UDS_KEY_CODE13_SPEC>`"]
pub type UDS_KEY_CODE13 = crate::Reg<uds_key_code13::UDS_KEY_CODE13_SPEC>;
#[doc = "."]
pub mod uds_key_code13;
#[doc = "PRINCE_REGION0_HEADER0 register accessor: an alias for `Reg<PRINCE_REGION0_HEADER0_SPEC>`"]
pub type PRINCE_REGION0_HEADER0 = crate::Reg<prince_region0_header0::PRINCE_REGION0_HEADER0_SPEC>;
#[doc = "."]
pub mod prince_region0_header0;
#[doc = "PRINCE_REGION0_KEY_CODE0 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE0_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE0 =
    crate::Reg<prince_region0_key_code0::PRINCE_REGION0_KEY_CODE0_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code0;
#[doc = "PRINCE_REGION0_HEADER1 register accessor: an alias for `Reg<PRINCE_REGION0_HEADER1_SPEC>`"]
pub type PRINCE_REGION0_HEADER1 = crate::Reg<prince_region0_header1::PRINCE_REGION0_HEADER1_SPEC>;
#[doc = "."]
pub mod prince_region0_header1;
#[doc = "PRINCE_REGION0_KEY_CODE1 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE1_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE1 =
    crate::Reg<prince_region0_key_code1::PRINCE_REGION0_KEY_CODE1_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code1;
#[doc = "PRINCE_REGION0_BODY0 register accessor: an alias for `Reg<PRINCE_REGION0_BODY0_SPEC>`"]
pub type PRINCE_REGION0_BODY0 = crate::Reg<prince_region0_body0::PRINCE_REGION0_BODY0_SPEC>;
#[doc = "."]
pub mod prince_region0_body0;
#[doc = "PRINCE_REGION0_KEY_CODE2 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE2_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE2 =
    crate::Reg<prince_region0_key_code2::PRINCE_REGION0_KEY_CODE2_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code2;
#[doc = "PRINCE_REGION0_BODY1 register accessor: an alias for `Reg<PRINCE_REGION0_BODY1_SPEC>`"]
pub type PRINCE_REGION0_BODY1 = crate::Reg<prince_region0_body1::PRINCE_REGION0_BODY1_SPEC>;
#[doc = "."]
pub mod prince_region0_body1;
#[doc = "PRINCE_REGION0_KEY_CODE3 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE3_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE3 =
    crate::Reg<prince_region0_key_code3::PRINCE_REGION0_KEY_CODE3_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code3;
#[doc = "PRINCE_REGION0_BODY2 register accessor: an alias for `Reg<PRINCE_REGION0_BODY2_SPEC>`"]
pub type PRINCE_REGION0_BODY2 = crate::Reg<prince_region0_body2::PRINCE_REGION0_BODY2_SPEC>;
#[doc = "."]
pub mod prince_region0_body2;
#[doc = "PRINCE_REGION0_KEY_CODE4 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE4_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE4 =
    crate::Reg<prince_region0_key_code4::PRINCE_REGION0_KEY_CODE4_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code4;
#[doc = "PRINCE_REGION0_BODY3 register accessor: an alias for `Reg<PRINCE_REGION0_BODY3_SPEC>`"]
pub type PRINCE_REGION0_BODY3 = crate::Reg<prince_region0_body3::PRINCE_REGION0_BODY3_SPEC>;
#[doc = "."]
pub mod prince_region0_body3;
#[doc = "PRINCE_REGION0_KEY_CODE5 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE5_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE5 =
    crate::Reg<prince_region0_key_code5::PRINCE_REGION0_KEY_CODE5_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code5;
#[doc = "PRINCE_REGION0_BODY4 register accessor: an alias for `Reg<PRINCE_REGION0_BODY4_SPEC>`"]
pub type PRINCE_REGION0_BODY4 = crate::Reg<prince_region0_body4::PRINCE_REGION0_BODY4_SPEC>;
#[doc = "."]
pub mod prince_region0_body4;
#[doc = "PRINCE_REGION0_KEY_CODE6 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE6_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE6 =
    crate::Reg<prince_region0_key_code6::PRINCE_REGION0_KEY_CODE6_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code6;
#[doc = "PRINCE_REGION0_BODY5 register accessor: an alias for `Reg<PRINCE_REGION0_BODY5_SPEC>`"]
pub type PRINCE_REGION0_BODY5 = crate::Reg<prince_region0_body5::PRINCE_REGION0_BODY5_SPEC>;
#[doc = "."]
pub mod prince_region0_body5;
#[doc = "PRINCE_REGION0_KEY_CODE7 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE7_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE7 =
    crate::Reg<prince_region0_key_code7::PRINCE_REGION0_KEY_CODE7_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code7;
#[doc = "PRINCE_REGION0_BODY6 register accessor: an alias for `Reg<PRINCE_REGION0_BODY6_SPEC>`"]
pub type PRINCE_REGION0_BODY6 = crate::Reg<prince_region0_body6::PRINCE_REGION0_BODY6_SPEC>;
#[doc = "."]
pub mod prince_region0_body6;
#[doc = "PRINCE_REGION0_KEY_CODE8 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE8_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE8 =
    crate::Reg<prince_region0_key_code8::PRINCE_REGION0_KEY_CODE8_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code8;
#[doc = "PRINCE_REGION0_BODY7 register accessor: an alias for `Reg<PRINCE_REGION0_BODY7_SPEC>`"]
pub type PRINCE_REGION0_BODY7 = crate::Reg<prince_region0_body7::PRINCE_REGION0_BODY7_SPEC>;
#[doc = "."]
pub mod prince_region0_body7;
#[doc = "PRINCE_REGION0_KEY_CODE9 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE9_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE9 =
    crate::Reg<prince_region0_key_code9::PRINCE_REGION0_KEY_CODE9_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code9;
#[doc = "PRINCE_REGION0_BODY8 register accessor: an alias for `Reg<PRINCE_REGION0_BODY8_SPEC>`"]
pub type PRINCE_REGION0_BODY8 = crate::Reg<prince_region0_body8::PRINCE_REGION0_BODY8_SPEC>;
#[doc = "."]
pub mod prince_region0_body8;
#[doc = "PRINCE_REGION0_KEY_CODE10 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE10_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE10 =
    crate::Reg<prince_region0_key_code10::PRINCE_REGION0_KEY_CODE10_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code10;
#[doc = "PRINCE_REGION0_BODY9 register accessor: an alias for `Reg<PRINCE_REGION0_BODY9_SPEC>`"]
pub type PRINCE_REGION0_BODY9 = crate::Reg<prince_region0_body9::PRINCE_REGION0_BODY9_SPEC>;
#[doc = "."]
pub mod prince_region0_body9;
#[doc = "PRINCE_REGION0_KEY_CODE11 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE11_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE11 =
    crate::Reg<prince_region0_key_code11::PRINCE_REGION0_KEY_CODE11_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code11;
#[doc = "PRINCE_REGION0_BODY10 register accessor: an alias for `Reg<PRINCE_REGION0_BODY10_SPEC>`"]
pub type PRINCE_REGION0_BODY10 = crate::Reg<prince_region0_body10::PRINCE_REGION0_BODY10_SPEC>;
#[doc = "."]
pub mod prince_region0_body10;
#[doc = "PRINCE_REGION0_KEY_CODE12 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE12_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE12 =
    crate::Reg<prince_region0_key_code12::PRINCE_REGION0_KEY_CODE12_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code12;
#[doc = "PRINCE_REGION0_BODY11 register accessor: an alias for `Reg<PRINCE_REGION0_BODY11_SPEC>`"]
pub type PRINCE_REGION0_BODY11 = crate::Reg<prince_region0_body11::PRINCE_REGION0_BODY11_SPEC>;
#[doc = "."]
pub mod prince_region0_body11;
#[doc = "PRINCE_REGION0_KEY_CODE13 register accessor: an alias for `Reg<PRINCE_REGION0_KEY_CODE13_SPEC>`"]
pub type PRINCE_REGION0_KEY_CODE13 =
    crate::Reg<prince_region0_key_code13::PRINCE_REGION0_KEY_CODE13_SPEC>;
#[doc = "."]
pub mod prince_region0_key_code13;
#[doc = "PRINCE_REGION1_HEADER0 register accessor: an alias for `Reg<PRINCE_REGION1_HEADER0_SPEC>`"]
pub type PRINCE_REGION1_HEADER0 = crate::Reg<prince_region1_header0::PRINCE_REGION1_HEADER0_SPEC>;
#[doc = "."]
pub mod prince_region1_header0;
#[doc = "PRINCE_REGION1_KEY_CODE0 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE0_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE0 =
    crate::Reg<prince_region1_key_code0::PRINCE_REGION1_KEY_CODE0_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code0;
#[doc = "PRINCE_REGION1_HEADER1 register accessor: an alias for `Reg<PRINCE_REGION1_HEADER1_SPEC>`"]
pub type PRINCE_REGION1_HEADER1 = crate::Reg<prince_region1_header1::PRINCE_REGION1_HEADER1_SPEC>;
#[doc = "."]
pub mod prince_region1_header1;
#[doc = "PRINCE_REGION1_KEY_CODE1 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE1_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE1 =
    crate::Reg<prince_region1_key_code1::PRINCE_REGION1_KEY_CODE1_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code1;
#[doc = "PRINCE_REGION1_BODY0 register accessor: an alias for `Reg<PRINCE_REGION1_BODY0_SPEC>`"]
pub type PRINCE_REGION1_BODY0 = crate::Reg<prince_region1_body0::PRINCE_REGION1_BODY0_SPEC>;
#[doc = "."]
pub mod prince_region1_body0;
#[doc = "PRINCE_REGION1_KEY_CODE2 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE2_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE2 =
    crate::Reg<prince_region1_key_code2::PRINCE_REGION1_KEY_CODE2_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code2;
#[doc = "PRINCE_REGION1_BODY1 register accessor: an alias for `Reg<PRINCE_REGION1_BODY1_SPEC>`"]
pub type PRINCE_REGION1_BODY1 = crate::Reg<prince_region1_body1::PRINCE_REGION1_BODY1_SPEC>;
#[doc = "."]
pub mod prince_region1_body1;
#[doc = "PRINCE_REGION1_KEY_CODE3 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE3_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE3 =
    crate::Reg<prince_region1_key_code3::PRINCE_REGION1_KEY_CODE3_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code3;
#[doc = "PRINCE_REGION1_BODY2 register accessor: an alias for `Reg<PRINCE_REGION1_BODY2_SPEC>`"]
pub type PRINCE_REGION1_BODY2 = crate::Reg<prince_region1_body2::PRINCE_REGION1_BODY2_SPEC>;
#[doc = "."]
pub mod prince_region1_body2;
#[doc = "PRINCE_REGION1_KEY_CODE4 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE4_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE4 =
    crate::Reg<prince_region1_key_code4::PRINCE_REGION1_KEY_CODE4_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code4;
#[doc = "PRINCE_REGION1_BODY3 register accessor: an alias for `Reg<PRINCE_REGION1_BODY3_SPEC>`"]
pub type PRINCE_REGION1_BODY3 = crate::Reg<prince_region1_body3::PRINCE_REGION1_BODY3_SPEC>;
#[doc = "."]
pub mod prince_region1_body3;
#[doc = "PRINCE_REGION1_KEY_CODE5 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE5_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE5 =
    crate::Reg<prince_region1_key_code5::PRINCE_REGION1_KEY_CODE5_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code5;
#[doc = "PRINCE_REGION1_BODY4 register accessor: an alias for `Reg<PRINCE_REGION1_BODY4_SPEC>`"]
pub type PRINCE_REGION1_BODY4 = crate::Reg<prince_region1_body4::PRINCE_REGION1_BODY4_SPEC>;
#[doc = "."]
pub mod prince_region1_body4;
#[doc = "PRINCE_REGION1_KEY_CODE6 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE6_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE6 =
    crate::Reg<prince_region1_key_code6::PRINCE_REGION1_KEY_CODE6_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code6;
#[doc = "PRINCE_REGION1_BODY5 register accessor: an alias for `Reg<PRINCE_REGION1_BODY5_SPEC>`"]
pub type PRINCE_REGION1_BODY5 = crate::Reg<prince_region1_body5::PRINCE_REGION1_BODY5_SPEC>;
#[doc = "."]
pub mod prince_region1_body5;
#[doc = "PRINCE_REGION1_KEY_CODE7 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE7_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE7 =
    crate::Reg<prince_region1_key_code7::PRINCE_REGION1_KEY_CODE7_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code7;
#[doc = "PRINCE_REGION1_BODY6 register accessor: an alias for `Reg<PRINCE_REGION1_BODY6_SPEC>`"]
pub type PRINCE_REGION1_BODY6 = crate::Reg<prince_region1_body6::PRINCE_REGION1_BODY6_SPEC>;
#[doc = "."]
pub mod prince_region1_body6;
#[doc = "PRINCE_REGION1_KEY_CODE8 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE8_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE8 =
    crate::Reg<prince_region1_key_code8::PRINCE_REGION1_KEY_CODE8_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code8;
#[doc = "PRINCE_REGION1_BODY7 register accessor: an alias for `Reg<PRINCE_REGION1_BODY7_SPEC>`"]
pub type PRINCE_REGION1_BODY7 = crate::Reg<prince_region1_body7::PRINCE_REGION1_BODY7_SPEC>;
#[doc = "."]
pub mod prince_region1_body7;
#[doc = "PRINCE_REGION1_KEY_CODE9 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE9_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE9 =
    crate::Reg<prince_region1_key_code9::PRINCE_REGION1_KEY_CODE9_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code9;
#[doc = "PRINCE_REGION1_BODY8 register accessor: an alias for `Reg<PRINCE_REGION1_BODY8_SPEC>`"]
pub type PRINCE_REGION1_BODY8 = crate::Reg<prince_region1_body8::PRINCE_REGION1_BODY8_SPEC>;
#[doc = "."]
pub mod prince_region1_body8;
#[doc = "PRINCE_REGION1_KEY_CODE10 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE10_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE10 =
    crate::Reg<prince_region1_key_code10::PRINCE_REGION1_KEY_CODE10_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code10;
#[doc = "PRINCE_REGION1_BODY9 register accessor: an alias for `Reg<PRINCE_REGION1_BODY9_SPEC>`"]
pub type PRINCE_REGION1_BODY9 = crate::Reg<prince_region1_body9::PRINCE_REGION1_BODY9_SPEC>;
#[doc = "."]
pub mod prince_region1_body9;
#[doc = "PRINCE_REGION1_KEY_CODE11 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE11_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE11 =
    crate::Reg<prince_region1_key_code11::PRINCE_REGION1_KEY_CODE11_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code11;
#[doc = "PRINCE_REGION1_BODY10 register accessor: an alias for `Reg<PRINCE_REGION1_BODY10_SPEC>`"]
pub type PRINCE_REGION1_BODY10 = crate::Reg<prince_region1_body10::PRINCE_REGION1_BODY10_SPEC>;
#[doc = "."]
pub mod prince_region1_body10;
#[doc = "PRINCE_REGION1_KEY_CODE12 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE12_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE12 =
    crate::Reg<prince_region1_key_code12::PRINCE_REGION1_KEY_CODE12_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code12;
#[doc = "PRINCE_REGION1_BODY11 register accessor: an alias for `Reg<PRINCE_REGION1_BODY11_SPEC>`"]
pub type PRINCE_REGION1_BODY11 = crate::Reg<prince_region1_body11::PRINCE_REGION1_BODY11_SPEC>;
#[doc = "."]
pub mod prince_region1_body11;
#[doc = "PRINCE_REGION1_KEY_CODE13 register accessor: an alias for `Reg<PRINCE_REGION1_KEY_CODE13_SPEC>`"]
pub type PRINCE_REGION1_KEY_CODE13 =
    crate::Reg<prince_region1_key_code13::PRINCE_REGION1_KEY_CODE13_SPEC>;
#[doc = "."]
pub mod prince_region1_key_code13;
#[doc = "PRINCE_REGION2_HEADER0 register accessor: an alias for `Reg<PRINCE_REGION2_HEADER0_SPEC>`"]
pub type PRINCE_REGION2_HEADER0 = crate::Reg<prince_region2_header0::PRINCE_REGION2_HEADER0_SPEC>;
#[doc = "."]
pub mod prince_region2_header0;
#[doc = "PRINCE_REGION2_KEY_CODE0 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE0_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE0 =
    crate::Reg<prince_region2_key_code0::PRINCE_REGION2_KEY_CODE0_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code0;
#[doc = "PRINCE_REGION2_HEADER1 register accessor: an alias for `Reg<PRINCE_REGION2_HEADER1_SPEC>`"]
pub type PRINCE_REGION2_HEADER1 = crate::Reg<prince_region2_header1::PRINCE_REGION2_HEADER1_SPEC>;
#[doc = "."]
pub mod prince_region2_header1;
#[doc = "PRINCE_REGION2_KEY_CODE1 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE1_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE1 =
    crate::Reg<prince_region2_key_code1::PRINCE_REGION2_KEY_CODE1_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code1;
#[doc = "PRINCE_REGION2_BODY0 register accessor: an alias for `Reg<PRINCE_REGION2_BODY0_SPEC>`"]
pub type PRINCE_REGION2_BODY0 = crate::Reg<prince_region2_body0::PRINCE_REGION2_BODY0_SPEC>;
#[doc = "."]
pub mod prince_region2_body0;
#[doc = "PRINCE_REGION2_KEY_CODE2 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE2_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE2 =
    crate::Reg<prince_region2_key_code2::PRINCE_REGION2_KEY_CODE2_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code2;
#[doc = "PRINCE_REGION2_BODY1 register accessor: an alias for `Reg<PRINCE_REGION2_BODY1_SPEC>`"]
pub type PRINCE_REGION2_BODY1 = crate::Reg<prince_region2_body1::PRINCE_REGION2_BODY1_SPEC>;
#[doc = "."]
pub mod prince_region2_body1;
#[doc = "PRINCE_REGION2_KEY_CODE3 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE3_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE3 =
    crate::Reg<prince_region2_key_code3::PRINCE_REGION2_KEY_CODE3_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code3;
#[doc = "PRINCE_REGION2_BODY2 register accessor: an alias for `Reg<PRINCE_REGION2_BODY2_SPEC>`"]
pub type PRINCE_REGION2_BODY2 = crate::Reg<prince_region2_body2::PRINCE_REGION2_BODY2_SPEC>;
#[doc = "."]
pub mod prince_region2_body2;
#[doc = "PRINCE_REGION2_KEY_CODE4 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE4_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE4 =
    crate::Reg<prince_region2_key_code4::PRINCE_REGION2_KEY_CODE4_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code4;
#[doc = "PRINCE_REGION2_BODY3 register accessor: an alias for `Reg<PRINCE_REGION2_BODY3_SPEC>`"]
pub type PRINCE_REGION2_BODY3 = crate::Reg<prince_region2_body3::PRINCE_REGION2_BODY3_SPEC>;
#[doc = "."]
pub mod prince_region2_body3;
#[doc = "PRINCE_REGION2_KEY_CODE5 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE5_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE5 =
    crate::Reg<prince_region2_key_code5::PRINCE_REGION2_KEY_CODE5_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code5;
#[doc = "PRINCE_REGION2_BODY4 register accessor: an alias for `Reg<PRINCE_REGION2_BODY4_SPEC>`"]
pub type PRINCE_REGION2_BODY4 = crate::Reg<prince_region2_body4::PRINCE_REGION2_BODY4_SPEC>;
#[doc = "."]
pub mod prince_region2_body4;
#[doc = "PRINCE_REGION2_KEY_CODE6 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE6_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE6 =
    crate::Reg<prince_region2_key_code6::PRINCE_REGION2_KEY_CODE6_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code6;
#[doc = "PRINCE_REGION2_BODY5 register accessor: an alias for `Reg<PRINCE_REGION2_BODY5_SPEC>`"]
pub type PRINCE_REGION2_BODY5 = crate::Reg<prince_region2_body5::PRINCE_REGION2_BODY5_SPEC>;
#[doc = "."]
pub mod prince_region2_body5;
#[doc = "PRINCE_REGION2_KEY_CODE7 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE7_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE7 =
    crate::Reg<prince_region2_key_code7::PRINCE_REGION2_KEY_CODE7_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code7;
#[doc = "PRINCE_REGION2_BODY6 register accessor: an alias for `Reg<PRINCE_REGION2_BODY6_SPEC>`"]
pub type PRINCE_REGION2_BODY6 = crate::Reg<prince_region2_body6::PRINCE_REGION2_BODY6_SPEC>;
#[doc = "."]
pub mod prince_region2_body6;
#[doc = "PRINCE_REGION2_KEY_CODE8 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE8_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE8 =
    crate::Reg<prince_region2_key_code8::PRINCE_REGION2_KEY_CODE8_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code8;
#[doc = "PRINCE_REGION2_BODY7 register accessor: an alias for `Reg<PRINCE_REGION2_BODY7_SPEC>`"]
pub type PRINCE_REGION2_BODY7 = crate::Reg<prince_region2_body7::PRINCE_REGION2_BODY7_SPEC>;
#[doc = "."]
pub mod prince_region2_body7;
#[doc = "PRINCE_REGION2_KEY_CODE9 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE9_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE9 =
    crate::Reg<prince_region2_key_code9::PRINCE_REGION2_KEY_CODE9_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code9;
#[doc = "PRINCE_REGION2_BODY8 register accessor: an alias for `Reg<PRINCE_REGION2_BODY8_SPEC>`"]
pub type PRINCE_REGION2_BODY8 = crate::Reg<prince_region2_body8::PRINCE_REGION2_BODY8_SPEC>;
#[doc = "."]
pub mod prince_region2_body8;
#[doc = "PRINCE_REGION2_KEY_CODE10 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE10_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE10 =
    crate::Reg<prince_region2_key_code10::PRINCE_REGION2_KEY_CODE10_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code10;
#[doc = "PRINCE_REGION2_BODY9 register accessor: an alias for `Reg<PRINCE_REGION2_BODY9_SPEC>`"]
pub type PRINCE_REGION2_BODY9 = crate::Reg<prince_region2_body9::PRINCE_REGION2_BODY9_SPEC>;
#[doc = "."]
pub mod prince_region2_body9;
#[doc = "PRINCE_REGION2_KEY_CODE11 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE11_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE11 =
    crate::Reg<prince_region2_key_code11::PRINCE_REGION2_KEY_CODE11_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code11;
#[doc = "PRINCE_REGION2_BODY10 register accessor: an alias for `Reg<PRINCE_REGION2_BODY10_SPEC>`"]
pub type PRINCE_REGION2_BODY10 = crate::Reg<prince_region2_body10::PRINCE_REGION2_BODY10_SPEC>;
#[doc = "."]
pub mod prince_region2_body10;
#[doc = "PRINCE_REGION2_KEY_CODE12 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE12_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE12 =
    crate::Reg<prince_region2_key_code12::PRINCE_REGION2_KEY_CODE12_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code12;
#[doc = "PRINCE_REGION2_BODY11 register accessor: an alias for `Reg<PRINCE_REGION2_BODY11_SPEC>`"]
pub type PRINCE_REGION2_BODY11 = crate::Reg<prince_region2_body11::PRINCE_REGION2_BODY11_SPEC>;
#[doc = "."]
pub mod prince_region2_body11;
#[doc = "PRINCE_REGION2_KEY_CODE13 register accessor: an alias for `Reg<PRINCE_REGION2_KEY_CODE13_SPEC>`"]
pub type PRINCE_REGION2_KEY_CODE13 =
    crate::Reg<prince_region2_key_code13::PRINCE_REGION2_KEY_CODE13_SPEC>;
#[doc = "."]
pub mod prince_region2_key_code13;
