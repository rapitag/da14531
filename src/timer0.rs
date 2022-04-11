#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0 control register"]
    pub timer0_ctrl_reg: crate::Reg<timer0_ctrl_reg::TIMER0_CTRL_REG_SPEC>,
    #[doc = "0x02 - Timer0 on control register"]
    pub timer0_on_reg: crate::Reg<timer0_on_reg::TIMER0_ON_REG_SPEC>,
    #[doc = "0x04 - 16 bits reload value for Timer0"]
    pub timer0_reload_m_reg: crate::Reg<timer0_reload_m_reg::TIMER0_RELOAD_M_REG_SPEC>,
    #[doc = "0x06 - 16 bits reload value for Timer0"]
    pub timer0_reload_n_reg: crate::Reg<timer0_reload_n_reg::TIMER0_RELOAD_N_REG_SPEC>,
    #[doc = "0x08 - Frequency for PWM 2,3,4,5,6 and 7"]
    pub triple_pwm_frequency: crate::Reg<triple_pwm_frequency::TRIPLE_PWM_FREQUENCY_SPEC>,
    #[doc = "0x0a - Defines start Cycle for PWM2"]
    pub pwm2_start_cycle: crate::Reg<pwm2_start_cycle::PWM2_START_CYCLE_SPEC>,
    #[doc = "0x0c - Defines start Cycle for PWM3"]
    pub pwm3_start_cycle: crate::Reg<pwm3_start_cycle::PWM3_START_CYCLE_SPEC>,
    #[doc = "0x0e - Defines start Cycle for PWM4"]
    pub pwm4_start_cycle: crate::Reg<pwm4_start_cycle::PWM4_START_CYCLE_SPEC>,
    #[doc = "0x10 - Defines start Cycle for PWM5"]
    pub pwm5_start_cycle: crate::Reg<pwm5_start_cycle::PWM5_START_CYCLE_SPEC>,
    #[doc = "0x12 - Defines start Cycle for PWM6"]
    pub pwm6_start_cycle: crate::Reg<pwm6_start_cycle::PWM6_START_CYCLE_SPEC>,
    #[doc = "0x14 - Defines start Cycle for PWM7"]
    pub pwm7_start_cycle: crate::Reg<pwm7_start_cycle::PWM7_START_CYCLE_SPEC>,
    #[doc = "0x16 - Defines end Cycle for PWM2"]
    pub pwm2_end_cycle: crate::Reg<pwm2_end_cycle::PWM2_END_CYCLE_SPEC>,
    #[doc = "0x18 - Defines end Cycle for PWM3"]
    pub pwm3_end_cycle: crate::Reg<pwm3_end_cycle::PWM3_END_CYCLE_SPEC>,
    #[doc = "0x1a - Defines end Cycle for PWM4"]
    pub pwm4_end_cycle: crate::Reg<pwm4_end_cycle::PWM4_END_CYCLE_SPEC>,
    #[doc = "0x1c - Defines end Cycle for PWM5"]
    pub pwm5_end_cycle: crate::Reg<pwm5_end_cycle::PWM5_END_CYCLE_SPEC>,
    #[doc = "0x1e - Defines end Cycle for PWM6"]
    pub pwm6_end_cycle: crate::Reg<pwm6_end_cycle::PWM6_END_CYCLE_SPEC>,
    #[doc = "0x20 - Defines end Cycle for PWM7"]
    pub pwm7_end_cycle: crate::Reg<pwm7_end_cycle::PWM7_END_CYCLE_SPEC>,
    #[doc = "0x22 - PWM 2,3,4,5,6,7 Control"]
    pub triple_pwm_ctrl_reg: crate::Reg<triple_pwm_ctrl_reg::TRIPLE_PWM_CTRL_REG_SPEC>,
}
#[doc = "PWM2_END_CYCLE register accessor: an alias for `Reg<PWM2_END_CYCLE_SPEC>`"]
pub type PWM2_END_CYCLE = crate::Reg<pwm2_end_cycle::PWM2_END_CYCLE_SPEC>;
#[doc = "Defines end Cycle for PWM2"]
pub mod pwm2_end_cycle;
#[doc = "PWM2_START_CYCLE register accessor: an alias for `Reg<PWM2_START_CYCLE_SPEC>`"]
pub type PWM2_START_CYCLE = crate::Reg<pwm2_start_cycle::PWM2_START_CYCLE_SPEC>;
#[doc = "Defines start Cycle for PWM2"]
pub mod pwm2_start_cycle;
#[doc = "PWM3_END_CYCLE register accessor: an alias for `Reg<PWM3_END_CYCLE_SPEC>`"]
pub type PWM3_END_CYCLE = crate::Reg<pwm3_end_cycle::PWM3_END_CYCLE_SPEC>;
#[doc = "Defines end Cycle for PWM3"]
pub mod pwm3_end_cycle;
#[doc = "PWM3_START_CYCLE register accessor: an alias for `Reg<PWM3_START_CYCLE_SPEC>`"]
pub type PWM3_START_CYCLE = crate::Reg<pwm3_start_cycle::PWM3_START_CYCLE_SPEC>;
#[doc = "Defines start Cycle for PWM3"]
pub mod pwm3_start_cycle;
#[doc = "PWM4_END_CYCLE register accessor: an alias for `Reg<PWM4_END_CYCLE_SPEC>`"]
pub type PWM4_END_CYCLE = crate::Reg<pwm4_end_cycle::PWM4_END_CYCLE_SPEC>;
#[doc = "Defines end Cycle for PWM4"]
pub mod pwm4_end_cycle;
#[doc = "PWM4_START_CYCLE register accessor: an alias for `Reg<PWM4_START_CYCLE_SPEC>`"]
pub type PWM4_START_CYCLE = crate::Reg<pwm4_start_cycle::PWM4_START_CYCLE_SPEC>;
#[doc = "Defines start Cycle for PWM4"]
pub mod pwm4_start_cycle;
#[doc = "PWM5_END_CYCLE register accessor: an alias for `Reg<PWM5_END_CYCLE_SPEC>`"]
pub type PWM5_END_CYCLE = crate::Reg<pwm5_end_cycle::PWM5_END_CYCLE_SPEC>;
#[doc = "Defines end Cycle for PWM5"]
pub mod pwm5_end_cycle;
#[doc = "PWM5_START_CYCLE register accessor: an alias for `Reg<PWM5_START_CYCLE_SPEC>`"]
pub type PWM5_START_CYCLE = crate::Reg<pwm5_start_cycle::PWM5_START_CYCLE_SPEC>;
#[doc = "Defines start Cycle for PWM5"]
pub mod pwm5_start_cycle;
#[doc = "PWM6_END_CYCLE register accessor: an alias for `Reg<PWM6_END_CYCLE_SPEC>`"]
pub type PWM6_END_CYCLE = crate::Reg<pwm6_end_cycle::PWM6_END_CYCLE_SPEC>;
#[doc = "Defines end Cycle for PWM6"]
pub mod pwm6_end_cycle;
#[doc = "PWM6_START_CYCLE register accessor: an alias for `Reg<PWM6_START_CYCLE_SPEC>`"]
pub type PWM6_START_CYCLE = crate::Reg<pwm6_start_cycle::PWM6_START_CYCLE_SPEC>;
#[doc = "Defines start Cycle for PWM6"]
pub mod pwm6_start_cycle;
#[doc = "PWM7_END_CYCLE register accessor: an alias for `Reg<PWM7_END_CYCLE_SPEC>`"]
pub type PWM7_END_CYCLE = crate::Reg<pwm7_end_cycle::PWM7_END_CYCLE_SPEC>;
#[doc = "Defines end Cycle for PWM7"]
pub mod pwm7_end_cycle;
#[doc = "PWM7_START_CYCLE register accessor: an alias for `Reg<PWM7_START_CYCLE_SPEC>`"]
pub type PWM7_START_CYCLE = crate::Reg<pwm7_start_cycle::PWM7_START_CYCLE_SPEC>;
#[doc = "Defines start Cycle for PWM7"]
pub mod pwm7_start_cycle;
#[doc = "TIMER0_CTRL_REG register accessor: an alias for `Reg<TIMER0_CTRL_REG_SPEC>`"]
pub type TIMER0_CTRL_REG = crate::Reg<timer0_ctrl_reg::TIMER0_CTRL_REG_SPEC>;
#[doc = "Timer0 control register"]
pub mod timer0_ctrl_reg;
#[doc = "TIMER0_ON_REG register accessor: an alias for `Reg<TIMER0_ON_REG_SPEC>`"]
pub type TIMER0_ON_REG = crate::Reg<timer0_on_reg::TIMER0_ON_REG_SPEC>;
#[doc = "Timer0 on control register"]
pub mod timer0_on_reg;
#[doc = "TIMER0_RELOAD_M_REG register accessor: an alias for `Reg<TIMER0_RELOAD_M_REG_SPEC>`"]
pub type TIMER0_RELOAD_M_REG = crate::Reg<timer0_reload_m_reg::TIMER0_RELOAD_M_REG_SPEC>;
#[doc = "16 bits reload value for Timer0"]
pub mod timer0_reload_m_reg;
#[doc = "TIMER0_RELOAD_N_REG register accessor: an alias for `Reg<TIMER0_RELOAD_N_REG_SPEC>`"]
pub type TIMER0_RELOAD_N_REG = crate::Reg<timer0_reload_n_reg::TIMER0_RELOAD_N_REG_SPEC>;
#[doc = "16 bits reload value for Timer0"]
pub mod timer0_reload_n_reg;
#[doc = "TRIPLE_PWM_CTRL_REG register accessor: an alias for `Reg<TRIPLE_PWM_CTRL_REG_SPEC>`"]
pub type TRIPLE_PWM_CTRL_REG = crate::Reg<triple_pwm_ctrl_reg::TRIPLE_PWM_CTRL_REG_SPEC>;
#[doc = "PWM 2,3,4,5,6,7 Control"]
pub mod triple_pwm_ctrl_reg;
#[doc = "TRIPLE_PWM_FREQUENCY register accessor: an alias for `Reg<TRIPLE_PWM_FREQUENCY_SPEC>`"]
pub type TRIPLE_PWM_FREQUENCY = crate::Reg<triple_pwm_frequency::TRIPLE_PWM_FREQUENCY_SPEC>;
#[doc = "Frequency for PWM 2,3,4,5,6 and 7"]
pub mod triple_pwm_frequency;
