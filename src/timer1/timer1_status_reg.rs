#[doc = "Register `TIMER1_STATUS_REG` reader"]
pub struct R(crate::R<TIMER1_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_STATUS_REG` writer"]
pub struct W(crate::W<TIMER1_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_STATUS_REG_SPEC>;
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
impl From<crate::W<TIMER1_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_IN2_OVRFLW` reader - 1 = New IN2 event occurred while Interrupt was pending. TIMER1_CAPCNT2_VALUE_REG gives the time stamp of the first event."]
pub struct TIMER1_IN2_OVRFLW_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN2_OVRFLW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN2_OVRFLW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN2_OVRFLW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN1_OVRFLW` reader - 1 = New IN1 event occurred while Interrupt was pending. TIMER1_CAPCNT1_VALUE_REG gives the time stamp of the first event."]
pub struct TIMER1_IN1_OVRFLW_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN1_OVRFLW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN1_OVRFLW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN1_OVRFLW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN2_EVENT` reader - 1 = Pending Capture 2 interrupt. It has be clear writing 1 to TIMER1_CLR_IN2_EVENT"]
pub struct TIMER1_IN2_EVENT_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN2_EVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN2_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN2_EVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN1_EVENT` reader - 1 = Pending Capture 1 interrupt. It has be clear writing 1 to TIMER1_CLR_IN1_EVENT"]
pub struct TIMER1_IN1_EVENT_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN1_EVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN1_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN1_EVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_TIMER_EVENT` reader - 1 = Pending Timer interrupt. it has be clear writing 1' to TIMER1_CLR_TIMER_EVENT"]
pub struct TIMER1_TIMER_EVENT_R(crate::FieldReader<bool, bool>);
impl TIMER1_TIMER_EVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_TIMER_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_TIMER_EVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_TIMER_VALUE` reader - Gives the current timer value"]
pub struct TIMER1_TIMER_VALUE_R(crate::FieldReader<u16, u16>);
impl TIMER1_TIMER_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER1_TIMER_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_TIMER_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15 - 1 = New IN2 event occurred while Interrupt was pending. TIMER1_CAPCNT2_VALUE_REG gives the time stamp of the first event."]
    #[inline(always)]
    pub fn timer1_in2_ovrflw(&self) -> TIMER1_IN2_OVRFLW_R {
        TIMER1_IN2_OVRFLW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - 1 = New IN1 event occurred while Interrupt was pending. TIMER1_CAPCNT1_VALUE_REG gives the time stamp of the first event."]
    #[inline(always)]
    pub fn timer1_in1_ovrflw(&self) -> TIMER1_IN1_OVRFLW_R {
        TIMER1_IN1_OVRFLW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - 1 = Pending Capture 2 interrupt. It has be clear writing 1 to TIMER1_CLR_IN2_EVENT"]
    #[inline(always)]
    pub fn timer1_in2_event(&self) -> TIMER1_IN2_EVENT_R {
        TIMER1_IN2_EVENT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - 1 = Pending Capture 1 interrupt. It has be clear writing 1 to TIMER1_CLR_IN1_EVENT"]
    #[inline(always)]
    pub fn timer1_in1_event(&self) -> TIMER1_IN1_EVENT_R {
        TIMER1_IN1_EVENT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - 1 = Pending Timer interrupt. it has be clear writing 1' to TIMER1_CLR_TIMER_EVENT"]
    #[inline(always)]
    pub fn timer1_timer_event(&self) -> TIMER1_TIMER_EVENT_R {
        TIMER1_TIMER_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 0:10 - Gives the current timer value"]
    #[inline(always)]
    pub fn timer1_timer_value(&self) -> TIMER1_TIMER_VALUE_R {
        TIMER1_TIMER_VALUE_R::new((self.bits & 0x07ff) as u16)
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
#[doc = "Timer1 counter value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_status_reg](index.html) module"]
pub struct TIMER1_STATUS_REG_SPEC;
impl crate::RegisterSpec for TIMER1_STATUS_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_status_reg::R](R) reader structure"]
impl crate::Readable for TIMER1_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_status_reg::W](W) writer structure"]
impl crate::Writable for TIMER1_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_STATUS_REG to value 0"]
impl crate::Resettable for TIMER1_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
