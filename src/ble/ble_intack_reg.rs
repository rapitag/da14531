#[doc = "Register `BLE_INTACK_REG` reader"]
pub struct R(crate::R<BLE_INTACK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_INTACK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_INTACK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_INTACK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_INTACK_REG` writer"]
pub struct W(crate::W<BLE_INTACK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_INTACK_REG_SPEC>;
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
impl From<crate::W<BLE_INTACK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_INTACK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWINTACK` writer - SW triggered interrupt acknowledgement bit Software writing 1 acknowledges the SW triggered interrupt. This bit resets SWINTSTAT and SWINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct SWINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWINTACK_W<'a> {
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
#[doc = "Field `EVENTAPFAINTACK` writer - End of event / Anticipated Pre-Fetch Abort interrupt acknowledgement bit Software writing 1 acknowledges the End of event / Anticipated Pre-Fetch Abort interrupt. This bit resets EVENTAPFAINTSTAT and EVENTAPFAINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct EVENTAPFAINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTAPFAINTACK_W<'a> {
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
#[doc = "Field `FINETGTIMINTACK` writer - Fine Target Timer interrupt acknowledgement bit Software writing 1 acknowledges the Fine Timer interrupt. This bit resets FINETGTIMINTSTAT and FINETGTIMINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct FINETGTIMINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETGTIMINTACK_W<'a> {
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
#[doc = "Field `GROSSTGTIMINTACK` writer - Gross Target Timer interrupt acknowledgement bit Software writing 1 acknowledges the Gross Timer interrupt. This bit resets GROSSTGTIMINTSTAT and GROSSTGTIMINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct GROSSTGTIMINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> GROSSTGTIMINTACK_W<'a> {
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
#[doc = "Field `ERRORINTACK` writer - Error interrupt acknowledgement bit Software writing 1 acknowledges the Error interrupt. This bit resets ERRORINTSTAT and ERRORINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct ERRORINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORINTACK_W<'a> {
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
#[doc = "Field `CRYPTINTACK` writer - Encryption engine interrupt acknowledgement bit Software writing 1 acknowledges the Encryption engine interrupt. This bit resets CRYPTINTSTAT and CRYPTINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct CRYPTINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTINTACK_W<'a> {
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
#[doc = "Field `EVENTINTACK` writer - End of Event interrupt acknowledgment bit Software writing 1 acknowledges the End of Advertising / Scanning / Connection interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct EVENTINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTINTACK_W<'a> {
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
#[doc = "Field `SLPINTACK` writer - End of Deep Sleep interrupt acknowledgment bit Software writing 1 acknowledges the End of Sleep Mode interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct SLPINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPINTACK_W<'a> {
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
#[doc = "Field `RXINTACK` writer - Packet Reception interrupt acknowledgment bit Software writing 1 acknowledges the Rx interrupt. This bit resets RXINTSTAT and RXINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct RXINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINTACK_W<'a> {
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
#[doc = "Field `CSCNTINTACK` writer - 625us base time reference interrupt acknowledgment bit Software writing 1 acknowledges the CLKN interrupt. This bit resets CLKINTSTAT and CLKINTRAWSTAT flags. Resets at 0 when action is performed"]
pub struct CSCNTINTACK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCNTINTACK_W<'a> {
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
impl W {
    #[doc = "Bit 9 - SW triggered interrupt acknowledgement bit Software writing 1 acknowledges the SW triggered interrupt. This bit resets SWINTSTAT and SWINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn swintack(&mut self) -> SWINTACK_W {
        SWINTACK_W { w: self }
    }
    #[doc = "Bit 8 - End of event / Anticipated Pre-Fetch Abort interrupt acknowledgement bit Software writing 1 acknowledges the End of event / Anticipated Pre-Fetch Abort interrupt. This bit resets EVENTAPFAINTSTAT and EVENTAPFAINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn eventapfaintack(&mut self) -> EVENTAPFAINTACK_W {
        EVENTAPFAINTACK_W { w: self }
    }
    #[doc = "Bit 7 - Fine Target Timer interrupt acknowledgement bit Software writing 1 acknowledges the Fine Timer interrupt. This bit resets FINETGTIMINTSTAT and FINETGTIMINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn finetgtimintack(&mut self) -> FINETGTIMINTACK_W {
        FINETGTIMINTACK_W { w: self }
    }
    #[doc = "Bit 6 - Gross Target Timer interrupt acknowledgement bit Software writing 1 acknowledges the Gross Timer interrupt. This bit resets GROSSTGTIMINTSTAT and GROSSTGTIMINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn grosstgtimintack(&mut self) -> GROSSTGTIMINTACK_W {
        GROSSTGTIMINTACK_W { w: self }
    }
    #[doc = "Bit 5 - Error interrupt acknowledgement bit Software writing 1 acknowledges the Error interrupt. This bit resets ERRORINTSTAT and ERRORINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn errorintack(&mut self) -> ERRORINTACK_W {
        ERRORINTACK_W { w: self }
    }
    #[doc = "Bit 4 - Encryption engine interrupt acknowledgement bit Software writing 1 acknowledges the Encryption engine interrupt. This bit resets CRYPTINTSTAT and CRYPTINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn cryptintack(&mut self) -> CRYPTINTACK_W {
        CRYPTINTACK_W { w: self }
    }
    #[doc = "Bit 3 - End of Event interrupt acknowledgment bit Software writing 1 acknowledges the End of Advertising / Scanning / Connection interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn eventintack(&mut self) -> EVENTINTACK_W {
        EVENTINTACK_W { w: self }
    }
    #[doc = "Bit 2 - End of Deep Sleep interrupt acknowledgment bit Software writing 1 acknowledges the End of Sleep Mode interrupt. This bit resets SLPINTSTAT and SLPINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn slpintack(&mut self) -> SLPINTACK_W {
        SLPINTACK_W { w: self }
    }
    #[doc = "Bit 1 - Packet Reception interrupt acknowledgment bit Software writing 1 acknowledges the Rx interrupt. This bit resets RXINTSTAT and RXINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn rxintack(&mut self) -> RXINTACK_W {
        RXINTACK_W { w: self }
    }
    #[doc = "Bit 0 - 625us base time reference interrupt acknowledgment bit Software writing 1 acknowledges the CLKN interrupt. This bit resets CLKINTSTAT and CLKINTRAWSTAT flags. Resets at 0 when action is performed"]
    #[inline(always)]
    pub fn cscntintack(&mut self) -> CSCNTINTACK_W {
        CSCNTINTACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt acknowledge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_intack_reg](index.html) module"]
pub struct BLE_INTACK_REG_SPEC;
impl crate::RegisterSpec for BLE_INTACK_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_intack_reg::R](R) reader structure"]
impl crate::Readable for BLE_INTACK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_intack_reg::W](W) writer structure"]
impl crate::Writable for BLE_INTACK_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_INTACK_REG to value 0"]
impl crate::Resettable for BLE_INTACK_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
