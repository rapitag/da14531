#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Control Register"]
    pub i2c_con_reg: crate::Reg<i2c_con_reg::I2C_CON_REG_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - I2C Target Address Register"]
    pub i2c_tar_reg: crate::Reg<i2c_tar_reg::I2C_TAR_REG_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - I2C Slave Address Register"]
    pub i2c_sar_reg: crate::Reg<i2c_sar_reg::I2C_SAR_REG_SPEC>,
    _reserved3: [u8; 0x06],
    #[doc = "0x10 - I2C Rx/Tx Data Buffer and Command Register"]
    pub i2c_data_cmd_reg: crate::Reg<i2c_data_cmd_reg::I2C_DATA_CMD_REG_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Standard Speed I2C Clock SCL High Count Register"]
    pub i2c_ss_scl_hcnt_reg: crate::Reg<i2c_ss_scl_hcnt_reg::I2C_SS_SCL_HCNT_REG_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x18 - Standard Speed I2C Clock SCL Low Count Register"]
    pub i2c_ss_scl_lcnt_reg: crate::Reg<i2c_ss_scl_lcnt_reg::I2C_SS_SCL_LCNT_REG_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x1c - Fast Speed I2C Clock SCL High Count Register"]
    pub i2c_fs_scl_hcnt_reg: crate::Reg<i2c_fs_scl_hcnt_reg::I2C_FS_SCL_HCNT_REG_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x20 - Fast Speed I2C Clock SCL Low Count Register"]
    pub i2c_fs_scl_lcnt_reg: crate::Reg<i2c_fs_scl_lcnt_reg::I2C_FS_SCL_LCNT_REG_SPEC>,
    _reserved8: [u8; 0x0a],
    #[doc = "0x2c - I2C Interrupt Status Register"]
    pub i2c_intr_stat_reg: crate::Reg<i2c_intr_stat_reg::I2C_INTR_STAT_REG_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x30 - I2C Interrupt Mask Register"]
    pub i2c_intr_mask_reg: crate::Reg<i2c_intr_mask_reg::I2C_INTR_MASK_REG_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x34 - I2C Raw Interrupt Status Register"]
    pub i2c_raw_intr_stat_reg: crate::Reg<i2c_raw_intr_stat_reg::I2C_RAW_INTR_STAT_REG_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x38 - I2C Receive FIFO Threshold Register"]
    pub i2c_rx_tl_reg: crate::Reg<i2c_rx_tl_reg::I2C_RX_TL_REG_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x3c - I2C Transmit FIFO Threshold Register"]
    pub i2c_tx_tl_reg: crate::Reg<i2c_tx_tl_reg::I2C_TX_TL_REG_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x40 - Clear Combined and Individual Interrupt Register"]
    pub i2c_clr_intr_reg: crate::Reg<i2c_clr_intr_reg::I2C_CLR_INTR_REG_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    pub i2c_clr_rx_under_reg: crate::Reg<i2c_clr_rx_under_reg::I2C_CLR_RX_UNDER_REG_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    pub i2c_clr_rx_over_reg: crate::Reg<i2c_clr_rx_over_reg::I2C_CLR_RX_OVER_REG_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    pub i2c_clr_tx_over_reg: crate::Reg<i2c_clr_tx_over_reg::I2C_CLR_TX_OVER_REG_SPEC>,
    _reserved17: [u8; 0x02],
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    pub i2c_clr_rd_req_reg: crate::Reg<i2c_clr_rd_req_reg::I2C_CLR_RD_REQ_REG_SPEC>,
    _reserved18: [u8; 0x02],
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    pub i2c_clr_tx_abrt_reg: crate::Reg<i2c_clr_tx_abrt_reg::I2C_CLR_TX_ABRT_REG_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    pub i2c_clr_rx_done_reg: crate::Reg<i2c_clr_rx_done_reg::I2C_CLR_RX_DONE_REG_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    pub i2c_clr_activity_reg: crate::Reg<i2c_clr_activity_reg::I2C_CLR_ACTIVITY_REG_SPEC>,
    _reserved21: [u8; 0x02],
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    pub i2c_clr_stop_det_reg: crate::Reg<i2c_clr_stop_det_reg::I2C_CLR_STOP_DET_REG_SPEC>,
    _reserved22: [u8; 0x02],
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    pub i2c_clr_start_det_reg: crate::Reg<i2c_clr_start_det_reg::I2C_CLR_START_DET_REG_SPEC>,
    _reserved23: [u8; 0x02],
    #[doc = "0x68 - Clear GEN_CALL Interrupt Register"]
    pub i2c_clr_gen_call_reg: crate::Reg<i2c_clr_gen_call_reg::I2C_CLR_GEN_CALL_REG_SPEC>,
    _reserved24: [u8; 0x02],
    #[doc = "0x6c - I2C Enable Register"]
    pub i2c_enable_reg: crate::Reg<i2c_enable_reg::I2C_ENABLE_REG_SPEC>,
    _reserved25: [u8; 0x02],
    #[doc = "0x70 - I2C Status Register"]
    pub i2c_status_reg: crate::Reg<i2c_status_reg::I2C_STATUS_REG_SPEC>,
    _reserved26: [u8; 0x02],
    #[doc = "0x74 - I2C Transmit FIFO Level Register"]
    pub i2c_txflr_reg: crate::Reg<i2c_txflr_reg::I2C_TXFLR_REG_SPEC>,
    _reserved27: [u8; 0x02],
    #[doc = "0x78 - I2C Receive FIFO Level Register"]
    pub i2c_rxflr_reg: crate::Reg<i2c_rxflr_reg::I2C_RXFLR_REG_SPEC>,
    _reserved28: [u8; 0x02],
    #[doc = "0x7c - I2C SDA Hold Time Length Register"]
    pub i2c_sda_hold_reg: crate::Reg<i2c_sda_hold_reg::I2C_SDA_HOLD_REG_SPEC>,
    _reserved29: [u8; 0x02],
    #[doc = "0x80 - I2C Transmit Abort Source Register"]
    pub i2c_tx_abrt_source_reg: crate::Reg<i2c_tx_abrt_source_reg::I2C_TX_ABRT_SOURCE_REG_SPEC>,
    _reserved30: [u8; 0x06],
    #[doc = "0x88 - DMA Control Register"]
    pub i2c_dma_cr_reg: crate::Reg<i2c_dma_cr_reg::I2C_DMA_CR_REG_SPEC>,
    _reserved31: [u8; 0x02],
    #[doc = "0x8c - DMA Transmit Data Level Register"]
    pub i2c_dma_tdlr_reg: crate::Reg<i2c_dma_tdlr_reg::I2C_DMA_TDLR_REG_SPEC>,
    _reserved32: [u8; 0x02],
    #[doc = "0x90 - I2C Receive Data Level Register"]
    pub i2c_dma_rdlr_reg: crate::Reg<i2c_dma_rdlr_reg::I2C_DMA_RDLR_REG_SPEC>,
    _reserved33: [u8; 0x02],
    #[doc = "0x94 - I2C SDA Setup Register"]
    pub i2c_sda_setup_reg: crate::Reg<i2c_sda_setup_reg::I2C_SDA_SETUP_REG_SPEC>,
    _reserved34: [u8; 0x02],
    #[doc = "0x98 - I2C ACK General Call Register"]
    pub i2c_ack_general_call_reg:
        crate::Reg<i2c_ack_general_call_reg::I2C_ACK_GENERAL_CALL_REG_SPEC>,
    _reserved35: [u8; 0x02],
    #[doc = "0x9c - I2C Enable Status Register"]
    pub i2c_enable_status_reg: crate::Reg<i2c_enable_status_reg::I2C_ENABLE_STATUS_REG_SPEC>,
    _reserved36: [u8; 0x02],
    #[doc = "0xa0 - I2C SS and FS spike suppression limit Size"]
    pub i2c_ic_fs_spklen_reg: crate::Reg<i2c_ic_fs_spklen_reg::I2C_IC_FS_SPKLEN_REG_SPEC>,
    _reserved37: [u8; 0x52],
    #[doc = "0xf4 - "]
    pub i2c_comp_param1_reg: crate::Reg<i2c_comp_param1_reg::I2C_COMP_PARAM1_REG_SPEC>,
    #[doc = "0xf6 - "]
    pub i2c_comp_param2_reg: crate::Reg<i2c_comp_param2_reg::I2C_COMP_PARAM2_REG_SPEC>,
    #[doc = "0xf8 - "]
    pub i2c_comp_version_reg: crate::Reg<i2c_comp_version_reg::I2C_COMP_VERSION_REG_SPEC>,
    #[doc = "0xfa - "]
    pub i2c_comp2_version: crate::Reg<i2c_comp2_version::I2C_COMP2_VERSION_SPEC>,
    #[doc = "0xfc - "]
    pub i2c_comp_type_reg: crate::Reg<i2c_comp_type_reg::I2C_COMP_TYPE_REG_SPEC>,
    #[doc = "0xfe - "]
    pub i2c_comp_type2_reg: crate::Reg<i2c_comp_type2_reg::I2C_COMP_TYPE2_REG_SPEC>,
}
#[doc = "I2C_ACK_GENERAL_CALL_REG register accessor: an alias for `Reg<I2C_ACK_GENERAL_CALL_REG_SPEC>`"]
pub type I2C_ACK_GENERAL_CALL_REG =
    crate::Reg<i2c_ack_general_call_reg::I2C_ACK_GENERAL_CALL_REG_SPEC>;
