#[doc = "Register `BLE_CNTL2_REG` reader"]
pub struct R(crate::R<BLE_CNTL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_CNTL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_CNTL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_CNTL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_CNTL2_REG` writer"]
pub struct W(crate::W<BLE_CNTL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_CNTL2_REG_SPEC>;
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
impl From<crate::W<BLE_CNTL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_CNTL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLE_PHY_ERR_MSK_N` reader - "]
pub struct BLE_PHY_ERR_MSK_N_R(crate::FieldReader<bool, bool>);
impl BLE_PHY_ERR_MSK_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_PHY_ERR_MSK_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_PHY_ERR_MSK_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_PHY_ERR_MSK_N` writer - "]
pub struct BLE_PHY_ERR_MSK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_PHY_ERR_MSK_N_W<'a> {
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
#[doc = "Field `BLE_ARP_ERR_MSK_N` reader - When cleared to \"0\" then it masks the BLE_ARP_ERR_STAT in order to not trigger a BLE_ERROR_IRQ."]
pub struct BLE_ARP_ERR_MSK_N_R(crate::FieldReader<bool, bool>);
impl BLE_ARP_ERR_MSK_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_ARP_ERR_MSK_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_ARP_ERR_MSK_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_ARP_ERR_MSK_N` writer - When cleared to \"0\" then it masks the BLE_ARP_ERR_STAT in order to not trigger a BLE_ERROR_IRQ."]
pub struct BLE_ARP_ERR_MSK_N_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_ARP_ERR_MSK_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `BLE_ARP_PHY_ERR_STAT` reader - When set to \"1\" then an error occured in BLE ARP sub-block and the BLE_GEN_IRQ will be aserted. It will be set if the ARP_ERROR or PHY_ERROR will be asserted and if the BLE_ARP_ERR_MSK is set to \"1\". Writing the value \"1\" will acknowledge and clear this field."]
pub struct BLE_ARP_PHY_ERR_STAT_R(crate::FieldReader<bool, bool>);
impl BLE_ARP_PHY_ERR_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_ARP_PHY_ERR_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_ARP_PHY_ERR_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_ARP_PHY_ERR_STAT` writer - When set to \"1\" then an error occured in BLE ARP sub-block and the BLE_GEN_IRQ will be aserted. It will be set if the ARP_ERROR or PHY_ERROR will be asserted and if the BLE_ARP_ERR_MSK is set to \"1\". Writing the value \"1\" will acknowledge and clear this field."]
pub struct BLE_ARP_PHY_ERR_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_ARP_PHY_ERR_STAT_W<'a> {
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
#[doc = "Field `BLE_RSSI_SEL` reader - 0: (default) Select Peak-hold RSSI value during the SYNC_FOUND event: CS->RXRSSI\\[7:0\\]
= RF_RSSI_RESULT_REG->RSSI_LATCHED_RD\\[9:2\\]. 1: Select the Average RSSI value during the SYNC_FOUND event: CS->RXRSSI\\[7:0\\]
= RF_RSSI_RESULT_REG->RSSI_AVG_RD\\[9:2\\]."]
pub struct BLE_RSSI_SEL_R(crate::FieldReader<bool, bool>);
impl BLE_RSSI_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_RSSI_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_RSSI_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_RSSI_SEL` writer - 0: (default) Select Peak-hold RSSI value during the SYNC_FOUND event: CS->RXRSSI\\[7:0\\]
= RF_RSSI_RESULT_REG->RSSI_LATCHED_RD\\[9:2\\]. 1: Select the Average RSSI value during the SYNC_FOUND event: CS->RXRSSI\\[7:0\\]
= RF_RSSI_RESULT_REG->RSSI_AVG_RD\\[9:2\\]."]
pub struct BLE_RSSI_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_RSSI_SEL_W<'a> {
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
#[doc = "Field `WAKEUPLPSTAT` reader - The status of the BLE_WAKEUP_LP_IRQ. The Interrupt Service Routine of BLE_WAKEUP_LP_IRQ should return only when the WAKEUPLPSTAT is cleared. Note that BLE_WAKEUP_LP_IRQ is automatically acknowledged after the power up of the Radio Subsystem, plus one Low Power Clock period."]
pub struct WAKEUPLPSTAT_R(crate::FieldReader<bool, bool>);
impl WAKEUPLPSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPLPSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPLPSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_RPL_SPI` reader - Keep to 0."]
pub struct SW_RPL_SPI_R(crate::FieldReader<bool, bool>);
impl SW_RPL_SPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_RPL_SPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_RPL_SPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_RPL_SPI` writer - Keep to 0."]
pub struct SW_RPL_SPI_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RPL_SPI_W<'a> {
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
#[doc = "Field `BB_ONLY` reader - Keep to 0."]
pub struct BB_ONLY_R(crate::FieldReader<bool, bool>);
impl BB_ONLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_ONLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_ONLY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB_ONLY` writer - Keep to 0."]
pub struct BB_ONLY_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_ONLY_W<'a> {
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
#[doc = "Field `BLE_PTI_SOURCE_SEL` reader - 0: Provide to COEX block the PTI value indicated by the Control Structure. Recommended value is \"0\". 1: Provide to COEX block the PTI value generated dynamically by the BLE core, which is based on the PTI of the Control Structure."]
pub struct BLE_PTI_SOURCE_SEL_R(crate::FieldReader<bool, bool>);
impl BLE_PTI_SOURCE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_PTI_SOURCE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_PTI_SOURCE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_PTI_SOURCE_SEL` writer - 0: Provide to COEX block the PTI value indicated by the Control Structure. Recommended value is \"0\". 1: Provide to COEX block the PTI value generated dynamically by the BLE core, which is based on the PTI of the Control Structure."]
pub struct BLE_PTI_SOURCE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_PTI_SOURCE_SEL_W<'a> {
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
#[doc = "Field `BLE_CLK_SEL` reader - BLE Clock Select. Specifies the BLE master clock absolute frequency in MHz. Typical values are 16 and 8. Value depends on the selected XTAL frequency and the value of CLK_RADIO_REG\\[BLE_DIV\\]
bitfield. For example, if XTAL oscillates at 16MHz and CLK_RADIO_REG\\[BLE_DIV\\]
= 1 (divide by 2), then BLE master clock frequency is 8MHz and BLE_CLK_SEL should be set to value 8. The selected BLE master clock frequency (affected by BLE_DIV and BLE_CLK_SEL) must be modified and set only during the initialization time, i.e. before setting BLE_RWBLECNTL_REG\\[RWBLE_EN\\]
to 1. Refer also to BLE_RWBLECONF_REG\\[CLK_SEL\\]."]
pub struct BLE_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl BLE_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLE_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_CLK_SEL` writer - BLE Clock Select. Specifies the BLE master clock absolute frequency in MHz. Typical values are 16 and 8. Value depends on the selected XTAL frequency and the value of CLK_RADIO_REG\\[BLE_DIV\\]
bitfield. For example, if XTAL oscillates at 16MHz and CLK_RADIO_REG\\[BLE_DIV\\]
= 1 (divide by 2), then BLE master clock frequency is 8MHz and BLE_CLK_SEL should be set to value 8. The selected BLE master clock frequency (affected by BLE_DIV and BLE_CLK_SEL) must be modified and set only during the initialization time, i.e. before setting BLE_RWBLECNTL_REG\\[RWBLE_EN\\]
to 1. Refer also to BLE_RWBLECONF_REG\\[CLK_SEL\\]."]
pub struct BLE_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 9)) | ((value as u32 & 0x3f) << 9);
        self.w
    }
}
#[doc = "Field `RADIO_PWRDN_ALLOW` reader - This active high signal indicates when it is allowed for the BLE core (embedded in the Radio sub-System power domain) to be powered down. After the assertion of the BLE_DEEPSLCNTL_REG\\[DEEP_SLEEP_ON\\]
a hardware sequence based on the Low Power clock will cause the assertion of RADIO_PWRDN_ALLOW. The RADIO_PWRDN_ALLOW will be cleared to \"0\" when the BLE core exits from the sleep state, i.e. when the BLE_SLP_IRQ will be asserted."]
pub struct RADIO_PWRDN_ALLOW_R(crate::FieldReader<bool, bool>);
impl RADIO_PWRDN_ALLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_PWRDN_ALLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_PWRDN_ALLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON_LP_CLK` reader - The SW can only write a \"0\" to this bit. Whenever a positive edge of the low power clock used by the BLE Timers is detected, then the HW will automatically set this bit to \"1\". This functionality will not work if BLE Timer is in reset state (refer to CLK_RADIO_REG\\[BLE_LP_RESET\\]). This bit can be used for SW synchronization, to debug the low power clock, etc."]
pub struct MON_LP_CLK_R(crate::FieldReader<bool, bool>);
impl MON_LP_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MON_LP_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON_LP_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_CLK_STAT` reader - 0: BLE uses low power clock 1: BLE uses master clock"]
pub struct BLE_CLK_STAT_R(crate::FieldReader<bool, bool>);
impl BLE_CLK_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_CLK_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_CLK_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_DIAG_OVR` reader - 1: Overrule BLE_DIAG. 0: BLE_DIAG is not overruled."]
pub struct BLE_DIAG_OVR_R(crate::FieldReader<bool, bool>);
impl BLE_DIAG_OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_DIAG_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_DIAG_OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_DIAG_OVR` writer - 1: Overrule BLE_DIAG. 0: BLE_DIAG is not overruled."]
pub struct BLE_DIAG_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_DIAG_OVR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `EMACCERRMSK` reader - Exchange Memory Access Error Mask: When cleared to \"0\" the EM_ACC_ERR will not cause an BLE_ERROR_IRQ interrupt. When set to \"1\" an BLE_ERROR_IRQ will be generated as long as EM_ACC_ERR is \"1\"."]
pub struct EMACCERRMSK_R(crate::FieldReader<bool, bool>);
impl EMACCERRMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMACCERRMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMACCERRMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMACCERRMSK` writer - Exchange Memory Access Error Mask: When cleared to \"0\" the EM_ACC_ERR will not cause an BLE_ERROR_IRQ interrupt. When set to \"1\" an BLE_ERROR_IRQ will be generated as long as EM_ACC_ERR is \"1\"."]
pub struct EMACCERRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EMACCERRMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `EMACCERRACK` writer - Exchange Memory Access Error Acknowledge. When the SW writes a \"1\" to this bit then the EMACCERRSTAT bit will be cleared. When the SW writes \"0\" it will have no affect. The read value is always \"0\"."]
pub struct EMACCERRACK_W<'a> {
    w: &'a mut W,
}
impl<'a> EMACCERRACK_W<'a> {
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
#[doc = "Field `EMACCERRSTAT` reader - Exchange Memory Access Error Status: The bit is read-only and can be cleared only by writing a \"1\" at EMACCERRACK bitfield. This bit will be set to \"1\" by the hardware when the controller will access an EM page that is not mapped according to the EM_MAPPING value. When this bit is \"1\" then the BLE_ERROR_IRQ will be asserted as long as EMACCERRMSK is \"1\"."]
pub struct EMACCERRSTAT_R(crate::FieldReader<bool, bool>);
impl EMACCERRSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMACCERRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMACCERRSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ble_phy_err_msk_n(&self) -> BLE_PHY_ERR_MSK_N_R {
        BLE_PHY_ERR_MSK_N_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - When cleared to \"0\" then it masks the BLE_ARP_ERR_STAT in order to not trigger a BLE_ERROR_IRQ."]
    #[inline(always)]
    pub fn ble_arp_err_msk_n(&self) -> BLE_ARP_ERR_MSK_N_R {
        BLE_ARP_ERR_MSK_N_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - When set to \"1\" then an error occured in BLE ARP sub-block and the BLE_GEN_IRQ will be aserted. It will be set if the ARP_ERROR or PHY_ERROR will be asserted and if the BLE_ARP_ERR_MSK is set to \"1\". Writing the value \"1\" will acknowledge and clear this field."]
    #[inline(always)]
    pub fn ble_arp_phy_err_stat(&self) -> BLE_ARP_PHY_ERR_STAT_R {
        BLE_ARP_PHY_ERR_STAT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - 0: (default) Select Peak-hold RSSI value during the SYNC_FOUND event: CS->RXRSSI\\[7:0\\]
= RF_RSSI_RESULT_REG->RSSI_LATCHED_RD\\[9:2\\]. 1: Select the Average RSSI value during the SYNC_FOUND event: CS->RXRSSI\\[7:0\\]
= RF_RSSI_RESULT_REG->RSSI_AVG_RD\\[9:2\\]."]
    #[inline(always)]
    pub fn ble_rssi_sel(&self) -> BLE_RSSI_SEL_R {
        BLE_RSSI_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - The status of the BLE_WAKEUP_LP_IRQ. The Interrupt Service Routine of BLE_WAKEUP_LP_IRQ should return only when the WAKEUPLPSTAT is cleared. Note that BLE_WAKEUP_LP_IRQ is automatically acknowledged after the power up of the Radio Subsystem, plus one Low Power Clock period."]
    #[inline(always)]
    pub fn wakeuplpstat(&self) -> WAKEUPLPSTAT_R {
        WAKEUPLPSTAT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Keep to 0."]
    #[inline(always)]
    pub fn sw_rpl_spi(&self) -> SW_RPL_SPI_R {
        SW_RPL_SPI_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Keep to 0."]
    #[inline(always)]
    pub fn bb_only(&self) -> BB_ONLY_R {
        BB_ONLY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: Provide to COEX block the PTI value indicated by the Control Structure. Recommended value is \"0\". 1: Provide to COEX block the PTI value generated dynamically by the BLE core, which is based on the PTI of the Control Structure."]
    #[inline(always)]
    pub fn ble_pti_source_sel(&self) -> BLE_PTI_SOURCE_SEL_R {
        BLE_PTI_SOURCE_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 9:14 - BLE Clock Select. Specifies the BLE master clock absolute frequency in MHz. Typical values are 16 and 8. Value depends on the selected XTAL frequency and the value of CLK_RADIO_REG\\[BLE_DIV\\]
bitfield. For example, if XTAL oscillates at 16MHz and CLK_RADIO_REG\\[BLE_DIV\\]
= 1 (divide by 2), then BLE master clock frequency is 8MHz and BLE_CLK_SEL should be set to value 8. The selected BLE master clock frequency (affected by BLE_DIV and BLE_CLK_SEL) must be modified and set only during the initialization time, i.e. before setting BLE_RWBLECNTL_REG\\[RWBLE_EN\\]
to 1. Refer also to BLE_RWBLECONF_REG\\[CLK_SEL\\]."]
    #[inline(always)]
    pub fn ble_clk_sel(&self) -> BLE_CLK_SEL_R {
        BLE_CLK_SEL_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - This active high signal indicates when it is allowed for the BLE core (embedded in the Radio sub-System power domain) to be powered down. After the assertion of the BLE_DEEPSLCNTL_REG\\[DEEP_SLEEP_ON\\]
a hardware sequence based on the Low Power clock will cause the assertion of RADIO_PWRDN_ALLOW. The RADIO_PWRDN_ALLOW will be cleared to \"0\" when the BLE core exits from the sleep state, i.e. when the BLE_SLP_IRQ will be asserted."]
    #[inline(always)]
    pub fn radio_pwrdn_allow(&self) -> RADIO_PWRDN_ALLOW_R {
        RADIO_PWRDN_ALLOW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - The SW can only write a \"0\" to this bit. Whenever a positive edge of the low power clock used by the BLE Timers is detected, then the HW will automatically set this bit to \"1\". This functionality will not work if BLE Timer is in reset state (refer to CLK_RADIO_REG\\[BLE_LP_RESET\\]). This bit can be used for SW synchronization, to debug the low power clock, etc."]
    #[inline(always)]
    pub fn mon_lp_clk(&self) -> MON_LP_CLK_R {
        MON_LP_CLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: BLE uses low power clock 1: BLE uses master clock"]
    #[inline(always)]
    pub fn ble_clk_stat(&self) -> BLE_CLK_STAT_R {
        BLE_CLK_STAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Overrule BLE_DIAG. 0: BLE_DIAG is not overruled."]
    #[inline(always)]
    pub fn ble_diag_ovr(&self) -> BLE_DIAG_OVR_R {
        BLE_DIAG_OVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Exchange Memory Access Error Mask: When cleared to \"0\" the EM_ACC_ERR will not cause an BLE_ERROR_IRQ interrupt. When set to \"1\" an BLE_ERROR_IRQ will be generated as long as EM_ACC_ERR is \"1\"."]
    #[inline(always)]
    pub fn emaccerrmsk(&self) -> EMACCERRMSK_R {
        EMACCERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Exchange Memory Access Error Status: The bit is read-only and can be cleared only by writing a \"1\" at EMACCERRACK bitfield. This bit will be set to \"1\" by the hardware when the controller will access an EM page that is not mapped according to the EM_MAPPING value. When this bit is \"1\" then the BLE_ERROR_IRQ will be asserted as long as EMACCERRMSK is \"1\"."]
    #[inline(always)]
    pub fn emaccerrstat(&self) -> EMACCERRSTAT_R {
        EMACCERRSTAT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ble_phy_err_msk_n(&mut self) -> BLE_PHY_ERR_MSK_N_W {
        BLE_PHY_ERR_MSK_N_W { w: self }
    }
    #[doc = "Bit 23 - When cleared to \"0\" then it masks the BLE_ARP_ERR_STAT in order to not trigger a BLE_ERROR_IRQ."]
    #[inline(always)]
    pub fn ble_arp_err_msk_n(&mut self) -> BLE_ARP_ERR_MSK_N_W {
        BLE_ARP_ERR_MSK_N_W { w: self }
    }
    #[doc = "Bit 22 - When set to \"1\" then an error occured in BLE ARP sub-block and the BLE_GEN_IRQ will be aserted. It will be set if the ARP_ERROR or PHY_ERROR will be asserted and if the BLE_ARP_ERR_MSK is set to \"1\". Writing the value \"1\" will acknowledge and clear this field."]
    #[inline(always)]
    pub fn ble_arp_phy_err_stat(&mut self) -> BLE_ARP_PHY_ERR_STAT_W {
        BLE_ARP_PHY_ERR_STAT_W { w: self }
    }
    #[doc = "Bit 21 - 0: (default) Select Peak-hold RSSI value during the SYNC_FOUND event: CS->RXRSSI\\[7:0\\]
= RF_RSSI_RESULT_REG->RSSI_LATCHED_RD\\[9:2\\]. 1: Select the Average RSSI value during the SYNC_FOUND event: CS->RXRSSI\\[7:0\\]
= RF_RSSI_RESULT_REG->RSSI_AVG_RD\\[9:2\\]."]
    #[inline(always)]
    pub fn ble_rssi_sel(&mut self) -> BLE_RSSI_SEL_W {
        BLE_RSSI_SEL_W { w: self }
    }
    #[doc = "Bit 19 - Keep to 0."]
    #[inline(always)]
    pub fn sw_rpl_spi(&mut self) -> SW_RPL_SPI_W {
        SW_RPL_SPI_W { w: self }
    }
    #[doc = "Bit 18 - Keep to 0."]
    #[inline(always)]
    pub fn bb_only(&mut self) -> BB_ONLY_W {
        BB_ONLY_W { w: self }
    }
    #[doc = "Bit 17 - 0: Provide to COEX block the PTI value indicated by the Control Structure. Recommended value is \"0\". 1: Provide to COEX block the PTI value generated dynamically by the BLE core, which is based on the PTI of the Control Structure."]
    #[inline(always)]
    pub fn ble_pti_source_sel(&mut self) -> BLE_PTI_SOURCE_SEL_W {
        BLE_PTI_SOURCE_SEL_W { w: self }
    }
    #[doc = "Bits 9:14 - BLE Clock Select. Specifies the BLE master clock absolute frequency in MHz. Typical values are 16 and 8. Value depends on the selected XTAL frequency and the value of CLK_RADIO_REG\\[BLE_DIV\\]
bitfield. For example, if XTAL oscillates at 16MHz and CLK_RADIO_REG\\[BLE_DIV\\]
= 1 (divide by 2), then BLE master clock frequency is 8MHz and BLE_CLK_SEL should be set to value 8. The selected BLE master clock frequency (affected by BLE_DIV and BLE_CLK_SEL) must be modified and set only during the initialization time, i.e. before setting BLE_RWBLECNTL_REG\\[RWBLE_EN\\]
to 1. Refer also to BLE_RWBLECONF_REG\\[CLK_SEL\\]."]
    #[inline(always)]
    pub fn ble_clk_sel(&mut self) -> BLE_CLK_SEL_W {
        BLE_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 3 - 1: Overrule BLE_DIAG. 0: BLE_DIAG is not overruled."]
    #[inline(always)]
    pub fn ble_diag_ovr(&mut self) -> BLE_DIAG_OVR_W {
        BLE_DIAG_OVR_W { w: self }
    }
    #[doc = "Bit 2 - Exchange Memory Access Error Mask: When cleared to \"0\" the EM_ACC_ERR will not cause an BLE_ERROR_IRQ interrupt. When set to \"1\" an BLE_ERROR_IRQ will be generated as long as EM_ACC_ERR is \"1\"."]
    #[inline(always)]
    pub fn emaccerrmsk(&mut self) -> EMACCERRMSK_W {
        EMACCERRMSK_W { w: self }
    }
    #[doc = "Bit 1 - Exchange Memory Access Error Acknowledge. When the SW writes a \"1\" to this bit then the EMACCERRSTAT bit will be cleared. When the SW writes \"0\" it will have no affect. The read value is always \"0\"."]
    #[inline(always)]
    pub fn emaccerrack(&mut self) -> EMACCERRACK_W {
        EMACCERRACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLE Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_cntl2_reg](index.html) module"]
pub struct BLE_CNTL2_REG_SPEC;
impl crate::RegisterSpec for BLE_CNTL2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_cntl2_reg::R](R) reader structure"]
impl crate::Readable for BLE_CNTL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_cntl2_reg::W](W) writer structure"]
impl crate::Writable for BLE_CNTL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_CNTL2_REG to value 0x04"]
impl crate::Resettable for BLE_CNTL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
