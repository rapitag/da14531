#[doc = "Register `BLE_CURRENTRXDESCPTR_REG` reader"]
pub struct R(crate::R<BLE_CURRENTRXDESCPTR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_CURRENTRXDESCPTR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_CURRENTRXDESCPTR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_CURRENTRXDESCPTR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_CURRENTRXDESCPTR_REG` writer"]
pub struct W(crate::W<BLE_CURRENTRXDESCPTR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_CURRENTRXDESCPTR_REG_SPEC>;
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
impl From<crate::W<BLE_CURRENTRXDESCPTR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_CURRENTRXDESCPTR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETPTR` reader - Exchange Table Pointer that determines the starting point of the Exchange Table"]
pub struct ETPTR_R(crate::FieldReader<u16, u16>);
impl ETPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ETPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETPTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETPTR` writer - Exchange Table Pointer that determines the starting point of the Exchange Table"]
pub struct ETPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ETPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CURRENTRXDESCPTR` reader - Rx Descriptor Pointer that determines the starting point of the Receive Buffer Chained List"]
pub struct CURRENTRXDESCPTR_R(crate::FieldReader<u16, u16>);
impl CURRENTRXDESCPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CURRENTRXDESCPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRENTRXDESCPTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURRENTRXDESCPTR` writer - Rx Descriptor Pointer that determines the starting point of the Receive Buffer Chained List"]
pub struct CURRENTRXDESCPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENTRXDESCPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Exchange Table Pointer that determines the starting point of the Exchange Table"]
    #[inline(always)]
    pub fn etptr(&self) -> ETPTR_R {
        ETPTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:14 - Rx Descriptor Pointer that determines the starting point of the Receive Buffer Chained List"]
    #[inline(always)]
    pub fn currentrxdescptr(&self) -> CURRENTRXDESCPTR_R {
        CURRENTRXDESCPTR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Exchange Table Pointer that determines the starting point of the Exchange Table"]
    #[inline(always)]
    pub fn etptr(&mut self) -> ETPTR_W {
        ETPTR_W { w: self }
    }
    #[doc = "Bits 0:14 - Rx Descriptor Pointer that determines the starting point of the Receive Buffer Chained List"]
    #[inline(always)]
    pub fn currentrxdescptr(&mut self) -> CURRENTRXDESCPTR_W {
        CURRENTRXDESCPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Descriptor Pointer for the Receive Buffer Chained List\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_currentrxdescptr_reg](index.html) module"]
pub struct BLE_CURRENTRXDESCPTR_REG_SPEC;
impl crate::RegisterSpec for BLE_CURRENTRXDESCPTR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_currentrxdescptr_reg::R](R) reader structure"]
impl crate::Readable for BLE_CURRENTRXDESCPTR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_currentrxdescptr_reg::W](W) writer structure"]
impl crate::Writable for BLE_CURRENTRXDESCPTR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_CURRENTRXDESCPTR_REG to value 0"]
impl crate::Resettable for BLE_CURRENTRXDESCPTR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
