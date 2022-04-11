#[doc = "Register `BLE_VERSION_REG` reader"]
pub struct R(crate::R<BLE_VERSION_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_VERSION_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_VERSION_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_VERSION_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_VERSION_REG` writer"]
pub struct W(crate::W<BLE_VERSION_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_VERSION_REG_SPEC>;
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
impl From<crate::W<BLE_VERSION_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_VERSION_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TYP` reader - BLE Core Type"]
pub struct TYP_R(crate::FieldReader<u8, u8>);
impl TYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REL` reader - BLE Core version Major release number."]
pub struct REL_R(crate::FieldReader<u8, u8>);
impl REL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPG` reader - BLE Core upgrade Upgrade number."]
pub struct UPG_R(crate::FieldReader<u8, u8>);
impl UPG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UPG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUILD` reader - BLE Core Build Build number."]
pub struct BUILD_R(crate::FieldReader<u8, u8>);
impl BUILD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BUILD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUILD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:31 - BLE Core Type"]
    #[inline(always)]
    pub fn typ(&self) -> TYP_R {
        TYP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - BLE Core version Major release number."]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BLE Core upgrade Upgrade number."]
    #[inline(always)]
    pub fn upg(&self) -> UPG_R {
        UPG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BLE Core Build Build number."]
    #[inline(always)]
    pub fn build(&self) -> BUILD_R {
        BUILD_R::new((self.bits & 0xff) as u8)
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
#[doc = "Version register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_version_reg](index.html) module"]
pub struct BLE_VERSION_REG_SPEC;
impl crate::RegisterSpec for BLE_VERSION_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_version_reg::R](R) reader structure"]
impl crate::Readable for BLE_VERSION_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_version_reg::W](W) writer structure"]
impl crate::Writable for BLE_VERSION_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_VERSION_REG to value 0x0701_0000"]
impl crate::Resettable for BLE_VERSION_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0701_0000
    }
}
