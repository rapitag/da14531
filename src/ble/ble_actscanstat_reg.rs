#[doc = "Register `BLE_ACTSCANSTAT_REG` reader"]
pub struct R(crate::R<BLE_ACTSCANSTAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_ACTSCANSTAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_ACTSCANSTAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_ACTSCANSTAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_ACTSCANSTAT_REG` writer"]
pub struct W(crate::W<BLE_ACTSCANSTAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_ACTSCANSTAT_REG_SPEC>;
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
impl From<crate::W<BLE_ACTSCANSTAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_ACTSCANSTAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKOFF` reader - Active scan mode back-off counter initialization value."]
pub struct BACKOFF_R(crate::FieldReader<u16, u16>);
impl BACKOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BACKOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKOFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPPERLIMIT` reader - Active scan mode upper limit counter value."]
pub struct UPPERLIMIT_R(crate::FieldReader<u16, u16>);
impl UPPERLIMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UPPERLIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPPERLIMIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:24 - Active scan mode back-off counter initialization value."]
    #[inline(always)]
    pub fn backoff(&self) -> BACKOFF_R {
        BACKOFF_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8 - Active scan mode upper limit counter value."]
    #[inline(always)]
    pub fn upperlimit(&self) -> UPPERLIMIT_R {
        UPPERLIMIT_R::new((self.bits & 0x01ff) as u16)
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
#[doc = "Active scan register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_actscanstat_reg](index.html) module"]
pub struct BLE_ACTSCANSTAT_REG_SPEC;
impl crate::RegisterSpec for BLE_ACTSCANSTAT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_actscanstat_reg::R](R) reader structure"]
impl crate::Readable for BLE_ACTSCANSTAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_actscanstat_reg::W](W) writer structure"]
impl crate::Writable for BLE_ACTSCANSTAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_ACTSCANSTAT_REG to value 0x0001_0001"]
impl crate::Resettable for BLE_ACTSCANSTAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0001
    }
}
