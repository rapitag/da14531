#[doc = "Register `BLE_INTSTAT_REG` reader"]
pub struct R(crate::R<BLE_INTSTAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_INTSTAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_INTSTAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_INTSTAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_INTSTAT_REG` writer"]
pub struct W(crate::W<BLE_INTSTAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_INTSTAT_REG_SPEC>;
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
impl From<crate::W<BLE_INTSTAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_INTSTAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWINTSTAT` reader - SW triggered interrupt status 0: No SW triggered interrupt. 1: A SW triggered interrupt is pending"]
pub struct SWINTSTAT_R(crate::FieldReader<bool, bool>);
impl SWINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTAPFAINTSTAT` reader - End of event / Anticipated Pre-Fetch Abort interrupt status 0: No End of Event interrupt. 1: An End of Event interrupt is pending."]
pub struct EVENTAPFAINTSTAT_R(crate::FieldReader<bool, bool>);
impl EVENTAPFAINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENTAPFAINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTAPFAINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINETGTIMINTSTAT` reader - Masked Fine Target Timer Error interrupt status 0: No Fine Target Timer interrupt. 1: A Fine Target Timer interrupt is pending."]
pub struct FINETGTIMINTSTAT_R(crate::FieldReader<bool, bool>);
impl FINETGTIMINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FINETGTIMINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINETGTIMINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GROSSTGTIMINTSTAT` reader - Masked Gross Target Timer interrupt status 0: No Gross Target Timer interrupt. 1: A Gross Target Timer interrupt is pending."]
pub struct GROSSTGTIMINTSTAT_R(crate::FieldReader<bool, bool>);
impl GROSSTGTIMINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GROSSTGTIMINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GROSSTGTIMINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRORINTSTAT` reader - Masked Error interrupt status 0: No Error interrupt. 1: An Error interrupt is pending."]
pub struct ERRORINTSTAT_R(crate::FieldReader<bool, bool>);
impl ERRORINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRORINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRORINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTINTSTAT` reader - Masked Encryption engine interrupt status 0: No Encryption / Decryption interrupt. 1: An Encryption / Decryption interrupt is pending."]
pub struct CRYPTINTSTAT_R(crate::FieldReader<bool, bool>);
impl CRYPTINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTINTSTAT` reader - Masked End of Event interrupt status 0: No End of Advertising / Scanning / Connection interrupt. 1: An End of Advertising / Scanning / Connection interrupt is pending."]
pub struct EVENTINTSTAT_R(crate::FieldReader<bool, bool>);
impl EVENTINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENTINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLPINTSTAT` reader - Masked Sleep interrupt status 0: No End of Sleep Mode interrupt. 1: An End of Sleep Mode interrupt is pending."]
pub struct SLPINTSTAT_R(crate::FieldReader<bool, bool>);
impl SLPINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLPINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLPINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXINTSTAT` reader - Masked Packet Reception interrupt status 0: No Rx interrupt. 1: An Rx interrupt is pending."]
pub struct RXINTSTAT_R(crate::FieldReader<bool, bool>);
impl RXINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSCNTINTSTAT` reader - Masked 625us base time reference interrupt status 0: No 625us Base Time interrupt. 1: A 625us Base Time interrupt is pending."]
pub struct CSCNTINTSTAT_R(crate::FieldReader<bool, bool>);
impl CSCNTINTSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSCNTINTSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSCNTINTSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 9 - SW triggered interrupt status 0: No SW triggered interrupt. 1: A SW triggered interrupt is pending"]
    #[inline(always)]
    pub fn swintstat(&self) -> SWINTSTAT_R {
        SWINTSTAT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - End of event / Anticipated Pre-Fetch Abort interrupt status 0: No End of Event interrupt. 1: An End of Event interrupt is pending."]
    #[inline(always)]
    pub fn eventapfaintstat(&self) -> EVENTAPFAINTSTAT_R {
        EVENTAPFAINTSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Masked Fine Target Timer Error interrupt status 0: No Fine Target Timer interrupt. 1: A Fine Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn finetgtimintstat(&self) -> FINETGTIMINTSTAT_R {
        FINETGTIMINTSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Masked Gross Target Timer interrupt status 0: No Gross Target Timer interrupt. 1: A Gross Target Timer interrupt is pending."]
    #[inline(always)]
    pub fn grosstgtimintstat(&self) -> GROSSTGTIMINTSTAT_R {
        GROSSTGTIMINTSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked Error interrupt status 0: No Error interrupt. 1: An Error interrupt is pending."]
    #[inline(always)]
    pub fn errorintstat(&self) -> ERRORINTSTAT_R {
        ERRORINTSTAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked Encryption engine interrupt status 0: No Encryption / Decryption interrupt. 1: An Encryption / Decryption interrupt is pending."]
    #[inline(always)]
    pub fn cryptintstat(&self) -> CRYPTINTSTAT_R {
        CRYPTINTSTAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked End of Event interrupt status 0: No End of Advertising / Scanning / Connection interrupt. 1: An End of Advertising / Scanning / Connection interrupt is pending."]
    #[inline(always)]
    pub fn eventintstat(&self) -> EVENTINTSTAT_R {
        EVENTINTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked Sleep interrupt status 0: No End of Sleep Mode interrupt. 1: An End of Sleep Mode interrupt is pending."]
    #[inline(always)]
    pub fn slpintstat(&self) -> SLPINTSTAT_R {
        SLPINTSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Masked Packet Reception interrupt status 0: No Rx interrupt. 1: An Rx interrupt is pending."]
    #[inline(always)]
    pub fn rxintstat(&self) -> RXINTSTAT_R {
        RXINTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Masked 625us base time reference interrupt status 0: No 625us Base Time interrupt. 1: A 625us Base Time interrupt is pending."]
    #[inline(always)]
    pub fn cscntintstat(&self) -> CSCNTINTSTAT_R {
        CSCNTINTSTAT_R::new((self.bits & 1) != 0)
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
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_intstat_reg](index.html) module"]
pub struct BLE_INTSTAT_REG_SPEC;
impl crate::RegisterSpec for BLE_INTSTAT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_intstat_reg::R](R) reader structure"]
impl crate::Readable for BLE_INTSTAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_intstat_reg::W](W) writer structure"]
impl crate::Writable for BLE_INTSTAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_INTSTAT_REG to value 0"]
impl crate::Resettable for BLE_INTSTAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
