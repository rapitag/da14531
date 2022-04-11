#[doc = "Register `RTC_CONTROL_REG` reader"]
pub struct R(crate::R<RTC_CONTROL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CONTROL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CONTROL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CONTROL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CONTROL_REG` writer"]
pub struct W(crate::W<RTC_CONTROL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CONTROL_REG_SPEC>;
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
impl From<crate::W<RTC_CONTROL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CONTROL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CAL_DISABLE` reader - When this field is set high the RTC stops incrementing the calendar value."]
pub struct RTC_CAL_DISABLE_R(crate::FieldReader<bool, bool>);
impl RTC_CAL_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CAL_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CAL_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CAL_DISABLE` writer - When this field is set high the RTC stops incrementing the calendar value."]
pub struct RTC_CAL_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CAL_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RTC_TIME_DISABLE` reader - When this field is set high the RTC stops incrementing the time value."]
pub struct RTC_TIME_DISABLE_R(crate::FieldReader<bool, bool>);
impl RTC_TIME_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TIME_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_DISABLE` writer - When this field is set high the RTC stops incrementing the time value."]
pub struct RTC_TIME_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - When this field is set high the RTC stops incrementing the calendar value."]
    #[inline(always)]
    pub fn rtc_cal_disable(&self) -> RTC_CAL_DISABLE_R {
        RTC_CAL_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - When this field is set high the RTC stops incrementing the time value."]
    #[inline(always)]
    pub fn rtc_time_disable(&self) -> RTC_TIME_DISABLE_R {
        RTC_TIME_DISABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When this field is set high the RTC stops incrementing the calendar value."]
    #[inline(always)]
    pub fn rtc_cal_disable(&mut self) -> RTC_CAL_DISABLE_W {
        RTC_CAL_DISABLE_W { w: self }
    }
    #[doc = "Bit 0 - When this field is set high the RTC stops incrementing the time value."]
    #[inline(always)]
    pub fn rtc_time_disable(&mut self) -> RTC_TIME_DISABLE_W {
        RTC_TIME_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_control_reg](index.html) module"]
pub struct RTC_CONTROL_REG_SPEC;
impl crate::RegisterSpec for RTC_CONTROL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_control_reg::R](R) reader structure"]
impl crate::Readable for RTC_CONTROL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_control_reg::W](W) writer structure"]
impl crate::Writable for RTC_CONTROL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CONTROL_REG to value 0x03"]
impl crate::Resettable for RTC_CONTROL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
