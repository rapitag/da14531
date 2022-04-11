#[doc = "Register `BLE_RFTESTRXSTAT_REG` reader"]
pub struct R(crate::R<BLE_RFTESTRXSTAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_RFTESTRXSTAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_RFTESTRXSTAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_RFTESTRXSTAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_RFTESTRXSTAT_REG` writer"]
pub struct W(crate::W<BLE_RFTESTRXSTAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_RFTESTRXSTAT_REG_SPEC>;
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
impl From<crate::W<BLE_RFTESTRXSTAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_RFTESTRXSTAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPKTCNT` reader - Reports number of correctly received packet during Test Modes (no sync error, no CRC error). Value is valid if BLE_RFTESTCNTL_REG\\[RXPKTCNTEN\\]
is set"]
pub struct RXPKTCNT_R(crate::FieldReader<u32, u32>);
impl RXPKTCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXPKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPKTCNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Reports number of correctly received packet during Test Modes (no sync error, no CRC error). Value is valid if BLE_RFTESTCNTL_REG\\[RXPKTCNTEN\\]
is set"]
    #[inline(always)]
    pub fn rxpktcnt(&self) -> RXPKTCNT_R {
        RXPKTCNT_R::new(self.bits)
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
#[doc = "RF Testing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_rftestrxstat_reg](index.html) module"]
pub struct BLE_RFTESTRXSTAT_REG_SPEC;
impl crate::RegisterSpec for BLE_RFTESTRXSTAT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_rftestrxstat_reg::R](R) reader structure"]
impl crate::Readable for BLE_RFTESTRXSTAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_rftestrxstat_reg::W](W) writer structure"]
impl crate::Writable for BLE_RFTESTRXSTAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_RFTESTRXSTAT_REG to value 0"]
impl crate::Resettable for BLE_RFTESTRXSTAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
