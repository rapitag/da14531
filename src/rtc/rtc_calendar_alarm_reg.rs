#[doc = "Register `RTC_CALENDAR_ALARM_REG` reader"]
pub struct R(crate::R<RTC_CALENDAR_ALARM_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CALENDAR_ALARM_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CALENDAR_ALARM_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CALENDAR_ALARM_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CALENDAR_ALARM_REG` writer"]
pub struct W(crate::W<RTC_CALENDAR_ALARM_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CALENDAR_ALARM_REG_SPEC>;
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
impl From<crate::W<RTC_CALENDAR_ALARM_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CALENDAR_ALARM_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CAL_D_T` reader - Date tens. Represented in BCD digit (0-3)."]
pub struct RTC_CAL_D_T_R(crate::FieldReader<u8, u8>);
impl RTC_CAL_D_T_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_CAL_D_T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CAL_D_T_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CAL_D_T` writer - Date tens. Represented in BCD digit (0-3)."]
pub struct RTC_CAL_D_T_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CAL_D_T_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `RTC_CAL_D_U` reader - Date units. Represented in BCD digit (0-9)."]
pub struct RTC_CAL_D_U_R(crate::FieldReader<u8, u8>);
impl RTC_CAL_D_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_CAL_D_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CAL_D_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CAL_D_U` writer - Date units. Represented in BCD digit (0-9)."]
pub struct RTC_CAL_D_U_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CAL_D_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RTC_CAL_M_T` reader - Month tens. Represented in BCD digit (0-1)."]
pub struct RTC_CAL_M_T_R(crate::FieldReader<bool, bool>);
impl RTC_CAL_M_T_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CAL_M_T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CAL_M_T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CAL_M_T` writer - Month tens. Represented in BCD digit (0-1)."]
pub struct RTC_CAL_M_T_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CAL_M_T_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `RTC_CAL_M_U` reader - Month units. Represented in BCD digit (0-9)."]
pub struct RTC_CAL_M_U_R(crate::FieldReader<u8, u8>);
impl RTC_CAL_M_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_CAL_M_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CAL_M_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_CAL_M_U` writer - Month units. Represented in BCD digit (0-9)."]
pub struct RTC_CAL_M_U_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CAL_M_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13 - Date tens. Represented in BCD digit (0-3)."]
    #[inline(always)]
    pub fn rtc_cal_d_t(&self) -> RTC_CAL_D_T_R {
        RTC_CAL_D_T_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Date units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_d_u(&self) -> RTC_CAL_D_U_R {
        RTC_CAL_D_U_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Month tens. Represented in BCD digit (0-1)."]
    #[inline(always)]
    pub fn rtc_cal_m_t(&self) -> RTC_CAL_M_T_R {
        RTC_CAL_M_T_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Month units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_m_u(&self) -> RTC_CAL_M_U_R {
        RTC_CAL_M_U_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Date tens. Represented in BCD digit (0-3)."]
    #[inline(always)]
    pub fn rtc_cal_d_t(&mut self) -> RTC_CAL_D_T_W {
        RTC_CAL_D_T_W { w: self }
    }
    #[doc = "Bits 8:11 - Date units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_d_u(&mut self) -> RTC_CAL_D_U_W {
        RTC_CAL_D_U_W { w: self }
    }
    #[doc = "Bit 7 - Month tens. Represented in BCD digit (0-1)."]
    #[inline(always)]
    pub fn rtc_cal_m_t(&mut self) -> RTC_CAL_M_T_W {
        RTC_CAL_M_T_W { w: self }
    }
    #[doc = "Bits 3:6 - Month units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_cal_m_u(&mut self) -> RTC_CAL_M_U_W {
        RTC_CAL_M_U_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Calendar Alram Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_calendar_alarm_reg](index.html) module"]
pub struct RTC_CALENDAR_ALARM_REG_SPEC;
impl crate::RegisterSpec for RTC_CALENDAR_ALARM_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_calendar_alarm_reg::R](R) reader structure"]
impl crate::Readable for RTC_CALENDAR_ALARM_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_calendar_alarm_reg::W](W) writer structure"]
impl crate::Writable for RTC_CALENDAR_ALARM_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CALENDAR_ALARM_REG to value 0"]
impl crate::Resettable for RTC_CALENDAR_ALARM_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
