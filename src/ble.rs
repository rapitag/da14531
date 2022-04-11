#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BLE Control register"]
    pub ble_rwblecntl_reg: crate::Reg<ble_rwblecntl_reg::BLE_RWBLECNTL_REG_SPEC>,
    #[doc = "0x04 - Version register"]
    pub ble_version_reg: crate::Reg<ble_version_reg::BLE_VERSION_REG_SPEC>,
    #[doc = "0x08 - Configuration register"]
    pub ble_rwbleconf_reg: crate::Reg<ble_rwbleconf_reg::BLE_RWBLECONF_REG_SPEC>,
    #[doc = "0x0c - Interrupt controller register"]
    pub ble_intcntl_reg: crate::Reg<ble_intcntl_reg::BLE_INTCNTL_REG_SPEC>,
    #[doc = "0x10 - Interrupt status register"]
    pub ble_intstat_reg: crate::Reg<ble_intstat_reg::BLE_INTSTAT_REG_SPEC>,
    #[doc = "0x14 - Interrupt raw status register"]
    pub ble_intrawstat_reg: crate::Reg<ble_intrawstat_reg::BLE_INTRAWSTAT_REG_SPEC>,
    #[doc = "0x18 - Interrupt acknowledge register"]
    pub ble_intack_reg: crate::Reg<ble_intack_reg::BLE_INTACK_REG_SPEC>,
    #[doc = "0x1c - Base time reference counter"]
    pub ble_basetimecnt_reg: crate::Reg<ble_basetimecnt_reg::BLE_BASETIMECNT_REG_SPEC>,
    #[doc = "0x20 - Fine time reference counter"]
    pub ble_finetimecnt_reg: crate::Reg<ble_finetimecnt_reg::BLE_FINETIMECNT_REG_SPEC>,
    #[doc = "0x24 - BLE device address LSB register"]
    pub ble_bdaddrl_reg: crate::Reg<ble_bdaddrl_reg::BLE_BDADDRL_REG_SPEC>,
    #[doc = "0x28 - BLE device address MSB register"]
    pub ble_bdaddru_reg: crate::Reg<ble_bdaddru_reg::BLE_BDADDRU_REG_SPEC>,
    #[doc = "0x2c - Rx Descriptor Pointer for the Receive Buffer Chained List"]
    pub ble_currentrxdescptr_reg:
        crate::Reg<ble_currentrxdescptr_reg::BLE_CURRENTRXDESCPTR_REG_SPEC>,
    #[doc = "0x30 - Deep-Sleep control register"]
    pub ble_deepslcntl_reg: crate::Reg<ble_deepslcntl_reg::BLE_DEEPSLCNTL_REG_SPEC>,
    #[doc = "0x34 - Time (measured in Low Power clock cycles) in Deep Sleep Mode before waking-up the device"]
    pub ble_deepslwkup_reg: crate::Reg<ble_deepslwkup_reg::BLE_DEEPSLWKUP_REG_SPEC>,
    #[doc = "0x38 - Duration of the last deep sleep phase register"]
    pub ble_deepslstat_reg: crate::Reg<ble_deepslstat_reg::BLE_DEEPSLSTAT_REG_SPEC>,
    #[doc = "0x3c - Time in low power oscillator cycles register"]
    pub ble_enbpreset_reg: crate::Reg<ble_enbpreset_reg::BLE_ENBPRESET_REG_SPEC>,
    #[doc = "0x40 - Phase correction value register"]
    pub ble_finecntcorr_reg: crate::Reg<ble_finecntcorr_reg::BLE_FINECNTCORR_REG_SPEC>,
    #[doc = "0x44 - Base Time Counter"]
    pub ble_basetimecntcorr_reg: crate::Reg<ble_basetimecntcorr_reg::BLE_BASETIMECNTCORR_REG_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x50 - Diagnostics Register"]
    pub ble_diagcntl_reg: crate::Reg<ble_diagcntl_reg::BLE_DIAGCNTL_REG_SPEC>,
    #[doc = "0x54 - Debug use only"]
    pub ble_diagstat_reg: crate::Reg<ble_diagstat_reg::BLE_DIAGSTAT_REG_SPEC>,
    #[doc = "0x58 - Upper limit for the memory zone"]
    pub ble_debugaddmax_reg: crate::Reg<ble_debugaddmax_reg::BLE_DEBUGADDMAX_REG_SPEC>,
    #[doc = "0x5c - Lower limit for the memory zone"]
    pub ble_debugaddmin_reg: crate::Reg<ble_debugaddmin_reg::BLE_DEBUGADDMIN_REG_SPEC>,
    #[doc = "0x60 - Error Type Status registers"]
    pub ble_errortypestat_reg: crate::Reg<ble_errortypestat_reg::BLE_ERRORTYPESTAT_REG_SPEC>,
    #[doc = "0x64 - Software Profiling register"]
    pub ble_swprofiling_reg: crate::Reg<ble_swprofiling_reg::BLE_SWPROFILING_REG_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0x70 - Radio interface control register"]
    pub ble_radiocntl0_reg: crate::Reg<ble_radiocntl0_reg::BLE_RADIOCNTL0_REG_SPEC>,
    #[doc = "0x74 - Radio interface control register"]
    pub ble_radiocntl1_reg: crate::Reg<ble_radiocntl1_reg::BLE_RADIOCNTL1_REG_SPEC>,
    #[doc = "0x78 - Radio interface control register"]
    pub ble_radiocntl2_reg: crate::Reg<ble_radiocntl2_reg::BLE_RADIOCNTL2_REG_SPEC>,
    #[doc = "0x7c - Radio interface control register"]
    pub ble_radiocntl3_reg: crate::Reg<ble_radiocntl3_reg::BLE_RADIOCNTL3_REG_SPEC>,
    #[doc = "0x80 - RX/TX power up/down phase register"]
    pub ble_radiopwrupdn_reg: crate::Reg<ble_radiopwrupdn_reg::BLE_RADIOPWRUPDN_REG_SPEC>,
    _reserved29: [u8; 0x0c],
    #[doc = "0x90 - Advertising Channel Map"]
    pub ble_advchmap_reg: crate::Reg<ble_advchmap_reg::BLE_ADVCHMAP_REG_SPEC>,
    _reserved30: [u8; 0x0c],
    #[doc = "0xa0 - Advertising Packet Interval"]
    pub ble_advtim_reg: crate::Reg<ble_advtim_reg::BLE_ADVTIM_REG_SPEC>,
    #[doc = "0xa4 - Active scan register"]
    pub ble_actscanstat_reg: crate::Reg<ble_actscanstat_reg::BLE_ACTSCANSTAT_REG_SPEC>,
    _reserved32: [u8; 0x08],
    #[doc = "0xb0 - Start address of public devices list"]
    pub ble_wlpubaddptr_reg: crate::Reg<ble_wlpubaddptr_reg::BLE_WLPUBADDPTR_REG_SPEC>,
    #[doc = "0xb4 - Start address of private devices list"]
    pub ble_wlprivaddptr_reg: crate::Reg<ble_wlprivaddptr_reg::BLE_WLPRIVADDPTR_REG_SPEC>,
    #[doc = "0xb8 - Devices in white list"]
    pub ble_wlnbdev_reg: crate::Reg<ble_wlnbdev_reg::BLE_WLNBDEV_REG_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0xc0 - Start AES register"]
    pub ble_aescntl_reg: crate::Reg<ble_aescntl_reg::BLE_AESCNTL_REG_SPEC>,
    #[doc = "0xc4 - AES encryption key"]
    pub ble_aeskey31_0_reg: crate::Reg<ble_aeskey31_0_reg::BLE_AESKEY31_0_REG_SPEC>,
    #[doc = "0xc8 - AES encryption key"]
    pub ble_aeskey63_32_reg: crate::Reg<ble_aeskey63_32_reg::BLE_AESKEY63_32_REG_SPEC>,
    #[doc = "0xcc - AES encryption key"]
    pub ble_aeskey95_64_reg: crate::Reg<ble_aeskey95_64_reg::BLE_AESKEY95_64_REG_SPEC>,
    #[doc = "0xd0 - AES encryption key"]
    pub ble_aeskey127_96_reg: crate::Reg<ble_aeskey127_96_reg::BLE_AESKEY127_96_REG_SPEC>,
    #[doc = "0xd4 - Pointer to the block to encrypt/decrypt"]
    pub ble_aesptr_reg: crate::Reg<ble_aesptr_reg::BLE_AESPTR_REG_SPEC>,
    #[doc = "0xd8 - AES / CCM plain MIC value"]
    pub ble_txmicval_reg: crate::Reg<ble_txmicval_reg::BLE_TXMICVAL_REG_SPEC>,
    #[doc = "0xdc - AES / CCM plain MIC value"]
    pub ble_rxmicval_reg: crate::Reg<ble_rxmicval_reg::BLE_RXMICVAL_REG_SPEC>,
    #[doc = "0xe0 - RF Testing Register"]
    pub ble_rftestcntl_reg: crate::Reg<ble_rftestcntl_reg::BLE_RFTESTCNTL_REG_SPEC>,
    #[doc = "0xe4 - RF Testing Register"]
    pub ble_rftesttxstat_reg: crate::Reg<ble_rftesttxstat_reg::BLE_RFTESTTXSTAT_REG_SPEC>,
    #[doc = "0xe8 - RF Testing Register"]
    pub ble_rftestrxstat_reg: crate::Reg<ble_rftestrxstat_reg::BLE_RFTESTRXSTAT_REG_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0xf0 - Timing Generator Register"]
    pub ble_timgencntl_reg: crate::Reg<ble_timgencntl_reg::BLE_TIMGENCNTL_REG_SPEC>,
    #[doc = "0xf4 - Gross Timer Target value"]
    pub ble_grosstimtgt_reg: crate::Reg<ble_grosstimtgt_reg::BLE_GROSSTIMTGT_REG_SPEC>,
    #[doc = "0xf8 - Fine Timer Target value"]
    pub ble_finetimtgt_reg: crate::Reg<ble_finetimtgt_reg::BLE_FINETIMTGT_REG_SPEC>,
    #[doc = "0xfc - Samples the Base Time Counter"]
    pub ble_sampleclk_reg: crate::Reg<ble_sampleclk_reg::BLE_SAMPLECLK_REG_SPEC>,
    #[doc = "0x100 - Coexistence interface Control 0 Register"]
    pub ble_coexifcntl0_reg: crate::Reg<ble_coexifcntl0_reg::BLE_COEXIFCNTL0_REG_SPEC>,
    #[doc = "0x104 - Coexistence interface Control 1 Register"]
    pub ble_coexifcntl1_reg: crate::Reg<ble_coexifcntl1_reg::BLE_COEXIFCNTL1_REG_SPEC>,
    #[doc = "0x108 - Coexistence interface Priority 0 Register"]
    pub ble_blemprio0_reg: crate::Reg<ble_blemprio0_reg::BLE_BLEMPRIO0_REG_SPEC>,
    #[doc = "0x10c - Coexistence interface Priority 1 Register"]
    pub ble_blemprio1_reg: crate::Reg<ble_blemprio1_reg::BLE_BLEMPRIO1_REG_SPEC>,
    _reserved54: [u8; 0xf0],
    #[doc = "0x200 - BLE Control Register 2"]
    pub ble_cntl2_reg: crate::Reg<ble_cntl2_reg::BLE_CNTL2_REG_SPEC>,
    _reserved55: [u8; 0x04],
    #[doc = "0x208 - Exchange Memory Base Register"]
    pub ble_em_base_reg: crate::Reg<ble_em_base_reg::BLE_EM_BASE_REG_SPEC>,
    #[doc = "0x20c - Debug use only"]
    pub ble_diagcntl2_reg: crate::Reg<ble_diagcntl2_reg::BLE_DIAGCNTL2_REG_SPEC>,
    #[doc = "0x210 - Debug use only"]
    pub ble_diagcntl3_reg: crate::Reg<ble_diagcntl3_reg::BLE_DIAGCNTL3_REG_SPEC>,
}
#[doc = "BLE_ACTSCANSTAT_REG register accessor: an alias for `Reg<BLE_ACTSCANSTAT_REG_SPEC>`"]
pub type BLE_ACTSCANSTAT_REG = crate::Reg<ble_actscanstat_reg::BLE_ACTSCANSTAT_REG_SPEC>;
#[doc = "Active scan register"]
pub mod ble_actscanstat_reg;
#[doc = "BLE_ADVCHMAP_REG register accessor: an alias for `Reg<BLE_ADVCHMAP_REG_SPEC>`"]
pub type BLE_ADVCHMAP_REG = crate::Reg<ble_advchmap_reg::BLE_ADVCHMAP_REG_SPEC>;
#[doc = "Advertising Channel Map"]
pub mod ble_advchmap_reg;
#[doc = "BLE_ADVTIM_REG register accessor: an alias for `Reg<BLE_ADVTIM_REG_SPEC>`"]
pub type BLE_ADVTIM_REG = crate::Reg<ble_advtim_reg::BLE_ADVTIM_REG_SPEC>;
#[doc = "Advertising Packet Interval"]
pub mod ble_advtim_reg;
#[doc = "BLE_AESCNTL_REG register accessor: an alias for `Reg<BLE_AESCNTL_REG_SPEC>`"]
pub type BLE_AESCNTL_REG = crate::Reg<ble_aescntl_reg::BLE_AESCNTL_REG_SPEC>;
#[doc = "Start AES register"]
pub mod ble_aescntl_reg;
#[doc = "BLE_AESKEY127_96_REG register accessor: an alias for `Reg<BLE_AESKEY127_96_REG_SPEC>`"]
pub type BLE_AESKEY127_96_REG = crate::Reg<ble_aeskey127_96_reg::BLE_AESKEY127_96_REG_SPEC>;
#[doc = "AES encryption key"]
pub mod ble_aeskey127_96_reg;
#[doc = "BLE_AESKEY31_0_REG register accessor: an alias for `Reg<BLE_AESKEY31_0_REG_SPEC>`"]
pub type BLE_AESKEY31_0_REG = crate::Reg<ble_aeskey31_0_reg::BLE_AESKEY31_0_REG_SPEC>;
#[doc = "AES encryption key"]
pub mod ble_aeskey31_0_reg;
#[doc = "BLE_AESKEY63_32_REG register accessor: an alias for `Reg<BLE_AESKEY63_32_REG_SPEC>`"]
pub type BLE_AESKEY63_32_REG = crate::Reg<ble_aeskey63_32_reg::BLE_AESKEY63_32_REG_SPEC>;
#[doc = "AES encryption key"]
pub mod ble_aeskey63_32_reg;
#[doc = "BLE_AESKEY95_64_REG register accessor: an alias for `Reg<BLE_AESKEY95_64_REG_SPEC>`"]
pub type BLE_AESKEY95_64_REG = crate::Reg<ble_aeskey95_64_reg::BLE_AESKEY95_64_REG_SPEC>;
#[doc = "AES encryption key"]
pub mod ble_aeskey95_64_reg;
#[doc = "BLE_AESPTR_REG register accessor: an alias for `Reg<BLE_AESPTR_REG_SPEC>`"]
pub type BLE_AESPTR_REG = crate::Reg<ble_aesptr_reg::BLE_AESPTR_REG_SPEC>;
#[doc = "Pointer to the block to encrypt/decrypt"]
pub mod ble_aesptr_reg;
#[doc = "BLE_BASETIMECNTCORR_REG register accessor: an alias for `Reg<BLE_BASETIMECNTCORR_REG_SPEC>`"]
pub type BLE_BASETIMECNTCORR_REG =
    crate::Reg<ble_basetimecntcorr_reg::BLE_BASETIMECNTCORR_REG_SPEC>;
