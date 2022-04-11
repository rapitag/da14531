#[doc = "Register `BLE_SWPROFILING_REG` reader"]
pub struct R(crate::R<BLE_SWPROFILING_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_SWPROFILING_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_SWPROFILING_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_SWPROFILING_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_SWPROFILING_REG` writer"]
pub struct W(crate::W<BLE_SWPROFILING_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_SWPROFILING_REG_SPEC>;
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
impl From<crate::W<BLE_SWPROFILING_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_SWPROFILING_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWPROFVAL` reader - Software Profiling register: used by BLE Software for profiling purpose: this value is copied on Diagnostic port"]
pub struct SWPROFVAL_R(crate::FieldReader<u32, u32>);
impl SWPROFVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SWPROFVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPROFVAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPROFVAL` writer - Software Profiling register: used by BLE Software for profiling purpose: this value is copied on Diagnostic port"]
pub struct SWPROFVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPROFVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Software Profiling register: used by BLE Software for profiling purpose: this value is copied on Diagnostic port"]
    #[inline(always)]
    pub fn swprofval(&self) -> SWPROFVAL_R {
        SWPROFVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Software Profiling register: used by BLE Software for profiling purpose: this value is copied on Diagnostic port"]
    #[inline(always)]
    pub fn swprofval(&mut self) -> SWPROFVAL_W {
        SWPROFVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Profiling register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_swprofiling_reg](index.html) module"]
pub struct BLE_SWPROFILING_REG_SPEC;
impl crate::RegisterSpec for BLE_SWPROFILING_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_swprofiling_reg::R](R) reader structure"]
impl crate::Readable for BLE_SWPROFILING_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_swprofiling_reg::W](W) writer structure"]
impl crate::Writable for BLE_SWPROFILING_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_SWPROFILING_REG to value 0"]
impl crate::Resettable for BLE_SWPROFILING_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
