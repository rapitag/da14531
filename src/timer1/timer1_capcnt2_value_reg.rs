#[doc = "Register `TIMER1_CAPCNT2_VALUE_REG` reader"]
pub struct R(crate::R<TIMER1_CAPCNT2_VALUE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_CAPCNT2_VALUE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_CAPCNT2_VALUE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_CAPCNT2_VALUE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_CAPCNT2_VALUE_REG` writer"]
pub struct W(crate::W<TIMER1_CAPCNT2_VALUE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_CAPCNT2_VALUE_REG_SPEC>;
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
impl From<crate::W<TIMER1_CAPCNT2_VALUE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_CAPCNT2_VALUE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_CAPCNT2_RTC_HIGH` reader - In Counter mode : Not used In Capture mode: Gives the RTC time stamp (high part) when an IN2 event was occurred"]
pub struct TIMER1_CAPCNT2_RTC_HIGH_R(crate::FieldReader<u16, u16>);
impl TIMER1_CAPCNT2_RTC_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER1_CAPCNT2_RTC_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_CAPCNT2_RTC_HIGH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_CAPCNT2_VALUE` reader - In Counter mode : Gives the number of timer clock cycles minus 1 which was measured during TIMER1_IN2_PERIOD_MAX periods of IN2 In Capture mode (TIMER1_IN2_STAMP_TYPE=0) : Gives the Counter value when an IN2 event was occurred In Capture mode (TIMER1_IN2_STAMP_TYPE=1) : Gives the RTC time stamp (low part) when an IN2 event was occurred"]
pub struct TIMER1_CAPCNT2_VALUE_R(crate::FieldReader<u16, u16>);
impl TIMER1_CAPCNT2_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER1_CAPCNT2_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_CAPCNT2_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 11:21 - In Counter mode : Not used In Capture mode: Gives the RTC time stamp (high part) when an IN2 event was occurred"]
    #[inline(always)]
    pub fn timer1_capcnt2_rtc_high(&self) -> TIMER1_CAPCNT2_RTC_HIGH_R {
        TIMER1_CAPCNT2_RTC_HIGH_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10 - In Counter mode : Gives the number of timer clock cycles minus 1 which was measured during TIMER1_IN2_PERIOD_MAX periods of IN2 In Capture mode (TIMER1_IN2_STAMP_TYPE=0) : Gives the Counter value when an IN2 event was occurred In Capture mode (TIMER1_IN2_STAMP_TYPE=1) : Gives the RTC time stamp (low part) when an IN2 event was occurred"]
    #[inline(always)]
    pub fn timer1_capcnt2_value(&self) -> TIMER1_CAPCNT2_VALUE_R {
        TIMER1_CAPCNT2_VALUE_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer1 value for event on GPIO2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_capcnt2_value_reg](index.html) module"]
pub struct TIMER1_CAPCNT2_VALUE_REG_SPEC;
impl crate::RegisterSpec for TIMER1_CAPCNT2_VALUE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_capcnt2_value_reg::R](R) reader structure"]
impl crate::Readable for TIMER1_CAPCNT2_VALUE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_capcnt2_value_reg::W](W) writer structure"]
impl crate::Writable for TIMER1_CAPCNT2_VALUE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_CAPCNT2_VALUE_REG to value 0"]
impl crate::Resettable for TIMER1_CAPCNT2_VALUE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
