#[doc = "Register `BLE_INTRAWSTAT_REG` reader"]
pub struct R(crate::R<BLE_INTRAWSTAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_INTRAWSTAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_INTRAWSTAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_INTRAWSTAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_INTRAWSTAT_REG` writer"]
pub struct W(crate::W<BLE_INTRAWSTAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_INTRAWSTAT_REG_SPEC>;
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
impl From<crate::W<BLE_INTRAWSTAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_INTRAWSTAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWINTRAWSTAT` reader - SW triggered interrupt raw status 0: No SW triggered interrupt. 1: A SW triggered interrupt is pending."]
pub struct SWINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl SWINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTAPFAINTRAWSTAT` reader - End of event / Anticipated Pre-Fetch Abort interrupt raw status 0: No End of Event interrupt. 1: An End of Event interrupt is pending."]
pub struct EVENTAPFAINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl EVENTAPFAINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENTAPFAINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTAPFAINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINETGTIMINTRAWSTAT` reader - Fine Target Timer Error interrupt raw status 0: No Fine Target Timer interrupt. 1: A Fine Target Timer interrupt is pending."]
pub struct FINETGTIMINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl FINETGTIMINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FINETGTIMINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINETGTIMINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GROSSTGTIMINTRAWSTAT` reader - Gross Target Timer interrupt raw status 0: No Gross Target Timer interrupt. 1: A Gross Target Timer interrupt is pending."]
pub struct GROSSTGTIMINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl GROSSTGTIMINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GROSSTGTIMINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GROSSTGTIMINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRORINTRAWSTAT` reader - Error interrupt raw status 0: No Error interrupt. 1: An Error interrupt is pending."]
pub struct ERRORINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl ERRORINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRORINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRORINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTINTRAWSTAT` reader - Encryption engine interrupt raw status 0: No Encryption / Decryption interrupt. 1: An Encryption / Decryption interrupt is pending."]
pub struct CRYPTINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl CRYPTINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTINTRAWSTAT` reader - End of Event interrupt raw status 0: No End of Advertising / Scanning / Connection interrupt. 1: An End of Advertising / Scanning / Connection interrupt is pending."]
pub struct EVENTINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl EVENTINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENTINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLPINTRAWSTAT` reader - Sleep interrupt raw status 0: No End of Sleep Mode interrupt. 1: An End of Sleep Mode interrupt is pending."]
pub struct SLPINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl SLPINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLPINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLPINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXINTRAWSTAT` reader - Packet Reception interrupt raw status 0: No Rx interrupt. 1: An Rx interrupt is pending."]
pub struct RXINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl RXINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSCNTINTRAWSTAT` reader - 625us base time reference interrupt raw status 0: No 625us Base Time interrupt. 1: A 625us Base Time interrupt is pending."]
pub struct CSCNTINTRAWSTAT_R(crate::FieldReader<bool, bool>);
impl CSCNTINTRAWSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSCNTINTRAWSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSCNTINTRAWSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 9 - SW triggered interrupt raw status 0: No SW triggered interrupt. 1: A SW triggered interrupt is pending."]
    #[inline(always)]
    pub fn swintrawstat(&self) -> SWINTRAWSTAT_R {
        SWINTRAWSTAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - End of event / Anticipated Pre-Fetch Abort interrupt raw status 0: No End of Event interrupt. 1: An End of Event interrupt is pending."]
    #[inline(always)]
    pub fn eventapfaintrawstat(&self) -> EVENTAPFAINTRAWSTAT_R {
        EVENTAPFAINTRAWSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Fine Target Timer Error interrupt raw status 0: No Fine Target Timer interrupt. 1: A Fine Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn finetgtimintrawstat(&self) -> FINETGTIMINTRAWSTAT_R {
        FINETGTIMINTRAWSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Gross Target Timer interrupt raw status 0: No Gross Target Timer interrupt. 1: A Gross Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn grosstgtimintrawstat(&self) -> GROSSTGTIMINTRAWSTAT_R {
        GROSSTGTIMINTRAWSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt raw status 0: No Error interrupt. 1: An Error interrupt is pending."]
    #[inline(always)]
    pub fn errorintrawstat(&self) -> ERRORINTRAWSTAT_R {
        ERRORINTRAWSTAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Encryption engine interrupt raw status 0: No Encryption / Decryption interrupt. 1: An Encryption / Decryption interrupt is pending."]
    #[inline(always)]
    pub fn cryptintrawstat(&self) -> CRYPTINTRAWSTAT_R {
        CRYPTINTRAWSTAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Event interrupt raw status 0: No End of Advertising / Scanning / Connection interrupt. 1: An End of Advertising / Scanning / Connection interrupt is pending."]
    #[inline(always)]
    pub fn eventintrawstat(&self) -> EVENTINTRAWSTAT_R {
        EVENTINTRAWSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep interrupt raw status 0: No End of Sleep Mode interrupt. 1: An End of Sleep Mode interrupt is pending."]
    #[inline(always)]
    pub fn slpintrawstat(&self) -> SLPINTRAWSTAT_R {
        SLPINTRAWSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Packet Reception interrupt raw status 0: No Rx interrupt. 1: An Rx interrupt is pending."]
    #[inline(always)]
    pub fn rxintrawstat(&self) -> RXINTRAWSTAT_R {
        RXINTRAWSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 625us base time reference interrupt raw status 0: No 625us Base Time interrupt. 1: A 625us Base Time interrupt is pending."]
    #[inline(always)]
    pub fn cscntintrawstat(&self) -> CSCNTINTRAWSTAT_R {
        CSCNTINTRAWSTAT_R::new((self.bits & 1) != 0)
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
#[doc = "Interrupt raw status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_intrawstat_reg](index.html) module"]
pub struct BLE_INTRAWSTAT_REG_SPEC;
impl crate::RegisterSpec for BLE_INTRAWSTAT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_intrawstat_reg::R](R) reader structure"]
impl crate::Readable for BLE_INTRAWSTAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_intrawstat_reg::W](W) writer structure"]
impl crate::Writable for BLE_INTRAWSTAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_INTRAWSTAT_REG to value 0"]
impl crate::Resettable for BLE_INTRAWSTAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
