#[doc = "Register `BLE_ERRORTYPESTAT_REG` reader"]
pub struct R(crate::R<BLE_ERRORTYPESTAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_ERRORTYPESTAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_ERRORTYPESTAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_ERRORTYPESTAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_ERRORTYPESTAT_REG` writer"]
pub struct W(crate::W<BLE_ERRORTYPESTAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_ERRORTYPESTAT_REG_SPEC>;
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
impl From<crate::W<BLE_ERRORTYPESTAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_ERRORTYPESTAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONCEVTIRQ_ERROR` reader - Indicates whether two consecutive and concurrent ble_event_irq have been generated, and not acknowledged in time by the BLE Software. 0: No error 1: Error occurred"]
pub struct CONCEVTIRQ_ERROR_R(crate::FieldReader<bool, bool>);
impl CONCEVTIRQ_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONCEVTIRQ_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONCEVTIRQ_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDATA_PTR_ERROR` reader - Indicates whether Rx data buffer pointer value programmed is null: this is a major programming failure. 0: No error 1: Error occurred"]
pub struct RXDATA_PTR_ERROR_R(crate::FieldReader<bool, bool>);
impl RXDATA_PTR_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDATA_PTR_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATA_PTR_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDATA_PTR_ERROR` reader - Indicates whether Tx data buffer pointer value programmed is null during Advertising / Scanning / Initiating events, or during Master / Slave connections with non-null packet length: this is a major programming failure. 0: No error 1: Error occurred"]
pub struct TXDATA_PTR_ERROR_R(crate::FieldReader<bool, bool>);
impl TXDATA_PTR_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDATA_PTR_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDATA_PTR_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDESC_EMPTY_ERROR` reader - Indicates whether Rx Descriptor pointer value programmed in register is null: this is a major programming failure. 0: No error 1: Error occurred"]
pub struct RXDESC_EMPTY_ERROR_R(crate::FieldReader<bool, bool>);
impl RXDESC_EMPTY_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDESC_EMPTY_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDESC_EMPTY_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDESC_EMPTY_ERROR` reader - Indicates whether Tx Descriptor pointer value programmed in Control Structure is null during Advertising / Scanning / Initiating events: this is a major programming failure. 0: No error 1: Error occurred"]
pub struct TXDESC_EMPTY_ERROR_R(crate::FieldReader<bool, bool>);
impl TXDESC_EMPTY_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDESC_EMPTY_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDESC_EMPTY_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSFORMAT_ERROR` reader - Indicates whether CS-FORMAT has been programmed with an invalid value: this is a major software programming failure. 0: No error 1: Error occurred"]
pub struct CSFORMAT_ERROR_R(crate::FieldReader<bool, bool>);
impl CSFORMAT_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSFORMAT_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSFORMAT_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLCHMAP_ERROR` reader - Indicates Link Layer Channel Map error, happens when actual number of CS-LLCHMAP bit set to one is different from CS-NBCHGOOD at the beginning of Frequency Hopping process 0: No error 1: Error occurred"]
pub struct LLCHMAP_ERROR_R(crate::FieldReader<bool, bool>);
impl LLCHMAP_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LLCHMAP_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLCHMAP_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_UNDERRUN` reader - Indicates Advertising Interval Under run, occurs if time between two consecutive Advertising packet (in Advertising mode) is lower than the expected value. 0: No error 1: Error occurred"]
pub struct ADV_UNDERRUN_R(crate::FieldReader<bool, bool>);
impl ADV_UNDERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADV_UNDERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_UNDERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFS_UNDERRUN` reader - Indicates Inter Frame Space Under run, occurs if IFS time is not enough to update and read Control Structure/Descriptors, and/or White List parsing is not finished and/or Decryption time is too long to be finished on time 0: No error 1: Error occurred"]
pub struct IFS_UNDERRUN_R(crate::FieldReader<bool, bool>);
impl IFS_UNDERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IFS_UNDERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFS_UNDERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WHITELIST_ERROR` reader - Indicates White List Timeout error, occurs if White List parsing is not finished on time 0: No error 1: Error occurred"]
pub struct WHITELIST_ERROR_R(crate::FieldReader<bool, bool>);
impl WHITELIST_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WHITELIST_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WHITELIST_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVT_CNTL_APFM_ERROR` reader - Indicates Anticipated Pre-Fetch Mechanism error: happens when 2 consecutive events are programmed, and when the first event is not completely finished while second pre-fetch instant is reached. 0: No error 1: Error occured"]
pub struct EVT_CNTL_APFM_ERROR_R(crate::FieldReader<bool, bool>);
impl EVT_CNTL_APFM_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVT_CNTL_APFM_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVT_CNTL_APFM_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVT_SCHDL_APFM_ERROR` reader - Indicates Anticipated Pre-Fetch Mechanism error: happens when 2 consecutive events are programmed, and when the first event is not completely finished while second pre-fetch instant is reached. 0: No error 1: Error occured"]
pub struct EVT_SCHDL_APFM_ERROR_R(crate::FieldReader<bool, bool>);
impl EVT_SCHDL_APFM_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVT_SCHDL_APFM_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVT_SCHDL_APFM_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVT_SCHDL_ENTRY_ERROR` reader - Indicates Event Scheduler faced Invalid timing programing on two consecutive ET entries (e.g first one with 624s offset and second one with no offset) 0: No error 1: Error occurred"]
pub struct EVT_SCHDL_ENTRY_ERROR_R(crate::FieldReader<bool, bool>);
impl EVT_SCHDL_ENTRY_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVT_SCHDL_ENTRY_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVT_SCHDL_ENTRY_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVT_SCHDL_EMACC_ERROR` reader - Indicates Event Scheduler Exchange Memory access error, happens when Exchange Memory accesses are not served in time, and blocks the Exchange Table entry read 0: No error 1: Error occurred"]
pub struct EVT_SCHDL_EMACC_ERROR_R(crate::FieldReader<bool, bool>);
impl EVT_SCHDL_EMACC_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVT_SCHDL_EMACC_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVT_SCHDL_EMACC_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADIO_EMACC_ERROR` reader - Indicates Radio Controller Exchange Memory access error, happens when Exchange Memory accesses are not served in time and data are corrupted. 0: No error 1: Error occurred"]
pub struct RADIO_EMACC_ERROR_R(crate::FieldReader<bool, bool>);
impl RADIO_EMACC_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_EMACC_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_EMACC_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKTCNTL_EMACC_ERROR` reader - Indicates Packet Controller Exchange Memory access error, happens when Exchange Memory accesses are not served in time and Tx/Rx data are corrupted 0: No error 1: Error occurred"]
pub struct PKTCNTL_EMACC_ERROR_R(crate::FieldReader<bool, bool>);
impl PKTCNTL_EMACC_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PKTCNTL_EMACC_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTCNTL_EMACC_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCRYPT_ERROR` reader - Indicates real time decryption error, happens when AES-CCM decryption is too slow compared to Packet Controller requests. A 16-bytes block has to be decrypted prior the next block is received by the Packet Controller 0: No error 1: Error occurred"]
pub struct RXCRYPT_ERROR_R(crate::FieldReader<bool, bool>);
impl RXCRYPT_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXCRYPT_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCRYPT_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCRYPT_ERROR` reader - Indicates Real Time encryption error, happens when AES-CCM encryption is too slow compared to Packet Controller requests. A 16-bytes block has to be encrypted and prepared on Packet Controller request, and needs to be ready before the Packet Controller has to send ti 0: No error 1: Error occurred"]
pub struct TXCRYPT_ERROR_R(crate::FieldReader<bool, bool>);
impl TXCRYPT_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCRYPT_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCRYPT_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 17 - Indicates whether two consecutive and concurrent ble_event_irq have been generated, and not acknowledged in time by the BLE Software. 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn concevtirq_error(&self) -> CONCEVTIRQ_ERROR_R {
        CONCEVTIRQ_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates whether Rx data buffer pointer value programmed is null: this is a major programming failure. 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn rxdata_ptr_error(&self) -> RXDATA_PTR_ERROR_R {
        RXDATA_PTR_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates whether Tx data buffer pointer value programmed is null during Advertising / Scanning / Initiating events, or during Master / Slave connections with non-null packet length: this is a major programming failure. 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn txdata_ptr_error(&self) -> TXDATA_PTR_ERROR_R {
        TXDATA_PTR_ERROR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates whether Rx Descriptor pointer value programmed in register is null: this is a major programming failure. 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn rxdesc_empty_error(&self) -> RXDESC_EMPTY_ERROR_R {
        RXDESC_EMPTY_ERROR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates whether Tx Descriptor pointer value programmed in Control Structure is null during Advertising / Scanning / Initiating events: this is a major programming failure. 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn txdesc_empty_error(&self) -> TXDESC_EMPTY_ERROR_R {
        TXDESC_EMPTY_ERROR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates whether CS-FORMAT has been programmed with an invalid value: this is a major software programming failure. 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn csformat_error(&self) -> CSFORMAT_ERROR_R {
        CSFORMAT_ERROR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates Link Layer Channel Map error, happens when actual number of CS-LLCHMAP bit set to one is different from CS-NBCHGOOD at the beginning of Frequency Hopping process 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn llchmap_error(&self) -> LLCHMAP_ERROR_R {
        LLCHMAP_ERROR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates Advertising Interval Under run, occurs if time between two consecutive Advertising packet (in Advertising mode) is lower than the expected value. 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn adv_underrun(&self) -> ADV_UNDERRUN_R {
        ADV_UNDERRUN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates Inter Frame Space Under run, occurs if IFS time is not enough to update and read Control Structure/Descriptors, and/or White List parsing is not finished and/or Decryption time is too long to be finished on time 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn ifs_underrun(&self) -> IFS_UNDERRUN_R {
        IFS_UNDERRUN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates White List Timeout error, occurs if White List parsing is not finished on time 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn whitelist_error(&self) -> WHITELIST_ERROR_R {
        WHITELIST_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates Anticipated Pre-Fetch Mechanism error: happens when 2 consecutive events are programmed, and when the first event is not completely finished while second pre-fetch instant is reached. 0: No error 1: Error occured"]
    #[inline(always)]
    pub fn evt_cntl_apfm_error(&self) -> EVT_CNTL_APFM_ERROR_R {
        EVT_CNTL_APFM_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates Anticipated Pre-Fetch Mechanism error: happens when 2 consecutive events are programmed, and when the first event is not completely finished while second pre-fetch instant is reached. 0: No error 1: Error occured"]
    #[inline(always)]
    pub fn evt_schdl_apfm_error(&self) -> EVT_SCHDL_APFM_ERROR_R {
        EVT_SCHDL_APFM_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates Event Scheduler faced Invalid timing programing on two consecutive ET entries (e.g first one with 624s offset and second one with no offset) 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn evt_schdl_entry_error(&self) -> EVT_SCHDL_ENTRY_ERROR_R {
        EVT_SCHDL_ENTRY_ERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates Event Scheduler Exchange Memory access error, happens when Exchange Memory accesses are not served in time, and blocks the Exchange Table entry read 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn evt_schdl_emacc_error(&self) -> EVT_SCHDL_EMACC_ERROR_R {
        EVT_SCHDL_EMACC_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates Radio Controller Exchange Memory access error, happens when Exchange Memory accesses are not served in time and data are corrupted. 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn radio_emacc_error(&self) -> RADIO_EMACC_ERROR_R {
        RADIO_EMACC_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates Packet Controller Exchange Memory access error, happens when Exchange Memory accesses are not served in time and Tx/Rx data are corrupted 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn pktcntl_emacc_error(&self) -> PKTCNTL_EMACC_ERROR_R {
        PKTCNTL_EMACC_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates real time decryption error, happens when AES-CCM decryption is too slow compared to Packet Controller requests. A 16-bytes block has to be decrypted prior the next block is received by the Packet Controller 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn rxcrypt_error(&self) -> RXCRYPT_ERROR_R {
        RXCRYPT_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Indicates Real Time encryption error, happens when AES-CCM encryption is too slow compared to Packet Controller requests. A 16-bytes block has to be encrypted and prepared on Packet Controller request, and needs to be ready before the Packet Controller has to send ti 0: No error 1: Error occurred"]
    #[inline(always)]
    pub fn txcrypt_error(&self) -> TXCRYPT_ERROR_R {
        TXCRYPT_ERROR_R::new((self.bits & 1) != 0)
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
#[doc = "Error Type Status registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_errortypestat_reg](index.html) module"]
pub struct BLE_ERRORTYPESTAT_REG_SPEC;
impl crate::RegisterSpec for BLE_ERRORTYPESTAT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_errortypestat_reg::R](R) reader structure"]
impl crate::Readable for BLE_ERRORTYPESTAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_errortypestat_reg::W](W) writer structure"]
impl crate::Writable for BLE_ERRORTYPESTAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_ERRORTYPESTAT_REG to value 0"]
impl crate::Resettable for BLE_ERRORTYPESTAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
