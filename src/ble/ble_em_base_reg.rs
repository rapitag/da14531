#[doc = "Register `BLE_EM_BASE_REG` reader"]
pub struct R(crate::R<BLE_EM_BASE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_EM_BASE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_EM_BASE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_EM_BASE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_EM_BASE_REG` writer"]
pub struct W(crate::W<BLE_EM_BASE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_EM_BASE_REG_SPEC>;
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
impl From<crate::W<BLE_EM_BASE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_EM_BASE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLE_EM_BASE_16_10` reader - The physical address on the system memory map of the base of the Exchange Memory."]
pub struct BLE_EM_BASE_16_10_R(crate::FieldReader<u8, u8>);
impl BLE_EM_BASE_16_10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLE_EM_BASE_16_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_EM_BASE_16_10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_EM_BASE_16_10` writer - The physical address on the system memory map of the base of the Exchange Memory."]
pub struct BLE_EM_BASE_16_10_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_EM_BASE_16_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 10)) | ((value as u32 & 0x7f) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:16 - The physical address on the system memory map of the base of the Exchange Memory."]
    #[inline(always)]
    pub fn ble_em_base_16_10(&self) -> BLE_EM_BASE_16_10_R {
        BLE_EM_BASE_16_10_R::new(((self.bits >> 10) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 10:16 - The physical address on the system memory map of the base of the Exchange Memory."]
    #[inline(always)]
    pub fn ble_em_base_16_10(&mut self) -> BLE_EM_BASE_16_10_W {
        BLE_EM_BASE_16_10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Exchange Memory Base Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_em_base_reg](index.html) module"]
pub struct BLE_EM_BASE_REG_SPEC;
impl crate::RegisterSpec for BLE_EM_BASE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_em_base_reg::R](R) reader structure"]
impl crate::Readable for BLE_EM_BASE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_em_base_reg::W](W) writer structure"]
impl crate::Writable for BLE_EM_BASE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_EM_BASE_REG to value 0"]
impl crate::Resettable for BLE_EM_BASE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
