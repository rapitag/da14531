#[doc = "Register `BLE_TIMGENCNTL_REG` reader"]
pub struct R(crate::R<BLE_TIMGENCNTL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_TIMGENCNTL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_TIMGENCNTL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_TIMGENCNTL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_TIMGENCNTL_REG` writer"]
pub struct W(crate::W<BLE_TIMGENCNTL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_TIMGENCNTL_REG_SPEC>;
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
impl From<crate::W<BLE_TIMGENCNTL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_TIMGENCNTL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APFM_EN` reader - Controls the Anticipated pre-Fetch Abort mechanism 0: Disabled 1: Enabled"]
pub struct APFM_EN_R(crate::FieldReader<bool, bool>);
impl APFM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APFM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APFM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APFM_EN` writer - Controls the Anticipated pre-Fetch Abort mechanism 0: Disabled 1: Enabled"]
pub struct APFM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APFM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Field `PREFETCHABORT_TIME` reader - Defines the instant in usec at which immediate abort is required after anticipated pre-fetch abort."]
pub struct PREFETCHABORT_TIME_R(crate::FieldReader<u16, u16>);
impl PREFETCHABORT_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PREFETCHABORT_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREFETCHABORT_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREFETCHABORT_TIME` writer - Defines the instant in usec at which immediate abort is required after anticipated pre-fetch abort."]
pub struct PREFETCHABORT_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCHABORT_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `PREFETCH_TIME` reader - Defines Exchange Table pre-fetch instant in us"]
pub struct PREFETCH_TIME_R(crate::FieldReader<u16, u16>);
impl PREFETCH_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PREFETCH_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREFETCH_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREFETCH_TIME` writer - Defines Exchange Table pre-fetch instant in us"]
pub struct PREFETCH_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCH_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Controls the Anticipated pre-Fetch Abort mechanism 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn apfm_en(&self) -> APFM_EN_R {
        APFM_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Defines the instant in usec at which immediate abort is required after anticipated pre-fetch abort."]
    #[inline(always)]
    pub fn prefetchabort_time(&self) -> PREFETCHABORT_TIME_R {
        PREFETCHABORT_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:8 - Defines Exchange Table pre-fetch instant in us"]
    #[inline(always)]
    pub fn prefetch_time(&self) -> PREFETCH_TIME_R {
        PREFETCH_TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Controls the Anticipated pre-Fetch Abort mechanism 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn apfm_en(&mut self) -> APFM_EN_W {
        APFM_EN_W { w: self }
    }
    #[doc = "Bits 16:25 - Defines the instant in usec at which immediate abort is required after anticipated pre-fetch abort."]
    #[inline(always)]
    pub fn prefetchabort_time(&mut self) -> PREFETCHABORT_TIME_W {
        PREFETCHABORT_TIME_W { w: self }
    }
    #[doc = "Bits 0:8 - Defines Exchange Table pre-fetch instant in us"]
    #[inline(always)]
    pub fn prefetch_time(&mut self) -> PREFETCH_TIME_W {
        PREFETCH_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_timgencntl_reg](index.html) module"]
pub struct BLE_TIMGENCNTL_REG_SPEC;
impl crate::RegisterSpec for BLE_TIMGENCNTL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_timgencntl_reg::R](R) reader structure"]
impl crate::Readable for BLE_TIMGENCNTL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_timgencntl_reg::W](W) writer structure"]
impl crate::Writable for BLE_TIMGENCNTL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_TIMGENCNTL_REG to value 0x7fff_ffff"]
impl crate::Resettable for BLE_TIMGENCNTL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_ffff
    }
}
