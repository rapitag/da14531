#[doc = "Register `BLE_WLNBDEV_REG` reader"]
pub struct R(crate::R<BLE_WLNBDEV_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_WLNBDEV_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_WLNBDEV_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_WLNBDEV_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_WLNBDEV_REG` writer"]
pub struct W(crate::W<BLE_WLNBDEV_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_WLNBDEV_REG_SPEC>;
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
impl From<crate::W<BLE_WLNBDEV_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_WLNBDEV_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBPRIVDEV` reader - Number of private devices in the white list."]
pub struct NBPRIVDEV_R(crate::FieldReader<u8, u8>);
impl NBPRIVDEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NBPRIVDEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBPRIVDEV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBPRIVDEV` writer - Number of private devices in the white list."]
pub struct NBPRIVDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> NBPRIVDEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `NBPUBDEV` reader - Number of public devices in the white list."]
pub struct NBPUBDEV_R(crate::FieldReader<u8, u8>);
impl NBPUBDEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NBPUBDEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBPUBDEV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBPUBDEV` writer - Number of public devices in the white list."]
pub struct NBPUBDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> NBPUBDEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Number of private devices in the white list."]
    #[inline(always)]
    pub fn nbprivdev(&self) -> NBPRIVDEV_R {
        NBPRIVDEV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Number of public devices in the white list."]
    #[inline(always)]
    pub fn nbpubdev(&self) -> NBPUBDEV_R {
        NBPUBDEV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Number of private devices in the white list."]
    #[inline(always)]
    pub fn nbprivdev(&mut self) -> NBPRIVDEV_W {
        NBPRIVDEV_W { w: self }
    }
    #[doc = "Bits 0:7 - Number of public devices in the white list."]
    #[inline(always)]
    pub fn nbpubdev(&mut self) -> NBPUBDEV_W {
        NBPUBDEV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Devices in white list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_wlnbdev_reg](index.html) module"]
pub struct BLE_WLNBDEV_REG_SPEC;
impl crate::RegisterSpec for BLE_WLNBDEV_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_wlnbdev_reg::R](R) reader structure"]
impl crate::Readable for BLE_WLNBDEV_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_wlnbdev_reg::W](W) writer structure"]
impl crate::Writable for BLE_WLNBDEV_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_WLNBDEV_REG to value 0"]
impl crate::Resettable for BLE_WLNBDEV_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
