#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HCLK, PCLK, divider and clock gates"]
    pub clk_amba_reg: crate::Reg<clk_amba_reg::CLK_AMBA_REG_SPEC>,
    #[doc = "0x02 - Xtal frequency trimming register"]
    pub clk_freq_trim_reg: crate::Reg<clk_freq_trim_reg::CLK_FREQ_TRIM_REG_SPEC>,
    #[doc = "0x04 - Peripheral divider register"]
    pub clk_per_reg: crate::Reg<clk_per_reg::CLK_PER_REG_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x08 - Radio PLL control register"]
    pub clk_radio_reg: crate::Reg<clk_radio_reg::CLK_RADIO_REG_SPEC>,
    #[doc = "0x0a - Clock control register"]
    pub clk_ctrl_reg: crate::Reg<clk_ctrl_reg::CLK_CTRL_REG_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x10 - Power Management Unit control register"]
    pub pmu_ctrl_reg: crate::Reg<pmu_ctrl_reg::PMU_CTRL_REG_SPEC>,
    #[doc = "0x12 - System Control register"]
    pub sys_ctrl_reg: crate::Reg<sys_ctrl_reg::SYS_CTRL_REG_SPEC>,
    #[doc = "0x14 - System status register"]
    pub sys_stat_reg: crate::Reg<sys_stat_reg::SYS_STAT_REG_SPEC>,
    #[doc = "0x16 - Control trimming of the XTAL32M"]
    pub trim_ctrl_reg: crate::Reg<trim_ctrl_reg::TRIM_CTRL_REG_SPEC>,
    #[doc = "0x18 - Control power state of System RAMS"]
    pub ram_pwr_ctrl_reg: crate::Reg<ram_pwr_ctrl_reg::RAM_PWR_CTRL_REG_SPEC>,
    _reserved10: [u8; 0x06],
    #[doc = "0x20 - 32 kHz RC oscillator register"]
    pub clk_rc32k_reg: crate::Reg<clk_rc32k_reg::CLK_RC32K_REG_SPEC>,
    #[doc = "0x22 - 32 kHz XTAL oscillator register"]
    pub clk_xtal32k_reg: crate::Reg<clk_xtal32k_reg::CLK_XTAL32K_REG_SPEC>,
    #[doc = "0x24 - Fast RC control register"]
    pub clk_rc32m_reg: crate::Reg<clk_rc32m_reg::CLK_RC32M_REG_SPEC>,
    #[doc = "0x26 - RCX-oscillator control register"]
    pub clk_rcx_reg: crate::Reg<clk_rcx_reg::CLK_RCX_REG_SPEC>,
    #[doc = "0x28 - Bandgap trimming"]
    pub bandgap_reg: crate::Reg<bandgap_reg::BANDGAP_REG_SPEC>,
    #[doc = "0x2a - Status bit of analog (power management) circuits"]
    pub ana_status_reg: crate::Reg<ana_status_reg::ANA_STATUS_REG_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x30 - Trim values for XTAL32M"]
    pub xtal32m_start_reg: crate::Reg<xtal32m_start_reg::XTAL32M_START_REG_SPEC>,
    #[doc = "0x32 - Read back value of current XTAL trimming"]
    pub xtal32m_trstat_reg: crate::Reg<xtal32m_trstat_reg::XTAL32M_TRSTAT_REG_SPEC>,
    #[doc = "0x34 - Control register for XTALRDY IRQ"]
    pub xtalrdy_ctrl_reg: crate::Reg<xtalrdy_ctrl_reg::XTALRDY_CTRL_REG_SPEC>,
    #[doc = "0x36 - "]
    pub xtalrdy_stat_reg: crate::Reg<xtalrdy_stat_reg::XTALRDY_STAT_REG_SPEC>,
    #[doc = "0x38 - Control bits for XTAL32M"]
    pub xtal32m_ctrl0_reg: crate::Reg<xtal32m_ctrl0_reg::XTAL32M_CTRL0_REG_SPEC>,
    _reserved21: [u8; 0x06],
    #[doc = "0x40 - Selects a GPIO pin for POR generation"]
    pub por_pin_reg: crate::Reg<por_pin_reg::POR_PIN_REG_SPEC>,
    #[doc = "0x42 - Time for POR to happen"]
    pub por_timer_reg: crate::Reg<por_timer_reg::POR_TIMER_REG_SPEC>,
    _reserved23: [u8; 0x0c],
    #[doc = "0x50 - Bandgap refresh interval during sleep"]
    pub pmu_sleep_reg: crate::Reg<pmu_sleep_reg::PMU_SLEEP_REG_SPEC>,
    #[doc = "0x52 - Power management control"]
    pub power_ctrl_reg: crate::Reg<power_ctrl_reg::POWER_CTRL_REG_SPEC>,
    #[doc = "0x54 - Power management level and trim settings"]
    pub power_level_reg: crate::Reg<power_level_reg::POWER_LEVEL_REG_SPEC>,
}
#[doc = "ANA_STATUS_REG register accessor: an alias for `Reg<ANA_STATUS_REG_SPEC>`"]
pub type ANA_STATUS_REG = crate::Reg<ana_status_reg::ANA_STATUS_REG_SPEC>;
#[doc = "Status bit of analog (power management) circuits"]
pub mod ana_status_reg;
#[doc = "BANDGAP_REG register accessor: an alias for `Reg<BANDGAP_REG_SPEC>`"]
pub type BANDGAP_REG = crate::Reg<bandgap_reg::BANDGAP_REG_SPEC>;
#[doc = "Bandgap trimming"]
pub mod bandgap_reg;
#[doc = "CLK_AMBA_REG register accessor: an alias for `Reg<CLK_AMBA_REG_SPEC>`"]
pub type CLK_AMBA_REG = crate::Reg<clk_amba_reg::CLK_AMBA_REG_SPEC>;
#[doc = "HCLK, PCLK, divider and clock gates"]
pub mod clk_amba_reg;
#[doc = "CLK_CTRL_REG register accessor: an alias for `Reg<CLK_CTRL_REG_SPEC>`"]
pub type CLK_CTRL_REG = crate::Reg<clk_ctrl_reg::CLK_CTRL_REG_SPEC>;
#[doc = "Clock control register"]
pub mod clk_ctrl_reg;
#[doc = "CLK_FREQ_TRIM_REG register accessor: an alias for `Reg<CLK_FREQ_TRIM_REG_SPEC>`"]
pub type CLK_FREQ_TRIM_REG = crate::Reg<clk_freq_trim_reg::CLK_FREQ_TRIM_REG_SPEC>;
#[doc = "Xtal frequency trimming register"]
pub mod clk_freq_trim_reg;
#[doc = "CLK_PER_REG register accessor: an alias for `Reg<CLK_PER_REG_SPEC>`"]
pub type CLK_PER_REG = crate::Reg<clk_per_reg::CLK_PER_REG_SPEC>;
#[doc = "Peripheral divider register"]
pub mod clk_per_reg;
#[doc = "CLK_RADIO_REG register accessor: an alias for `Reg<CLK_RADIO_REG_SPEC>`"]
pub type CLK_RADIO_REG = crate::Reg<clk_radio_reg::CLK_RADIO_REG_SPEC>;
#[doc = "Radio PLL control register"]
pub mod clk_radio_reg;
#[doc = "CLK_RC32K_REG register accessor: an alias for `Reg<CLK_RC32K_REG_SPEC>`"]
pub type CLK_RC32K_REG = crate::Reg<clk_rc32k_reg::CLK_RC32K_REG_SPEC>;
#[doc = "32 kHz RC oscillator register"]
pub mod clk_rc32k_reg;
#[doc = "CLK_RC32M_REG register accessor: an alias for `Reg<CLK_RC32M_REG_SPEC>`"]
pub type CLK_RC32M_REG = crate::Reg<clk_rc32m_reg::CLK_RC32M_REG_SPEC>;
#[doc = "Fast RC control register"]
pub mod clk_rc32m_reg;
#[doc = "CLK_RCX_REG register accessor: an alias for `Reg<CLK_RCX_REG_SPEC>`"]
pub type CLK_RCX_REG = crate::Reg<clk_rcx_reg::CLK_RCX_REG_SPEC>;
#[doc = "RCX-oscillator control register"]
pub mod clk_rcx_reg;
#[doc = "CLK_XTAL32K_REG register accessor: an alias for `Reg<CLK_XTAL32K_REG_SPEC>`"]
pub type CLK_XTAL32K_REG = crate::Reg<clk_xtal32k_reg::CLK_XTAL32K_REG_SPEC>;
#[doc = "32 kHz XTAL oscillator register"]
pub mod clk_xtal32k_reg;
#[doc = "PMU_CTRL_REG register accessor: an alias for `Reg<PMU_CTRL_REG_SPEC>`"]
pub type PMU_CTRL_REG = crate::Reg<pmu_ctrl_reg::PMU_CTRL_REG_SPEC>;
#[doc = "Power Management Unit control register"]
pub mod pmu_ctrl_reg;
#[doc = "PMU_SLEEP_REG register accessor: an alias for `Reg<PMU_SLEEP_REG_SPEC>`"]
pub type PMU_SLEEP_REG = crate::Reg<pmu_sleep_reg::PMU_SLEEP_REG_SPEC>;
#[doc = "Bandgap refresh interval during sleep"]
pub mod pmu_sleep_reg;
#[doc = "POR_PIN_REG register accessor: an alias for `Reg<POR_PIN_REG_SPEC>`"]
pub type POR_PIN_REG = crate::Reg<por_pin_reg::POR_PIN_REG_SPEC>;
#[doc = "Selects a GPIO pin for POR generation"]
pub mod por_pin_reg;
#[doc = "POR_TIMER_REG register accessor: an alias for `Reg<POR_TIMER_REG_SPEC>`"]
pub type POR_TIMER_REG = crate::Reg<por_timer_reg::POR_TIMER_REG_SPEC>;
#[doc = "Time for POR to happen"]
pub mod por_timer_reg;
#[doc = "POWER_CTRL_REG register accessor: an alias for `Reg<POWER_CTRL_REG_SPEC>`"]
pub type POWER_CTRL_REG = crate::Reg<power_ctrl_reg::POWER_CTRL_REG_SPEC>;
#[doc = "Power management control"]
pub mod power_ctrl_reg;
#[doc = "POWER_LEVEL_REG register accessor: an alias for `Reg<POWER_LEVEL_REG_SPEC>`"]
pub type POWER_LEVEL_REG = crate::Reg<power_level_reg::POWER_LEVEL_REG_SPEC>;
#[doc = "Power management level and trim settings"]
pub mod power_level_reg;
#[doc = "RAM_PWR_CTRL_REG register accessor: an alias for `Reg<RAM_PWR_CTRL_REG_SPEC>`"]
pub type RAM_PWR_CTRL_REG = crate::Reg<ram_pwr_ctrl_reg::RAM_PWR_CTRL_REG_SPEC>;
#[doc = "Control power state of System RAMS"]
pub mod ram_pwr_ctrl_reg;
#[doc = "SYS_CTRL_REG register accessor: an alias for `Reg<SYS_CTRL_REG_SPEC>`"]
pub type SYS_CTRL_REG = crate::Reg<sys_ctrl_reg::SYS_CTRL_REG_SPEC>;
#[doc = "System Control register"]
pub mod sys_ctrl_reg;
#[doc = "SYS_STAT_REG register accessor: an alias for `Reg<SYS_STAT_REG_SPEC>`"]
pub type SYS_STAT_REG = crate::Reg<sys_stat_reg::SYS_STAT_REG_SPEC>;
#[doc = "System status register"]
pub mod sys_stat_reg;
#[doc = "TRIM_CTRL_REG register accessor: an alias for `Reg<TRIM_CTRL_REG_SPEC>`"]
pub type TRIM_CTRL_REG = crate::Reg<trim_ctrl_reg::TRIM_CTRL_REG_SPEC>;
#[doc = "Control trimming of the XTAL32M"]
pub mod trim_ctrl_reg;
#[doc = "XTAL32M_CTRL0_REG register accessor: an alias for `Reg<XTAL32M_CTRL0_REG_SPEC>`"]
pub type XTAL32M_CTRL0_REG = crate::Reg<xtal32m_ctrl0_reg::XTAL32M_CTRL0_REG_SPEC>;
#[doc = "Control bits for XTAL32M"]
pub mod xtal32m_ctrl0_reg;
#[doc = "XTAL32M_START_REG register accessor: an alias for `Reg<XTAL32M_START_REG_SPEC>`"]
pub type XTAL32M_START_REG = crate::Reg<xtal32m_start_reg::XTAL32M_START_REG_SPEC>;
#[doc = "Trim values for XTAL32M"]
pub mod xtal32m_start_reg;
#[doc = "XTAL32M_TRSTAT_REG register accessor: an alias for `Reg<XTAL32M_TRSTAT_REG_SPEC>`"]
pub type XTAL32M_TRSTAT_REG = crate::Reg<xtal32m_trstat_reg::XTAL32M_TRSTAT_REG_SPEC>;
#[doc = "Read back value of current XTAL trimming"]
pub mod xtal32m_trstat_reg;
#[doc = "XTALRDY_CTRL_REG register accessor: an alias for `Reg<XTALRDY_CTRL_REG_SPEC>`"]
pub type XTALRDY_CTRL_REG = crate::Reg<xtalrdy_ctrl_reg::XTALRDY_CTRL_REG_SPEC>;
#[doc = "Control register for XTALRDY IRQ"]
pub mod xtalrdy_ctrl_reg;
#[doc = "XTALRDY_STAT_REG register accessor: an alias for `Reg<XTALRDY_STAT_REG_SPEC>`"]
pub type XTALRDY_STAT_REG = crate::Reg<xtalrdy_stat_reg::XTALRDY_STAT_REG_SPEC>;
#[doc = ""]
pub mod xtalrdy_stat_reg;
