#[doc = "Register `RTC_INTERRUPT_MASK_REG` reader"]
pub struct R(crate::R<RTC_INTERRUPT_MASK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_INTERRUPT_MASK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_INTERRUPT_MASK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_INTERRUPT_MASK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_INTERRUPT_MASK_REG` writer"]
pub struct W(crate::W<RTC_INTERRUPT_MASK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_INTERRUPT_MASK_REG_SPEC>;
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
impl From<crate::W<RTC_INTERRUPT_MASK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_INTERRUPT_MASK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_ALRM_INT_MSK` reader - Mask alarm interrupt. It can be cleared (set) by setting corresponding bit (ALRM) in Interrupt Enable Register (Interrupt Disable Register)."]
pub struct RTC_ALRM_INT_MSK_R(crate::FieldReader<bool, bool>);
impl RTC_ALRM_INT_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ALRM_INT_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ALRM_INT_MSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MNTH_INT_MSK` reader - Mask month interrupt. It can be cleared (set) by setting corresponding bit (MNTH) in Interrupt Enable Register (Interrupt Disable Register)."]
pub struct RTC_MNTH_INT_MSK_R(crate::FieldReader<bool, bool>);
impl RTC_MNTH_INT_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MNTH_INT_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MNTH_INT_MSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DATE_INT_MSK` reader - Mask date interrupt. It can be cleared (set) by setting corresponding bit (DATE) in Interrupt Enable Register (Interrupt Disable Register)."]
pub struct RTC_DATE_INT_MSK_R(crate::FieldReader<bool, bool>);
impl RTC_DATE_INT_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_DATE_INT_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DATE_INT_MSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_HOUR_INT_MSK` reader - Mask hour interrupt. It can be cleared (set) by setting corresponding bit (HOUR) in Interrupt Enable Register (Interrupt Disable Register)."]
pub struct RTC_HOUR_INT_MSK_R(crate::FieldReader<bool, bool>);
impl RTC_HOUR_INT_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_HOUR_INT_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_HOUR_INT_MSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_MIN_INT_MSK` reader - Mask minute interrupt. It can be cleared (set) by setting corresponding bit (MIN) in Interrupt Enable Register (Interrupt Disable Register)."]
pub struct RTC_MIN_INT_MSK_R(crate::FieldReader<bool, bool>);
impl RTC_MIN_INT_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MIN_INT_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_MIN_INT_MSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SEC_INT_MSK` reader - Mask second interrupt. It can be cleared (set) by setting corresponding bit (SEC) in Interrupt Enable Register (Interrupt Disable Register)."]
pub struct RTC_SEC_INT_MSK_R(crate::FieldReader<bool, bool>);
impl RTC_SEC_INT_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_SEC_INT_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SEC_INT_MSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_HOS_INT_MSK` reader - Mask hundredths of a second interrupt. It can be cleared (set) by setting corresponding bit (HOS) in Interrupt Enable Register (Interrupt Disable Register)."]
pub struct RTC_HOS_INT_MSK_R(crate::FieldReader<bool, bool>);
impl RTC_HOS_INT_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_HOS_INT_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_HOS_INT_MSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - Mask alarm interrupt. It can be cleared (set) by setting corresponding bit (ALRM) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_alrm_int_msk(&self) -> RTC_ALRM_INT_MSK_R {
        RTC_ALRM_INT_MSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask month interrupt. It can be cleared (set) by setting corresponding bit (MNTH) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_mnth_int_msk(&self) -> RTC_MNTH_INT_MSK_R {
        RTC_MNTH_INT_MSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask date interrupt. It can be cleared (set) by setting corresponding bit (DATE) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_date_int_msk(&self) -> RTC_DATE_INT_MSK_R {
        RTC_DATE_INT_MSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask hour interrupt. It can be cleared (set) by setting corresponding bit (HOUR) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_hour_int_msk(&self) -> RTC_HOUR_INT_MSK_R {
        RTC_HOUR_INT_MSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask minute interrupt. It can be cleared (set) by setting corresponding bit (MIN) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_min_int_msk(&self) -> RTC_MIN_INT_MSK_R {
        RTC_MIN_INT_MSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Mask second interrupt. It can be cleared (set) by setting corresponding bit (SEC) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_sec_int_msk(&self) -> RTC_SEC_INT_MSK_R {
        RTC_SEC_INT_MSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Mask hundredths of a second interrupt. It can be cleared (set) by setting corresponding bit (HOS) in Interrupt Enable Register (Interrupt Disable Register)."]
    #[inline(always)]
    pub fn rtc_hos_int_msk(&self) -> RTC_HOS_INT_MSK_R {
        RTC_HOS_INT_MSK_R::new((self.bits & 1) != 0)
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
#[doc = "RTC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_interrupt_mask_reg](index.html) module"]
pub struct RTC_INTERRUPT_MASK_REG_SPEC;
impl crate::RegisterSpec for RTC_INTERRUPT_MASK_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_interrupt_mask_reg::R](R) reader structure"]
impl crate::Readable for RTC_INTERRUPT_MASK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_interrupt_mask_reg::W](W) writer structure"]
impl crate::Writable for RTC_INTERRUPT_MASK_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_INTERRUPT_MASK_REG to value 0x7f"]
impl crate::Resettable for RTC_INTERRUPT_MASK_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
