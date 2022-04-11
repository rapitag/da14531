#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip identification register 1."]
    pub chip_id1_reg: crate::Reg<chip_id1_reg::CHIP_ID1_REG_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Chip identification register 2."]
    pub chip_id2_reg: crate::Reg<chip_id2_reg::CHIP_ID2_REG_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - Chip identification register 3."]
    pub chip_id3_reg: crate::Reg<chip_id3_reg::CHIP_ID3_REG_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - Chip identification register 4."]
    pub chip_id4_reg: crate::Reg<chip_id4_reg::CHIP_ID4_REG_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - "]
    pub chip_swc_reg: crate::Reg<chip_swc_reg::CHIP_SWC_REG_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - "]
    pub chip_revision_reg: crate::Reg<chip_revision_reg::CHIP_REVISION_REG_SPEC>,
    _reserved6: [u8; 0xe2],
    #[doc = "0xf8 - "]
    pub chip_test1_reg: crate::Reg<chip_test1_reg::CHIP_TEST1_REG_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0xfc - "]
    pub chip_test2_reg: crate::Reg<chip_test2_reg::CHIP_TEST2_REG_SPEC>,
}
#[doc = "CHIP_ID1_REG register accessor: an alias for `Reg<CHIP_ID1_REG_SPEC>`"]
pub type CHIP_ID1_REG = crate::Reg<chip_id1_reg::CHIP_ID1_REG_SPEC>;
#[doc = "Chip identification register 1."]
pub mod chip_id1_reg;
#[doc = "CHIP_ID2_REG register accessor: an alias for `Reg<CHIP_ID2_REG_SPEC>`"]
pub type CHIP_ID2_REG = crate::Reg<chip_id2_reg::CHIP_ID2_REG_SPEC>;
#[doc = "Chip identification register 2."]
pub mod chip_id2_reg;
#[doc = "CHIP_ID3_REG register accessor: an alias for `Reg<CHIP_ID3_REG_SPEC>`"]
pub type CHIP_ID3_REG = crate::Reg<chip_id3_reg::CHIP_ID3_REG_SPEC>;
#[doc = "Chip identification register 3."]
pub mod chip_id3_reg;
#[doc = "CHIP_ID4_REG register accessor: an alias for `Reg<CHIP_ID4_REG_SPEC>`"]
pub type CHIP_ID4_REG = crate::Reg<chip_id4_reg::CHIP_ID4_REG_SPEC>;
#[doc = "Chip identification register 4."]
pub mod chip_id4_reg;
#[doc = "CHIP_REVISION_REG register accessor: an alias for `Reg<CHIP_REVISION_REG_SPEC>`"]
pub type CHIP_REVISION_REG = crate::Reg<chip_revision_reg::CHIP_REVISION_REG_SPEC>;
#[doc = ""]
pub mod chip_revision_reg;
#[doc = "CHIP_SWC_REG register accessor: an alias for `Reg<CHIP_SWC_REG_SPEC>`"]
pub type CHIP_SWC_REG = crate::Reg<chip_swc_reg::CHIP_SWC_REG_SPEC>;
#[doc = ""]
pub mod chip_swc_reg;
#[doc = "CHIP_TEST1_REG register accessor: an alias for `Reg<CHIP_TEST1_REG_SPEC>`"]
pub type CHIP_TEST1_REG = crate::Reg<chip_test1_reg::CHIP_TEST1_REG_SPEC>;
#[doc = ""]
pub mod chip_test1_reg;
#[doc = "CHIP_TEST2_REG register accessor: an alias for `Reg<CHIP_TEST2_REG_SPEC>`"]
pub type CHIP_TEST2_REG = crate::Reg<chip_test2_reg::CHIP_TEST2_REG_SPEC>;
#[doc = ""]
pub mod chip_test2_reg;
