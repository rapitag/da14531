#[doc = "Register `RTC_ALARM_ENABLE_REG` reader"]
pub struct R(crate::R<RTC_ALARM_ENABLE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_ALARM_ENABLE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_ALARM_ENABLE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_ALARM_ENABLE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_ALARM_ENABLE_REG` writer"]
pub struct W(crate::W<RTC_ALARM_ENABLE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_ALARM_ENABLE_REG_SPEC>;
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
impl From<crate::W<RTC_ALARM_ENABLE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_ALARM_ENABLE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_ALARM_MNTH_EN` reader - Alarm on month enable. Enable to trigger alarm when data specified in Calendar Alarm Register (M_T and M_U) has been reached."]
pub struct RTC_ALARM_MNTH_EN_R(crate::FieldReader<bool, bool>);
impl RTC_ALARM_MNTH_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ALARM_MNTH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ALARM_MNTH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ALARM_MNTH_EN` writer - Alarm on month enable. Enable to trigger alarm when data specified in Calendar Alarm Register (M_T and M_U) has been reached."]
pub struct RTC_ALARM_MNTH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_MNTH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `RTC_ALARM_DATE_EN` reader - Alarm on date enable. Enable to trigger alarm when data specified in Calendar Alarm Register (D_T and D_U) has been reached."]
pub struct RTC_ALARM_DATE_EN_R(crate::FieldReader<bool, bool>);
impl RTC_ALARM_DATE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ALARM_DATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ALARM_DATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ALARM_DATE_EN` writer - Alarm on date enable. Enable to trigger alarm when data specified in Calendar Alarm Register (D_T and D_U) has been reached."]
pub struct RTC_ALARM_DATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_DATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `RTC_ALARM_HOUR_EN` reader - Alarm on hour enable. Enable to trigger alarm when data specified in Time Alarm Register (PM, HR_T and HR_U) has been reached."]
pub struct RTC_ALARM_HOUR_EN_R(crate::FieldReader<bool, bool>);
impl RTC_ALARM_HOUR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ALARM_HOUR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ALARM_HOUR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ALARM_HOUR_EN` writer - Alarm on hour enable. Enable to trigger alarm when data specified in Time Alarm Register (PM, HR_T and HR_U) has been reached."]
pub struct RTC_ALARM_HOUR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_HOUR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `RTC_ALARM_MIN_EN` reader - Alarm on minute enable. Enable to trigger alarm when data specified in Time Alarm Register (M_T and M_U) has been reached."]
pub struct RTC_ALARM_MIN_EN_R(crate::FieldReader<bool, bool>);
impl RTC_ALARM_MIN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ALARM_MIN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ALARM_MIN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ALARM_MIN_EN` writer - Alarm on minute enable. Enable to trigger alarm when data specified in Time Alarm Register (M_T and M_U) has been reached."]
pub struct RTC_ALARM_MIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_MIN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RTC_ALARM_SEC_EN` reader - Alarm on second enable. Enable to trigger alarm when data specified in Time Alarm Register (S_T and S_U) has been reached."]
pub struct RTC_ALARM_SEC_EN_R(crate::FieldReader<bool, bool>);
impl RTC_ALARM_SEC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ALARM_SEC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ALARM_SEC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ALARM_SEC_EN` writer - Alarm on second enable. Enable to trigger alarm when data specified in Time Alarm Register (S_T and S_U) has been reached."]
pub struct RTC_ALARM_SEC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_SEC_EN_W<'a> {
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
#[doc = "Field `RTC_ALARM_HOS_EN` reader - Alarm on hundredths of a second enable. Enable to trigger alarm when data specified in Time Alarm Register (H_T and H_U) has been reached."]
pub struct RTC_ALARM_HOS_EN_R(crate::FieldReader<bool, bool>);
impl RTC_ALARM_HOS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ALARM_HOS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ALARM_HOS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ALARM_HOS_EN` writer - Alarm on hundredths of a second enable. Enable to trigger alarm when data specified in Time Alarm Register (H_T and H_U) has been reached."]
pub struct RTC_ALARM_HOS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_HOS_EN_W<'a> {
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
    #[doc = "Bit 5 - Alarm on month enable. Enable to trigger alarm when data specified in Calendar Alarm Register (M_T and M_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_mnth_en(&self) -> RTC_ALARM_MNTH_EN_R {
        RTC_ALARM_MNTH_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Alarm on date enable. Enable to trigger alarm when data specified in Calendar Alarm Register (D_T and D_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_date_en(&self) -> RTC_ALARM_DATE_EN_R {
        RTC_ALARM_DATE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Alarm on hour enable. Enable to trigger alarm when data specified in Time Alarm Register (PM, HR_T and HR_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_hour_en(&self) -> RTC_ALARM_HOUR_EN_R {
        RTC_ALARM_HOUR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Alarm on minute enable. Enable to trigger alarm when data specified in Time Alarm Register (M_T and M_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_min_en(&self) -> RTC_ALARM_MIN_EN_R {
        RTC_ALARM_MIN_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm on second enable. Enable to trigger alarm when data specified in Time Alarm Register (S_T and S_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_sec_en(&self) -> RTC_ALARM_SEC_EN_R {
        RTC_ALARM_SEC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Alarm on hundredths of a second enable. Enable to trigger alarm when data specified in Time Alarm Register (H_T and H_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_hos_en(&self) -> RTC_ALARM_HOS_EN_R {
        RTC_ALARM_HOS_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Alarm on month enable. Enable to trigger alarm when data specified in Calendar Alarm Register (M_T and M_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_mnth_en(&mut self) -> RTC_ALARM_MNTH_EN_W {
        RTC_ALARM_MNTH_EN_W { w: self }
    }
    #[doc = "Bit 4 - Alarm on date enable. Enable to trigger alarm when data specified in Calendar Alarm Register (D_T and D_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_date_en(&mut self) -> RTC_ALARM_DATE_EN_W {
        RTC_ALARM_DATE_EN_W { w: self }
    }
    #[doc = "Bit 3 - Alarm on hour enable. Enable to trigger alarm when data specified in Time Alarm Register (PM, HR_T and HR_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_hour_en(&mut self) -> RTC_ALARM_HOUR_EN_W {
        RTC_ALARM_HOUR_EN_W { w: self }
    }
    #[doc = "Bit 2 - Alarm on minute enable. Enable to trigger alarm when data specified in Time Alarm Register (M_T and M_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_min_en(&mut self) -> RTC_ALARM_MIN_EN_W {
        RTC_ALARM_MIN_EN_W { w: self }
    }
    #[doc = "Bit 1 - Alarm on second enable. Enable to trigger alarm when data specified in Time Alarm Register (S_T and S_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_sec_en(&mut self) -> RTC_ALARM_SEC_EN_W {
        RTC_ALARM_SEC_EN_W { w: self }
    }
    #[doc = "Bit 0 - Alarm on hundredths of a second enable. Enable to trigger alarm when data specified in Time Alarm Register (H_T and H_U) has been reached."]
    #[inline(always)]
    pub fn rtc_alarm_hos_en(&mut self) -> RTC_ALARM_HOS_EN_W {
        RTC_ALARM_HOS_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Alarm Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_alarm_enable_reg](index.html) module"]
pub struct RTC_ALARM_ENABLE_REG_SPEC;
impl crate::RegisterSpec for RTC_ALARM_ENABLE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_alarm_enable_reg::R](R) reader structure"]
impl crate::Readable for RTC_ALARM_ENABLE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_alarm_enable_reg::W](W) writer structure"]
impl crate::Writable for RTC_ALARM_ENABLE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_ALARM_ENABLE_REG to value 0"]
impl crate::Resettable for RTC_ALARM_ENABLE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
