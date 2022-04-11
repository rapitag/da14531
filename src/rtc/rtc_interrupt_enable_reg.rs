#[doc = "Register `RTC_INTERRUPT_ENABLE_REG` reader"]
pub struct R(crate::R<RTC_INTERRUPT_ENABLE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_INTERRUPT_ENABLE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_INTERRUPT_ENABLE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_INTERRUPT_ENABLE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_INTERRUPT_ENABLE_REG` writer"]
pub struct W(crate::W<RTC_INTERRUPT_ENABLE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_INTERRUPT_ENABLE_REG_SPEC>;
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
impl From<crate::W<RTC_INTERRUPT_ENABLE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_INTERRUPT_ENABLE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_ALRM_INT_EN` writer - Interrupt on alarm enable. Enable to issue the interrupt when alarm event occurred."]
pub struct RTC_ALRM_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALRM_INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `RTC_MNTH_INT_EN` writer - Interrupt on month enable. Enable to issue the interrupt when month event occurred."]
pub struct RTC_MNTH_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MNTH_INT_EN_W<'a> {
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
#[doc = "Field `RTC_DATE_INT_EN` writer - Interrupt on date enable. Enable to issue the interrupt when date event occurred."]
pub struct RTC_DATE_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DATE_INT_EN_W<'a> {
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
#[doc = "Field `RTC_HOUR_INT_EN` writer - Interrupt on hour enable. Enable to issue the interrupt when hour event occurred."]
pub struct RTC_HOUR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_HOUR_INT_EN_W<'a> {
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
#[doc = "Field `RTC_MIN_INT_EN` writer - Interrupt on minute enable. Enable to issue the interrupt when minute event occurred."]
pub struct RTC_MIN_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MIN_INT_EN_W<'a> {
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
#[doc = "Field `RTC_SEC_INT_EN` writer - Interrupt on second enable. Enable to issue the interrupt when second event occurred."]
pub struct RTC_SEC_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SEC_INT_EN_W<'a> {
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
#[doc = "Field `RTC_HOS_INT_EN` writer - Interrupt on hundredths of a second enable. Enable to issue the interrupt when hundredths of a second event occurred."]
pub struct RTC_HOS_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_HOS_INT_EN_W<'a> {
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
impl W {
    #[doc = "Bit 6 - Interrupt on alarm enable. Enable to issue the interrupt when alarm event occurred."]
    #[inline(always)]
    pub fn rtc_alrm_int_en(&mut self) -> RTC_ALRM_INT_EN_W {
        RTC_ALRM_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt on month enable. Enable to issue the interrupt when month event occurred."]
    #[inline(always)]
    pub fn rtc_mnth_int_en(&mut self) -> RTC_MNTH_INT_EN_W {
        RTC_MNTH_INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt on date enable. Enable to issue the interrupt when date event occurred."]
    #[inline(always)]
    pub fn rtc_date_int_en(&mut self) -> RTC_DATE_INT_EN_W {
        RTC_DATE_INT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt on hour enable. Enable to issue the interrupt when hour event occurred."]
    #[inline(always)]
    pub fn rtc_hour_int_en(&mut self) -> RTC_HOUR_INT_EN_W {
        RTC_HOUR_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt on minute enable. Enable to issue the interrupt when minute event occurred."]
    #[inline(always)]
    pub fn rtc_min_int_en(&mut self) -> RTC_MIN_INT_EN_W {
        RTC_MIN_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt on second enable. Enable to issue the interrupt when second event occurred."]
    #[inline(always)]
    pub fn rtc_sec_int_en(&mut self) -> RTC_SEC_INT_EN_W {
        RTC_SEC_INT_EN_W { w: self }
    }
    #[doc = "Bit 0 - Interrupt on hundredths of a second enable. Enable to issue the interrupt when hundredths of a second event occurred."]
    #[inline(always)]
    pub fn rtc_hos_int_en(&mut self) -> RTC_HOS_INT_EN_W {
        RTC_HOS_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_interrupt_enable_reg](index.html) module"]
pub struct RTC_INTERRUPT_ENABLE_REG_SPEC;
impl crate::RegisterSpec for RTC_INTERRUPT_ENABLE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_interrupt_enable_reg::R](R) reader structure"]
impl crate::Readable for RTC_INTERRUPT_ENABLE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_interrupt_enable_reg::W](W) writer structure"]
impl crate::Writable for RTC_INTERRUPT_ENABLE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_INTERRUPT_ENABLE_REG to value 0"]
impl crate::Resettable for RTC_INTERRUPT_ENABLE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
