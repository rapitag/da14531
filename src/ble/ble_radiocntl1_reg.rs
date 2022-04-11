#[doc = "Register `BLE_RADIOCNTL1_REG` reader"]
pub struct R(crate::R<BLE_RADIOCNTL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_RADIOCNTL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_RADIOCNTL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_RADIOCNTL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_RADIOCNTL1_REG` writer"]
pub struct W(crate::W<BLE_RADIOCNTL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_RADIOCNTL1_REG_SPEC>;
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
impl From<crate::W<BLE_RADIOCNTL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_RADIOCNTL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XRFSEL` reader - Extended radio selection field, Must be set to \"2\"."]
pub struct XRFSEL_R(crate::FieldReader<u8, u8>);
impl XRFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XRFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XRFSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XRFSEL` writer - Extended radio selection field, Must be set to \"2\"."]
pub struct XRFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XRFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:20 - Extended radio selection field, Must be set to \"2\"."]
    #[inline(always)]
    pub fn xrfsel(&self) -> XRFSEL_R {
        XRFSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - Extended radio selection field, Must be set to \"2\"."]
    #[inline(always)]
    pub fn xrfsel(&mut self) -> XRFSEL_W {
        XRFSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Radio interface control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_radiocntl1_reg](index.html) module"]
pub struct BLE_RADIOCNTL1_REG_SPEC;
impl crate::RegisterSpec for BLE_RADIOCNTL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_radiocntl1_reg::R](R) reader structure"]
impl crate::Readable for BLE_RADIOCNTL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_radiocntl1_reg::W](W) writer structure"]
impl crate::Writable for BLE_RADIOCNTL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_RADIOCNTL1_REG to value 0"]
impl crate::Resettable for BLE_RADIOCNTL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
