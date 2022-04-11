#[doc = "Register `BLE_FINECNTCORR_REG` reader"]
pub struct R(crate::R<BLE_FINECNTCORR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_FINECNTCORR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_FINECNTCORR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_FINECNTCORR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_FINECNTCORR_REG` writer"]
pub struct W(crate::W<BLE_FINECNTCORR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_FINECNTCORR_REG_SPEC>;
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
impl From<crate::W<BLE_FINECNTCORR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_FINECNTCORR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINECNTCORR` reader - Phase correction value for the 625us reference counter (i.e. Fine Counter) in us."]
pub struct FINECNTCORR_R(crate::FieldReader<u16, u16>);
impl FINECNTCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FINECNTCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINECNTCORR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINECNTCORR` writer - Phase correction value for the 625us reference counter (i.e. Fine Counter) in us."]
pub struct FINECNTCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> FINECNTCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Phase correction value for the 625us reference counter (i.e. Fine Counter) in us."]
    #[inline(always)]
    pub fn finecntcorr(&self) -> FINECNTCORR_R {
        FINECNTCORR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Phase correction value for the 625us reference counter (i.e. Fine Counter) in us."]
    #[inline(always)]
    pub fn finecntcorr(&mut self) -> FINECNTCORR_W {
        FINECNTCORR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase correction value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_finecntcorr_reg](index.html) module"]
pub struct BLE_FINECNTCORR_REG_SPEC;
impl crate::RegisterSpec for BLE_FINECNTCORR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_finecntcorr_reg::R](R) reader structure"]
impl crate::Readable for BLE_FINECNTCORR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_finecntcorr_reg::W](W) writer structure"]
impl crate::Writable for BLE_FINECNTCORR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_FINECNTCORR_REG to value 0"]
impl crate::Resettable for BLE_FINECNTCORR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
