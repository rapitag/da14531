#[doc = "Register `SYS_STAT_REG` reader"]
pub struct R(crate::R<SYS_STAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_STAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_STAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_STAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_STAT_REG` writer"]
pub struct W(crate::W<SYS_STAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_STAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYS_STAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_STAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32M_SETTLED` reader - Indicates that XTAL32M has had its settle time, as defined by TRIM_CTRL_REG\\[XTAL_SETTLE_N\\]"]
pub struct XTAL32M_SETTLED_R(crate::FieldReader<bool, bool>);
impl XTAL32M_SETTLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32M_SETTLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32M_SETTLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32M_TRIM_READY` reader - Indicates that XTAL trimming mechanism is ready, i.e. the trimming equals CLK_FREQ_TRIM_REG."]
pub struct XTAL32M_TRIM_READY_R(crate::FieldReader<bool, bool>);
impl XTAL32M_TRIM_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32M_TRIM_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32M_TRIM_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_IS_UP` reader - Indicates that the SW debugger is attached and in connection with the ARM."]
pub struct DBG_IS_UP_R(crate::FieldReader<bool, bool>);
impl DBG_IS_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_IS_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_IS_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM_IS_UP` reader - Indicates that PD_TIM is functional"]
pub struct TIM_IS_UP_R(crate::FieldReader<bool, bool>);
impl TIM_IS_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIM_IS_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM_IS_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM_IS_DOWN` reader - Indicates that PD_TIM is in power down"]
pub struct TIM_IS_DOWN_R(crate::FieldReader<bool, bool>);
impl TIM_IS_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIM_IS_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM_IS_DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAD_IS_UP` reader - Indicates that PD_RAD is functional"]
pub struct RAD_IS_UP_R(crate::FieldReader<bool, bool>);
impl RAD_IS_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAD_IS_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAD_IS_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAD_IS_DOWN` reader - Indicates that PD_RAD is in power down"]
pub struct RAD_IS_DOWN_R(crate::FieldReader<bool, bool>);
impl RAD_IS_DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAD_IS_DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAD_IS_DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Indicates that XTAL32M has had its settle time, as defined by TRIM_CTRL_REG\\[XTAL_SETTLE_N\\]"]
    #[inline(always)]
    pub fn xtal32m_settled(&self) -> XTAL32M_SETTLED_R {
        XTAL32M_SETTLED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that XTAL trimming mechanism is ready, i.e. the trimming equals CLK_FREQ_TRIM_REG."]
    #[inline(always)]
    pub fn xtal32m_trim_ready(&self) -> XTAL32M_TRIM_READY_R {
        XTAL32M_TRIM_READY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that the SW debugger is attached and in connection with the ARM."]
    #[inline(always)]
    pub fn dbg_is_up(&self) -> DBG_IS_UP_R {
        DBG_IS_UP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that PD_TIM is functional"]
    #[inline(always)]
    pub fn tim_is_up(&self) -> TIM_IS_UP_R {
        TIM_IS_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that PD_TIM is in power down"]
    #[inline(always)]
    pub fn tim_is_down(&self) -> TIM_IS_DOWN_R {
        TIM_IS_DOWN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that PD_RAD is functional"]
    #[inline(always)]
    pub fn rad_is_up(&self) -> RAD_IS_UP_R {
        RAD_IS_UP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Indicates that PD_RAD is in power down"]
    #[inline(always)]
    pub fn rad_is_down(&self) -> RAD_IS_DOWN_R {
        RAD_IS_DOWN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_stat_reg](index.html) module"]
pub struct SYS_STAT_REG_SPEC;
impl crate::RegisterSpec for SYS_STAT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sys_stat_reg::R](R) reader structure"]
impl crate::Readable for SYS_STAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_stat_reg::W](W) writer structure"]
impl crate::Writable for SYS_STAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_STAT_REG to value 0x45"]
impl crate::Resettable for SYS_STAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x45
    }
}
