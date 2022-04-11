#[doc = "Register `BLE_RWBLECONF_REG` reader"]
pub struct R(crate::R<BLE_RWBLECONF_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_RWBLECONF_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_RWBLECONF_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_RWBLECONF_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_RWBLECONF_REG` writer"]
pub struct W(crate::W<BLE_RWBLECONF_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_RWBLECONF_REG_SPEC>;
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
impl From<crate::W<BLE_RWBLECONF_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_RWBLECONF_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD_WIDTH` reader - Value of the RW_BLE_ADDRESS_WIDTH parameter concerted into binary."]
pub struct ADD_WIDTH_R(crate::FieldReader<u8, u8>);
impl ADD_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADD_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFIF` reader - Radio Interface ID"]
pub struct RFIF_R(crate::FieldReader<u8, u8>);
impl RFIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_SEL` reader - Operating Frequency (in MHz)"]
pub struct CLK_SEL_R(crate::FieldReader<u8, u8>);
impl CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECIPHER` reader - 0: AES deciphering not present"]
pub struct DECIPHER_R(crate::FieldReader<bool, bool>);
impl DECIPHER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DECIPHER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECIPHER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMMODE` reader - 0: BLE Core is used as a standalone BLE device"]
pub struct DMMODE_R(crate::FieldReader<bool, bool>);
impl DMMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTMODE` reader - 1: Interrupts are trigger level generated, i.e. stays active at 1 till acknowledgement"]
pub struct INTMODE_R(crate::FieldReader<bool, bool>);
impl INTMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COEX` reader - 1: WLAN Coexistence mechanism present"]
pub struct COEX_R(crate::FieldReader<bool, bool>);
impl COEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USEDBG` reader - 1: Diagnostic port instantiated"]
pub struct USEDBG_R(crate::FieldReader<bool, bool>);
impl USEDBG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USEDBG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USEDBG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USECRYPT` reader - 1: AES-CCM Encryption block present"]
pub struct USECRYPT_R(crate::FieldReader<bool, bool>);
impl USECRYPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USECRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USECRYPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSWIDTH` reader - Processor bus width: 1: 32 bits"]
pub struct BUSWIDTH_R(crate::FieldReader<bool, bool>);
impl BUSWIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSWIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSWIDTH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:29 - Value of the RW_BLE_ADDRESS_WIDTH parameter concerted into binary."]
    #[inline(always)]
    pub fn add_width(&self) -> ADD_WIDTH_R {
        ADD_WIDTH_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Radio Interface ID"]
    #[inline(always)]
    pub fn rfif(&self) -> RFIF_R {
        RFIF_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Operating Frequency (in MHz)"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 0: AES deciphering not present"]
    #[inline(always)]
    pub fn decipher(&self) -> DECIPHER_R {
        DECIPHER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: BLE Core is used as a standalone BLE device"]
    #[inline(always)]
    pub fn dmmode(&self) -> DMMODE_R {
        DMMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Interrupts are trigger level generated, i.e. stays active at 1 till acknowledgement"]
    #[inline(always)]
    pub fn intmode(&self) -> INTMODE_R {
        INTMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: WLAN Coexistence mechanism present"]
    #[inline(always)]
    pub fn coex(&self) -> COEX_R {
        COEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Diagnostic port instantiated"]
    #[inline(always)]
    pub fn usedbg(&self) -> USEDBG_R {
        USEDBG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - 1: AES-CCM Encryption block present"]
    #[inline(always)]
    pub fn usecrypt(&self) -> USECRYPT_R {
        USECRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Processor bus width: 1: 32 bits"]
    #[inline(always)]
    pub fn buswidth(&self) -> BUSWIDTH_R {
        BUSWIDTH_R::new((self.bits & 1) != 0)
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
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_rwbleconf_reg](index.html) module"]
pub struct BLE_RWBLECONF_REG_SPEC;
impl crate::RegisterSpec for BLE_RWBLECONF_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_rwbleconf_reg::R](R) reader structure"]
impl crate::Readable for BLE_RWBLECONF_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_rwbleconf_reg::W](W) writer structure"]
impl crate::Writable for BLE_RWBLECONF_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_RWBLECONF_REG to value 0x0f02_001f"]
impl crate::Resettable for BLE_RWBLECONF_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f02_001f
    }
}