#[doc = "Base Time Counter"]
pub mod ble_basetimecntcorr_reg;
#[doc = "BLE_BASETIMECNT_REG register accessor: an alias for `Reg<BLE_BASETIMECNT_REG_SPEC>`"]
pub type BLE_BASETIMECNT_REG = crate::Reg<ble_basetimecnt_reg::BLE_BASETIMECNT_REG_SPEC>;
#[doc = "Base time reference counter"]
pub mod ble_basetimecnt_reg;
#[doc = "BLE_BDADDRL_REG register accessor: an alias for `Reg<BLE_BDADDRL_REG_SPEC>`"]
pub type BLE_BDADDRL_REG = crate::Reg<ble_bdaddrl_reg::BLE_BDADDRL_REG_SPEC>;
#[doc = "BLE device address LSB register"]
pub mod ble_bdaddrl_reg;
#[doc = "BLE_BDADDRU_REG register accessor: an alias for `Reg<BLE_BDADDRU_REG_SPEC>`"]
pub type BLE_BDADDRU_REG = crate::Reg<ble_bdaddru_reg::BLE_BDADDRU_REG_SPEC>;
#[doc = "BLE device address MSB register"]
pub mod ble_bdaddru_reg;
#[doc = "BLE_BLEMPRIO0_REG register accessor: an alias for `Reg<BLE_BLEMPRIO0_REG_SPEC>`"]
pub type BLE_BLEMPRIO0_REG = crate::Reg<ble_blemprio0_reg::BLE_BLEMPRIO0_REG_SPEC>;
#[doc = "Coexistence interface Priority 0 Register"]
pub mod ble_blemprio0_reg;
#[doc = "BLE_BLEMPRIO1_REG register accessor: an alias for `Reg<BLE_BLEMPRIO1_REG_SPEC>`"]
pub type BLE_BLEMPRIO1_REG = crate::Reg<ble_blemprio1_reg::BLE_BLEMPRIO1_REG_SPEC>;
#[doc = "Coexistence interface Priority 1 Register"]
pub mod ble_blemprio1_reg;
#[doc = "BLE_CNTL2_REG register accessor: an alias for `Reg<BLE_CNTL2_REG_SPEC>`"]
pub type BLE_CNTL2_REG = crate::Reg<ble_cntl2_reg::BLE_CNTL2_REG_SPEC>;
#[doc = "BLE Control Register 2"]
pub mod ble_cntl2_reg;
#[doc = "BLE_COEXIFCNTL0_REG register accessor: an alias for `Reg<BLE_COEXIFCNTL0_REG_SPEC>`"]
pub type BLE_COEXIFCNTL0_REG = crate::Reg<ble_coexifcntl0_reg::BLE_COEXIFCNTL0_REG_SPEC>;
#[doc = "Coexistence interface Control 0 Register"]
pub mod ble_coexifcntl0_reg;
#[doc = "BLE_COEXIFCNTL1_REG register accessor: an alias for `Reg<BLE_COEXIFCNTL1_REG_SPEC>`"]
pub type BLE_COEXIFCNTL1_REG = crate::Reg<ble_coexifcntl1_reg::BLE_COEXIFCNTL1_REG_SPEC>;
#[doc = "Coexistence interface Control 1 Register"]
pub mod ble_coexifcntl1_reg;
#[doc = "BLE_CURRENTRXDESCPTR_REG register accessor: an alias for `Reg<BLE_CURRENTRXDESCPTR_REG_SPEC>`"]
pub type BLE_CURRENTRXDESCPTR_REG =
    crate::Reg<ble_currentrxdescptr_reg::BLE_CURRENTRXDESCPTR_REG_SPEC>;
