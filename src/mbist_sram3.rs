#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub mbist_sram3_addr_reg: crate::Reg<mbist_sram3_addr_reg::MBIST_SRAM3_ADDR_REG_SPEC>,
    #[doc = "0x02 - "]
    pub mbist_sram3_state_reg: crate::Reg<mbist_sram3_state_reg::MBIST_SRAM3_STATE_REG_SPEC>,
    #[doc = "0x04 - "]
    pub mbist_sram3_rd_msb_reg: crate::Reg<mbist_sram3_rd_msb_reg::MBIST_SRAM3_RD_MSB_REG_SPEC>,
    #[doc = "0x06 - "]
    pub mbist_sram3_rd_lsb_reg: crate::Reg<mbist_sram3_rd_lsb_reg::MBIST_SRAM3_RD_LSB_REG_SPEC>,
}
#[doc = "MBIST_SRAM3_ADDR_REG register accessor: an alias for `Reg<MBIST_SRAM3_ADDR_REG_SPEC>`"]
pub type MBIST_SRAM3_ADDR_REG = crate::Reg<mbist_sram3_addr_reg::MBIST_SRAM3_ADDR_REG_SPEC>;
#[doc = ""]
pub mod mbist_sram3_addr_reg;
#[doc = "MBIST_SRAM3_RD_LSB_REG register accessor: an alias for `Reg<MBIST_SRAM3_RD_LSB_REG_SPEC>`"]
pub type MBIST_SRAM3_RD_LSB_REG = crate::Reg<mbist_sram3_rd_lsb_reg::MBIST_SRAM3_RD_LSB_REG_SPEC>;
#[doc = ""]
pub mod mbist_sram3_rd_lsb_reg;
#[doc = "MBIST_SRAM3_RD_MSB_REG register accessor: an alias for `Reg<MBIST_SRAM3_RD_MSB_REG_SPEC>`"]
pub type MBIST_SRAM3_RD_MSB_REG = crate::Reg<mbist_sram3_rd_msb_reg::MBIST_SRAM3_RD_MSB_REG_SPEC>;
#[doc = ""]
pub mod mbist_sram3_rd_msb_reg;
#[doc = "MBIST_SRAM3_STATE_REG register accessor: an alias for `Reg<MBIST_SRAM3_STATE_REG_SPEC>`"]
pub type MBIST_SRAM3_STATE_REG = crate::Reg<mbist_sram3_state_reg::MBIST_SRAM3_STATE_REG_SPEC>;
#[doc = ""]
pub mod mbist_sram3_state_reg;
