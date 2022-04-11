#[doc = "Register `TIMER1_CTRL_REG` reader"]
pub struct R(crate::R<TIMER1_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_CTRL_REG` writer"]
pub struct W(crate::W<TIMER1_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_CTRL_REG_SPEC>;
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
impl From<crate::W<TIMER1_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_CLK_EN` reader - 0 = timer1 clock is disabled 1 = timer1 clock is enabled"]
pub struct TIMER1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_CLK_EN` writer - 0 = timer1 clock is disabled 1 = timer1 clock is enabled"]
pub struct TIMER1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `TIMER1_USE_SYS_CLK` reader - 0 = Timer1 use the clock LP clock 1 = Timer1 use the system clock"]
pub struct TIMER1_USE_SYS_CLK_R(crate::FieldReader<bool, bool>);
impl TIMER1_USE_SYS_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_USE_SYS_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_USE_SYS_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_USE_SYS_CLK` writer - 0 = Timer1 use the clock LP clock 1 = Timer1 use the system clock"]
pub struct TIMER1_USE_SYS_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_USE_SYS_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `TIMER1_FREE_RUN_MODE_EN` reader - Applicable when timer counts up 1 = timer1 goes to zero when it reaches the max value. 0 = timer1 goes to zero when it reaches the reload value."]
pub struct TIMER1_FREE_RUN_MODE_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_FREE_RUN_MODE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_FREE_RUN_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_FREE_RUN_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_FREE_RUN_MODE_EN` writer - Applicable when timer counts up 1 = timer1 goes to zero when it reaches the max value. 0 = timer1 goes to zero when it reaches the reload value."]
pub struct TIMER1_FREE_RUN_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_FREE_RUN_MODE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `TIMER1_IRQ_EN` reader - 0 = timer1 IRQ masked 1 = timer1 IRQ unmasked"]
pub struct TIMER1_IRQ_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IRQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IRQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IRQ_EN` writer - 0 = timer1 IRQ masked 1 = timer1 IRQ unmasked"]
pub struct TIMER1_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IRQ_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `TIMER1_COUNT_DOWN_EN` reader - 0 = timer1 counts up 1 = timer1 counts down"]
pub struct TIMER1_COUNT_DOWN_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_COUNT_DOWN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_COUNT_DOWN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_COUNT_DOWN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_COUNT_DOWN_EN` writer - 0 = timer1 counts up 1 = timer1 counts down"]
pub struct TIMER1_COUNT_DOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_COUNT_DOWN_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `TIMER1_ENABLE` reader - 0 = Timer1 disabled 1 = Timer1 enabled"]
pub struct TIMER1_ENABLE_R(crate::FieldReader<bool, bool>);
impl TIMER1_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_ENABLE` writer - 0 = Timer1 disabled 1 = Timer1 enabled"]
pub struct TIMER1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `TIMER1_RELOAD` reader - Reload or max value in timer mode. Actual delay is the register value plus synchronization time (3 clock cycles)"]
pub struct TIMER1_RELOAD_R(crate::FieldReader<u16, u16>);
impl TIMER1_RELOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER1_RELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_RELOAD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_RELOAD` writer - Reload or max value in timer mode. Actual delay is the register value plus synchronization time (3 clock cycles)"]
pub struct TIMER1_RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_RELOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - 0 = timer1 clock is disabled 1 = timer1 clock is enabled"]
    #[inline(always)]
    pub fn timer1_clk_en(&self) -> TIMER1_CLK_EN_R {
        TIMER1_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - 0 = Timer1 use the clock LP clock 1 = Timer1 use the system clock"]
    #[inline(always)]
    pub fn timer1_use_sys_clk(&self) -> TIMER1_USE_SYS_CLK_R {
        TIMER1_USE_SYS_CLK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Applicable when timer counts up 1 = timer1 goes to zero when it reaches the max value. 0 = timer1 goes to zero when it reaches the reload value."]
    #[inline(always)]
    pub fn timer1_free_run_mode_en(&self) -> TIMER1_FREE_RUN_MODE_EN_R {
        TIMER1_FREE_RUN_MODE_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - 0 = timer1 IRQ masked 1 = timer1 IRQ unmasked"]
    #[inline(always)]
    pub fn timer1_irq_en(&self) -> TIMER1_IRQ_EN_R {
        TIMER1_IRQ_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - 0 = timer1 counts up 1 = timer1 counts down"]
    #[inline(always)]
    pub fn timer1_count_down_en(&self) -> TIMER1_COUNT_DOWN_EN_R {
        TIMER1_COUNT_DOWN_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - 0 = Timer1 disabled 1 = Timer1 enabled"]
    #[inline(always)]
    pub fn timer1_enable(&self) -> TIMER1_ENABLE_R {
        TIMER1_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 0:10 - Reload or max value in timer mode. Actual delay is the register value plus synchronization time (3 clock cycles)"]
    #[inline(always)]
    pub fn timer1_reload(&self) -> TIMER1_RELOAD_R {
        TIMER1_RELOAD_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - 0 = timer1 clock is disabled 1 = timer1 clock is enabled"]
    #[inline(always)]
    pub fn timer1_clk_en(&mut self) -> TIMER1_CLK_EN_W {
        TIMER1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 15 - 0 = Timer1 use the clock LP clock 1 = Timer1 use the system clock"]
    #[inline(always)]
    pub fn timer1_use_sys_clk(&mut self) -> TIMER1_USE_SYS_CLK_W {
        TIMER1_USE_SYS_CLK_W { w: self }
    }
    #[doc = "Bit 14 - Applicable when timer counts up 1 = timer1 goes to zero when it reaches the max value. 0 = timer1 goes to zero when it reaches the reload value."]
    #[inline(always)]
    pub fn timer1_free_run_mode_en(&mut self) -> TIMER1_FREE_RUN_MODE_EN_W {
        TIMER1_FREE_RUN_MODE_EN_W { w: self }
    }
    #[doc = "Bit 13 - 0 = timer1 IRQ masked 1 = timer1 IRQ unmasked"]
    #[inline(always)]
    pub fn timer1_irq_en(&mut self) -> TIMER1_IRQ_EN_W {
        TIMER1_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 12 - 0 = timer1 counts up 1 = timer1 counts down"]
    #[inline(always)]
    pub fn timer1_count_down_en(&mut self) -> TIMER1_COUNT_DOWN_EN_W {
        TIMER1_COUNT_DOWN_EN_W { w: self }
    }
    #[doc = "Bit 11 - 0 = Timer1 disabled 1 = Timer1 enabled"]
    #[inline(always)]
    pub fn timer1_enable(&mut self) -> TIMER1_ENABLE_W {
        TIMER1_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:10 - Reload or max value in timer mode. Actual delay is the register value plus synchronization time (3 clock cycles)"]
    #[inline(always)]
    pub fn timer1_reload(&mut self) -> TIMER1_RELOAD_W {
        TIMER1_RELOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_ctrl_reg](index.html) module"]
pub struct TIMER1_CTRL_REG_SPEC;
impl crate::RegisterSpec for TIMER1_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_ctrl_reg::R](R) reader structure"]
impl crate::Readable for TIMER1_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_ctrl_reg::W](W) writer structure"]
impl crate::Writable for TIMER1_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_CTRL_REG to value 0"]
impl crate::Resettable for TIMER1_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
