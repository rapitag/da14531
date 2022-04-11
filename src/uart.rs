#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Receive Buffer Register/Transmit Holding Register/Divisor Latch Low"]
    pub uart_rbr_thr_dll_reg: crate::Reg<uart_rbr_thr_dll_reg::UART_RBR_THR_DLL_REG_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Interrupt Enable Register/Divisor Latch High"]
    pub uart_ier_dlh_reg: crate::Reg<uart_ier_dlh_reg::UART_IER_DLH_REG_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - Interrupt Identification Register/FIFO Control Register"]
    pub uart_iir_fcr_reg: crate::Reg<uart_iir_fcr_reg::UART_IIR_FCR_REG_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - Line Control Register"]
    pub uart_lcr_reg: crate::Reg<uart_lcr_reg::UART_LCR_REG_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - Modem Control Register"]
    pub uart_mcr_reg: crate::Reg<uart_mcr_reg::UART_MCR_REG_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - Line Status Register"]
    pub uart_lsr_reg: crate::Reg<uart_lsr_reg::UART_LSR_REG_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - Modem Status Register"]
    pub uart_msr_reg: crate::Reg<uart_msr_reg::UART_MSR_REG_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - Scratchpad Register"]
    pub uart_scr_reg: crate::Reg<uart_scr_reg::UART_SCR_REG_SPEC>,
    _reserved8: [u8; 0x12],
    #[doc = "0x30 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr0_reg: crate::Reg<uart_srbr_sthr0_reg::UART_SRBR_STHR0_REG_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x34 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr1_reg: crate::Reg<uart_srbr_sthr1_reg::UART_SRBR_STHR1_REG_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x38 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr2_reg: crate::Reg<uart_srbr_sthr2_reg::UART_SRBR_STHR2_REG_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x3c - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr3_reg: crate::Reg<uart_srbr_sthr3_reg::UART_SRBR_STHR3_REG_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x40 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr4_reg: crate::Reg<uart_srbr_sthr4_reg::UART_SRBR_STHR4_REG_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x44 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr5_reg: crate::Reg<uart_srbr_sthr5_reg::UART_SRBR_STHR5_REG_SPEC>,
    _reserved14: [u8; 0x02],
    #[doc = "0x48 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr6_reg: crate::Reg<uart_srbr_sthr6_reg::UART_SRBR_STHR6_REG_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x4c - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr7_reg: crate::Reg<uart_srbr_sthr7_reg::UART_SRBR_STHR7_REG_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x50 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr8_reg: crate::Reg<uart_srbr_sthr8_reg::UART_SRBR_STHR8_REG_SPEC>,
    _reserved17: [u8; 0x02],
    #[doc = "0x54 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr9_reg: crate::Reg<uart_srbr_sthr9_reg::UART_SRBR_STHR9_REG_SPEC>,
    _reserved18: [u8; 0x02],
    #[doc = "0x58 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr10_reg: crate::Reg<uart_srbr_sthr10_reg::UART_SRBR_STHR10_REG_SPEC>,
    _reserved19: [u8; 0x02],
    #[doc = "0x5c - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr11_reg: crate::Reg<uart_srbr_sthr11_reg::UART_SRBR_STHR11_REG_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x60 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr12_reg: crate::Reg<uart_srbr_sthr12_reg::UART_SRBR_STHR12_REG_SPEC>,
    _reserved21: [u8; 0x02],
    #[doc = "0x64 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr13_reg: crate::Reg<uart_srbr_sthr13_reg::UART_SRBR_STHR13_REG_SPEC>,
    _reserved22: [u8; 0x02],
    #[doc = "0x68 - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr14_reg: crate::Reg<uart_srbr_sthr14_reg::UART_SRBR_STHR14_REG_SPEC>,
    _reserved23: [u8; 0x02],
    #[doc = "0x6c - Shadow Receive/Transmit Buffer Register"]
    pub uart_srbr_sthr15_reg: crate::Reg<uart_srbr_sthr15_reg::UART_SRBR_STHR15_REG_SPEC>,
    _reserved24: [u8; 0x02],
    #[doc = "0x70 - FIFO Access Register"]
    pub uart_far_reg: crate::Reg<uart_far_reg::UART_FAR_REG_SPEC>,
    _reserved25: [u8; 0x0a],
    #[doc = "0x7c - UART Status Register"]
    pub uart_usr_reg: crate::Reg<uart_usr_reg::UART_USR_REG_SPEC>,
    _reserved26: [u8; 0x02],
    #[doc = "0x80 - Transmit FIFO Level"]
    pub uart_tfl_reg: crate::Reg<uart_tfl_reg::UART_TFL_REG_SPEC>,
    _reserved27: [u8; 0x02],
    #[doc = "0x84 - Receive FIFO Level"]
    pub uart_rfl_reg: crate::Reg<uart_rfl_reg::UART_RFL_REG_SPEC>,
    _reserved28: [u8; 0x02],
    #[doc = "0x88 - Software Reset Register."]
    pub uart_srr_reg: crate::Reg<uart_srr_reg::UART_SRR_REG_SPEC>,
    _reserved29: [u8; 0x02],
    #[doc = "0x8c - Shadow Request to Send"]
    pub uart_srts_reg: crate::Reg<uart_srts_reg::UART_SRTS_REG_SPEC>,
    _reserved30: [u8; 0x02],
    #[doc = "0x90 - Shadow Break Control Register"]
    pub uart_sbcr_reg: crate::Reg<uart_sbcr_reg::UART_SBCR_REG_SPEC>,
    _reserved31: [u8; 0x02],
    #[doc = "0x94 - Shadow DMA Mode"]
    pub uart_sdmam_reg: crate::Reg<uart_sdmam_reg::UART_SDMAM_REG_SPEC>,
    _reserved32: [u8; 0x02],
    #[doc = "0x98 - Shadow FIFO Enable"]
    pub uart_sfe_reg: crate::Reg<uart_sfe_reg::UART_SFE_REG_SPEC>,
    _reserved33: [u8; 0x02],
    #[doc = "0x9c - Shadow RCVR Trigger"]
    pub uart_srt_reg: crate::Reg<uart_srt_reg::UART_SRT_REG_SPEC>,
    _reserved34: [u8; 0x02],
    #[doc = "0xa0 - Shadow TX Empty Trigger"]
    pub uart_stet_reg: crate::Reg<uart_stet_reg::UART_STET_REG_SPEC>,
    _reserved35: [u8; 0x02],
    #[doc = "0xa4 - Halt TX"]
    pub uart_htx_reg: crate::Reg<uart_htx_reg::UART_HTX_REG_SPEC>,
    _reserved36: [u8; 0x02],
    #[doc = "0xa8 - DMA Software Acknowledge"]
    pub uart_dmasa_reg: crate::Reg<uart_dmasa_reg::UART_DMASA_REG_SPEC>,
    _reserved37: [u8; 0x16],
    #[doc = "0xc0 - Divisor Latch Fraction Register"]
    pub uart_dlf_reg: crate::Reg<uart_dlf_reg::UART_DLF_REG_SPEC>,
    _reserved38: [u8; 0x36],
    #[doc = "0xf8 - Component Version"]
    pub uart_ucv_reg: crate::Reg<uart_ucv_reg::UART_UCV_REG_SPEC>,
    #[doc = "0xfa - Component Version"]
    pub uart_ucv_high_reg: crate::Reg<uart_ucv_high_reg::UART_UCV_HIGH_REG_SPEC>,
    #[doc = "0xfc - Component Type Register"]
    pub uart_ctr_reg: crate::Reg<uart_ctr_reg::UART_CTR_REG_SPEC>,
    #[doc = "0xfe - Component Type Register"]
    pub uart_ctr_high_reg: crate::Reg<uart_ctr_high_reg::UART_CTR_HIGH_REG_SPEC>,
}
#[doc = "UART_CTR_HIGH_REG register accessor: an alias for `Reg<UART_CTR_HIGH_REG_SPEC>`"]
pub type UART_CTR_HIGH_REG = crate::Reg<uart_ctr_high_reg::UART_CTR_HIGH_REG_SPEC>;
#[doc = "Component Type Register"]
pub mod uart_ctr_high_reg;
#[doc = "UART_CTR_REG register accessor: an alias for `Reg<UART_CTR_REG_SPEC>`"]
pub type UART_CTR_REG = crate::Reg<uart_ctr_reg::UART_CTR_REG_SPEC>;
#[doc = "Component Type Register"]
pub mod uart_ctr_reg;
#[doc = "UART_DLF_REG register accessor: an alias for `Reg<UART_DLF_REG_SPEC>`"]
pub type UART_DLF_REG = crate::Reg<uart_dlf_reg::UART_DLF_REG_SPEC>;
#[doc = "Divisor Latch Fraction Register"]
pub mod uart_dlf_reg;
#[doc = "UART_DMASA_REG register accessor: an alias for `Reg<UART_DMASA_REG_SPEC>`"]
pub type UART_DMASA_REG = crate::Reg<uart_dmasa_reg::UART_DMASA_REG_SPEC>;
#[doc = "DMA Software Acknowledge"]
pub mod uart_dmasa_reg;
#[doc = "UART_FAR_REG register accessor: an alias for `Reg<UART_FAR_REG_SPEC>`"]
pub type UART_FAR_REG = crate::Reg<uart_far_reg::UART_FAR_REG_SPEC>;
#[doc = "FIFO Access Register"]
pub mod uart_far_reg;
#[doc = "UART_HTX_REG register accessor: an alias for `Reg<UART_HTX_REG_SPEC>`"]
pub type UART_HTX_REG = crate::Reg<uart_htx_reg::UART_HTX_REG_SPEC>;
#[doc = "Halt TX"]
pub mod uart_htx_reg;
#[doc = "UART_IER_DLH_REG register accessor: an alias for `Reg<UART_IER_DLH_REG_SPEC>`"]
pub type UART_IER_DLH_REG = crate::Reg<uart_ier_dlh_reg::UART_IER_DLH_REG_SPEC>;
#[doc = "Interrupt Enable Register/Divisor Latch High"]
pub mod uart_ier_dlh_reg;
#[doc = "UART_IIR_FCR_REG register accessor: an alias for `Reg<UART_IIR_FCR_REG_SPEC>`"]
pub type UART_IIR_FCR_REG = crate::Reg<uart_iir_fcr_reg::UART_IIR_FCR_REG_SPEC>;
#[doc = "Interrupt Identification Register/FIFO Control Register"]
pub mod uart_iir_fcr_reg;
#[doc = "UART_LCR_REG register accessor: an alias for `Reg<UART_LCR_REG_SPEC>`"]
pub type UART_LCR_REG = crate::Reg<uart_lcr_reg::UART_LCR_REG_SPEC>;
#[doc = "Line Control Register"]
pub mod uart_lcr_reg;
#[doc = "UART_LSR_REG register accessor: an alias for `Reg<UART_LSR_REG_SPEC>`"]
pub type UART_LSR_REG = crate::Reg<uart_lsr_reg::UART_LSR_REG_SPEC>;
#[doc = "Line Status Register"]
pub mod uart_lsr_reg;
#[doc = "UART_MCR_REG register accessor: an alias for `Reg<UART_MCR_REG_SPEC>`"]
pub type UART_MCR_REG = crate::Reg<uart_mcr_reg::UART_MCR_REG_SPEC>;
#[doc = "Modem Control Register"]
pub mod uart_mcr_reg;
#[doc = "UART_MSR_REG register accessor: an alias for `Reg<UART_MSR_REG_SPEC>`"]
pub type UART_MSR_REG = crate::Reg<uart_msr_reg::UART_MSR_REG_SPEC>;
#[doc = "Modem Status Register"]
pub mod uart_msr_reg;
#[doc = "UART_RBR_THR_DLL_REG register accessor: an alias for `Reg<UART_RBR_THR_DLL_REG_SPEC>`"]
pub type UART_RBR_THR_DLL_REG = crate::Reg<uart_rbr_thr_dll_reg::UART_RBR_THR_DLL_REG_SPEC>;
#[doc = "Receive Buffer Register/Transmit Holding Register/Divisor Latch Low"]
pub mod uart_rbr_thr_dll_reg;
#[doc = "UART_RFL_REG register accessor: an alias for `Reg<UART_RFL_REG_SPEC>`"]
pub type UART_RFL_REG = crate::Reg<uart_rfl_reg::UART_RFL_REG_SPEC>;
#[doc = "Receive FIFO Level"]
pub mod uart_rfl_reg;
#[doc = "UART_SBCR_REG register accessor: an alias for `Reg<UART_SBCR_REG_SPEC>`"]
pub type UART_SBCR_REG = crate::Reg<uart_sbcr_reg::UART_SBCR_REG_SPEC>;
#[doc = "Shadow Break Control Register"]
pub mod uart_sbcr_reg;
#[doc = "UART_SCR_REG register accessor: an alias for `Reg<UART_SCR_REG_SPEC>`"]
pub type UART_SCR_REG = crate::Reg<uart_scr_reg::UART_SCR_REG_SPEC>;
#[doc = "Scratchpad Register"]
pub mod uart_scr_reg;
#[doc = "UART_SDMAM_REG register accessor: an alias for `Reg<UART_SDMAM_REG_SPEC>`"]
pub type UART_SDMAM_REG = crate::Reg<uart_sdmam_reg::UART_SDMAM_REG_SPEC>;
#[doc = "Shadow DMA Mode"]
pub mod uart_sdmam_reg;
#[doc = "UART_SFE_REG register accessor: an alias for `Reg<UART_SFE_REG_SPEC>`"]
pub type UART_SFE_REG = crate::Reg<uart_sfe_reg::UART_SFE_REG_SPEC>;
#[doc = "Shadow FIFO Enable"]
pub mod uart_sfe_reg;
#[doc = "UART_SRBR_STHR0_REG register accessor: an alias for `Reg<UART_SRBR_STHR0_REG_SPEC>`"]
pub type UART_SRBR_STHR0_REG = crate::Reg<uart_srbr_sthr0_reg::UART_SRBR_STHR0_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr0_reg;
#[doc = "UART_SRBR_STHR10_REG register accessor: an alias for `Reg<UART_SRBR_STHR10_REG_SPEC>`"]
pub type UART_SRBR_STHR10_REG = crate::Reg<uart_srbr_sthr10_reg::UART_SRBR_STHR10_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr10_reg;
#[doc = "UART_SRBR_STHR11_REG register accessor: an alias for `Reg<UART_SRBR_STHR11_REG_SPEC>`"]
pub type UART_SRBR_STHR11_REG = crate::Reg<uart_srbr_sthr11_reg::UART_SRBR_STHR11_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr11_reg;
#[doc = "UART_SRBR_STHR12_REG register accessor: an alias for `Reg<UART_SRBR_STHR12_REG_SPEC>`"]
pub type UART_SRBR_STHR12_REG = crate::Reg<uart_srbr_sthr12_reg::UART_SRBR_STHR12_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr12_reg;
#[doc = "UART_SRBR_STHR13_REG register accessor: an alias for `Reg<UART_SRBR_STHR13_REG_SPEC>`"]
pub type UART_SRBR_STHR13_REG = crate::Reg<uart_srbr_sthr13_reg::UART_SRBR_STHR13_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr13_reg;
#[doc = "UART_SRBR_STHR14_REG register accessor: an alias for `Reg<UART_SRBR_STHR14_REG_SPEC>`"]
pub type UART_SRBR_STHR14_REG = crate::Reg<uart_srbr_sthr14_reg::UART_SRBR_STHR14_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr14_reg;
#[doc = "UART_SRBR_STHR15_REG register accessor: an alias for `Reg<UART_SRBR_STHR15_REG_SPEC>`"]
pub type UART_SRBR_STHR15_REG = crate::Reg<uart_srbr_sthr15_reg::UART_SRBR_STHR15_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr15_reg;
#[doc = "UART_SRBR_STHR1_REG register accessor: an alias for `Reg<UART_SRBR_STHR1_REG_SPEC>`"]
pub type UART_SRBR_STHR1_REG = crate::Reg<uart_srbr_sthr1_reg::UART_SRBR_STHR1_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr1_reg;
#[doc = "UART_SRBR_STHR2_REG register accessor: an alias for `Reg<UART_SRBR_STHR2_REG_SPEC>`"]
pub type UART_SRBR_STHR2_REG = crate::Reg<uart_srbr_sthr2_reg::UART_SRBR_STHR2_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr2_reg;
#[doc = "UART_SRBR_STHR3_REG register accessor: an alias for `Reg<UART_SRBR_STHR3_REG_SPEC>`"]
pub type UART_SRBR_STHR3_REG = crate::Reg<uart_srbr_sthr3_reg::UART_SRBR_STHR3_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr3_reg;
#[doc = "UART_SRBR_STHR4_REG register accessor: an alias for `Reg<UART_SRBR_STHR4_REG_SPEC>`"]
pub type UART_SRBR_STHR4_REG = crate::Reg<uart_srbr_sthr4_reg::UART_SRBR_STHR4_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr4_reg;
#[doc = "UART_SRBR_STHR5_REG register accessor: an alias for `Reg<UART_SRBR_STHR5_REG_SPEC>`"]
pub type UART_SRBR_STHR5_REG = crate::Reg<uart_srbr_sthr5_reg::UART_SRBR_STHR5_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr5_reg;
#[doc = "UART_SRBR_STHR6_REG register accessor: an alias for `Reg<UART_SRBR_STHR6_REG_SPEC>`"]
pub type UART_SRBR_STHR6_REG = crate::Reg<uart_srbr_sthr6_reg::UART_SRBR_STHR6_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr6_reg;
#[doc = "UART_SRBR_STHR7_REG register accessor: an alias for `Reg<UART_SRBR_STHR7_REG_SPEC>`"]
pub type UART_SRBR_STHR7_REG = crate::Reg<uart_srbr_sthr7_reg::UART_SRBR_STHR7_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr7_reg;
#[doc = "UART_SRBR_STHR8_REG register accessor: an alias for `Reg<UART_SRBR_STHR8_REG_SPEC>`"]
pub type UART_SRBR_STHR8_REG = crate::Reg<uart_srbr_sthr8_reg::UART_SRBR_STHR8_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr8_reg;
#[doc = "UART_SRBR_STHR9_REG register accessor: an alias for `Reg<UART_SRBR_STHR9_REG_SPEC>`"]
pub type UART_SRBR_STHR9_REG = crate::Reg<uart_srbr_sthr9_reg::UART_SRBR_STHR9_REG_SPEC>;
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub mod uart_srbr_sthr9_reg;
#[doc = "UART_SRR_REG register accessor: an alias for `Reg<UART_SRR_REG_SPEC>`"]
pub type UART_SRR_REG = crate::Reg<uart_srr_reg::UART_SRR_REG_SPEC>;
#[doc = "Software Reset Register."]
pub mod uart_srr_reg;
#[doc = "UART_SRTS_REG register accessor: an alias for `Reg<UART_SRTS_REG_SPEC>`"]
pub type UART_SRTS_REG = crate::Reg<uart_srts_reg::UART_SRTS_REG_SPEC>;
#[doc = "Shadow Request to Send"]
pub mod uart_srts_reg;
#[doc = "UART_SRT_REG register accessor: an alias for `Reg<UART_SRT_REG_SPEC>`"]
pub type UART_SRT_REG = crate::Reg<uart_srt_reg::UART_SRT_REG_SPEC>;
#[doc = "Shadow RCVR Trigger"]
pub mod uart_srt_reg;
#[doc = "UART_STET_REG register accessor: an alias for `Reg<UART_STET_REG_SPEC>`"]
pub type UART_STET_REG = crate::Reg<uart_stet_reg::UART_STET_REG_SPEC>;
#[doc = "Shadow TX Empty Trigger"]
pub mod uart_stet_reg;
#[doc = "UART_TFL_REG register accessor: an alias for `Reg<UART_TFL_REG_SPEC>`"]
pub type UART_TFL_REG = crate::Reg<uart_tfl_reg::UART_TFL_REG_SPEC>;
#[doc = "Transmit FIFO Level"]
pub mod uart_tfl_reg;
#[doc = "UART_UCV_HIGH_REG register accessor: an alias for `Reg<UART_UCV_HIGH_REG_SPEC>`"]
pub type UART_UCV_HIGH_REG = crate::Reg<uart_ucv_high_reg::UART_UCV_HIGH_REG_SPEC>;
#[doc = "Component Version"]
pub mod uart_ucv_high_reg;
#[doc = "UART_UCV_REG register accessor: an alias for `Reg<UART_UCV_REG_SPEC>`"]
pub type UART_UCV_REG = crate::Reg<uart_ucv_reg::UART_UCV_REG_SPEC>;
#[doc = "Component Version"]
pub mod uart_ucv_reg;
#[doc = "UART_USR_REG register accessor: an alias for `Reg<UART_USR_REG_SPEC>`"]
pub type UART_USR_REG = crate::Reg<uart_usr_reg::UART_USR_REG_SPEC>;
#[doc = "UART Status Register"]
pub mod uart_usr_reg;
