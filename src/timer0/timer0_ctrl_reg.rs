#[doc = "Register `TIMER0_CTRL_REG` reader"]
pub struct R(crate::R<TIMER0_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0_CTRL_REG` writer"]
pub struct W(crate::W<TIMER0_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_CTRL_REG_SPEC>;
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
impl From<crate::W<TIMER0_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_MODE` reader - 0 = PWM signals are '1' during high time. 1 = PWM signals send out the (fast) clock divided by 2 during high time. So it will be in the range of 1 to 8 MHz."]
pub struct PWM_MODE_R(crate::FieldReader<bool, bool>);
impl PWM_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWM_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM_MODE` writer - 0 = PWM signals are '1' during high time. 1 = PWM signals send out the (fast) clock divided by 2 during high time. So it will be in the range of 1 to 8 MHz."]
pub struct PWM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `TIM0_CLK_DIV` reader - 1 = Timer0 uses selected clock frequency as is. 0 = Timer0 uses selected clock frequency divided by 10. Note that this applies only to the ON-counter."]
pub struct TIM0_CLK_DIV_R(crate::FieldReader<bool, bool>);
impl TIM0_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIM0_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM0_CLK_DIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM0_CLK_DIV` writer - 1 = Timer0 uses selected clock frequency as is. 0 = Timer0 uses selected clock frequency divided by 10. Note that this applies only to the ON-counter."]
pub struct TIM0_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM0_CLK_DIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `TIM0_CLK_SEL` reader - 1 = Timer0 uses 16, 8, 4 or 2 MHz (fast) clock frequency. 0 = Timer0 uses LP clock"]
pub struct TIM0_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl TIM0_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIM0_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM0_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM0_CLK_SEL` writer - 1 = Timer0 uses 16, 8, 4 or 2 MHz (fast) clock frequency. 0 = Timer0 uses LP clock"]
pub struct TIM0_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM0_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `TIM0_CTRL` reader - 0 = Timer0 is off and in reset state. 1 = Timer0 is running."]
pub struct TIM0_CTRL_R(crate::FieldReader<bool, bool>);
impl TIM0_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIM0_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM0_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM0_CTRL` writer - 0 = Timer0 is off and in reset state. 1 = Timer0 is running."]
pub struct TIM0_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM0_CTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - 0 = PWM signals are '1' during high time. 1 = PWM signals send out the (fast) clock divided by 2 during high time. So it will be in the range of 1 to 8 MHz."]
    #[inline(always)]
    pub fn pwm_mode(&self) -> PWM_MODE_R {
        PWM_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 = Timer0 uses selected clock frequency as is. 0 = Timer0 uses selected clock frequency divided by 10. Note that this applies only to the ON-counter."]
    #[inline(always)]
    pub fn tim0_clk_div(&self) -> TIM0_CLK_DIV_R {
        TIM0_CLK_DIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - 1 = Timer0 uses 16, 8, 4 or 2 MHz (fast) clock frequency. 0 = Timer0 uses LP clock"]
    #[inline(always)]
    pub fn tim0_clk_sel(&self) -> TIM0_CLK_SEL_R {
        TIM0_CLK_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 0 = Timer0 is off and in reset state. 1 = Timer0 is running."]
    #[inline(always)]
    pub fn tim0_ctrl(&self) -> TIM0_CTRL_R {
        TIM0_CTRL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0 = PWM signals are '1' during high time. 1 = PWM signals send out the (fast) clock divided by 2 during high time. So it will be in the range of 1 to 8 MHz."]
    #[inline(always)]
    pub fn pwm_mode(&mut self) -> PWM_MODE_W {
        PWM_MODE_W { w: self }
    }
    #[doc = "Bit 2 - 1 = Timer0 uses selected clock frequency as is. 0 = Timer0 uses selected clock frequency divided by 10. Note that this applies only to the ON-counter."]
    #[inline(always)]
    pub fn tim0_clk_div(&mut self) -> TIM0_CLK_DIV_W {
        TIM0_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 1 - 1 = Timer0 uses 16, 8, 4 or 2 MHz (fast) clock frequency. 0 = Timer0 uses LP clock"]
    #[inline(always)]
    pub fn tim0_clk_sel(&mut self) -> TIM0_CLK_SEL_W {
        TIM0_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 0 - 0 = Timer0 is off and in reset state. 1 = Timer0 is running."]
    #[inline(always)]
    pub fn tim0_ctrl(&mut self) -> TIM0_CTRL_W {
        TIM0_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_ctrl_reg](index.html) module"]
pub struct TIMER0_CTRL_REG_SPEC;
impl crate::RegisterSpec for TIMER0_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [timer0_ctrl_reg::R](R) reader structure"]
impl crate::Readable for TIMER0_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_ctrl_reg::W](W) writer structure"]
impl crate::Writable for TIMER0_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER0_CTRL_REG to value 0"]
impl crate::Resettable for TIMER0_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
