#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode register"]
    pub otpc_mode_reg: crate::Reg<otpc_mode_reg::OTPC_MODE_REG_SPEC>,
    #[doc = "0x04 - Status register"]
    pub otpc_stat_reg: crate::Reg<otpc_stat_reg::OTPC_STAT_REG_SPEC>,
    #[doc = "0x08 - The address of the word that will be programmed, when the PROG mode is used."]
    pub otpc_paddr_reg: crate::Reg<otpc_paddr_reg::OTPC_PADDR_REG_SPEC>,
    #[doc = "0x0c - The 32-bit word that will be programmed, when the PROG mode is used."]
    pub otpc_pword_reg: crate::Reg<otpc_pword_reg::OTPC_PWORD_REG_SPEC>,
    #[doc = "0x10 - Various timing parameters of the OTP cell."]
    pub otpc_tim1_reg: crate::Reg<otpc_tim1_reg::OTPC_TIM1_REG_SPEC>,
    #[doc = "0x14 - Various timing parameters of the OTP cell."]
    pub otpc_tim2_reg: crate::Reg<otpc_tim2_reg::OTPC_TIM2_REG_SPEC>,
    #[doc = "0x18 - AHB master start address"]
    pub otpc_ahbadr_reg: crate::Reg<otpc_ahbadr_reg::OTPC_AHBADR_REG_SPEC>,
    #[doc = "0x1c - OTP cell start address"]
    pub otpc_celadr_reg: crate::Reg<otpc_celadr_reg::OTPC_CELADR_REG_SPEC>,
    #[doc = "0x20 - Number of words"]
    pub otpc_nwords_reg: crate::Reg<otpc_nwords_reg::OTPC_NWORDS_REG_SPEC>,
}
#[doc = "OTPC_AHBADR_REG register accessor: an alias for `Reg<OTPC_AHBADR_REG_SPEC>`"]
pub type OTPC_AHBADR_REG = crate::Reg<otpc_ahbadr_reg::OTPC_AHBADR_REG_SPEC>;
#[doc = "AHB master start address"]
pub mod otpc_ahbadr_reg;
#[doc = "OTPC_CELADR_REG register accessor: an alias for `Reg<OTPC_CELADR_REG_SPEC>`"]
pub type OTPC_CELADR_REG = crate::Reg<otpc_celadr_reg::OTPC_CELADR_REG_SPEC>;
#[doc = "OTP cell start address"]
pub mod otpc_celadr_reg;
#[doc = "OTPC_MODE_REG register accessor: an alias for `Reg<OTPC_MODE_REG_SPEC>`"]
pub type OTPC_MODE_REG = crate::Reg<otpc_mode_reg::OTPC_MODE_REG_SPEC>;
#[doc = "Mode register"]
pub mod otpc_mode_reg;
#[doc = "OTPC_NWORDS_REG register accessor: an alias for `Reg<OTPC_NWORDS_REG_SPEC>`"]
pub type OTPC_NWORDS_REG = crate::Reg<otpc_nwords_reg::OTPC_NWORDS_REG_SPEC>;
#[doc = "Number of words"]
pub mod otpc_nwords_reg;
#[doc = "OTPC_PADDR_REG register accessor: an alias for `Reg<OTPC_PADDR_REG_SPEC>`"]
pub type OTPC_PADDR_REG = crate::Reg<otpc_paddr_reg::OTPC_PADDR_REG_SPEC>;
#[doc = "The address of the word that will be programmed, when the PROG mode is used."]
pub mod otpc_paddr_reg;
#[doc = "OTPC_PWORD_REG register accessor: an alias for `Reg<OTPC_PWORD_REG_SPEC>`"]
pub type OTPC_PWORD_REG = crate::Reg<otpc_pword_reg::OTPC_PWORD_REG_SPEC>;
#[doc = "The 32-bit word that will be programmed, when the PROG mode is used."]
pub mod otpc_pword_reg;
#[doc = "OTPC_STAT_REG register accessor: an alias for `Reg<OTPC_STAT_REG_SPEC>`"]
pub type OTPC_STAT_REG = crate::Reg<otpc_stat_reg::OTPC_STAT_REG_SPEC>;
#[doc = "Status register"]
pub mod otpc_stat_reg;
#[doc = "OTPC_TIM1_REG register accessor: an alias for `Reg<OTPC_TIM1_REG_SPEC>`"]
pub type OTPC_TIM1_REG = crate::Reg<otpc_tim1_reg::OTPC_TIM1_REG_SPEC>;
#[doc = "Various timing parameters of the OTP cell."]
pub mod otpc_tim1_reg;
#[doc = "OTPC_TIM2_REG register accessor: an alias for `Reg<OTPC_TIM2_REG_SPEC>`"]
pub type OTPC_TIM2_REG = crate::Reg<otpc_tim2_reg::OTPC_TIM2_REG_SPEC>;
#[doc = "Various timing parameters of the OTP cell."]
pub mod otpc_tim2_reg;
