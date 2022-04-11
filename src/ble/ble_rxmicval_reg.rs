#[doc = "Register `BLE_RXMICVAL_REG` reader"]
pub struct R(crate::R<BLE_RXMICVAL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_RXMICVAL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_RXMICVAL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_RXMICVAL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_RXMICVAL_REG` writer"]
pub struct W(crate::W<BLE_RXMICVAL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_RXMICVAL_REG_SPEC>;
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
impl From<crate::W<BLE_RXMICVAL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_RXMICVAL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXMICVAL` reader - AES-CCM plain MIC value. Valid on once MIC has been extracted from Rx packet."]
pub struct RXMICVAL_R(crate::FieldReader<u32, u32>);
impl RXMICVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXMICVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMICVAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - AES-CCM plain MIC value. Valid on once MIC has been extracted from Rx packet."]
    #[inline(always)]
    pub fn rxmicval(&self) -> RXMICVAL_R {
        RXMICVAL_R::new(self.bits)
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
#[doc = "AES / CCM plain MIC value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_rxmicval_reg](index.html) module"]
pub struct BLE_RXMICVAL_REG_SPEC;
impl crate::RegisterSpec for BLE_RXMICVAL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_rxmicval_reg::R](R) reader structure"]
impl crate::Readable for BLE_RXMICVAL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_rxmicval_reg::W](W) writer structure"]
impl crate::Writable for BLE_RXMICVAL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_RXMICVAL_REG to value 0"]
impl crate::Resettable for BLE_RXMICVAL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
