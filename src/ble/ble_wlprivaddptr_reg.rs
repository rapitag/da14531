#[doc = "Register `BLE_WLPRIVADDPTR_REG` reader"]
pub struct R(crate::R<BLE_WLPRIVADDPTR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_WLPRIVADDPTR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_WLPRIVADDPTR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_WLPRIVADDPTR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_WLPRIVADDPTR_REG` writer"]
pub struct W(crate::W<BLE_WLPRIVADDPTR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_WLPRIVADDPTR_REG_SPEC>;
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
impl From<crate::W<BLE_WLPRIVADDPTR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_WLPRIVADDPTR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLPRIVADDPTR` reader - Start address pointer of the private devices white list."]
pub struct WLPRIVADDPTR_R(crate::FieldReader<u16, u16>);
impl WLPRIVADDPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WLPRIVADDPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLPRIVADDPTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLPRIVADDPTR` writer - Start address pointer of the private devices white list."]
pub struct WLPRIVADDPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WLPRIVADDPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Start address pointer of the private devices white list."]
    #[inline(always)]
    pub fn wlprivaddptr(&self) -> WLPRIVADDPTR_R {
        WLPRIVADDPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start address pointer of the private devices white list."]
    #[inline(always)]
    pub fn wlprivaddptr(&mut self) -> WLPRIVADDPTR_W {
        WLPRIVADDPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start address of private devices list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_wlprivaddptr_reg](index.html) module"]
pub struct BLE_WLPRIVADDPTR_REG_SPEC;
impl crate::RegisterSpec for BLE_WLPRIVADDPTR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_wlprivaddptr_reg::R](R) reader structure"]
impl crate::Readable for BLE_WLPRIVADDPTR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_wlprivaddptr_reg::W](W) writer structure"]
impl crate::Writable for BLE_WLPRIVADDPTR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_WLPRIVADDPTR_REG to value 0"]
impl crate::Resettable for BLE_WLPRIVADDPTR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
