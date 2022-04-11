#[doc = "Register `BLE_RWBLECNTL_REG` reader"]
pub struct R(crate::R<BLE_RWBLECNTL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_RWBLECNTL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_RWBLECNTL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_RWBLECNTL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_RWBLECNTL_REG` writer"]
pub struct W(crate::W<BLE_RWBLECNTL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_RWBLECNTL_REG_SPEC>;
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
impl From<crate::W<BLE_RWBLECNTL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_RWBLECNTL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER_SOFT_RST` writer - Reset the complete BLE Core except registers and timing generator, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
pub struct MASTER_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_SOFT_RST_W<'a> {
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
#[doc = "Field `MASTER_TGSOFT_RST` writer - Reset the timing generator, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
pub struct MASTER_TGSOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_TGSOFT_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `REG_SOFT_RST` reader - Reset the complete register block, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0. Note that INT STAT will not be cleared, so the user should also write to BLE_INTACK_REG after the SW Reset"]
pub struct REG_SOFT_RST_R(crate::FieldReader<bool, bool>);
impl REG_SOFT_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_SOFT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SOFT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_SOFT_RST` writer - Reset the complete register block, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0. Note that INT STAT will not be cleared, so the user should also write to BLE_INTACK_REG after the SW Reset"]
pub struct REG_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SOFT_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `SWINT_REQ` writer - Forces the generation of ble_sw_irq when written with a 1, and proper masking is set. Resets at 0 when action is performed. No action happens if it is written with 0."]
pub struct SWINT_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SWINT_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `RFTEST_ABORT` writer - Abort the current RF Testing defined as per CS-FORMAT when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0. Note that when RFTEST_ABORT is requested: 1) In case of infinite Tx, the Packet Controller FSM stops at the end of the current byte in process, and processes accordingly the packet CRC. 2) In case of Infinite Rx, the Packet Controller FSM either stops as the end of the current Packet reception (if Access address has been detected), or simply stop the processing switching off the RF."]
pub struct RFTEST_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> RFTEST_ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `ADVERT_ABORT` writer - Abort the current Advertising event when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
pub struct ADVERT_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVERT_ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `SCAN_ABORT` writer - Abort the current scan window when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
pub struct SCAN_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `MD_DSB` reader - 0: Normal operation of MD bits management 1: Allow a single Tx/Rx exchange whatever the MD bits are. value forced by SW from Tx Descriptorvalue just saved in Rx Descriptor during reception"]
pub struct MD_DSB_R(crate::FieldReader<bool, bool>);
impl MD_DSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MD_DSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_DSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MD_DSB` writer - 0: Normal operation of MD bits management 1: Allow a single Tx/Rx exchange whatever the MD bits are. value forced by SW from Tx Descriptorvalue just saved in Rx Descriptor during reception"]
pub struct MD_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_DSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `SN_DSB` reader - 0: Normal operation of Sequence number 1: Sequence Number Management disabled: value forced by SW from Tx Descriptorvalue ignored in Rx, where no SN error reported."]
pub struct SN_DSB_R(crate::FieldReader<bool, bool>);
impl SN_DSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SN_DSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SN_DSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SN_DSB` writer - 0: Normal operation of Sequence number 1: Sequence Number Management disabled: value forced by SW from Tx Descriptorvalue ignored in Rx, where no SN error reported."]
pub struct SN_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SN_DSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `NESN_DSB` reader - 0: Normal operation of Acknowledge 1: Acknowledge scheme disabled: value forced by SW from Tx Descriptorvalue ignored in Rx, where no NESN error reported."]
pub struct NESN_DSB_R(crate::FieldReader<bool, bool>);
impl NESN_DSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NESN_DSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NESN_DSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NESN_DSB` writer - 0: Normal operation of Acknowledge 1: Acknowledge scheme disabled: value forced by SW from Tx Descriptorvalue ignored in Rx, where no NESN error reported."]
pub struct NESN_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> NESN_DSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `CRYPT_DSB` reader - 0: Normal operation. Encryption / Decryption enabled. 1: Encryption / Decryption disabled. Note that if CS-CRYPT_EN is set, then MIC is generated, and only data encryption is disabled, meaning data sent are plain data."]
pub struct CRYPT_DSB_R(crate::FieldReader<bool, bool>);
impl CRYPT_DSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPT_DSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPT_DSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPT_DSB` writer - 0: Normal operation. Encryption / Decryption enabled. 1: Encryption / Decryption disabled. Note that if CS-CRYPT_EN is set, then MIC is generated, and only data encryption is disabled, meaning data sent are plain data."]
pub struct CRYPT_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPT_DSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `WHIT_DSB` reader - 0: Normal operation. Whitening enabled. 1: Whitening disabled."]
pub struct WHIT_DSB_R(crate::FieldReader<bool, bool>);
impl WHIT_DSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WHIT_DSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WHIT_DSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WHIT_DSB` writer - 0: Normal operation. Whitening enabled. 1: Whitening disabled."]
pub struct WHIT_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WHIT_DSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `CRC_DSB` reader - 0: Normal operation. CRC removed from data stream. 1: CRC stripping disabled on Rx packets, CRC replaced by 0x000 in Tx."]
pub struct CRC_DSB_R(crate::FieldReader<bool, bool>);
impl CRC_DSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_DSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_DSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_DSB` writer - 0: Normal operation. CRC removed from data stream. 1: CRC stripping disabled on Rx packets, CRC replaced by 0x000 in Tx."]
pub struct CRC_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_DSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `HOP_REMAP_DSB` reader - 0: Normal operation. Frequency Hopping Remapping algorithm enabled. 1: Frequency Hopping Remapping algorithm disabled"]
pub struct HOP_REMAP_DSB_R(crate::FieldReader<bool, bool>);
impl HOP_REMAP_DSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOP_REMAP_DSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOP_REMAP_DSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOP_REMAP_DSB` writer - 0: Normal operation. Frequency Hopping Remapping algorithm enabled. 1: Frequency Hopping Remapping algorithm disabled"]
pub struct HOP_REMAP_DSB_W<'a> {
    w: &'a mut W,
}
impl<'a> HOP_REMAP_DSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `ADVERTFILT_EN` reader - Advertising Channels Error Filtering Enable control 0: BLE Core reports all errors to RW-BLE Software 1: BLE Core reports only correctly received packet, without error to RW-BLE Software"]
pub struct ADVERTFILT_EN_R(crate::FieldReader<bool, bool>);
impl ADVERTFILT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADVERTFILT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADVERTFILT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADVERTFILT_EN` writer - Advertising Channels Error Filtering Enable control 0: BLE Core reports all errors to RW-BLE Software 1: BLE Core reports only correctly received packet, without error to RW-BLE Software"]
pub struct ADVERTFILT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVERTFILT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `RWBLE_EN` reader - 0: Disable BLE Core Exchange Table pre-fetch mechanism. 1: Enable BLE Core Exchange table pre-fetch mechanism."]
pub struct RWBLE_EN_R(crate::FieldReader<bool, bool>);
impl RWBLE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWBLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWBLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWBLE_EN` writer - 0: Disable BLE Core Exchange Table pre-fetch mechanism. 1: Enable BLE Core Exchange table pre-fetch mechanism."]
pub struct RWBLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWBLE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `RXWINSZDEF` reader - Default Rx Window size in us. Used when device: is master connectedperforms its second receipt.0 is not a valid value. Recommended value is 10 (in decimal)."]
pub struct RXWINSZDEF_R(crate::FieldReader<u8, u8>);
impl RXWINSZDEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXWINSZDEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXWINSZDEF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXWINSZDEF` writer - Default Rx Window size in us. Used when device: is master connectedperforms its second receipt.0 is not a valid value. Recommended value is 10 (in decimal)."]
pub struct RXWINSZDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWINSZDEF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SYNCERR` reader - Indicates the maximum number of errors allowed to recognize the synchronization word."]
pub struct SYNCERR_R(crate::FieldReader<u8, u8>);
impl SYNCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYNCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCERR` writer - Indicates the maximum number of errors allowed to recognize the synchronization word."]
pub struct SYNCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - Reset the complete register block, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0. Note that INT STAT will not be cleared, so the user should also write to BLE_INTACK_REG after the SW Reset"]
    #[inline(always)]
    pub fn reg_soft_rst(&self) -> REG_SOFT_RST_R {
        REG_SOFT_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 22 - 0: Normal operation of MD bits management 1: Allow a single Tx/Rx exchange whatever the MD bits are. value forced by SW from Tx Descriptorvalue just saved in Rx Descriptor during reception"]
    #[inline(always)]
    pub fn md_dsb(&self) -> MD_DSB_R {
        MD_DSB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - 0: Normal operation of Sequence number 1: Sequence Number Management disabled: value forced by SW from Tx Descriptorvalue ignored in Rx, where no SN error reported."]
    #[inline(always)]
    pub fn sn_dsb(&self) -> SN_DSB_R {
        SN_DSB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - 0: Normal operation of Acknowledge 1: Acknowledge scheme disabled: value forced by SW from Tx Descriptorvalue ignored in Rx, where no NESN error reported."]
    #[inline(always)]
    pub fn nesn_dsb(&self) -> NESN_DSB_R {
        NESN_DSB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - 0: Normal operation. Encryption / Decryption enabled. 1: Encryption / Decryption disabled. Note that if CS-CRYPT_EN is set, then MIC is generated, and only data encryption is disabled, meaning data sent are plain data."]
    #[inline(always)]
    pub fn crypt_dsb(&self) -> CRYPT_DSB_R {
        CRYPT_DSB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - 0: Normal operation. Whitening enabled. 1: Whitening disabled."]
    #[inline(always)]
    pub fn whit_dsb(&self) -> WHIT_DSB_R {
        WHIT_DSB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: Normal operation. CRC removed from data stream. 1: CRC stripping disabled on Rx packets, CRC replaced by 0x000 in Tx."]
    #[inline(always)]
    pub fn crc_dsb(&self) -> CRC_DSB_R {
        CRC_DSB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: Normal operation. Frequency Hopping Remapping algorithm enabled. 1: Frequency Hopping Remapping algorithm disabled"]
    #[inline(always)]
    pub fn hop_remap_dsb(&self) -> HOP_REMAP_DSB_R {
        HOP_REMAP_DSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 9 - Advertising Channels Error Filtering Enable control 0: BLE Core reports all errors to RW-BLE Software 1: BLE Core reports only correctly received packet, without error to RW-BLE Software"]
    #[inline(always)]
    pub fn advertfilt_en(&self) -> ADVERTFILT_EN_R {
        ADVERTFILT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Disable BLE Core Exchange Table pre-fetch mechanism. 1: Enable BLE Core Exchange table pre-fetch mechanism."]
    #[inline(always)]
    pub fn rwble_en(&self) -> RWBLE_EN_R {
        RWBLE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Default Rx Window size in us. Used when device: is master connectedperforms its second receipt.0 is not a valid value. Recommended value is 10 (in decimal)."]
    #[inline(always)]
    pub fn rxwinszdef(&self) -> RXWINSZDEF_R {
        RXWINSZDEF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - Indicates the maximum number of errors allowed to recognize the synchronization word."]
    #[inline(always)]
    pub fn syncerr(&self) -> SYNCERR_R {
        SYNCERR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Reset the complete BLE Core except registers and timing generator, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn master_soft_rst(&mut self) -> MASTER_SOFT_RST_W {
        MASTER_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 30 - Reset the timing generator, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn master_tgsoft_rst(&mut self) -> MASTER_TGSOFT_RST_W {
        MASTER_TGSOFT_RST_W { w: self }
    }
    #[doc = "Bit 29 - Reset the complete register block, when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0. Note that INT STAT will not be cleared, so the user should also write to BLE_INTACK_REG after the SW Reset"]
    #[inline(always)]
    pub fn reg_soft_rst(&mut self) -> REG_SOFT_RST_W {
        REG_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 28 - Forces the generation of ble_sw_irq when written with a 1, and proper masking is set. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn swint_req(&mut self) -> SWINT_REQ_W {
        SWINT_REQ_W { w: self }
    }
    #[doc = "Bit 26 - Abort the current RF Testing defined as per CS-FORMAT when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0. Note that when RFTEST_ABORT is requested: 1) In case of infinite Tx, the Packet Controller FSM stops at the end of the current byte in process, and processes accordingly the packet CRC. 2) In case of Infinite Rx, the Packet Controller FSM either stops as the end of the current Packet reception (if Access address has been detected), or simply stop the processing switching off the RF."]
    #[inline(always)]
    pub fn rftest_abort(&mut self) -> RFTEST_ABORT_W {
        RFTEST_ABORT_W { w: self }
    }
    #[doc = "Bit 25 - Abort the current Advertising event when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn advert_abort(&mut self) -> ADVERT_ABORT_W {
        ADVERT_ABORT_W { w: self }
    }
    #[doc = "Bit 24 - Abort the current scan window when written with a 1. Resets at 0 when action is performed. No action happens if it is written with 0."]
    #[inline(always)]
    pub fn scan_abort(&mut self) -> SCAN_ABORT_W {
        SCAN_ABORT_W { w: self }
    }
    #[doc = "Bit 22 - 0: Normal operation of MD bits management 1: Allow a single Tx/Rx exchange whatever the MD bits are. value forced by SW from Tx Descriptorvalue just saved in Rx Descriptor during reception"]
    #[inline(always)]
    pub fn md_dsb(&mut self) -> MD_DSB_W {
        MD_DSB_W { w: self }
    }
    #[doc = "Bit 21 - 0: Normal operation of Sequence number 1: Sequence Number Management disabled: value forced by SW from Tx Descriptorvalue ignored in Rx, where no SN error reported."]
    #[inline(always)]
    pub fn sn_dsb(&mut self) -> SN_DSB_W {
        SN_DSB_W { w: self }
    }
    #[doc = "Bit 20 - 0: Normal operation of Acknowledge 1: Acknowledge scheme disabled: value forced by SW from Tx Descriptorvalue ignored in Rx, where no NESN error reported."]
    #[inline(always)]
    pub fn nesn_dsb(&mut self) -> NESN_DSB_W {
        NESN_DSB_W { w: self }
    }
    #[doc = "Bit 19 - 0: Normal operation. Encryption / Decryption enabled. 1: Encryption / Decryption disabled. Note that if CS-CRYPT_EN is set, then MIC is generated, and only data encryption is disabled, meaning data sent are plain data."]
    #[inline(always)]
    pub fn crypt_dsb(&mut self) -> CRYPT_DSB_W {
        CRYPT_DSB_W { w: self }
    }
    #[doc = "Bit 18 - 0: Normal operation. Whitening enabled. 1: Whitening disabled."]
    #[inline(always)]
    pub fn whit_dsb(&mut self) -> WHIT_DSB_W {
        WHIT_DSB_W { w: self }
    }
    #[doc = "Bit 17 - 0: Normal operation. CRC removed from data stream. 1: CRC stripping disabled on Rx packets, CRC replaced by 0x000 in Tx."]
    #[inline(always)]
    pub fn crc_dsb(&mut self) -> CRC_DSB_W {
        CRC_DSB_W { w: self }
    }
    #[doc = "Bit 16 - 0: Normal operation. Frequency Hopping Remapping algorithm enabled. 1: Frequency Hopping Remapping algorithm disabled"]
    #[inline(always)]
    pub fn hop_remap_dsb(&mut self) -> HOP_REMAP_DSB_W {
        HOP_REMAP_DSB_W { w: self }
    }
    #[doc = "Bit 9 - Advertising Channels Error Filtering Enable control 0: BLE Core reports all errors to RW-BLE Software 1: BLE Core reports only correctly received packet, without error to RW-BLE Software"]
    #[inline(always)]
    pub fn advertfilt_en(&mut self) -> ADVERTFILT_EN_W {
        ADVERTFILT_EN_W { w: self }
    }
    #[doc = "Bit 8 - 0: Disable BLE Core Exchange Table pre-fetch mechanism. 1: Enable BLE Core Exchange table pre-fetch mechanism."]
    #[inline(always)]
    pub fn rwble_en(&mut self) -> RWBLE_EN_W {
        RWBLE_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - Default Rx Window size in us. Used when device: is master connectedperforms its second receipt.0 is not a valid value. Recommended value is 10 (in decimal)."]
    #[inline(always)]
    pub fn rxwinszdef(&mut self) -> RXWINSZDEF_W {
        RXWINSZDEF_W { w: self }
    }
    #[doc = "Bits 0:2 - Indicates the maximum number of errors allowed to recognize the synchronization word."]
    #[inline(always)]
    pub fn syncerr(&mut self) -> SYNCERR_W {
        SYNCERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLE Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_rwblecntl_reg](index.html) module"]
pub struct BLE_RWBLECNTL_REG_SPEC;
impl crate::RegisterSpec for BLE_RWBLECNTL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_rwblecntl_reg::R](R) reader structure"]
impl crate::Readable for BLE_RWBLECNTL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_rwblecntl_reg::W](W) writer structure"]
impl crate::Writable for BLE_RWBLECNTL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_RWBLECNTL_REG to value 0"]
impl crate::Resettable for BLE_RWBLECNTL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
