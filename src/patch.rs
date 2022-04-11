#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub patch_valid_reg: crate::Reg<patch_valid_reg::PATCH_VALID_REG_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - "]
    pub patch_addr0_reg: crate::Reg<patch_addr0_reg::PATCH_ADDR0_REG_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x28 - "]
    pub patch_addr1_reg: crate::Reg<patch_addr1_reg::PATCH_ADDR1_REG_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x30 - "]
    pub patch_addr2_reg: crate::Reg<patch_addr2_reg::PATCH_ADDR2_REG_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x38 - "]
    pub patch_addr3_reg: crate::Reg<patch_addr3_reg::PATCH_ADDR3_REG_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x40 - "]
    pub patch_addr4_reg: crate::Reg<patch_addr4_reg::PATCH_ADDR4_REG_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x48 - "]
    pub patch_addr5_reg: crate::Reg<patch_addr5_reg::PATCH_ADDR5_REG_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x50 - "]
    pub patch_addr6_reg: crate::Reg<patch_addr6_reg::PATCH_ADDR6_REG_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x58 - "]
    pub patch_addr7_reg: crate::Reg<patch_addr7_reg::PATCH_ADDR7_REG_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x60 - "]
    pub patch_addr8_reg: crate::Reg<patch_addr8_reg::PATCH_ADDR8_REG_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x68 - "]
    pub patch_addr9_reg: crate::Reg<patch_addr9_reg::PATCH_ADDR9_REG_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x70 - "]
    pub patch_addr10_reg: crate::Reg<patch_addr10_reg::PATCH_ADDR10_REG_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x78 - "]
    pub patch_addr11_reg: crate::Reg<patch_addr11_reg::PATCH_ADDR11_REG_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x80 - "]
    pub patch_addr12_reg: crate::Reg<patch_addr12_reg::PATCH_ADDR12_REG_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x88 - "]
    pub patch_addr13_reg: crate::Reg<patch_addr13_reg::PATCH_ADDR13_REG_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x90 - "]
    pub patch_addr14_reg: crate::Reg<patch_addr14_reg::PATCH_ADDR14_REG_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x98 - "]
    pub patch_addr15_reg: crate::Reg<patch_addr15_reg::PATCH_ADDR15_REG_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0xa0 - "]
    pub patch_addr16_reg: crate::Reg<patch_addr16_reg::PATCH_ADDR16_REG_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0xa8 - "]
    pub patch_addr17_reg: crate::Reg<patch_addr17_reg::PATCH_ADDR17_REG_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0xb0 - "]
    pub patch_addr18_reg: crate::Reg<patch_addr18_reg::PATCH_ADDR18_REG_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0xb8 - "]
    pub patch_addr19_reg: crate::Reg<patch_addr19_reg::PATCH_ADDR19_REG_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0xc0 - "]
    pub patch_addr20_reg: crate::Reg<patch_addr20_reg::PATCH_ADDR20_REG_SPEC>,
    #[doc = "0xc4 - "]
    pub patch_data20_reg: crate::Reg<patch_data20_reg::PATCH_DATA20_REG_SPEC>,
    #[doc = "0xc8 - "]
    pub patch_addr21_reg: crate::Reg<patch_addr21_reg::PATCH_ADDR21_REG_SPEC>,
    #[doc = "0xcc - "]
    pub patch_data21_reg: crate::Reg<patch_data21_reg::PATCH_DATA21_REG_SPEC>,
}
#[doc = "PATCH_ADDR0_REG register accessor: an alias for `Reg<PATCH_ADDR0_REG_SPEC>`"]
pub type PATCH_ADDR0_REG = crate::Reg<patch_addr0_reg::PATCH_ADDR0_REG_SPEC>;
#[doc = ""]
pub mod patch_addr0_reg;
#[doc = "PATCH_ADDR10_REG register accessor: an alias for `Reg<PATCH_ADDR10_REG_SPEC>`"]
pub type PATCH_ADDR10_REG = crate::Reg<patch_addr10_reg::PATCH_ADDR10_REG_SPEC>;
#[doc = ""]
pub mod patch_addr10_reg;
#[doc = "PATCH_ADDR11_REG register accessor: an alias for `Reg<PATCH_ADDR11_REG_SPEC>`"]
pub type PATCH_ADDR11_REG = crate::Reg<patch_addr11_reg::PATCH_ADDR11_REG_SPEC>;
#[doc = ""]
pub mod patch_addr11_reg;
#[doc = "PATCH_ADDR12_REG register accessor: an alias for `Reg<PATCH_ADDR12_REG_SPEC>`"]
pub type PATCH_ADDR12_REG = crate::Reg<patch_addr12_reg::PATCH_ADDR12_REG_SPEC>;
#[doc = ""]
pub mod patch_addr12_reg;
#[doc = "PATCH_ADDR13_REG register accessor: an alias for `Reg<PATCH_ADDR13_REG_SPEC>`"]
pub type PATCH_ADDR13_REG = crate::Reg<patch_addr13_reg::PATCH_ADDR13_REG_SPEC>;
#[doc = ""]
pub mod patch_addr13_reg;
#[doc = "PATCH_ADDR14_REG register accessor: an alias for `Reg<PATCH_ADDR14_REG_SPEC>`"]
pub type PATCH_ADDR14_REG = crate::Reg<patch_addr14_reg::PATCH_ADDR14_REG_SPEC>;
#[doc = ""]
pub mod patch_addr14_reg;
#[doc = "PATCH_ADDR15_REG register accessor: an alias for `Reg<PATCH_ADDR15_REG_SPEC>`"]
pub type PATCH_ADDR15_REG = crate::Reg<patch_addr15_reg::PATCH_ADDR15_REG_SPEC>;
#[doc = ""]
pub mod patch_addr15_reg;
#[doc = "PATCH_ADDR16_REG register accessor: an alias for `Reg<PATCH_ADDR16_REG_SPEC>`"]
pub type PATCH_ADDR16_REG = crate::Reg<patch_addr16_reg::PATCH_ADDR16_REG_SPEC>;
#[doc = ""]
pub mod patch_addr16_reg;
#[doc = "PATCH_ADDR17_REG register accessor: an alias for `Reg<PATCH_ADDR17_REG_SPEC>`"]
pub type PATCH_ADDR17_REG = crate::Reg<patch_addr17_reg::PATCH_ADDR17_REG_SPEC>;
#[doc = ""]
pub mod patch_addr17_reg;
#[doc = "PATCH_ADDR18_REG register accessor: an alias for `Reg<PATCH_ADDR18_REG_SPEC>`"]
pub type PATCH_ADDR18_REG = crate::Reg<patch_addr18_reg::PATCH_ADDR18_REG_SPEC>;
#[doc = ""]
pub mod patch_addr18_reg;
#[doc = "PATCH_ADDR19_REG register accessor: an alias for `Reg<PATCH_ADDR19_REG_SPEC>`"]
pub type PATCH_ADDR19_REG = crate::Reg<patch_addr19_reg::PATCH_ADDR19_REG_SPEC>;
#[doc = ""]
pub mod patch_addr19_reg;
#[doc = "PATCH_ADDR1_REG register accessor: an alias for `Reg<PATCH_ADDR1_REG_SPEC>`"]
pub type PATCH_ADDR1_REG = crate::Reg<patch_addr1_reg::PATCH_ADDR1_REG_SPEC>;
#[doc = ""]
pub mod patch_addr1_reg;
#[doc = "PATCH_ADDR20_REG register accessor: an alias for `Reg<PATCH_ADDR20_REG_SPEC>`"]
pub type PATCH_ADDR20_REG = crate::Reg<patch_addr20_reg::PATCH_ADDR20_REG_SPEC>;
#[doc = ""]
pub mod patch_addr20_reg;
#[doc = "PATCH_ADDR21_REG register accessor: an alias for `Reg<PATCH_ADDR21_REG_SPEC>`"]
pub type PATCH_ADDR21_REG = crate::Reg<patch_addr21_reg::PATCH_ADDR21_REG_SPEC>;
#[doc = ""]
pub mod patch_addr21_reg;
#[doc = "PATCH_ADDR2_REG register accessor: an alias for `Reg<PATCH_ADDR2_REG_SPEC>`"]
pub type PATCH_ADDR2_REG = crate::Reg<patch_addr2_reg::PATCH_ADDR2_REG_SPEC>;
#[doc = ""]
pub mod patch_addr2_reg;
#[doc = "PATCH_ADDR3_REG register accessor: an alias for `Reg<PATCH_ADDR3_REG_SPEC>`"]
pub type PATCH_ADDR3_REG = crate::Reg<patch_addr3_reg::PATCH_ADDR3_REG_SPEC>;
#[doc = ""]
pub mod patch_addr3_reg;
#[doc = "PATCH_ADDR4_REG register accessor: an alias for `Reg<PATCH_ADDR4_REG_SPEC>`"]
pub type PATCH_ADDR4_REG = crate::Reg<patch_addr4_reg::PATCH_ADDR4_REG_SPEC>;
#[doc = ""]
pub mod patch_addr4_reg;
#[doc = "PATCH_ADDR5_REG register accessor: an alias for `Reg<PATCH_ADDR5_REG_SPEC>`"]
pub type PATCH_ADDR5_REG = crate::Reg<patch_addr5_reg::PATCH_ADDR5_REG_SPEC>;
#[doc = ""]
pub mod patch_addr5_reg;
#[doc = "PATCH_ADDR6_REG register accessor: an alias for `Reg<PATCH_ADDR6_REG_SPEC>`"]
pub type PATCH_ADDR6_REG = crate::Reg<patch_addr6_reg::PATCH_ADDR6_REG_SPEC>;
#[doc = ""]
pub mod patch_addr6_reg;
#[doc = "PATCH_ADDR7_REG register accessor: an alias for `Reg<PATCH_ADDR7_REG_SPEC>`"]
pub type PATCH_ADDR7_REG = crate::Reg<patch_addr7_reg::PATCH_ADDR7_REG_SPEC>;
#[doc = ""]
pub mod patch_addr7_reg;
#[doc = "PATCH_ADDR8_REG register accessor: an alias for `Reg<PATCH_ADDR8_REG_SPEC>`"]
pub type PATCH_ADDR8_REG = crate::Reg<patch_addr8_reg::PATCH_ADDR8_REG_SPEC>;
#[doc = ""]
pub mod patch_addr8_reg;
#[doc = "PATCH_ADDR9_REG register accessor: an alias for `Reg<PATCH_ADDR9_REG_SPEC>`"]
pub type PATCH_ADDR9_REG = crate::Reg<patch_addr9_reg::PATCH_ADDR9_REG_SPEC>;
#[doc = ""]
pub mod patch_addr9_reg;
#[doc = "PATCH_DATA20_REG register accessor: an alias for `Reg<PATCH_DATA20_REG_SPEC>`"]
pub type PATCH_DATA20_REG = crate::Reg<patch_data20_reg::PATCH_DATA20_REG_SPEC>;
#[doc = ""]
pub mod patch_data20_reg;
#[doc = "PATCH_DATA21_REG register accessor: an alias for `Reg<PATCH_DATA21_REG_SPEC>`"]
pub type PATCH_DATA21_REG = crate::Reg<patch_data21_reg::PATCH_DATA21_REG_SPEC>;
#[doc = ""]
pub mod patch_data21_reg;
#[doc = "PATCH_VALID_REG register accessor: an alias for `Reg<PATCH_VALID_REG_SPEC>`"]
pub type PATCH_VALID_REG = crate::Reg<patch_valid_reg::PATCH_VALID_REG_SPEC>;
#[doc = ""]
pub mod patch_valid_reg;
