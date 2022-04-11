#[doc = "Register `BLE_COEXIFCNTL1_REG` reader"]
pub struct R(crate::R<BLE_COEXIFCNTL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_COEXIFCNTL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_COEXIFCNTL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_COEXIFCNTL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_COEXIFCNTL1_REG` writer"]
pub struct W(crate::W<BLE_COEXIFCNTL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_COEXIFCNTL1_REG_SPEC>;
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
impl From<crate::W<BLE_COEXIFCNTL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_COEXIFCNTL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLCPRXTHR` reader - Applies on ble_rx if WLCRXPRIOMODE equals 10 Determines the threshold for Rx priority setting. If ble_pti\\[3:0\\]
output value is greater than WLCPRXTHR, then Rx Bluetooth Low Energy priority is considered as high, and must be provided to the WLAN coexistence interface"]
pub struct WLCPRXTHR_R(crate::FieldReader<u8, u8>);
impl WLCPRXTHR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLCPRXTHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLCPRXTHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLCPRXTHR` writer - Applies on ble_rx if WLCRXPRIOMODE equals 10 Determines the threshold for Rx priority setting. If ble_pti\\[3:0\\]
output value is greater than WLCPRXTHR, then Rx Bluetooth Low Energy priority is considered as high, and must be provided to the WLAN coexistence interface"]
pub struct WLCPRXTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCPRXTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `WLCPTXTHR` reader - Applies on ble_tx if WLCTXPRIOMODE equals 10 Determines the threshold for priority setting. If ble_pti\\[3:0\\]
output value is greater than WLCPTXTHR, then Tx Bluetooth Low Energy priority is considered as high, and must be provided to the WLAN coexistence interface"]
pub struct WLCPTXTHR_R(crate::FieldReader<u8, u8>);
impl WLCPTXTHR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLCPTXTHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLCPTXTHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLCPTXTHR` writer - Applies on ble_tx if WLCTXPRIOMODE equals 10 Determines the threshold for priority setting. If ble_pti\\[3:0\\]
output value is greater than WLCPTXTHR, then Tx Bluetooth Low Energy priority is considered as high, and must be provided to the WLAN coexistence interface"]
pub struct WLCPTXTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCPTXTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `WLCPDURATION` reader - Applies on ble_tx if WLCTXPRIOMODE equals 10 Applies on ble_rx if WLCRXPRIOMODE equals 10 Determines how many s the priority information must be maintained Note that if WLCPDURATION = 0x00, then Tx/Rx priority levels are maintained till Tx/Rx EN are de-asserted."]
pub struct WLCPDURATION_R(crate::FieldReader<u8, u8>);
impl WLCPDURATION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLCPDURATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLCPDURATION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLCPDURATION` writer - Applies on ble_tx if WLCTXPRIOMODE equals 10 Applies on ble_rx if WLCRXPRIOMODE equals 10 Determines how many s the priority information must be maintained Note that if WLCPDURATION = 0x00, then Tx/Rx priority levels are maintained till Tx/Rx EN are de-asserted."]
pub struct WLCPDURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCPDURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `WLCPDELAY` reader - Applies on ble_tx if WLCTXPRIOMODE equals 10. Applies on ble_rx if WLCRXPRIOMODE equals 10. Determines the delay (in us) in Tx/Rx enables rises the time Bluetooth Low energy Tx/Rx priority has to be provided ."]
pub struct WLCPDELAY_R(crate::FieldReader<u8, u8>);
impl WLCPDELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLCPDELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLCPDELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLCPDELAY` writer - Applies on ble_tx if WLCTXPRIOMODE equals 10. Applies on ble_rx if WLCRXPRIOMODE equals 10. Determines the delay (in us) in Tx/Rx enables rises the time Bluetooth Low energy Tx/Rx priority has to be provided ."]
pub struct WLCPDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCPDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - Applies on ble_rx if WLCRXPRIOMODE equals 10 Determines the threshold for Rx priority setting. If ble_pti\\[3:0\\]
output value is greater than WLCPRXTHR, then Rx Bluetooth Low Energy priority is considered as high, and must be provided to the WLAN coexistence interface"]
    #[inline(always)]
    pub fn wlcprxthr(&self) -> WLCPRXTHR_R {
        WLCPRXTHR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Applies on ble_tx if WLCTXPRIOMODE equals 10 Determines the threshold for priority setting. If ble_pti\\[3:0\\]
output value is greater than WLCPTXTHR, then Tx Bluetooth Low Energy priority is considered as high, and must be provided to the WLAN coexistence interface"]
    #[inline(always)]
    pub fn wlcptxthr(&self) -> WLCPTXTHR_R {
        WLCPTXTHR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - Applies on ble_tx if WLCTXPRIOMODE equals 10 Applies on ble_rx if WLCRXPRIOMODE equals 10 Determines how many s the priority information must be maintained Note that if WLCPDURATION = 0x00, then Tx/Rx priority levels are maintained till Tx/Rx EN are de-asserted."]
    #[inline(always)]
    pub fn wlcpduration(&self) -> WLCPDURATION_R {
        WLCPDURATION_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6 - Applies on ble_tx if WLCTXPRIOMODE equals 10. Applies on ble_rx if WLCRXPRIOMODE equals 10. Determines the delay (in us) in Tx/Rx enables rises the time Bluetooth Low energy Tx/Rx priority has to be provided ."]
    #[inline(always)]
    pub fn wlcpdelay(&self) -> WLCPDELAY_R {
        WLCPDELAY_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - Applies on ble_rx if WLCRXPRIOMODE equals 10 Determines the threshold for Rx priority setting. If ble_pti\\[3:0\\]
output value is greater than WLCPRXTHR, then Rx Bluetooth Low Energy priority is considered as high, and must be provided to the WLAN coexistence interface"]
    #[inline(always)]
    pub fn wlcprxthr(&mut self) -> WLCPRXTHR_W {
        WLCPRXTHR_W { w: self }
    }
    #[doc = "Bits 16:20 - Applies on ble_tx if WLCTXPRIOMODE equals 10 Determines the threshold for priority setting. If ble_pti\\[3:0\\]
output value is greater than WLCPTXTHR, then Tx Bluetooth Low Energy priority is considered as high, and must be provided to the WLAN coexistence interface"]
    #[inline(always)]
    pub fn wlcptxthr(&mut self) -> WLCPTXTHR_W {
        WLCPTXTHR_W { w: self }
    }
    #[doc = "Bits 8:14 - Applies on ble_tx if WLCTXPRIOMODE equals 10 Applies on ble_rx if WLCRXPRIOMODE equals 10 Determines how many s the priority information must be maintained Note that if WLCPDURATION = 0x00, then Tx/Rx priority levels are maintained till Tx/Rx EN are de-asserted."]
    #[inline(always)]
    pub fn wlcpduration(&mut self) -> WLCPDURATION_W {
        WLCPDURATION_W { w: self }
    }
    #[doc = "Bits 0:6 - Applies on ble_tx if WLCTXPRIOMODE equals 10. Applies on ble_rx if WLCRXPRIOMODE equals 10. Determines the delay (in us) in Tx/Rx enables rises the time Bluetooth Low energy Tx/Rx priority has to be provided ."]
    #[inline(always)]
    pub fn wlcpdelay(&mut self) -> WLCPDELAY_W {
        WLCPDELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coexistence interface Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_coexifcntl1_reg](index.html) module"]
pub struct BLE_COEXIFCNTL1_REG_SPEC;
impl crate::RegisterSpec for BLE_COEXIFCNTL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_coexifcntl1_reg::R](R) reader structure"]
impl crate::Readable for BLE_COEXIFCNTL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_coexifcntl1_reg::W](W) writer structure"]
impl crate::Writable for BLE_COEXIFCNTL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_COEXIFCNTL1_REG to value 0"]
impl crate::Resettable for BLE_COEXIFCNTL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
