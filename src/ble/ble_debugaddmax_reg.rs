#[doc = "Register `BLE_DEBUGADDMAX_REG` reader"]
pub struct R(crate::R<BLE_DEBUGADDMAX_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_DEBUGADDMAX_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_DEBUGADDMAX_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_DEBUGADDMAX_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_DEBUGADDMAX_REG` writer"]
pub struct W(crate::W<BLE_DEBUGADDMAX_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_DEBUGADDMAX_REG_SPEC>;
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
impl From<crate::W<BLE_DEBUGADDMAX_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_DEBUGADDMAX_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_ADDMAX` reader - Upper limit for the Register zone indicated by the reg_inzone flag"]
pub struct REG_ADDMAX_R(crate::FieldReader<u16, u16>);
impl REG_ADDMAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REG_ADDMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_ADDMAX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_ADDMAX` writer - Upper limit for the Register zone indicated by the reg_inzone flag"]
pub struct REG_ADDMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ADDMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `EM_ADDMAX` reader - Upper limit for the Exchange Memory zone indicated by the em_inzone flag"]
pub struct EM_ADDMAX_R(crate::FieldReader<u16, u16>);
impl EM_ADDMAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        EM_ADDMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM_ADDMAX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM_ADDMAX` writer - Upper limit for the Exchange Memory zone indicated by the em_inzone flag"]
pub struct EM_ADDMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_ADDMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Upper limit for the Register zone indicated by the reg_inzone flag"]
    #[inline(always)]
    pub fn reg_addmax(&self) -> REG_ADDMAX_R {
        REG_ADDMAX_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Upper limit for the Exchange Memory zone indicated by the em_inzone flag"]
    #[inline(always)]
    pub fn em_addmax(&self) -> EM_ADDMAX_R {
        EM_ADDMAX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Upper limit for the Register zone indicated by the reg_inzone flag"]
    #[inline(always)]
    pub fn reg_addmax(&mut self) -> REG_ADDMAX_W {
        REG_ADDMAX_W { w: self }
    }
    #[doc = "Bits 0:15 - Upper limit for the Exchange Memory zone indicated by the em_inzone flag"]
    #[inline(always)]
    pub fn em_addmax(&mut self) -> EM_ADDMAX_W {
        EM_ADDMAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper limit for the memory zone\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_debugaddmax_reg](index.html) module"]
pub struct BLE_DEBUGADDMAX_REG_SPEC;
impl crate::RegisterSpec for BLE_DEBUGADDMAX_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_debugaddmax_reg::R](R) reader structure"]
impl crate::Readable for BLE_DEBUGADDMAX_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_debugaddmax_reg::W](W) writer structure"]
impl crate::Writable for BLE_DEBUGADDMAX_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_DEBUGADDMAX_REG to value 0"]
impl crate::Resettable for BLE_DEBUGADDMAX_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
