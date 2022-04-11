#[doc = "Register `PMU_SLEEP_REG` reader"]
pub struct R(crate::R<PMU_SLEEP_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMU_SLEEP_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMU_SLEEP_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMU_SLEEP_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMU_SLEEP_REG` writer"]
pub struct W(crate::W<PMU_SLEEP_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMU_SLEEP_REG_SPEC>;
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
impl From<crate::W<PMU_SLEEP_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMU_SLEEP_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BG_REFRESH_INTERVAL` reader - Defines the refresh interval of reference voltages (bandgap activation and sampling), in units of 2ms."]
pub struct BG_REFRESH_INTERVAL_R(crate::FieldReader<u16, u16>);
impl BG_REFRESH_INTERVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BG_REFRESH_INTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BG_REFRESH_INTERVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BG_REFRESH_INTERVAL` writer - Defines the refresh interval of reference voltages (bandgap activation and sampling), in units of 2ms."]
pub struct BG_REFRESH_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BG_REFRESH_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u16 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Defines the refresh interval of reference voltages (bandgap activation and sampling), in units of 2ms."]
    #[inline(always)]
    pub fn bg_refresh_interval(&self) -> BG_REFRESH_INTERVAL_R {
        BG_REFRESH_INTERVAL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Defines the refresh interval of reference voltages (bandgap activation and sampling), in units of 2ms."]
    #[inline(always)]
    pub fn bg_refresh_interval(&mut self) -> BG_REFRESH_INTERVAL_W {
        BG_REFRESH_INTERVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bandgap refresh interval during sleep\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmu_sleep_reg](index.html) module"]
pub struct PMU_SLEEP_REG_SPEC;
impl crate::RegisterSpec for PMU_SLEEP_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmu_sleep_reg::R](R) reader structure"]
impl crate::Readable for PMU_SLEEP_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmu_sleep_reg::W](W) writer structure"]
impl crate::Writable for PMU_SLEEP_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMU_SLEEP_REG to value 0x80"]
impl crate::Resettable for PMU_SLEEP_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
