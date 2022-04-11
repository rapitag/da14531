#[doc = "Register `BLE_TXMICVAL_REG` reader"]
pub struct R(crate::R<BLE_TXMICVAL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_TXMICVAL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_TXMICVAL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_TXMICVAL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_TXMICVAL_REG` writer"]
pub struct W(crate::W<BLE_TXMICVAL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_TXMICVAL_REG_SPEC>;
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
impl From<crate::W<BLE_TXMICVAL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_TXMICVAL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXMICVAL` reader - AES-CCM plain MIC value. Valid on when MIC has been calculated (in Tx)"]
pub struct TXMICVAL_R(crate::FieldReader<u32, u32>);
impl TXMICVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TXMICVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMICVAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - AES-CCM plain MIC value. Valid on when MIC has been calculated (in Tx)"]
    #[inline(always)]
    pub fn txmicval(&self) -> TXMICVAL_R {
        TXMICVAL_R::new(self.bits)
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
#[doc = "AES / CCM plain MIC value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_txmicval_reg](index.html) module"]
pub struct BLE_TXMICVAL_REG_SPEC;
impl crate::RegisterSpec for BLE_TXMICVAL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_txmicval_reg::R](R) reader structure"]
impl crate::Readable for BLE_TXMICVAL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_txmicval_reg::W](W) writer structure"]
impl crate::Writable for BLE_TXMICVAL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_TXMICVAL_REG to value 0"]
impl crate::Resettable for BLE_TXMICVAL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
