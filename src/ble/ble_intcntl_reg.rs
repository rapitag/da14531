#[doc = "Register `BLE_INTCNTL_REG` reader"]
pub struct R(crate::R<BLE_INTCNTL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_INTCNTL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_INTCNTL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_INTCNTL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_INTCNTL_REG` writer"]
pub struct W(crate::W<BLE_INTCNTL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_INTCNTL_REG_SPEC>;
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
impl From<crate::W<BLE_INTCNTL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_INTCNTL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSCNTDEVMSK` reader - CSCNT interrupt mask during event. This bit allows to enable CSCNT interrupt generation during events (i.e. advertising, scanning, initiating, and connection) 0: CSCNT Interrupt not generated during events. 1: CSCNT Interrupt generated during events."]
pub struct CSCNTDEVMSK_R(crate::FieldReader<bool, bool>);
impl CSCNTDEVMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSCNTDEVMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSCNTDEVMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSCNTDEVMSK` writer - CSCNT interrupt mask during event. This bit allows to enable CSCNT interrupt generation during events (i.e. advertising, scanning, initiating, and connection) 0: CSCNT Interrupt not generated during events. 1: CSCNT Interrupt generated during events."]
pub struct CSCNTDEVMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCNTDEVMSK_W<'a> {
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
#[doc = "Field `SWINTMSK` reader - SW triggered interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct SWINTMSK_R(crate::FieldReader<bool, bool>);
impl SWINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWINTMSK` writer - SW triggered interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct SWINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWINTMSK_W<'a> {
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
#[doc = "Field `EVENTAPFAINTMSK` reader - End of event / anticipated pre-fetch abort interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct EVENTAPFAINTMSK_R(crate::FieldReader<bool, bool>);
impl EVENTAPFAINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENTAPFAINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTAPFAINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTAPFAINTMSK` writer - End of event / anticipated pre-fetch abort interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct EVENTAPFAINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTAPFAINTMSK_W<'a> {
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
#[doc = "Field `FINETGTIMINTMSK` reader - Fine Target Timer Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct FINETGTIMINTMSK_R(crate::FieldReader<bool, bool>);
impl FINETGTIMINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FINETGTIMINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINETGTIMINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINETGTIMINTMSK` writer - Fine Target Timer Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct FINETGTIMINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETGTIMINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `GROSSTGTIMINTMSK` reader - Gross Target Timer Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct GROSSTGTIMINTMSK_R(crate::FieldReader<bool, bool>);
impl GROSSTGTIMINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GROSSTGTIMINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GROSSTGTIMINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GROSSTGTIMINTMSK` writer - Gross Target Timer Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct GROSSTGTIMINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GROSSTGTIMINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `ERRORINTMSK` reader - Error Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct ERRORINTMSK_R(crate::FieldReader<bool, bool>);
impl ERRORINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRORINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRORINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRORINTMSK` writer - Error Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct ERRORINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `CRYPTINTMSK` reader - Encryption engine Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct CRYPTINTMSK_R(crate::FieldReader<bool, bool>);
impl CRYPTINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPTINTMSK` writer - Encryption engine Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct CRYPTINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `EVENTINTMSK` reader - End of event Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct EVENTINTMSK_R(crate::FieldReader<bool, bool>);
impl EVENTINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVENTINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTINTMSK` writer - End of event Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct EVENTINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTINTMSK_W<'a> {
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
#[doc = "Field `SLPINTMSK` reader - Sleep Mode Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct SLPINTMSK_R(crate::FieldReader<bool, bool>);
impl SLPINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLPINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLPINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLPINTMSK` writer - Sleep Mode Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct SLPINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPINTMSK_W<'a> {
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
#[doc = "Field `RXINTMSK` reader - Rx Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct RXINTMSK_R(crate::FieldReader<bool, bool>);
impl RXINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXINTMSK` writer - Rx Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct RXINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINTMSK_W<'a> {
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
#[doc = "Field `CSCNTINTMSK` reader - 625us Base Time Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct CSCNTINTMSK_R(crate::FieldReader<bool, bool>);
impl CSCNTINTMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSCNTINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSCNTINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSCNTINTMSK` writer - 625us Base Time Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
pub struct CSCNTINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCNTINTMSK_W<'a> {
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
    #[doc = "Bit 15 - CSCNT interrupt mask during event. This bit allows to enable CSCNT interrupt generation during events (i.e. advertising, scanning, initiating, and connection) 0: CSCNT Interrupt not generated during events. 1: CSCNT Interrupt generated during events."]
    #[inline(always)]
    pub fn cscntdevmsk(&self) -> CSCNTDEVMSK_R {
        CSCNTDEVMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 9 - SW triggered interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn swintmsk(&self) -> SWINTMSK_R {
        SWINTMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - End of event / anticipated pre-fetch abort interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn eventapfaintmsk(&self) -> EVENTAPFAINTMSK_R {
        EVENTAPFAINTMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Fine Target Timer Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn finetgtimintmsk(&self) -> FINETGTIMINTMSK_R {
        FINETGTIMINTMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Gross Target Timer Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn grosstgtimintmsk(&self) -> GROSSTGTIMINTMSK_R {
        GROSSTGTIMINTMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn errorintmsk(&self) -> ERRORINTMSK_R {
        ERRORINTMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Encryption engine Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn cryptintmsk(&self) -> CRYPTINTMSK_R {
        CRYPTINTMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - End of event Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn eventintmsk(&self) -> EVENTINTMSK_R {
        EVENTINTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep Mode Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn slpintmsk(&self) -> SLPINTMSK_R {
        SLPINTMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Rx Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn rxintmsk(&self) -> RXINTMSK_R {
        RXINTMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 625us Base Time Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn cscntintmsk(&self) -> CSCNTINTMSK_R {
        CSCNTINTMSK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - CSCNT interrupt mask during event. This bit allows to enable CSCNT interrupt generation during events (i.e. advertising, scanning, initiating, and connection) 0: CSCNT Interrupt not generated during events. 1: CSCNT Interrupt generated during events."]
    #[inline(always)]
    pub fn cscntdevmsk(&mut self) -> CSCNTDEVMSK_W {
        CSCNTDEVMSK_W { w: self }
    }
    #[doc = "Bit 9 - SW triggered interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn swintmsk(&mut self) -> SWINTMSK_W {
        SWINTMSK_W { w: self }
    }
    #[doc = "Bit 8 - End of event / anticipated pre-fetch abort interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn eventapfaintmsk(&mut self) -> EVENTAPFAINTMSK_W {
        EVENTAPFAINTMSK_W { w: self }
    }
    #[doc = "Bit 7 - Fine Target Timer Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn finetgtimintmsk(&mut self) -> FINETGTIMINTMSK_W {
        FINETGTIMINTMSK_W { w: self }
    }
    #[doc = "Bit 6 - Gross Target Timer Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn grosstgtimintmsk(&mut self) -> GROSSTGTIMINTMSK_W {
        GROSSTGTIMINTMSK_W { w: self }
    }
    #[doc = "Bit 5 - Error Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn errorintmsk(&mut self) -> ERRORINTMSK_W {
        ERRORINTMSK_W { w: self }
    }
    #[doc = "Bit 4 - Encryption engine Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn cryptintmsk(&mut self) -> CRYPTINTMSK_W {
        CRYPTINTMSK_W { w: self }
    }
    #[doc = "Bit 3 - End of event Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn eventintmsk(&mut self) -> EVENTINTMSK_W {
        EVENTINTMSK_W { w: self }
    }
    #[doc = "Bit 2 - Sleep Mode Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn slpintmsk(&mut self) -> SLPINTMSK_W {
        SLPINTMSK_W { w: self }
    }
    #[doc = "Bit 1 - Rx Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn rxintmsk(&mut self) -> RXINTMSK_W {
        RXINTMSK_W { w: self }
    }
    #[doc = "Bit 0 - 625us Base Time Interrupt Mask 0: Interrupt not generated 1: Interrupt generated"]
    #[inline(always)]
    pub fn cscntintmsk(&mut self) -> CSCNTINTMSK_W {
        CSCNTINTMSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt controller register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_intcntl_reg](index.html) module"]
pub struct BLE_INTCNTL_REG_SPEC;
impl crate::RegisterSpec for BLE_INTCNTL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_intcntl_reg::R](R) reader structure"]
impl crate::Readable for BLE_INTCNTL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_intcntl_reg::W](W) writer structure"]
impl crate::Writable for BLE_INTCNTL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_INTCNTL_REG to value 0x811f"]
impl crate::Resettable for BLE_INTCNTL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x811f
    }
}
