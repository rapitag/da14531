#[doc = "Register `BLE_SAMPLECLK_REG` reader"]
pub struct R(crate::R<BLE_SAMPLECLK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_SAMPLECLK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_SAMPLECLK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_SAMPLECLK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_SAMPLECLK_REG` writer"]
pub struct W(crate::W<BLE_SAMPLECLK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_SAMPLECLK_REG_SPEC>;
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
impl From<crate::W<BLE_SAMPLECLK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_SAMPLECLK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMP` writer - Writing a 1 samples the Base Time Counter value in BASETIMECNT register. Resets at 0 when action is performed."]
pub struct SAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_W<'a> {
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
    #[doc = "Bit 0 - Writing a 1 samples the Base Time Counter value in BASETIMECNT register. Resets at 0 when action is performed."]
    #[inline(always)]
    pub fn samp(&mut self) -> SAMP_W {
        SAMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Samples the Base Time Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_sampleclk_reg](index.html) module"]
pub struct BLE_SAMPLECLK_REG_SPEC;
impl crate::RegisterSpec for BLE_SAMPLECLK_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_sampleclk_reg::R](R) reader structure"]
impl crate::Readable for BLE_SAMPLECLK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_sampleclk_reg::W](W) writer structure"]
impl crate::Writable for BLE_SAMPLECLK_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_SAMPLECLK_REG to value 0"]
impl crate::Resettable for BLE_SAMPLECLK_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