#[doc = "I2C ACK General Call Register"]
pub mod i2c_ack_general_call_reg;
#[doc = "I2C_CLR_ACTIVITY_REG register accessor: an alias for `Reg<I2C_CLR_ACTIVITY_REG_SPEC>`"]
pub type I2C_CLR_ACTIVITY_REG = crate::Reg<i2c_clr_activity_reg::I2C_CLR_ACTIVITY_REG_SPEC>;
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod i2c_clr_activity_reg;
#[doc = "I2C_CLR_GEN_CALL_REG register accessor: an alias for `Reg<I2C_CLR_GEN_CALL_REG_SPEC>`"]
pub type I2C_CLR_GEN_CALL_REG = crate::Reg<i2c_clr_gen_call_reg::I2C_CLR_GEN_CALL_REG_SPEC>;
#[doc = "Clear GEN_CALL Interrupt Register"]
pub mod i2c_clr_gen_call_reg;
#[doc = "I2C_CLR_INTR_REG register accessor: an alias for `Reg<I2C_CLR_INTR_REG_SPEC>`"]
pub type I2C_CLR_INTR_REG = crate::Reg<i2c_clr_intr_reg::I2C_CLR_INTR_REG_SPEC>;
#[doc = "Clear Combined and Individual Interrupt Register"]
pub mod i2c_clr_intr_reg;
#[doc = "I2C_CLR_RD_REQ_REG register accessor: an alias for `Reg<I2C_CLR_RD_REQ_REG_SPEC>`"]
pub type I2C_CLR_RD_REQ_REG = crate::Reg<i2c_clr_rd_req_reg::I2C_CLR_RD_REQ_REG_SPEC>;
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod i2c_clr_rd_req_reg;
#[doc = "I2C_CLR_RX_DONE_REG register accessor: an alias for `Reg<I2C_CLR_RX_DONE_REG_SPEC>`"]
pub type I2C_CLR_RX_DONE_REG = crate::Reg<i2c_clr_rx_done_reg::I2C_CLR_RX_DONE_REG_SPEC>;
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod i2c_clr_rx_done_reg;
#[doc = "I2C_CLR_RX_OVER_REG register accessor: an alias for `Reg<I2C_CLR_RX_OVER_REG_SPEC>`"]
pub type I2C_CLR_RX_OVER_REG = crate::Reg<i2c_clr_rx_over_reg::I2C_CLR_RX_OVER_REG_SPEC>;
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod i2c_clr_rx_over_reg;
#[doc = "I2C_CLR_RX_UNDER_REG register accessor: an alias for `Reg<I2C_CLR_RX_UNDER_REG_SPEC>`"]
pub type I2C_CLR_RX_UNDER_REG = crate::Reg<i2c_clr_rx_under_reg::I2C_CLR_RX_UNDER_REG_SPEC>;
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod i2c_clr_rx_under_reg;
#[doc = "I2C_CLR_START_DET_REG register accessor: an alias for `Reg<I2C_CLR_START_DET_REG_SPEC>`"]
pub type I2C_CLR_START_DET_REG = crate::Reg<i2c_clr_start_det_reg::I2C_CLR_START_DET_REG_SPEC>;
#[doc = "Clear START_DET Interrupt Register"]
pub mod i2c_clr_start_det_reg;
#[doc = "I2C_CLR_STOP_DET_REG register accessor: an alias for `Reg<I2C_CLR_STOP_DET_REG_SPEC>`"]
pub type I2C_CLR_STOP_DET_REG = crate::Reg<i2c_clr_stop_det_reg::I2C_CLR_STOP_DET_REG_SPEC>;
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod i2c_clr_stop_det_reg;
#[doc = "I2C_CLR_TX_ABRT_REG register accessor: an alias for `Reg<I2C_CLR_TX_ABRT_REG_SPEC>`"]
pub type I2C_CLR_TX_ABRT_REG = crate::Reg<i2c_clr_tx_abrt_reg::I2C_CLR_TX_ABRT_REG_SPEC>;
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod i2c_clr_tx_abrt_reg;
#[doc = "I2C_CLR_TX_OVER_REG register accessor: an alias for `Reg<I2C_CLR_TX_OVER_REG_SPEC>`"]
pub type I2C_CLR_TX_OVER_REG = crate::Reg<i2c_clr_tx_over_reg::I2C_CLR_TX_OVER_REG_SPEC>;
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod i2c_clr_tx_over_reg;
#[doc = "I2C_COMP2_VERSION register accessor: an alias for `Reg<I2C_COMP2_VERSION_SPEC>`"]
pub type I2C_COMP2_VERSION = crate::Reg<i2c_comp2_version::I2C_COMP2_VERSION_SPEC>;
#[doc = ""]
pub mod i2c_comp2_version;
#[doc = "I2C_COMP_PARAM1_REG register accessor: an alias for `Reg<I2C_COMP_PARAM1_REG_SPEC>`"]
pub type I2C_COMP_PARAM1_REG = crate::Reg<i2c_comp_param1_reg::I2C_COMP_PARAM1_REG_SPEC>;
#[doc = ""]
pub mod i2c_comp_param1_reg;
#[doc = "I2C_COMP_PARAM2_REG register accessor: an alias for `Reg<I2C_COMP_PARAM2_REG_SPEC>`"]
pub type I2C_COMP_PARAM2_REG = crate::Reg<i2c_comp_param2_reg::I2C_COMP_PARAM2_REG_SPEC>;
#[doc = ""]
pub mod i2c_comp_param2_reg;
#[doc = "I2C_COMP_TYPE2_REG register accessor: an alias for `Reg<I2C_COMP_TYPE2_REG_SPEC>`"]
pub type I2C_COMP_TYPE2_REG = crate::Reg<i2c_comp_type2_reg::I2C_COMP_TYPE2_REG_SPEC>;
#[doc = ""]
pub mod i2c_comp_type2_reg;
#[doc = "I2C_COMP_TYPE_REG register accessor: an alias for `Reg<I2C_COMP_TYPE_REG_SPEC>`"]
pub type I2C_COMP_TYPE_REG = crate::Reg<i2c_comp_type_reg::I2C_COMP_TYPE_REG_SPEC>;
#[doc = ""]
pub mod i2c_comp_type_reg;
#[doc = "I2C_COMP_VERSION_REG register accessor: an alias for `Reg<I2C_COMP_VERSION_REG_SPEC>`"]
pub type I2C_COMP_VERSION_REG = crate::Reg<i2c_comp_version_reg::I2C_COMP_VERSION_REG_SPEC>;
#[doc = ""]
pub mod i2c_comp_version_reg;
#[doc = "I2C_CON_REG register accessor: an alias for `Reg<I2C_CON_REG_SPEC>`"]
pub type I2C_CON_REG = crate::Reg<i2c_con_reg::I2C_CON_REG_SPEC>;
#[doc = "I2C Control Register"]
pub mod i2c_con_reg;
#[doc = "I2C_DATA_CMD_REG register accessor: an alias for `Reg<I2C_DATA_CMD_REG_SPEC>`"]
pub type I2C_DATA_CMD_REG = crate::Reg<i2c_data_cmd_reg::I2C_DATA_CMD_REG_SPEC>;
#[doc = "I2C Rx/Tx Data Buffer and Command Register"]
pub mod i2c_data_cmd_reg;
#[doc = "I2C_DMA_CR_REG register accessor: an alias for `Reg<I2C_DMA_CR_REG_SPEC>`"]
pub type I2C_DMA_CR_REG = crate::Reg<i2c_dma_cr_reg::I2C_DMA_CR_REG_SPEC>;
#[doc = "DMA Control Register"]
pub mod i2c_dma_cr_reg;
#[doc = "I2C_DMA_RDLR_REG register accessor: an alias for `Reg<I2C_DMA_RDLR_REG_SPEC>`"]
pub type I2C_DMA_RDLR_REG = crate::Reg<i2c_dma_rdlr_reg::I2C_DMA_RDLR_REG_SPEC>;
#[doc = "I2C Receive Data Level Register"]
pub mod i2c_dma_rdlr_reg;
#[doc = "I2C_DMA_TDLR_REG register accessor: an alias for `Reg<I2C_DMA_TDLR_REG_SPEC>`"]
pub type I2C_DMA_TDLR_REG = crate::Reg<i2c_dma_tdlr_reg::I2C_DMA_TDLR_REG_SPEC>;
#[doc = "DMA Transmit Data Level Register"]
pub mod i2c_dma_tdlr_reg;
#[doc = "I2C_ENABLE_REG register accessor: an alias for `Reg<I2C_ENABLE_REG_SPEC>`"]
pub type I2C_ENABLE_REG = crate::Reg<i2c_enable_reg::I2C_ENABLE_REG_SPEC>;
#[doc = "I2C Enable Register"]
pub mod i2c_enable_reg;
#[doc = "I2C_ENABLE_STATUS_REG register accessor: an alias for `Reg<I2C_ENABLE_STATUS_REG_SPEC>`"]
pub type I2C_ENABLE_STATUS_REG = crate::Reg<i2c_enable_status_reg::I2C_ENABLE_STATUS_REG_SPEC>;
#[doc = "I2C Enable Status Register"]
pub mod i2c_enable_status_reg;
#[doc = "I2C_FS_SCL_HCNT_REG register accessor: an alias for `Reg<I2C_FS_SCL_HCNT_REG_SPEC>`"]
pub type I2C_FS_SCL_HCNT_REG = crate::Reg<i2c_fs_scl_hcnt_reg::I2C_FS_SCL_HCNT_REG_SPEC>;
#[doc = "Fast Speed I2C Clock SCL High Count Register"]
pub mod i2c_fs_scl_hcnt_reg;
#[doc = "I2C_FS_SCL_LCNT_REG register accessor: an alias for `Reg<I2C_FS_SCL_LCNT_REG_SPEC>`"]
pub type I2C_FS_SCL_LCNT_REG = crate::Reg<i2c_fs_scl_lcnt_reg::I2C_FS_SCL_LCNT_REG_SPEC>;
#[doc = "Fast Speed I2C Clock SCL Low Count Register"]
pub mod i2c_fs_scl_lcnt_reg;
#[doc = "I2C_IC_FS_SPKLEN_REG register accessor: an alias for `Reg<I2C_IC_FS_SPKLEN_REG_SPEC>`"]
pub type I2C_IC_FS_SPKLEN_REG = crate::Reg<i2c_ic_fs_spklen_reg::I2C_IC_FS_SPKLEN_REG_SPEC>;
#[doc = "I2C SS and FS spike suppression limit Size"]
pub mod i2c_ic_fs_spklen_reg;
#[doc = "I2C_INTR_MASK_REG register accessor: an alias for `Reg<I2C_INTR_MASK_REG_SPEC>`"]
pub type I2C_INTR_MASK_REG = crate::Reg<i2c_intr_mask_reg::I2C_INTR_MASK_REG_SPEC>;
#[doc = "I2C Interrupt Mask Register"]
pub mod i2c_intr_mask_reg;
#[doc = "I2C_INTR_STAT_REG register accessor: an alias for `Reg<I2C_INTR_STAT_REG_SPEC>`"]
pub type I2C_INTR_STAT_REG = crate::Reg<i2c_intr_stat_reg::I2C_INTR_STAT_REG_SPEC>;
#[doc = "I2C Interrupt Status Register"]
pub mod i2c_intr_stat_reg;
#[doc = "I2C_RAW_INTR_STAT_REG register accessor: an alias for `Reg<I2C_RAW_INTR_STAT_REG_SPEC>`"]
pub type I2C_RAW_INTR_STAT_REG = crate::Reg<i2c_raw_intr_stat_reg::I2C_RAW_INTR_STAT_REG_SPEC>;
#[doc = "I2C Raw Interrupt Status Register"]
pub mod i2c_raw_intr_stat_reg;
#[doc = "I2C_RXFLR_REG register accessor: an alias for `Reg<I2C_RXFLR_REG_SPEC>`"]
pub type I2C_RXFLR_REG = crate::Reg<i2c_rxflr_reg::I2C_RXFLR_REG_SPEC>;
#[doc = "I2C Receive FIFO Level Register"]
pub mod i2c_rxflr_reg;
#[doc = "I2C_RX_TL_REG register accessor: an alias for `Reg<I2C_RX_TL_REG_SPEC>`"]
pub type I2C_RX_TL_REG = crate::Reg<i2c_rx_tl_reg::I2C_RX_TL_REG_SPEC>;
#[doc = "I2C Receive FIFO Threshold Register"]
pub mod i2c_rx_tl_reg;
#[doc = "I2C_SAR_REG register accessor: an alias for `Reg<I2C_SAR_REG_SPEC>`"]
pub type I2C_SAR_REG = crate::Reg<i2c_sar_reg::I2C_SAR_REG_SPEC>;
#[doc = "I2C Slave Address Register"]
pub mod i2c_sar_reg;
#[doc = "I2C_SDA_HOLD_REG register accessor: an alias for `Reg<I2C_SDA_HOLD_REG_SPEC>`"]
pub type I2C_SDA_HOLD_REG = crate::Reg<i2c_sda_hold_reg::I2C_SDA_HOLD_REG_SPEC>;
#[doc = "I2C SDA Hold Time Length Register"]
pub mod i2c_sda_hold_reg;
#[doc = "I2C_SDA_SETUP_REG register accessor: an alias for `Reg<I2C_SDA_SETUP_REG_SPEC>`"]
pub type I2C_SDA_SETUP_REG = crate::Reg<i2c_sda_setup_reg::I2C_SDA_SETUP_REG_SPEC>;
#[doc = "I2C SDA Setup Register"]
pub mod i2c_sda_setup_reg;
#[doc = "I2C_SS_SCL_HCNT_REG register accessor: an alias for `Reg<I2C_SS_SCL_HCNT_REG_SPEC>`"]
pub type I2C_SS_SCL_HCNT_REG = crate::Reg<i2c_ss_scl_hcnt_reg::I2C_SS_SCL_HCNT_REG_SPEC>;
#[doc = "Standard Speed I2C Clock SCL High Count Register"]
pub mod i2c_ss_scl_hcnt_reg;
#[doc = "I2C_SS_SCL_LCNT_REG register accessor: an alias for `Reg<I2C_SS_SCL_LCNT_REG_SPEC>`"]
pub type I2C_SS_SCL_LCNT_REG = crate::Reg<i2c_ss_scl_lcnt_reg::I2C_SS_SCL_LCNT_REG_SPEC>;
#[doc = "Standard Speed I2C Clock SCL Low Count Register"]
pub mod i2c_ss_scl_lcnt_reg;
#[doc = "I2C_STATUS_REG register accessor: an alias for `Reg<I2C_STATUS_REG_SPEC>`"]
pub type I2C_STATUS_REG = crate::Reg<i2c_status_reg::I2C_STATUS_REG_SPEC>;
#[doc = "I2C Status Register"]
pub mod i2c_status_reg;
#[doc = "I2C_TAR_REG register accessor: an alias for `Reg<I2C_TAR_REG_SPEC>`"]
pub type I2C_TAR_REG = crate::Reg<i2c_tar_reg::I2C_TAR_REG_SPEC>;
#[doc = "I2C Target Address Register"]
pub mod i2c_tar_reg;
#[doc = "I2C_TXFLR_REG register accessor: an alias for `Reg<I2C_TXFLR_REG_SPEC>`"]
pub type I2C_TXFLR_REG = crate::Reg<i2c_txflr_reg::I2C_TXFLR_REG_SPEC>;
#[doc = "I2C Transmit FIFO Level Register"]
pub mod i2c_txflr_reg;
#[doc = "I2C_TX_ABRT_SOURCE_REG register accessor: an alias for `Reg<I2C_TX_ABRT_SOURCE_REG_SPEC>`"]
pub type I2C_TX_ABRT_SOURCE_REG = crate::Reg<i2c_tx_abrt_source_reg::I2C_TX_ABRT_SOURCE_REG_SPEC>;
#[doc = "I2C Transmit Abort Source Register"]
pub mod i2c_tx_abrt_source_reg;
#[doc = "I2C_TX_TL_REG register accessor: an alias for `Reg<I2C_TX_TL_REG_SPEC>`"]
pub type I2C_TX_TL_REG = crate::Reg<i2c_tx_tl_reg::I2C_TX_TL_REG_SPEC>;
#[doc = "I2C Transmit FIFO Threshold Register"]
pub mod i2c_tx_tl_reg;
