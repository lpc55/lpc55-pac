#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Valid Key Sore Header : 0x95959595"]
    pub header: HEADER,
    #[doc = "0x04 - puf discharge time in ms."]
    pub puf_discharge_time_in_ms: PUF_DISCHARGE_TIME_IN_MS,
    #[doc = "0x08 - ."]
    pub activation_code: [ACTIVATION_CODE; 298],
    #[doc = "0x4b0 - ."]
    pub sbkey_header0: SBKEY_HEADER0,
    #[doc = "0x4b4 - ."]
    pub sbkey_header1: SBKEY_HEADER1,
    #[doc = "0x4b8 - ."]
    pub sbkey_body0: SBKEY_BODY0,
    #[doc = "0x4bc - ."]
    pub sbkey_body1: SBKEY_BODY1,
    #[doc = "0x4c0 - ."]
    pub sbkey_body2: SBKEY_BODY2,
    #[doc = "0x4c4 - ."]
    pub sbkey_body3: SBKEY_BODY3,
    #[doc = "0x4c8 - ."]
    pub sbkey_body4: SBKEY_BODY4,
    #[doc = "0x4cc - ."]
    pub sbkey_body5: SBKEY_BODY5,
    #[doc = "0x4d0 - ."]
    pub sbkey_body6: SBKEY_BODY6,
    #[doc = "0x4d4 - ."]
    pub sbkey_body7: SBKEY_BODY7,
    #[doc = "0x4d8 - ."]
    pub sbkey_body8: SBKEY_BODY8,
    #[doc = "0x4dc - ."]
    pub sbkey_body9: SBKEY_BODY9,
    #[doc = "0x4e0 - ."]
    pub sbkey_body10: SBKEY_BODY10,
    #[doc = "0x4e4 - ."]
    pub sbkey_body11: SBKEY_BODY11,
    #[doc = "0x4e8 - ."]
    pub user_kek_header0: USER_KEK_HEADER0,
    #[doc = "0x4ec - ."]
    pub user_kek_header1: USER_KEK_HEADER1,
    #[doc = "0x4f0 - ."]
    pub user_kek_body0: USER_KEK_BODY0,
    #[doc = "0x4f4 - ."]
    pub user_kek_body1: USER_KEK_BODY1,
    #[doc = "0x4f8 - ."]
    pub user_kek_body2: USER_KEK_BODY2,
    #[doc = "0x4fc - ."]
    pub user_kek_body3: USER_KEK_BODY3,
    #[doc = "0x500 - ."]
    pub user_kek_body4: USER_KEK_BODY4,
    #[doc = "0x504 - ."]
    pub user_kek_body5: USER_KEK_BODY5,
    #[doc = "0x508 - ."]
    pub user_kek_body6: USER_KEK_BODY6,
    #[doc = "0x50c - ."]
    pub user_kek_body7: USER_KEK_BODY7,
    #[doc = "0x510 - ."]
    pub user_kek_body8: USER_KEK_BODY8,
    #[doc = "0x514 - ."]
    pub user_kek_body9: USER_KEK_BODY9,
    #[doc = "0x518 - ."]
    pub user_kek_body10: USER_KEK_BODY10,
    #[doc = "0x51c - ."]
    pub user_kek_body11: USER_KEK_BODY11,
    #[doc = "0x520 - ."]
    pub uds_header0: UDS_HEADER0,
    #[doc = "0x524 - ."]
    pub uds_header1: UDS_HEADER1,
    #[doc = "0x528 - ."]
    pub uds_body0: UDS_BODY0,
    #[doc = "0x52c - ."]
    pub uds_body1: UDS_BODY1,
    #[doc = "0x530 - ."]
    pub uds_body2: UDS_BODY2,
    #[doc = "0x534 - ."]
    pub uds_body3: UDS_BODY3,
    #[doc = "0x538 - ."]
    pub uds_body4: UDS_BODY4,
    #[doc = "0x53c - ."]
    pub uds_body5: UDS_BODY5,
    #[doc = "0x540 - ."]
    pub uds_body6: UDS_BODY6,
    #[doc = "0x544 - ."]
    pub uds_body7: UDS_BODY7,
    #[doc = "0x548 - ."]
    pub uds_body8: UDS_BODY8,
    #[doc = "0x54c - ."]
    pub uds_body9: UDS_BODY9,
    #[doc = "0x550 - ."]
    pub uds_body10: UDS_BODY10,
    #[doc = "0x554 - ."]
    pub uds_body11: UDS_BODY11,
    #[doc = "0x558 - ."]
    pub prince_region0_header0: PRINCE_REGION0_HEADER0,
    #[doc = "0x55c - ."]
    pub prince_region0_header1: PRINCE_REGION0_HEADER1,
    #[doc = "0x560 - ."]
    pub prince_region0_body0: PRINCE_REGION0_BODY0,
    #[doc = "0x564 - ."]
    pub prince_region0_body1: PRINCE_REGION0_BODY1,
    #[doc = "0x568 - ."]
    pub prince_region0_body2: PRINCE_REGION0_BODY2,
    #[doc = "0x56c - ."]
    pub prince_region0_body3: PRINCE_REGION0_BODY3,
    #[doc = "0x570 - ."]
    pub prince_region0_body4: PRINCE_REGION0_BODY4,
    #[doc = "0x574 - ."]
    pub prince_region0_body5: PRINCE_REGION0_BODY5,
    #[doc = "0x578 - ."]
    pub prince_region0_body6: PRINCE_REGION0_BODY6,
    #[doc = "0x57c - ."]
    pub prince_region0_body7: PRINCE_REGION0_BODY7,
    #[doc = "0x580 - ."]
    pub prince_region0_body8: PRINCE_REGION0_BODY8,
    #[doc = "0x584 - ."]
    pub prince_region0_body9: PRINCE_REGION0_BODY9,
    #[doc = "0x588 - ."]
    pub prince_region0_body10: PRINCE_REGION0_BODY10,
    #[doc = "0x58c - ."]
    pub prince_region0_body11: PRINCE_REGION0_BODY11,
    #[doc = "0x590 - ."]
    pub prince_region1_header0: PRINCE_REGION1_HEADER0,
    #[doc = "0x594 - ."]
    pub prince_region1_header1: PRINCE_REGION1_HEADER1,
    #[doc = "0x598 - ."]
    pub prince_region1_body0: PRINCE_REGION1_BODY0,
    #[doc = "0x59c - ."]
    pub prince_region1_body1: PRINCE_REGION1_BODY1,
    #[doc = "0x5a0 - ."]
    pub prince_region1_body2: PRINCE_REGION1_BODY2,
    #[doc = "0x5a4 - ."]
    pub prince_region1_body3: PRINCE_REGION1_BODY3,
    #[doc = "0x5a8 - ."]
    pub prince_region1_body4: PRINCE_REGION1_BODY4,
    #[doc = "0x5ac - ."]
    pub prince_region1_body5: PRINCE_REGION1_BODY5,
    #[doc = "0x5b0 - ."]
    pub prince_region1_body6: PRINCE_REGION1_BODY6,
    #[doc = "0x5b4 - ."]
    pub prince_region1_body7: PRINCE_REGION1_BODY7,
    #[doc = "0x5b8 - ."]
    pub prince_region1_body8: PRINCE_REGION1_BODY8,
    #[doc = "0x5bc - ."]
    pub prince_region1_body9: PRINCE_REGION1_BODY9,
    #[doc = "0x5c0 - ."]
    pub prince_region1_body10: PRINCE_REGION1_BODY10,
    #[doc = "0x5c4 - ."]
    pub prince_region1_body11: PRINCE_REGION1_BODY11,
    #[doc = "0x5c8 - ."]
    pub prince_region2_header0: PRINCE_REGION2_HEADER0,
    #[doc = "0x5cc - ."]
    pub prince_region2_header1: PRINCE_REGION2_HEADER1,
    #[doc = "0x5d0 - ."]
    pub prince_region2_body0: PRINCE_REGION2_BODY0,
    #[doc = "0x5d4 - ."]
    pub prince_region2_body1: PRINCE_REGION2_BODY1,
    #[doc = "0x5d8 - ."]
    pub prince_region2_body2: PRINCE_REGION2_BODY2,
    #[doc = "0x5dc - ."]
    pub prince_region2_body3: PRINCE_REGION2_BODY3,
    #[doc = "0x5e0 - ."]
    pub prince_region2_body4: PRINCE_REGION2_BODY4,
    #[doc = "0x5e4 - ."]
    pub prince_region2_body5: PRINCE_REGION2_BODY5,
    #[doc = "0x5e8 - ."]
    pub prince_region2_body6: PRINCE_REGION2_BODY6,
    #[doc = "0x5ec - ."]
    pub prince_region2_body7: PRINCE_REGION2_BODY7,
    #[doc = "0x5f0 - ."]
    pub prince_region2_body8: PRINCE_REGION2_BODY8,
    #[doc = "0x5f4 - ."]
    pub prince_region2_body9: PRINCE_REGION2_BODY9,
    #[doc = "0x5f8 - ."]
    pub prince_region2_body10: PRINCE_REGION2_BODY10,
    #[doc = "0x5fc - ."]
    pub prince_region2_body11: PRINCE_REGION2_BODY11,
}
#[doc = "Valid Key Sore Header : 0x95959595"]
pub struct HEADER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Valid Key Sore Header : 0x95959595"]
pub mod header;
#[doc = "puf discharge time in ms."]
pub struct PUF_DISCHARGE_TIME_IN_MS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "puf discharge time in ms."]
pub mod puf_discharge_time_in_ms;
#[doc = "."]
pub struct ACTIVATION_CODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod activation_code;
#[doc = "."]
pub struct SBKEY_HEADER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_header0;
#[doc = "."]
pub struct SBKEY_KEY_CODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code0;
#[doc = "."]
pub struct SBKEY_HEADER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_header1;
#[doc = "."]
pub struct SBKEY_KEY_CODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code1;
#[doc = "."]
pub struct SBKEY_BODY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body0;
#[doc = "."]
pub struct SBKEY_KEY_CODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code2;
#[doc = "."]
pub struct SBKEY_BODY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body1;
#[doc = "."]
pub struct SBKEY_KEY_CODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code3;
#[doc = "."]
pub struct SBKEY_BODY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body2;
#[doc = "."]
pub struct SBKEY_KEY_CODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code4;
#[doc = "."]
pub struct SBKEY_BODY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body3;
#[doc = "."]
pub struct SBKEY_KEY_CODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code5;
#[doc = "."]
pub struct SBKEY_BODY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body4;
#[doc = "."]
pub struct SBKEY_KEY_CODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code6;
#[doc = "."]
pub struct SBKEY_BODY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body5;
#[doc = "."]
pub struct SBKEY_KEY_CODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code7;
#[doc = "."]
pub struct SBKEY_BODY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body6;
#[doc = "."]
pub struct SBKEY_KEY_CODE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code8;
#[doc = "."]
pub struct SBKEY_BODY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body7;
#[doc = "."]
pub struct SBKEY_KEY_CODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code9;
#[doc = "."]
pub struct SBKEY_BODY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body8;
#[doc = "."]
pub struct SBKEY_KEY_CODE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code10;
#[doc = "."]
pub struct SBKEY_BODY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body9;
#[doc = "."]
pub struct SBKEY_KEY_CODE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code11;
#[doc = "."]
pub struct SBKEY_BODY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body10;
#[doc = "."]
pub struct SBKEY_KEY_CODE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code12;
#[doc = "."]
pub struct SBKEY_BODY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_body11;
#[doc = "."]
pub struct SBKEY_KEY_CODE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod sbkey_key_code13;
#[doc = "."]
pub struct USER_KEK_HEADER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_header0;
#[doc = "."]
pub struct USER_KEK_KEY_CODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code0;
#[doc = "."]
pub struct USER_KEK_HEADER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_header1;
#[doc = "."]
pub struct USER_KEK_KEY_CODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code1;
#[doc = "."]
pub struct USER_KEK_BODY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body0;
#[doc = "."]
pub struct USER_KEK_KEY_CODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code2;
#[doc = "."]
pub struct USER_KEK_BODY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body1;
#[doc = "."]
pub struct USER_KEK_KEY_CODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code3;
#[doc = "."]
pub struct USER_KEK_BODY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body2;
#[doc = "."]
pub struct USER_KEK_KEY_CODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code4;
#[doc = "."]
pub struct USER_KEK_BODY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body3;
#[doc = "."]
pub struct USER_KEK_KEY_CODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code5;
#[doc = "."]
pub struct USER_KEK_BODY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body4;
#[doc = "."]
pub struct USER_KEK_KEY_CODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code6;
#[doc = "."]
pub struct USER_KEK_BODY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body5;
#[doc = "."]
pub struct USER_KEK_KEY_CODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code7;
#[doc = "."]
pub struct USER_KEK_BODY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body6;
#[doc = "."]
pub struct USER_KEK_KEY_CODE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code8;
#[doc = "."]
pub struct USER_KEK_BODY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body7;
#[doc = "."]
pub struct USER_KEK_KEY_CODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code9;
#[doc = "."]
pub struct USER_KEK_BODY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body8;
#[doc = "."]
pub struct USER_KEK_KEY_CODE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code10;
#[doc = "."]
pub struct USER_KEK_BODY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body9;
#[doc = "."]
pub struct USER_KEK_KEY_CODE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code11;
#[doc = "."]
pub struct USER_KEK_BODY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body10;
#[doc = "."]
pub struct USER_KEK_KEY_CODE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code12;
#[doc = "."]
pub struct USER_KEK_BODY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_body11;
#[doc = "."]
pub struct USER_KEK_KEY_CODE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod user_kek_key_code13;
#[doc = "."]
pub struct UDS_HEADER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_header0;
#[doc = "."]
pub struct UDS_KEY_CODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code0;
#[doc = "."]
pub struct UDS_HEADER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_header1;
#[doc = "."]
pub struct UDS_KEY_CODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code1;
#[doc = "."]
pub struct UDS_BODY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body0;
#[doc = "."]
pub struct UDS_KEY_CODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code2;
#[doc = "."]
pub struct UDS_BODY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body1;
#[doc = "."]
pub struct UDS_KEY_CODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code3;
#[doc = "."]
pub struct UDS_BODY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body2;
#[doc = "."]
pub struct UDS_KEY_CODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code4;
#[doc = "."]
pub struct UDS_BODY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body3;
#[doc = "."]
pub struct UDS_KEY_CODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code5;
#[doc = "."]
pub struct UDS_BODY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body4;
#[doc = "."]
pub struct UDS_KEY_CODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code6;
#[doc = "."]
pub struct UDS_BODY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body5;
#[doc = "."]
pub struct UDS_KEY_CODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code7;
#[doc = "."]
pub struct UDS_BODY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body6;
#[doc = "."]
pub struct UDS_KEY_CODE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code8;
#[doc = "."]
pub struct UDS_BODY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body7;
#[doc = "."]
pub struct UDS_KEY_CODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code9;
#[doc = "."]
pub struct UDS_BODY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body8;
#[doc = "."]
pub struct UDS_KEY_CODE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code10;
#[doc = "."]
pub struct UDS_BODY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body9;
#[doc = "."]
pub struct UDS_KEY_CODE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code11;
#[doc = "."]
pub struct UDS_BODY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body10;
#[doc = "."]
pub struct UDS_KEY_CODE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code12;
#[doc = "."]
pub struct UDS_BODY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_body11;
#[doc = "."]
pub struct UDS_KEY_CODE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod uds_key_code13;
#[doc = "."]
pub struct PRINCE_REGION0_HEADER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_header0;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code0;
#[doc = "."]
pub struct PRINCE_REGION0_HEADER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_header1;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code1;
#[doc = "."]
pub struct PRINCE_REGION0_BODY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body0;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code2;
#[doc = "."]
pub struct PRINCE_REGION0_BODY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body1;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code3;
#[doc = "."]
pub struct PRINCE_REGION0_BODY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body2;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code4;
#[doc = "."]
pub struct PRINCE_REGION0_BODY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body3;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code5;
#[doc = "."]
pub struct PRINCE_REGION0_BODY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body4;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code6;
#[doc = "."]
pub struct PRINCE_REGION0_BODY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body5;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code7;
#[doc = "."]
pub struct PRINCE_REGION0_BODY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body6;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code8;
#[doc = "."]
pub struct PRINCE_REGION0_BODY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body7;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code9;
#[doc = "."]
pub struct PRINCE_REGION0_BODY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body8;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code10;
#[doc = "."]
pub struct PRINCE_REGION0_BODY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body9;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code11;
#[doc = "."]
pub struct PRINCE_REGION0_BODY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body10;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code12;
#[doc = "."]
pub struct PRINCE_REGION0_BODY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_body11;
#[doc = "."]
pub struct PRINCE_REGION0_KEY_CODE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region0_key_code13;
#[doc = "."]
pub struct PRINCE_REGION1_HEADER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_header0;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code0;
#[doc = "."]
pub struct PRINCE_REGION1_HEADER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_header1;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code1;
#[doc = "."]
pub struct PRINCE_REGION1_BODY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body0;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code2;
#[doc = "."]
pub struct PRINCE_REGION1_BODY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body1;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code3;
#[doc = "."]
pub struct PRINCE_REGION1_BODY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body2;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code4;
#[doc = "."]
pub struct PRINCE_REGION1_BODY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body3;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code5;
#[doc = "."]
pub struct PRINCE_REGION1_BODY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body4;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code6;
#[doc = "."]
pub struct PRINCE_REGION1_BODY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body5;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code7;
#[doc = "."]
pub struct PRINCE_REGION1_BODY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body6;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code8;
#[doc = "."]
pub struct PRINCE_REGION1_BODY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body7;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code9;
#[doc = "."]
pub struct PRINCE_REGION1_BODY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body8;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code10;
#[doc = "."]
pub struct PRINCE_REGION1_BODY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body9;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code11;
#[doc = "."]
pub struct PRINCE_REGION1_BODY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body10;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code12;
#[doc = "."]
pub struct PRINCE_REGION1_BODY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_body11;
#[doc = "."]
pub struct PRINCE_REGION1_KEY_CODE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region1_key_code13;
#[doc = "."]
pub struct PRINCE_REGION2_HEADER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_header0;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code0;
#[doc = "."]
pub struct PRINCE_REGION2_HEADER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_header1;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code1;
#[doc = "."]
pub struct PRINCE_REGION2_BODY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body0;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code2;
#[doc = "."]
pub struct PRINCE_REGION2_BODY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body1;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code3;
#[doc = "."]
pub struct PRINCE_REGION2_BODY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body2;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code4;
#[doc = "."]
pub struct PRINCE_REGION2_BODY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body3;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code5;
#[doc = "."]
pub struct PRINCE_REGION2_BODY4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body4;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code6;
#[doc = "."]
pub struct PRINCE_REGION2_BODY5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body5;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code7;
#[doc = "."]
pub struct PRINCE_REGION2_BODY6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body6;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code8;
#[doc = "."]
pub struct PRINCE_REGION2_BODY7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body7;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code9;
#[doc = "."]
pub struct PRINCE_REGION2_BODY8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body8;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code10;
#[doc = "."]
pub struct PRINCE_REGION2_BODY9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body9;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code11;
#[doc = "."]
pub struct PRINCE_REGION2_BODY10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body10;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code12;
#[doc = "."]
pub struct PRINCE_REGION2_BODY11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_body11;
#[doc = "."]
pub struct PRINCE_REGION2_KEY_CODE13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "."]
pub mod prince_region2_key_code13;
