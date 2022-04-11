#[doc = "Register `BLE_AESKEY127_96_REG` reader"]
pub struct R(crate::R<BLE_AESKEY127_96_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_AESKEY127_96_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_AESKEY127_96_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_AESKEY127_96_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_AESKEY127_96_REG` writer"]
pub struct W(crate::W<BLE_AESKEY127_96_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_AESKEY127_96_REG_SPEC>;
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
impl From<crate::W<BLE_AESKEY127_96_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_AESKEY127_96_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESKEY127_96` reader - AES encryption 128-bit key. Bit 127 down to 96"]
pub struct AESKEY127_96_R(crate::FieldReader<u32, u32>);
impl AESKEY127_96_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        AESKEY127_96_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESKEY127_96_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESKEY127_96` writer - AES encryption 128-bit key. Bit 127 down to 96"]
pub struct AESKEY127_96_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY127_96_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES encryption 128-bit key. Bit 127 down to 96"]
    #[inline(always)]
    pub fn aeskey127_96(&self) -> AESKEY127_96_R {
        AESKEY127_96_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES encryption 128-bit key. Bit 127 down to 96"]
    #[inline(always)]
    pub fn aeskey127_96(&mut self) -> AESKEY127_96_W {
        AESKEY127_96_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES encryption key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_aeskey127_96_reg](index.html) module"]
pub struct BLE_AESKEY127_96_REG_SPEC;
impl crate::RegisterSpec for BLE_AESKEY127_96_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_aeskey127_96_reg::R](R) reader structure"]
impl crate::Readable for BLE_AESKEY127_96_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_aeskey127_96_reg::W](W) writer structure"]
impl crate::Writable for BLE_AESKEY127_96_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_AESKEY127_96_REG to value 0"]
impl crate::Resettable for BLE_AESKEY127_96_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
