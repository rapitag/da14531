#[doc = "Register `TRIPLE_PWM_FREQUENCY` reader"]
pub struct R(crate::R<TRIPLE_PWM_FREQUENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIPLE_PWM_FREQUENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIPLE_PWM_FREQUENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIPLE_PWM_FREQUENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIPLE_PWM_FREQUENCY` writer"]
pub struct W(crate::W<TRIPLE_PWM_FREQUENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIPLE_PWM_FREQUENCY_SPEC>;
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
impl From<crate::W<TRIPLE_PWM_FREQUENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIPLE_PWM_FREQUENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_FREQ` reader - Defines the frequeancy of PWM 2,3,4,5,,6 and 7. pwm freq = module Frequency / (value+1) module frequency is the LP_CLK when TRIPLE_PWM_CLK_SEL=0 else is the sys_clk divided by TMR_DIV"]
pub struct PWM_FREQ_R(crate::FieldReader<u16, u16>);
impl PWM_FREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PWM_FREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_FREQ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM_FREQ` writer - Defines the frequeancy of PWM 2,3,4,5,,6 and 7. pwm freq = module Frequency / (value+1) module frequency is the LP_CLK when TRIPLE_PWM_CLK_SEL=0 else is the sys_clk divided by TMR_DIV"]
pub struct PWM_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u16 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Defines the frequeancy of PWM 2,3,4,5,,6 and 7. pwm freq = module Frequency / (value+1) module frequency is the LP_CLK when TRIPLE_PWM_CLK_SEL=0 else is the sys_clk divided by TMR_DIV"]
    #[inline(always)]
    pub fn pwm_freq(&self) -> PWM_FREQ_R {
        PWM_FREQ_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Defines the frequeancy of PWM 2,3,4,5,,6 and 7. pwm freq = module Frequency / (value+1) module frequency is the LP_CLK when TRIPLE_PWM_CLK_SEL=0 else is the sys_clk divided by TMR_DIV"]
    #[inline(always)]
    pub fn pwm_freq(&mut self) -> PWM_FREQ_W {
        PWM_FREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency for PWM 2,3,4,5,6 and 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [triple_pwm_frequency](index.html) module"]
pub struct TRIPLE_PWM_FREQUENCY_SPEC;
impl crate::RegisterSpec for TRIPLE_PWM_FREQUENCY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [triple_pwm_frequency::R](R) reader structure"]
impl crate::Readable for TRIPLE_PWM_FREQUENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [triple_pwm_frequency::W](W) writer structure"]
impl crate::Writable for TRIPLE_PWM_FREQUENCY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIPLE_PWM_FREQUENCY to value 0"]
impl crate::Resettable for TRIPLE_PWM_FREQUENCY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