#[doc = "Rx Descriptor Pointer for the Receive Buffer Chained List"]
pub mod ble_currentrxdescptr_reg;
#[doc = "BLE_DEBUGADDMAX_REG register accessor: an alias for `Reg<BLE_DEBUGADDMAX_REG_SPEC>`"]
pub type BLE_DEBUGADDMAX_REG = crate::Reg<ble_debugaddmax_reg::BLE_DEBUGADDMAX_REG_SPEC>;
#[doc = "Upper limit for the memory zone"]
pub mod ble_debugaddmax_reg;
#[doc = "BLE_DEBUGADDMIN_REG register accessor: an alias for `Reg<BLE_DEBUGADDMIN_REG_SPEC>`"]
pub type BLE_DEBUGADDMIN_REG = crate::Reg<ble_debugaddmin_reg::BLE_DEBUGADDMIN_REG_SPEC>;
#[doc = "Lower limit for the memory zone"]
pub mod ble_debugaddmin_reg;
#[doc = "BLE_DEEPSLCNTL_REG register accessor: an alias for `Reg<BLE_DEEPSLCNTL_REG_SPEC>`"]
pub type BLE_DEEPSLCNTL_REG = crate::Reg<ble_deepslcntl_reg::BLE_DEEPSLCNTL_REG_SPEC>;
#[doc = "Deep-Sleep control register"]
pub mod ble_deepslcntl_reg;
#[doc = "BLE_DEEPSLSTAT_REG register accessor: an alias for `Reg<BLE_DEEPSLSTAT_REG_SPEC>`"]
pub type BLE_DEEPSLSTAT_REG = crate::Reg<ble_deepslstat_reg::BLE_DEEPSLSTAT_REG_SPEC>;
#[doc = "Duration of the last deep sleep phase register"]
pub mod ble_deepslstat_reg;
#[doc = "BLE_DEEPSLWKUP_REG register accessor: an alias for `Reg<BLE_DEEPSLWKUP_REG_SPEC>`"]
pub type BLE_DEEPSLWKUP_REG = crate::Reg<ble_deepslwkup_reg::BLE_DEEPSLWKUP_REG_SPEC>;
#[doc = "Time (measured in Low Power clock cycles) in Deep Sleep Mode before waking-up the device"]
pub mod ble_deepslwkup_reg;
#[doc = "BLE_DIAGCNTL2_REG register accessor: an alias for `Reg<BLE_DIAGCNTL2_REG_SPEC>`"]
pub type BLE_DIAGCNTL2_REG = crate::Reg<ble_diagcntl2_reg::BLE_DIAGCNTL2_REG_SPEC>;
#[doc = "Debug use only"]
pub mod ble_diagcntl2_reg;
#[doc = "BLE_DIAGCNTL3_REG register accessor: an alias for `Reg<BLE_DIAGCNTL3_REG_SPEC>`"]
pub type BLE_DIAGCNTL3_REG = crate::Reg<ble_diagcntl3_reg::BLE_DIAGCNTL3_REG_SPEC>;
#[doc = "Debug use only"]
pub mod ble_diagcntl3_reg;
#[doc = "BLE_DIAGCNTL_REG register accessor: an alias for `Reg<BLE_DIAGCNTL_REG_SPEC>`"]
pub type BLE_DIAGCNTL_REG = crate::Reg<ble_diagcntl_reg::BLE_DIAGCNTL_REG_SPEC>;
#[doc = "Diagnostics Register"]
pub mod ble_diagcntl_reg;
#[doc = "BLE_DIAGSTAT_REG register accessor: an alias for `Reg<BLE_DIAGSTAT_REG_SPEC>`"]
pub type BLE_DIAGSTAT_REG = crate::Reg<ble_diagstat_reg::BLE_DIAGSTAT_REG_SPEC>;
#[doc = "Debug use only"]
pub mod ble_diagstat_reg;
#[doc = "BLE_EM_BASE_REG register accessor: an alias for `Reg<BLE_EM_BASE_REG_SPEC>`"]
pub type BLE_EM_BASE_REG = crate::Reg<ble_em_base_reg::BLE_EM_BASE_REG_SPEC>;
#[doc = "Exchange Memory Base Register"]
pub mod ble_em_base_reg;
#[doc = "BLE_ENBPRESET_REG register accessor: an alias for `Reg<BLE_ENBPRESET_REG_SPEC>`"]
pub type BLE_ENBPRESET_REG = crate::Reg<ble_enbpreset_reg::BLE_ENBPRESET_REG_SPEC>;
#[doc = "Time in low power oscillator cycles register"]
pub mod ble_enbpreset_reg;
#[doc = "BLE_ERRORTYPESTAT_REG register accessor: an alias for `Reg<BLE_ERRORTYPESTAT_REG_SPEC>`"]
pub type BLE_ERRORTYPESTAT_REG = crate::Reg<ble_errortypestat_reg::BLE_ERRORTYPESTAT_REG_SPEC>;
#[doc = "Error Type Status registers"]
pub mod ble_errortypestat_reg;
#[doc = "BLE_FINECNTCORR_REG register accessor: an alias for `Reg<BLE_FINECNTCORR_REG_SPEC>`"]
pub type BLE_FINECNTCORR_REG = crate::Reg<ble_finecntcorr_reg::BLE_FINECNTCORR_REG_SPEC>;
#[doc = "Phase correction value register"]
pub mod ble_finecntcorr_reg;
#[doc = "BLE_FINETIMECNT_REG register accessor: an alias for `Reg<BLE_FINETIMECNT_REG_SPEC>`"]
pub type BLE_FINETIMECNT_REG = crate::Reg<ble_finetimecnt_reg::BLE_FINETIMECNT_REG_SPEC>;
#[doc = "Fine time reference counter"]
pub mod ble_finetimecnt_reg;
#[doc = "BLE_FINETIMTGT_REG register accessor: an alias for `Reg<BLE_FINETIMTGT_REG_SPEC>`"]
pub type BLE_FINETIMTGT_REG = crate::Reg<ble_finetimtgt_reg::BLE_FINETIMTGT_REG_SPEC>;
#[doc = "Fine Timer Target value"]
pub mod ble_finetimtgt_reg;
#[doc = "BLE_GROSSTIMTGT_REG register accessor: an alias for `Reg<BLE_GROSSTIMTGT_REG_SPEC>`"]
pub type BLE_GROSSTIMTGT_REG = crate::Reg<ble_grosstimtgt_reg::BLE_GROSSTIMTGT_REG_SPEC>;
#[doc = "Gross Timer Target value"]
pub mod ble_grosstimtgt_reg;
#[doc = "BLE_INTACK_REG register accessor: an alias for `Reg<BLE_INTACK_REG_SPEC>`"]
pub type BLE_INTACK_REG = crate::Reg<ble_intack_reg::BLE_INTACK_REG_SPEC>;
#[doc = "Interrupt acknowledge register"]
pub mod ble_intack_reg;
#[doc = "BLE_INTCNTL_REG register accessor: an alias for `Reg<BLE_INTCNTL_REG_SPEC>`"]
pub type BLE_INTCNTL_REG = crate::Reg<ble_intcntl_reg::BLE_INTCNTL_REG_SPEC>;
#[doc = "Interrupt controller register"]
pub mod ble_intcntl_reg;
#[doc = "BLE_INTRAWSTAT_REG register accessor: an alias for `Reg<BLE_INTRAWSTAT_REG_SPEC>`"]
pub type BLE_INTRAWSTAT_REG = crate::Reg<ble_intrawstat_reg::BLE_INTRAWSTAT_REG_SPEC>;
#[doc = "Interrupt raw status register"]
pub mod ble_intrawstat_reg;
#[doc = "BLE_INTSTAT_REG register accessor: an alias for `Reg<BLE_INTSTAT_REG_SPEC>`"]
pub type BLE_INTSTAT_REG = crate::Reg<ble_intstat_reg::BLE_INTSTAT_REG_SPEC>;
#[doc = "Interrupt status register"]
pub mod ble_intstat_reg;
#[doc = "BLE_RADIOCNTL0_REG register accessor: an alias for `Reg<BLE_RADIOCNTL0_REG_SPEC>`"]
pub type BLE_RADIOCNTL0_REG = crate::Reg<ble_radiocntl0_reg::BLE_RADIOCNTL0_REG_SPEC>;
#[doc = "Radio interface control register"]
pub mod ble_radiocntl0_reg;
#[doc = "BLE_RADIOCNTL1_REG register accessor: an alias for `Reg<BLE_RADIOCNTL1_REG_SPEC>`"]
pub type BLE_RADIOCNTL1_REG = crate::Reg<ble_radiocntl1_reg::BLE_RADIOCNTL1_REG_SPEC>;
#[doc = "Radio interface control register"]
pub mod ble_radiocntl1_reg;
#[doc = "BLE_RADIOCNTL2_REG register accessor: an alias for `Reg<BLE_RADIOCNTL2_REG_SPEC>`"]
pub type BLE_RADIOCNTL2_REG = crate::Reg<ble_radiocntl2_reg::BLE_RADIOCNTL2_REG_SPEC>;
#[doc = "Radio interface control register"]
pub mod ble_radiocntl2_reg;
#[doc = "BLE_RADIOCNTL3_REG register accessor: an alias for `Reg<BLE_RADIOCNTL3_REG_SPEC>`"]
pub type BLE_RADIOCNTL3_REG = crate::Reg<ble_radiocntl3_reg::BLE_RADIOCNTL3_REG_SPEC>;
#[doc = "Radio interface control register"]
pub mod ble_radiocntl3_reg;
#[doc = "BLE_RADIOPWRUPDN_REG register accessor: an alias for `Reg<BLE_RADIOPWRUPDN_REG_SPEC>`"]
pub type BLE_RADIOPWRUPDN_REG = crate::Reg<ble_radiopwrupdn_reg::BLE_RADIOPWRUPDN_REG_SPEC>;
#[doc = "RX/TX power up/down phase register"]
pub mod ble_radiopwrupdn_reg;
#[doc = "BLE_RFTESTCNTL_REG register accessor: an alias for `Reg<BLE_RFTESTCNTL_REG_SPEC>`"]
pub type BLE_RFTESTCNTL_REG = crate::Reg<ble_rftestcntl_reg::BLE_RFTESTCNTL_REG_SPEC>;
#[doc = "RF Testing Register"]
pub mod ble_rftestcntl_reg;
#[doc = "BLE_RFTESTRXSTAT_REG register accessor: an alias for `Reg<BLE_RFTESTRXSTAT_REG_SPEC>`"]
pub type BLE_RFTESTRXSTAT_REG = crate::Reg<ble_rftestrxstat_reg::BLE_RFTESTRXSTAT_REG_SPEC>;
#[doc = "RF Testing Register"]
pub mod ble_rftestrxstat_reg;
#[doc = "BLE_RFTESTTXSTAT_REG register accessor: an alias for `Reg<BLE_RFTESTTXSTAT_REG_SPEC>`"]
pub type BLE_RFTESTTXSTAT_REG = crate::Reg<ble_rftesttxstat_reg::BLE_RFTESTTXSTAT_REG_SPEC>;
#[doc = "RF Testing Register"]
pub mod ble_rftesttxstat_reg;
#[doc = "BLE_RWBLECNTL_REG register accessor: an alias for `Reg<BLE_RWBLECNTL_REG_SPEC>`"]
pub type BLE_RWBLECNTL_REG = crate::Reg<ble_rwblecntl_reg::BLE_RWBLECNTL_REG_SPEC>;
#[doc = "BLE Control register"]
pub mod ble_rwblecntl_reg;
#[doc = "BLE_RWBLECONF_REG register accessor: an alias for `Reg<BLE_RWBLECONF_REG_SPEC>`"]
pub type BLE_RWBLECONF_REG = crate::Reg<ble_rwbleconf_reg::BLE_RWBLECONF_REG_SPEC>;
#[doc = "Configuration register"]
pub mod ble_rwbleconf_reg;
#[doc = "BLE_RXMICVAL_REG register accessor: an alias for `Reg<BLE_RXMICVAL_REG_SPEC>`"]
pub type BLE_RXMICVAL_REG = crate::Reg<ble_rxmicval_reg::BLE_RXMICVAL_REG_SPEC>;
#[doc = "AES / CCM plain MIC value"]
pub mod ble_rxmicval_reg;
#[doc = "BLE_SAMPLECLK_REG register accessor: an alias for `Reg<BLE_SAMPLECLK_REG_SPEC>`"]
pub type BLE_SAMPLECLK_REG = crate::Reg<ble_sampleclk_reg::BLE_SAMPLECLK_REG_SPEC>;
#[doc = "Samples the Base Time Counter"]
pub mod ble_sampleclk_reg;
#[doc = "BLE_SWPROFILING_REG register accessor: an alias for `Reg<BLE_SWPROFILING_REG_SPEC>`"]
pub type BLE_SWPROFILING_REG = crate::Reg<ble_swprofiling_reg::BLE_SWPROFILING_REG_SPEC>;
#[doc = "Software Profiling register"]
pub mod ble_swprofiling_reg;
#[doc = "BLE_TIMGENCNTL_REG register accessor: an alias for `Reg<BLE_TIMGENCNTL_REG_SPEC>`"]
pub type BLE_TIMGENCNTL_REG = crate::Reg<ble_timgencntl_reg::BLE_TIMGENCNTL_REG_SPEC>;
#[doc = "Timing Generator Register"]
pub mod ble_timgencntl_reg;
#[doc = "BLE_TXMICVAL_REG register accessor: an alias for `Reg<BLE_TXMICVAL_REG_SPEC>`"]
pub type BLE_TXMICVAL_REG = crate::Reg<ble_txmicval_reg::BLE_TXMICVAL_REG_SPEC>;
#[doc = "AES / CCM plain MIC value"]
pub mod ble_txmicval_reg;
#[doc = "BLE_VERSION_REG register accessor: an alias for `Reg<BLE_VERSION_REG_SPEC>`"]
pub type BLE_VERSION_REG = crate::Reg<ble_version_reg::BLE_VERSION_REG_SPEC>;
#[doc = "Version register"]
pub mod ble_version_reg;
#[doc = "BLE_WLNBDEV_REG register accessor: an alias for `Reg<BLE_WLNBDEV_REG_SPEC>`"]
pub type BLE_WLNBDEV_REG = crate::Reg<ble_wlnbdev_reg::BLE_WLNBDEV_REG_SPEC>;
#[doc = "Devices in white list"]
pub mod ble_wlnbdev_reg;
#[doc = "BLE_WLPRIVADDPTR_REG register accessor: an alias for `Reg<BLE_WLPRIVADDPTR_REG_SPEC>`"]
pub type BLE_WLPRIVADDPTR_REG = crate::Reg<ble_wlprivaddptr_reg::BLE_WLPRIVADDPTR_REG_SPEC>;
#[doc = "Start address of private devices list"]
pub mod ble_wlprivaddptr_reg;
#[doc = "BLE_WLPUBADDPTR_REG register accessor: an alias for `Reg<BLE_WLPUBADDPTR_REG_SPEC>`"]
pub type BLE_WLPUBADDPTR_REG = crate::Reg<ble_wlpubaddptr_reg::BLE_WLPUBADDPTR_REG_SPEC>;
#[doc = "Start address of public devices list"]
pub mod ble_wlpubaddptr_reg;
