#[doc = "Register `BLE_DIAGSTAT_REG` reader"]
pub struct R(crate::R<BLE_DIAGSTAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_DIAGSTAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_DIAGSTAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_DIAGSTAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_DIAGSTAT_REG` writer"]
pub struct W(crate::W<BLE_DIAGSTAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_DIAGSTAT_REG_SPEC>;
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
impl From<crate::W<BLE_DIAGSTAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_DIAGSTAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAG3STAT` reader - Directly connected to ble_dbg3\\[7:0\\]
output. Debug use only."]
pub struct DIAG3STAT_R(crate::FieldReader<u8, u8>);
impl DIAG3STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG3STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG3STAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG2STAT` reader - Directly connected to ble_dbg2\\[7:0\\]
output. Debug use only."]
pub struct DIAG2STAT_R(crate::FieldReader<u8, u8>);
impl DIAG2STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG2STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG2STAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG1STAT` reader - Directly connected to ble_dbg1\\[7:0\\]
output. Debug use only."]
pub struct DIAG1STAT_R(crate::FieldReader<u8, u8>);
impl DIAG1STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG1STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG1STAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAG0STAT` reader - Directly connected to ble_dbg0\\[7:0\\]
output. Debug use only."]
pub struct DIAG0STAT_R(crate::FieldReader<u8, u8>);
impl DIAG0STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIAG0STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAG0STAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:31 - Directly connected to ble_dbg3\\[7:0\\]
output. Debug use only."]
    #[inline(always)]
    pub fn diag3stat(&self) -> DIAG3STAT_R {
        DIAG3STAT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Directly connected to ble_dbg2\\[7:0\\]
output. Debug use only."]
    #[inline(always)]
    pub fn diag2stat(&self) -> DIAG2STAT_R {
        DIAG2STAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Directly connected to ble_dbg1\\[7:0\\]
output. Debug use only."]
    #[inline(always)]
    pub fn diag1stat(&self) -> DIAG1STAT_R {
        DIAG1STAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Directly connected to ble_dbg0\\[7:0\\]
output. Debug use only."]
    #[inline(always)]
    pub fn diag0stat(&self) -> DIAG0STAT_R {
        DIAG0STAT_R::new((self.bits & 0xff) as u8)
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
#[doc = "Debug use only\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_diagstat_reg](index.html) module"]
pub struct BLE_DIAGSTAT_REG_SPEC;
impl crate::RegisterSpec for BLE_DIAGSTAT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_diagstat_reg::R](R) reader structure"]
impl crate::Readable for BLE_DIAGSTAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_diagstat_reg::W](W) writer structure"]
impl crate::Writable for BLE_DIAGSTAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_DIAGSTAT_REG to value 0"]
impl crate::Resettable for BLE_DIAGSTAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
