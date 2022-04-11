#[doc = "Register `BLE_RADIOPWRUPDN_REG` reader"]
pub struct R(crate::R<BLE_RADIOPWRUPDN_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_RADIOPWRUPDN_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_RADIOPWRUPDN_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_RADIOPWRUPDN_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_RADIOPWRUPDN_REG` writer"]
pub struct W(crate::W<BLE_RADIOPWRUPDN_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_RADIOPWRUPDN_REG_SPEC>;
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
impl From<crate::W<BLE_RADIOPWRUPDN_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_RADIOPWRUPDN_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTRIP_DELAY` reader - Defines round trip delay value. This value correspond to the addition of data latency in Tx and data latency in Rx. Value is in usec."]
pub struct RTRIP_DELAY_R(crate::FieldReader<u8, u8>);
impl RTRIP_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTRIP_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTRIP_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTRIP_DELAY` writer - Defines round trip delay value. This value correspond to the addition of data latency in Tx and data latency in Rx. Value is in usec."]
pub struct RTRIP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRIP_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
#[doc = "Field `RXPWRUP` reader - This register holds the length in s of the RX power up phase for the current radio device. Default value is 210 usec (reset value). Operating range depends on the selected radio."]
pub struct RXPWRUP_R(crate::FieldReader<u8, u8>);
impl RXPWRUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXPWRUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPWRUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPWRUP` writer - This register holds the length in s of the RX power up phase for the current radio device. Default value is 210 usec (reset value). Operating range depends on the selected radio."]
pub struct RXPWRUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWRUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TXPWRDN` reader - This register extends the length in s of the TX power down phase for the current radio device. Default value is 3 usec (reset value). Operating range depends on the selected radio."]
pub struct TXPWRDN_R(crate::FieldReader<u8, u8>);
impl TXPWRDN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXPWRDN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPWRDN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPWRDN` writer - This register extends the length in s of the TX power down phase for the current radio device. Default value is 3 usec (reset value). Operating range depends on the selected radio."]
pub struct TXPWRDN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWRDN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TXPWRUP` reader - This register holds the length in s of the TX power up phase for the current radio device. Default value is 210 usec (reset value). Operating range depends on the selected radio."]
pub struct TXPWRUP_R(crate::FieldReader<u8, u8>);
impl TXPWRUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXPWRUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPWRUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPWRUP` writer - This register holds the length in s of the TX power up phase for the current radio device. Default value is 210 usec (reset value). Operating range depends on the selected radio."]
pub struct TXPWRUP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWRUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:30 - Defines round trip delay value. This value correspond to the addition of data latency in Tx and data latency in Rx. Value is in usec."]
    #[inline(always)]
    pub fn rtrip_delay(&self) -> RTRIP_DELAY_R {
        RTRIP_DELAY_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - This register holds the length in s of the RX power up phase for the current radio device. Default value is 210 usec (reset value). Operating range depends on the selected radio."]
    #[inline(always)]
    pub fn rxpwrup(&self) -> RXPWRUP_R {
        RXPWRUP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - This register extends the length in s of the TX power down phase for the current radio device. Default value is 3 usec (reset value). Operating range depends on the selected radio."]
    #[inline(always)]
    pub fn txpwrdn(&self) -> TXPWRDN_R {
        TXPWRDN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - This register holds the length in s of the TX power up phase for the current radio device. Default value is 210 usec (reset value). Operating range depends on the selected radio."]
    #[inline(always)]
    pub fn txpwrup(&self) -> TXPWRUP_R {
        TXPWRUP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30 - Defines round trip delay value. This value correspond to the addition of data latency in Tx and data latency in Rx. Value is in usec."]
    #[inline(always)]
    pub fn rtrip_delay(&mut self) -> RTRIP_DELAY_W {
        RTRIP_DELAY_W { w: self }
    }
    #[doc = "Bits 16:23 - This register holds the length in s of the RX power up phase for the current radio device. Default value is 210 usec (reset value). Operating range depends on the selected radio."]
    #[inline(always)]
    pub fn rxpwrup(&mut self) -> RXPWRUP_W {
        RXPWRUP_W { w: self }
    }
    #[doc = "Bits 8:11 - This register extends the length in s of the TX power down phase for the current radio device. Default value is 3 usec (reset value). Operating range depends on the selected radio."]
    #[inline(always)]
    pub fn txpwrdn(&mut self) -> TXPWRDN_W {
        TXPWRDN_W { w: self }
    }
    #[doc = "Bits 0:7 - This register holds the length in s of the TX power up phase for the current radio device. Default value is 210 usec (reset value). Operating range depends on the selected radio."]
    #[inline(always)]
    pub fn txpwrup(&mut self) -> TXPWRUP_W {
        TXPWRUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX/TX power up/down phase register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_radiopwrupdn_reg](index.html) module"]
pub struct BLE_RADIOPWRUPDN_REG_SPEC;
impl crate::RegisterSpec for BLE_RADIOPWRUPDN_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_radiopwrupdn_reg::R](R) reader structure"]
impl crate::Readable for BLE_RADIOPWRUPDN_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_radiopwrupdn_reg::W](W) writer structure"]
impl crate::Writable for BLE_RADIOPWRUPDN_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_RADIOPWRUPDN_REG to value 0x00d2_03d2"]
impl crate::Resettable for BLE_RADIOPWRUPDN_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00d2_03d2
    }
}
