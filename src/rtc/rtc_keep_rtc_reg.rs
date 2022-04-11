#[doc = "Register `RTC_KEEP_RTC_REG` reader"]
pub struct R(crate::R<RTC_KEEP_RTC_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_KEEP_RTC_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_KEEP_RTC_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_KEEP_RTC_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_KEEP_RTC_REG` writer"]
pub struct W(crate::W<RTC_KEEP_RTC_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_KEEP_RTC_REG_SPEC>;
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
impl From<crate::W<RTC_KEEP_RTC_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_KEEP_RTC_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_KEEP` reader - Keep RTC. When high, the time and calendar registers and any other registers which directly affect or are affected by the time and calendar registers are NOT reset when software reset is applied. When low, the software reset will reset every register except the keep RTC and control registers."]
pub struct RTC_KEEP_R(crate::FieldReader<bool, bool>);
impl RTC_KEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_KEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_KEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_KEEP` writer - Keep RTC. When high, the time and calendar registers and any other registers which directly affect or are affected by the time and calendar registers are NOT reset when software reset is applied. When low, the software reset will reset every register except the keep RTC and control registers."]
pub struct RTC_KEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_KEEP_W<'a> {
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
    #[doc = "Bit 0 - Keep RTC. When high, the time and calendar registers and any other registers which directly affect or are affected by the time and calendar registers are NOT reset when software reset is applied. When low, the software reset will reset every register except the keep RTC and control registers."]
    #[inline(always)]
    pub fn rtc_keep(&self) -> RTC_KEEP_R {
        RTC_KEEP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RTC. When high, the time and calendar registers and any other registers which directly affect or are affected by the time and calendar registers are NOT reset when software reset is applied. When low, the software reset will reset every register except the keep RTC and control registers."]
    #[inline(always)]
    pub fn rtc_keep(&mut self) -> RTC_KEEP_W {
        RTC_KEEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Keep RTC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_keep_rtc_reg](index.html) module"]
pub struct RTC_KEEP_RTC_REG_SPEC;
impl crate::RegisterSpec for RTC_KEEP_RTC_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_keep_rtc_reg::R](R) reader structure"]
impl crate::Readable for RTC_KEEP_RTC_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_keep_rtc_reg::W](W) writer structure"]
impl crate::Writable for RTC_KEEP_RTC_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_KEEP_RTC_REG to value 0x01"]
impl crate::Resettable for RTC_KEEP_RTC_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
