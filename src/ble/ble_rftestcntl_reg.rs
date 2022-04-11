#[doc = "Register `BLE_RFTESTCNTL_REG` reader"]
pub struct R(crate::R<BLE_RFTESTCNTL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_RFTESTCNTL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_RFTESTCNTL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_RFTESTCNTL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_RFTESTCNTL_REG` writer"]
pub struct W(crate::W<BLE_RFTESTCNTL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_RFTESTCNTL_REG_SPEC>;
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
impl From<crate::W<BLE_RFTESTCNTL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_RFTESTCNTL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INFINITERX` reader - Applicable in RF Test Mode only 0: Normal mode of operation 1: Infinite Rx window"]
pub struct INFINITERX_R(crate::FieldReader<bool, bool>);
impl INFINITERX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFINITERX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFINITERX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFINITERX` writer - Applicable in RF Test Mode only 0: Normal mode of operation 1: Infinite Rx window"]
pub struct INFINITERX_W<'a> {
    w: &'a mut W,
}
impl<'a> INFINITERX_W<'a> {
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
#[doc = "Field `RXPKTCNTEN` reader - Applicable in RF Test Mode only 0: Rx packet count disabled 1: Rx packet count enabled, and reported in CS-RXCCMPKTCNT and BLE_RFTESTRXSTAT_REG\\[RXPKTCNT\\]
on RF abort command"]
pub struct RXPKTCNTEN_R(crate::FieldReader<bool, bool>);
impl RXPKTCNTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXPKTCNTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPKTCNTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPKTCNTEN` writer - Applicable in RF Test Mode only 0: Rx packet count disabled 1: Rx packet count enabled, and reported in CS-RXCCMPKTCNT and BLE_RFTESTRXSTAT_REG\\[RXPKTCNT\\]
on RF abort command"]
pub struct RXPKTCNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPKTCNTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `INFINITETX` reader - Applicable in RF Test Mode only 0: Normal mode of operation. 1: Infinite Tx packet / Normal start of a packet but endless payload"]
pub struct INFINITETX_R(crate::FieldReader<bool, bool>);
impl INFINITETX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFINITETX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFINITETX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFINITETX` writer - Applicable in RF Test Mode only 0: Normal mode of operation. 1: Infinite Tx packet / Normal start of a packet but endless payload"]
pub struct INFINITETX_W<'a> {
    w: &'a mut W,
}
impl<'a> INFINITETX_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `TXLENGTHSRC` reader - Applicable only in Tx/Rx RF Test mode 0: Normal mode of operation: TxDESC-TXADVLEN controls the Tx packet payload size 1: Uses BLE_RFTESTCNTL_REG\\[TXLENGTH\\]
packet length (can support up to 512 bytes transmit)"]
pub struct TXLENGTHSRC_R(crate::FieldReader<bool, bool>);
impl TXLENGTHSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXLENGTHSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLENGTHSRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLENGTHSRC` writer - Applicable only in Tx/Rx RF Test mode 0: Normal mode of operation: TxDESC-TXADVLEN controls the Tx packet payload size 1: Uses BLE_RFTESTCNTL_REG\\[TXLENGTH\\]
packet length (can support up to 512 bytes transmit)"]
pub struct TXLENGTHSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLENGTHSRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `PRBSTYPE` reader - Applicable only in Tx/Rx RF Test mode 0: Tx Packet Payload are PRBS9 type 1: Tx Packet Payload are PRBS15 type"]
pub struct PRBSTYPE_R(crate::FieldReader<bool, bool>);
impl PRBSTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRBSTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRBSTYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRBSTYPE` writer - Applicable only in Tx/Rx RF Test mode 0: Tx Packet Payload are PRBS9 type 1: Tx Packet Payload are PRBS15 type"]
pub struct PRBSTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRBSTYPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `TXPLDSRC` reader - Applicable only in Tx/Rx RF Test mode 0: Tx Packet Payload source is the Control Structure 1: Tx Packet Payload are PRBS generator"]
pub struct TXPLDSRC_R(crate::FieldReader<bool, bool>);
impl TXPLDSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXPLDSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPLDSRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPLDSRC` writer - Applicable only in Tx/Rx RF Test mode 0: Tx Packet Payload source is the Control Structure 1: Tx Packet Payload are PRBS generator"]
pub struct TXPLDSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPLDSRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `TXPKTCNTEN` reader - Applicable in RF Test Mode only 0: Tx packet count disabled 1: Tx packet count enabled, and reported in CS-TXCCMPKTCNT and BLE_RFTESTTXSTAT_REG\\[TXPKTCNT\\]
on RF abort command"]
pub struct TXPKTCNTEN_R(crate::FieldReader<bool, bool>);
impl TXPKTCNTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXPKTCNTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPKTCNTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPKTCNTEN` writer - Applicable in RF Test Mode only 0: Tx packet count disabled 1: Tx packet count enabled, and reported in CS-TXCCMPKTCNT and BLE_RFTESTTXSTAT_REG\\[TXPKTCNT\\]
on RF abort command"]
pub struct TXPKTCNTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPKTCNTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `TXLENGTH` reader - Applicable only for Tx/Rx RF Test mode, and valid when BLE_RFTESTCNTL_REG\\[TXLENGTHSRC\\]
= 1 Tx packet length in number of byte"]
pub struct TXLENGTH_R(crate::FieldReader<u16, u16>);
impl TXLENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TXLENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLENGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLENGTH` writer - Applicable only for Tx/Rx RF Test mode, and valid when BLE_RFTESTCNTL_REG\\[TXLENGTHSRC\\]
= 1 Tx packet length in number of byte"]
pub struct TXLENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Applicable in RF Test Mode only 0: Normal mode of operation 1: Infinite Rx window"]
    #[inline(always)]
    pub fn infiniterx(&self) -> INFINITERX_R {
        INFINITERX_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 27 - Applicable in RF Test Mode only 0: Rx packet count disabled 1: Rx packet count enabled, and reported in CS-RXCCMPKTCNT and BLE_RFTESTRXSTAT_REG\\[RXPKTCNT\\]
on RF abort command"]
    #[inline(always)]
    pub fn rxpktcnten(&self) -> RXPKTCNTEN_R {
        RXPKTCNTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 15 - Applicable in RF Test Mode only 0: Normal mode of operation. 1: Infinite Tx packet / Normal start of a packet but endless payload"]
    #[inline(always)]
    pub fn infinitetx(&self) -> INFINITETX_R {
        INFINITETX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Applicable only in Tx/Rx RF Test mode 0: Normal mode of operation: TxDESC-TXADVLEN controls the Tx packet payload size 1: Uses BLE_RFTESTCNTL_REG\\[TXLENGTH\\]
packet length (can support up to 512 bytes transmit)"]
    #[inline(always)]
    pub fn txlengthsrc(&self) -> TXLENGTHSRC_R {
        TXLENGTHSRC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Applicable only in Tx/Rx RF Test mode 0: Tx Packet Payload are PRBS9 type 1: Tx Packet Payload are PRBS15 type"]
    #[inline(always)]
    pub fn prbstype(&self) -> PRBSTYPE_R {
        PRBSTYPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Applicable only in Tx/Rx RF Test mode 0: Tx Packet Payload source is the Control Structure 1: Tx Packet Payload are PRBS generator"]
    #[inline(always)]
    pub fn txpldsrc(&self) -> TXPLDSRC_R {
        TXPLDSRC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Applicable in RF Test Mode only 0: Tx packet count disabled 1: Tx packet count enabled, and reported in CS-TXCCMPKTCNT and BLE_RFTESTTXSTAT_REG\\[TXPKTCNT\\]
on RF abort command"]
    #[inline(always)]
    pub fn txpktcnten(&self) -> TXPKTCNTEN_R {
        TXPKTCNTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 0:8 - Applicable only for Tx/Rx RF Test mode, and valid when BLE_RFTESTCNTL_REG\\[TXLENGTHSRC\\]
= 1 Tx packet length in number of byte"]
    #[inline(always)]
    pub fn txlength(&self) -> TXLENGTH_R {
        TXLENGTH_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Applicable in RF Test Mode only 0: Normal mode of operation 1: Infinite Rx window"]
    #[inline(always)]
    pub fn infiniterx(&mut self) -> INFINITERX_W {
        INFINITERX_W { w: self }
    }
    #[doc = "Bit 27 - Applicable in RF Test Mode only 0: Rx packet count disabled 1: Rx packet count enabled, and reported in CS-RXCCMPKTCNT and BLE_RFTESTRXSTAT_REG\\[RXPKTCNT\\]
on RF abort command"]
    #[inline(always)]
    pub fn rxpktcnten(&mut self) -> RXPKTCNTEN_W {
        RXPKTCNTEN_W { w: self }
    }
    #[doc = "Bit 15 - Applicable in RF Test Mode only 0: Normal mode of operation. 1: Infinite Tx packet / Normal start of a packet but endless payload"]
    #[inline(always)]
    pub fn infinitetx(&mut self) -> INFINITETX_W {
        INFINITETX_W { w: self }
    }
    #[doc = "Bit 14 - Applicable only in Tx/Rx RF Test mode 0: Normal mode of operation: TxDESC-TXADVLEN controls the Tx packet payload size 1: Uses BLE_RFTESTCNTL_REG\\[TXLENGTH\\]
packet length (can support up to 512 bytes transmit)"]
    #[inline(always)]
    pub fn txlengthsrc(&mut self) -> TXLENGTHSRC_W {
        TXLENGTHSRC_W { w: self }
    }
    #[doc = "Bit 13 - Applicable only in Tx/Rx RF Test mode 0: Tx Packet Payload are PRBS9 type 1: Tx Packet Payload are PRBS15 type"]
    #[inline(always)]
    pub fn prbstype(&mut self) -> PRBSTYPE_W {
        PRBSTYPE_W { w: self }
    }
    #[doc = "Bit 12 - Applicable only in Tx/Rx RF Test mode 0: Tx Packet Payload source is the Control Structure 1: Tx Packet Payload are PRBS generator"]
    #[inline(always)]
    pub fn txpldsrc(&mut self) -> TXPLDSRC_W {
        TXPLDSRC_W { w: self }
    }
    #[doc = "Bit 11 - Applicable in RF Test Mode only 0: Tx packet count disabled 1: Tx packet count enabled, and reported in CS-TXCCMPKTCNT and BLE_RFTESTTXSTAT_REG\\[TXPKTCNT\\]
on RF abort command"]
    #[inline(always)]
    pub fn txpktcnten(&mut self) -> TXPKTCNTEN_W {
        TXPKTCNTEN_W { w: self }
    }
    #[doc = "Bits 0:8 - Applicable only for Tx/Rx RF Test mode, and valid when BLE_RFTESTCNTL_REG\\[TXLENGTHSRC\\]
= 1 Tx packet length in number of byte"]
    #[inline(always)]
    pub fn txlength(&mut self) -> TXLENGTH_W {
        TXLENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF Testing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_rftestcntl_reg](index.html) module"]
pub struct BLE_RFTESTCNTL_REG_SPEC;
impl crate::RegisterSpec for BLE_RFTESTCNTL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_rftestcntl_reg::R](R) reader structure"]
impl crate::Readable for BLE_RFTESTCNTL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_rftestcntl_reg::W](W) writer structure"]
impl crate::Writable for BLE_RFTESTCNTL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_RFTESTCNTL_REG to value 0"]
impl crate::Resettable for BLE_RFTESTCNTL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
