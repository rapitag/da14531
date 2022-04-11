#[doc = "Register `RTC_TIME_ALARM_REG` reader"]
pub struct R(crate::R<RTC_TIME_ALARM_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_ALARM_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIME_ALARM_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIME_ALARM_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIME_ALARM_REG` writer"]
pub struct W(crate::W<RTC_TIME_ALARM_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIME_ALARM_REG_SPEC>;
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
impl From<crate::W<RTC_TIME_ALARM_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIME_ALARM_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_TIME_PM` reader - In 12 hour clock mode, indicates PM when set."]
pub struct RTC_TIME_PM_R(crate::FieldReader<bool, bool>);
impl RTC_TIME_PM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TIME_PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_PM` writer - In 12 hour clock mode, indicates PM when set."]
pub struct RTC_TIME_PM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_PM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `RTC_TIME_HR_T` reader - Hours tens. Represented in BCD digit (0-2)."]
pub struct RTC_TIME_HR_T_R(crate::FieldReader<u8, u8>);
impl RTC_TIME_HR_T_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_TIME_HR_T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_HR_T_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_HR_T` writer - Hours tens. Represented in BCD digit (0-2)."]
pub struct RTC_TIME_HR_T_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_HR_T_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `RTC_TIME_HR_U` reader - Hours units. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_HR_U_R(crate::FieldReader<u8, u8>);
impl RTC_TIME_HR_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_TIME_HR_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_HR_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_HR_U` writer - Hours units. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_HR_U_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_HR_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `RTC_TIME_M_T` reader - Minutes tens. Represented in BCD digit (0-5)."]
pub struct RTC_TIME_M_T_R(crate::FieldReader<u8, u8>);
impl RTC_TIME_M_T_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_TIME_M_T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_M_T_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_M_T` writer - Minutes tens. Represented in BCD digit (0-5)."]
pub struct RTC_TIME_M_T_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_M_T_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 20)) | ((value as u32 & 7) << 20);
        self.w
    }
}
#[doc = "Field `RTC_TIME_M_U` reader - Minutes units. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_M_U_R(crate::FieldReader<u8, u8>);
impl RTC_TIME_M_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_TIME_M_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_M_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_M_U` writer - Minutes units. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_M_U_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_M_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RTC_TIME_S_T` reader - Seconds tens. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_S_T_R(crate::FieldReader<u8, u8>);
impl RTC_TIME_S_T_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_TIME_S_T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_S_T_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_S_T` writer - Seconds tens. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_S_T_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_S_T_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 12)) | ((value as u32 & 7) << 12);
        self.w
    }
}
#[doc = "Field `RTC_TIME_S_U` reader - Seconds units. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_S_U_R(crate::FieldReader<u8, u8>);
impl RTC_TIME_S_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_TIME_S_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_S_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_S_U` writer - Seconds units. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_S_U_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_S_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RTC_TIME_H_T` reader - Hundredths of a second tens. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_H_T_R(crate::FieldReader<u8, u8>);
impl RTC_TIME_H_T_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_TIME_H_T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_H_T_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_H_T` writer - Hundredths of a second tens. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_H_T_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_H_T_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `RTC_TIME_H_U` reader - Hundredths of a second units. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_H_U_R(crate::FieldReader<u8, u8>);
impl RTC_TIME_H_U_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTC_TIME_H_U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_H_U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_TIME_H_U` writer - Hundredths of a second units. Represented in BCD digit (0-9)."]
pub struct RTC_TIME_H_U_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_H_U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - In 12 hour clock mode, indicates PM when set."]
    #[inline(always)]
    pub fn rtc_time_pm(&self) -> RTC_TIME_PM_R {
        RTC_TIME_PM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Hours tens. Represented in BCD digit (0-2)."]
    #[inline(always)]
    pub fn rtc_time_hr_t(&self) -> RTC_TIME_HR_T_R {
        RTC_TIME_HR_T_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Hours units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_hr_u(&self) -> RTC_TIME_HR_U_R {
        RTC_TIME_HR_U_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Minutes tens. Represented in BCD digit (0-5)."]
    #[inline(always)]
    pub fn rtc_time_m_t(&self) -> RTC_TIME_M_T_R {
        RTC_TIME_M_T_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Minutes units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_m_u(&self) -> RTC_TIME_M_U_R {
        RTC_TIME_M_U_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Seconds tens. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_s_t(&self) -> RTC_TIME_S_T_R {
        RTC_TIME_S_T_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Seconds units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_s_u(&self) -> RTC_TIME_S_U_R {
        RTC_TIME_S_U_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Hundredths of a second tens. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_h_t(&self) -> RTC_TIME_H_T_R {
        RTC_TIME_H_T_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Hundredths of a second units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_h_u(&self) -> RTC_TIME_H_U_R {
        RTC_TIME_H_U_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - In 12 hour clock mode, indicates PM when set."]
    #[inline(always)]
    pub fn rtc_time_pm(&mut self) -> RTC_TIME_PM_W {
        RTC_TIME_PM_W { w: self }
    }
    #[doc = "Bits 28:29 - Hours tens. Represented in BCD digit (0-2)."]
    #[inline(always)]
    pub fn rtc_time_hr_t(&mut self) -> RTC_TIME_HR_T_W {
        RTC_TIME_HR_T_W { w: self }
    }
    #[doc = "Bits 24:27 - Hours units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_hr_u(&mut self) -> RTC_TIME_HR_U_W {
        RTC_TIME_HR_U_W { w: self }
    }
    #[doc = "Bits 20:22 - Minutes tens. Represented in BCD digit (0-5)."]
    #[inline(always)]
    pub fn rtc_time_m_t(&mut self) -> RTC_TIME_M_T_W {
        RTC_TIME_M_T_W { w: self }
    }
    #[doc = "Bits 16:19 - Minutes units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_m_u(&mut self) -> RTC_TIME_M_U_W {
        RTC_TIME_M_U_W { w: self }
    }
    #[doc = "Bits 12:14 - Seconds tens. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_s_t(&mut self) -> RTC_TIME_S_T_W {
        RTC_TIME_S_T_W { w: self }
    }
    #[doc = "Bits 8:11 - Seconds units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_s_u(&mut self) -> RTC_TIME_S_U_W {
        RTC_TIME_S_U_W { w: self }
    }
    #[doc = "Bits 4:7 - Hundredths of a second tens. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_h_t(&mut self) -> RTC_TIME_H_T_W {
        RTC_TIME_H_T_W { w: self }
    }
    #[doc = "Bits 0:3 - Hundredths of a second units. Represented in BCD digit (0-9)."]
    #[inline(always)]
    pub fn rtc_time_h_u(&mut self) -> RTC_TIME_H_U_W {
        RTC_TIME_H_U_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_alarm_reg](index.html) module"]
pub struct RTC_TIME_ALARM_REG_SPEC;
impl crate::RegisterSpec for RTC_TIME_ALARM_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time_alarm_reg::R](R) reader structure"]
impl crate::Readable for RTC_TIME_ALARM_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_time_alarm_reg::W](W) writer structure"]
impl crate::Writable for RTC_TIME_ALARM_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIME_ALARM_REG to value 0"]
impl crate::Resettable for RTC_TIME_ALARM_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
