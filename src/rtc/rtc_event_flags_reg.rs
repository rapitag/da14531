#[doc = "Register `RTC_EVENT_FLAGS_REG` reader"]
pub struct R(crate::R<RTC_EVENT_FLAGS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_EVENT_FLAGS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_EVENT_FLAGS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_EVENT_FLAGS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_EVENT_FLAGS_REG` writer"]
pub struct W(crate::W<RTC_EVENT_FLAGS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_EVENT_FLAGS_REG_SPEC>;
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
impl From<crate::W<RTC_EVENT_FLAGS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_EVENT_FLAGS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_EVENT_ALRM` reader - Alarm event flag. Indicate that alarm event occurred since the last reset."]
pub struct RTC_EVENT_ALRM_R(crate::FieldReader<bool, bool>);
impl RTC_EVENT_ALRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_EVENT_ALRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_EVENT_ALRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_EVENT_MNTH` reader - Month rolls over event flag. Indicate that month rolls over event occurred since the last reset."]
pub struct RTC_EVENT_MNTH_R(crate::FieldReader<bool, bool>);
impl RTC_EVENT_MNTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_EVENT_MNTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_EVENT_MNTH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_EVENT_DATE` reader - Date rolls over event flag. Indicate that date rolls over event occurred since the last reset."]
pub struct RTC_EVENT_DATE_R(crate::FieldReader<bool, bool>);
impl RTC_EVENT_DATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_EVENT_DATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_EVENT_DATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_EVENT_HOUR` reader - Hour rolls over event flag. Indicate that hour rolls over event occurred since the last reset."]
pub struct RTC_EVENT_HOUR_R(crate::FieldReader<bool, bool>);
impl RTC_EVENT_HOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_EVENT_HOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_EVENT_HOUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_EVENT_MIN` reader - Minute rolls over event flag. Indicate that minute rolls over event occurred since the last reset."]
pub struct RTC_EVENT_MIN_R(crate::FieldReader<bool, bool>);
impl RTC_EVENT_MIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_EVENT_MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_EVENT_MIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_EVENT_SEC` reader - Second rolls over event flag. Indicate that second rolls over event occurred since the last reset."]
pub struct RTC_EVENT_SEC_R(crate::FieldReader<bool, bool>);
impl RTC_EVENT_SEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_EVENT_SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_EVENT_SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_EVENT_HOS` reader - Hundredths of a second event flag. Indicate that hundredths of a second rolls over event occurred since the last reset."]
pub struct RTC_EVENT_HOS_R(crate::FieldReader<bool, bool>);
impl RTC_EVENT_HOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_EVENT_HOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_EVENT_HOS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - Alarm event flag. Indicate that alarm event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_alrm(&self) -> RTC_EVENT_ALRM_R {
        RTC_EVENT_ALRM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Month rolls over event flag. Indicate that month rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_mnth(&self) -> RTC_EVENT_MNTH_R {
        RTC_EVENT_MNTH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Date rolls over event flag. Indicate that date rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_date(&self) -> RTC_EVENT_DATE_R {
        RTC_EVENT_DATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Hour rolls over event flag. Indicate that hour rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_hour(&self) -> RTC_EVENT_HOUR_R {
        RTC_EVENT_HOUR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Minute rolls over event flag. Indicate that minute rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_min(&self) -> RTC_EVENT_MIN_R {
        RTC_EVENT_MIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Second rolls over event flag. Indicate that second rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_sec(&self) -> RTC_EVENT_SEC_R {
        RTC_EVENT_SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Hundredths of a second event flag. Indicate that hundredths of a second rolls over event occurred since the last reset."]
    #[inline(always)]
    pub fn rtc_event_hos(&self) -> RTC_EVENT_HOS_R {
        RTC_EVENT_HOS_R::new((self.bits & 1) != 0)
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
#[doc = "RTC Event Flags Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_event_flags_reg](index.html) module"]
pub struct RTC_EVENT_FLAGS_REG_SPEC;
impl crate::RegisterSpec for RTC_EVENT_FLAGS_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_event_flags_reg::R](R) reader structure"]
impl crate::Readable for RTC_EVENT_FLAGS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_event_flags_reg::W](W) writer structure"]
impl crate::Writable for RTC_EVENT_FLAGS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_EVENT_FLAGS_REG to value 0"]
impl crate::Resettable for RTC_EVENT_FLAGS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
