#[doc = "Register `RTC_STATUS_REG` reader"]
pub struct R(crate::R<RTC_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_STATUS_REG` writer"]
pub struct W(crate::W<RTC_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_STATUS_REG_SPEC>;
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
impl From<crate::W<RTC_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_VALID_CAL_ALM` reader - Valid Calendar Alarm. If cleared then indicates that invalid entry occurred when writing to Calendar Alarm Register."]
pub struct RTC_VALID_CAL_ALM_R(crate::FieldReader<bool, bool>);
impl RTC_VALID_CAL_ALM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_VALID_CAL_ALM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_VALID_CAL_ALM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_VALID_TIME_ALM` reader - Valid Time Alarm. If cleared then indicates that invalid entry occurred when writing to Time Alarm Register."]
pub struct RTC_VALID_TIME_ALM_R(crate::FieldReader<bool, bool>);
impl RTC_VALID_TIME_ALM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_VALID_TIME_ALM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_VALID_TIME_ALM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_VALID_CAL` reader - Valid Calendar. If cleared then indicates that invalid entry occurred when writing to Calendar Register."]
pub struct RTC_VALID_CAL_R(crate::FieldReader<bool, bool>);
impl RTC_VALID_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_VALID_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_VALID_CAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_VALID_TIME` reader - Valid Time. If cleared then indicates that invalid entry occurred when writing to Time Register."]
pub struct RTC_VALID_TIME_R(crate::FieldReader<bool, bool>);
impl RTC_VALID_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_VALID_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_VALID_TIME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - Valid Calendar Alarm. If cleared then indicates that invalid entry occurred when writing to Calendar Alarm Register."]
    #[inline(always)]
    pub fn rtc_valid_cal_alm(&self) -> RTC_VALID_CAL_ALM_R {
        RTC_VALID_CAL_ALM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Valid Time Alarm. If cleared then indicates that invalid entry occurred when writing to Time Alarm Register."]
    #[inline(always)]
    pub fn rtc_valid_time_alm(&self) -> RTC_VALID_TIME_ALM_R {
        RTC_VALID_TIME_ALM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Valid Calendar. If cleared then indicates that invalid entry occurred when writing to Calendar Register."]
    #[inline(always)]
    pub fn rtc_valid_cal(&self) -> RTC_VALID_CAL_R {
        RTC_VALID_CAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Valid Time. If cleared then indicates that invalid entry occurred when writing to Time Register."]
    #[inline(always)]
    pub fn rtc_valid_time(&self) -> RTC_VALID_TIME_R {
        RTC_VALID_TIME_R::new((self.bits & 1) != 0)
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
#[doc = "RTC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_status_reg](index.html) module"]
pub struct RTC_STATUS_REG_SPEC;
impl crate::RegisterSpec for RTC_STATUS_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_status_reg::R](R) reader structure"]
impl crate::Readable for RTC_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_status_reg::W](W) writer structure"]
impl crate::Writable for RTC_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_STATUS_REG to value 0x0f"]
impl crate::Resettable for RTC_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
