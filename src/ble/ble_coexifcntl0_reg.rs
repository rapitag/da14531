#[doc = "Register `BLE_COEXIFCNTL0_REG` reader"]
pub struct R(crate::R<BLE_COEXIFCNTL0_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_COEXIFCNTL0_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_COEXIFCNTL0_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_COEXIFCNTL0_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_COEXIFCNTL0_REG` writer"]
pub struct W(crate::W<BLE_COEXIFCNTL0_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_COEXIFCNTL0_REG_SPEC>;
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
impl From<crate::W<BLE_COEXIFCNTL0_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_COEXIFCNTL0_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLCRXPRIOMODE` reader - Defines Bluetooth Low Energy packet ble_rx mode behavior. 00: Rx indication excluding Rx Power up delay (starts when correlator is enabled) 01: Rx indication including Rx Power up delay 10: Rx High priority indicator 11: n/a"]
pub struct WLCRXPRIOMODE_R(crate::FieldReader<u8, u8>);
impl WLCRXPRIOMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLCRXPRIOMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLCRXPRIOMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLCRXPRIOMODE` writer - Defines Bluetooth Low Energy packet ble_rx mode behavior. 00: Rx indication excluding Rx Power up delay (starts when correlator is enabled) 01: Rx indication including Rx Power up delay 10: Rx High priority indicator 11: n/a"]
pub struct WLCRXPRIOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCRXPRIOMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `WLCTXPRIOMODE` reader - Defines Bluetooth Low Energy packet ble_tx mode behavior 00: Tx indication excluding Tx Power up delay 01: Tx indication including Tx Power up delay 10: Tx High priority indicator 11: n/a"]
pub struct WLCTXPRIOMODE_R(crate::FieldReader<u8, u8>);
impl WLCTXPRIOMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLCTXPRIOMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLCTXPRIOMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLCTXPRIOMODE` writer - Defines Bluetooth Low Energy packet ble_tx mode behavior 00: Tx indication excluding Tx Power up delay 01: Tx indication including Tx Power up delay 10: Tx High priority indicator 11: n/a"]
pub struct WLCTXPRIOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WLCTXPRIOMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `WLANTXMSK` reader - Determines how wlan_tx impact BLE Tx and Rx 00: wlan_tx has no impact (default mode) 01: wlan_tx can stop BLE Tx, no impact on BLE Rx 10: wlan_tx can stop BLE Rx, no impact on BLE Tx 11: wlan_tx can stop both BLE Tx and BLE Rx"]
pub struct WLANTXMSK_R(crate::FieldReader<u8, u8>);
impl WLANTXMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLANTXMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLANTXMSK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLANTXMSK` writer - Determines how wlan_tx impact BLE Tx and Rx 00: wlan_tx has no impact (default mode) 01: wlan_tx can stop BLE Tx, no impact on BLE Rx 10: wlan_tx can stop BLE Rx, no impact on BLE Tx 11: wlan_tx can stop both BLE Tx and BLE Rx"]
pub struct WLANTXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WLANTXMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `WLANRXMSK` reader - Determines how wlan_rx impact BLE Tx and Rx 00: wlan_rx has no impact 01: wlan_rx can stop BLE Tx, no impact on BLE Rx (default mode) 10: wlan_rx can stop BLE Rx, no impact on BLE Tx 11: wlan_rx can stop both BLE Tx and BLE Rx"]
pub struct WLANRXMSK_R(crate::FieldReader<u8, u8>);
impl WLANRXMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLANRXMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLANRXMSK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLANRXMSK` writer - Determines how wlan_rx impact BLE Tx and Rx 00: wlan_rx has no impact 01: wlan_rx can stop BLE Tx, no impact on BLE Rx (default mode) 10: wlan_rx can stop BLE Rx, no impact on BLE Tx 11: wlan_rx can stop both BLE Tx and BLE Rx"]
pub struct WLANRXMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WLANRXMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `SYNCGEN_EN` reader - Determines whether ble_sync is generated or not. 0: ble_sync pulse not generated 1: ble_sync pulse generated"]
pub struct SYNCGEN_EN_R(crate::FieldReader<bool, bool>);
impl SYNCGEN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCGEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCGEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCGEN_EN` writer - Determines whether ble_sync is generated or not. 0: ble_sync pulse not generated 1: ble_sync pulse generated"]
pub struct SYNCGEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCGEN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `COEX_EN` reader - Enable / Disable control of the MWS/WLAN Coexistence control 0: Coexistence interface disabled 1: Coexistence interface enabled"]
pub struct COEX_EN_R(crate::FieldReader<bool, bool>);
impl COEX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COEX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COEX_EN` writer - Enable / Disable control of the MWS/WLAN Coexistence control 0: Coexistence interface disabled 1: Coexistence interface enabled"]
pub struct COEX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:21 - Defines Bluetooth Low Energy packet ble_rx mode behavior. 00: Rx indication excluding Rx Power up delay (starts when correlator is enabled) 01: Rx indication including Rx Power up delay 10: Rx High priority indicator 11: n/a"]
    #[inline(always)]
    pub fn wlcrxpriomode(&self) -> WLCRXPRIOMODE_R {
        WLCRXPRIOMODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Defines Bluetooth Low Energy packet ble_tx mode behavior 00: Tx indication excluding Tx Power up delay 01: Tx indication including Tx Power up delay 10: Tx High priority indicator 11: n/a"]
    #[inline(always)]
    pub fn wlctxpriomode(&self) -> WLCTXPRIOMODE_R {
        WLCTXPRIOMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Determines how wlan_tx impact BLE Tx and Rx 00: wlan_tx has no impact (default mode) 01: wlan_tx can stop BLE Tx, no impact on BLE Rx 10: wlan_tx can stop BLE Rx, no impact on BLE Tx 11: wlan_tx can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn wlantxmsk(&self) -> WLANTXMSK_R {
        WLANTXMSK_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Determines how wlan_rx impact BLE Tx and Rx 00: wlan_rx has no impact 01: wlan_rx can stop BLE Tx, no impact on BLE Rx (default mode) 10: wlan_rx can stop BLE Rx, no impact on BLE Tx 11: wlan_rx can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn wlanrxmsk(&self) -> WLANRXMSK_R {
        WLANRXMSK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 1 - Determines whether ble_sync is generated or not. 0: ble_sync pulse not generated 1: ble_sync pulse generated"]
    #[inline(always)]
    pub fn syncgen_en(&self) -> SYNCGEN_EN_R {
        SYNCGEN_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable / Disable control of the MWS/WLAN Coexistence control 0: Coexistence interface disabled 1: Coexistence interface enabled"]
    #[inline(always)]
    pub fn coex_en(&self) -> COEX_EN_R {
        COEX_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 20:21 - Defines Bluetooth Low Energy packet ble_rx mode behavior. 00: Rx indication excluding Rx Power up delay (starts when correlator is enabled) 01: Rx indication including Rx Power up delay 10: Rx High priority indicator 11: n/a"]
    #[inline(always)]
    pub fn wlcrxpriomode(&mut self) -> WLCRXPRIOMODE_W {
        WLCRXPRIOMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Defines Bluetooth Low Energy packet ble_tx mode behavior 00: Tx indication excluding Tx Power up delay 01: Tx indication including Tx Power up delay 10: Tx High priority indicator 11: n/a"]
    #[inline(always)]
    pub fn wlctxpriomode(&mut self) -> WLCTXPRIOMODE_W {
        WLCTXPRIOMODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Determines how wlan_tx impact BLE Tx and Rx 00: wlan_tx has no impact (default mode) 01: wlan_tx can stop BLE Tx, no impact on BLE Rx 10: wlan_tx can stop BLE Rx, no impact on BLE Tx 11: wlan_tx can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn wlantxmsk(&mut self) -> WLANTXMSK_W {
        WLANTXMSK_W { w: self }
    }
    #[doc = "Bits 4:5 - Determines how wlan_rx impact BLE Tx and Rx 00: wlan_rx has no impact 01: wlan_rx can stop BLE Tx, no impact on BLE Rx (default mode) 10: wlan_rx can stop BLE Rx, no impact on BLE Tx 11: wlan_rx can stop both BLE Tx and BLE Rx"]
    #[inline(always)]
    pub fn wlanrxmsk(&mut self) -> WLANRXMSK_W {
        WLANRXMSK_W { w: self }
    }
    #[doc = "Bit 1 - Determines whether ble_sync is generated or not. 0: ble_sync pulse not generated 1: ble_sync pulse generated"]
    #[inline(always)]
    pub fn syncgen_en(&mut self) -> SYNCGEN_EN_W {
        SYNCGEN_EN_W { w: self }
    }
    #[doc = "Bit 0 - Enable / Disable control of the MWS/WLAN Coexistence control 0: Coexistence interface disabled 1: Coexistence interface enabled"]
    #[inline(always)]
    pub fn coex_en(&mut self) -> COEX_EN_W {
        COEX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coexistence interface Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_coexifcntl0_reg](index.html) module"]
pub struct BLE_COEXIFCNTL0_REG_SPEC;
impl crate::RegisterSpec for BLE_COEXIFCNTL0_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_coexifcntl0_reg::R](R) reader structure"]
impl crate::Readable for BLE_COEXIFCNTL0_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_coexifcntl0_reg::W](W) writer structure"]
impl crate::Writable for BLE_COEXIFCNTL0_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_COEXIFCNTL0_REG to value 0x10"]
impl crate::Resettable for BLE_COEXIFCNTL0_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
