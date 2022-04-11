#[doc = "Register `PWM2_START_CYCLE` reader"]
pub struct R(crate::R<PWM2_START_CYCLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM2_START_CYCLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM2_START_CYCLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM2_START_CYCLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM2_START_CYCLE` writer"]
pub struct W(crate::W<PWM2_START_CYCLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM2_START_CYCLE_SPEC>;
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
impl From<crate::W<PWM2_START_CYCLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM2_START_CYCLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_CYCLE` reader - Defines the cycle in which the PWM becomes high. if start_cycle is larger than freq or end_cycle is equal to start_cycle, pwm out is always 0"]
pub struct START_CYCLE_R(crate::FieldReader<u16, u16>);
impl START_CYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        START_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_CYCLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_CYCLE` writer - Defines the cycle in which the PWM becomes high. if start_cycle is larger than freq or end_cycle is equal to start_cycle, pwm out is always 0"]
pub struct START_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u16 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Defines the cycle in which the PWM becomes high. if start_cycle is larger than freq or end_cycle is equal to start_cycle, pwm out is always 0"]
    #[inline(always)]
    pub fn start_cycle(&self) -> START_CYCLE_R {
        START_CYCLE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Defines the cycle in which the PWM becomes high. if start_cycle is larger than freq or end_cycle is equal to start_cycle, pwm out is always 0"]
    #[inline(always)]
    pub fn start_cycle(&mut self) -> START_CYCLE_W {
        START_CYCLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Defines start Cycle for PWM2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm2_start_cycle](index.html) module"]
pub struct PWM2_START_CYCLE_SPEC;
impl crate::RegisterSpec for PWM2_START_CYCLE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pwm2_start_cycle::R](R) reader structure"]
impl crate::Readable for PWM2_START_CYCLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm2_start_cycle::W](W) writer structure"]
impl crate::Writable for PWM2_START_CYCLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM2_START_CYCLE to value 0"]
impl crate::Resettable for PWM2_START_CYCLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
