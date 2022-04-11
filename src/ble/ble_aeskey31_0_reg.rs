#[doc = "Register `BLE_AESKEY31_0_REG` reader"]
pub struct R(crate::R<BLE_AESKEY31_0_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_AESKEY31_0_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_AESKEY31_0_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_AESKEY31_0_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_AESKEY31_0_REG` writer"]
pub struct W(crate::W<BLE_AESKEY31_0_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_AESKEY31_0_REG_SPEC>;
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
impl From<crate::W<BLE_AESKEY31_0_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_AESKEY31_0_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESKEY31_0` reader - AES encryption 128-bit key. Bit 31 down to 0"]
pub struct AESKEY31_0_R(crate::FieldReader<u32, u32>);
impl AESKEY31_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        AESKEY31_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESKEY31_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESKEY31_0` writer - AES encryption 128-bit key. Bit 31 down to 0"]
pub struct AESKEY31_0_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEY31_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES encryption 128-bit key. Bit 31 down to 0"]
    #[inline(always)]
    pub fn aeskey31_0(&self) -> AESKEY31_0_R {
        AESKEY31_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES encryption 128-bit key. Bit 31 down to 0"]
    #[inline(always)]
    pub fn aeskey31_0(&mut self) -> AESKEY31_0_W {
        AESKEY31_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES encryption key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_aeskey31_0_reg](index.html) module"]
pub struct BLE_AESKEY31_0_REG_SPEC;
impl crate::RegisterSpec for BLE_AESKEY31_0_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_aeskey31_0_reg::R](R) reader structure"]
impl crate::Readable for BLE_AESKEY31_0_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_aeskey31_0_reg::W](W) writer structure"]
impl crate::Writable for BLE_AESKEY31_0_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_AESKEY31_0_REG to value 0"]
impl crate::Resettable for BLE_AESKEY31_0_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
