#[doc = "Register `BLE_AESCNTL_REG` reader"]
pub struct R(crate::R<BLE_AESCNTL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_AESCNTL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_AESCNTL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_AESCNTL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_AESCNTL_REG` writer"]
pub struct W(crate::W<BLE_AESCNTL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_AESCNTL_REG_SPEC>;
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
impl From<crate::W<BLE_AESCNTL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_AESCNTL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_MODE` reader - 0: Cipher mode 1: Decipher mode"]
pub struct AES_MODE_R(crate::FieldReader<bool, bool>);
impl AES_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AES_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_MODE` writer - 0: Cipher mode 1: Decipher mode"]
pub struct AES_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `AES_START` writer - Writing a 1 starts AES-128 ciphering/deciphering process. This bit is reset once the process is finished (i.e. ble_crypt_irq interrupt occurs, even masked)"]
pub struct AES_START_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - 0: Cipher mode 1: Decipher mode"]
    #[inline(always)]
    pub fn aes_mode(&self) -> AES_MODE_R {
        AES_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 0: Cipher mode 1: Decipher mode"]
    #[inline(always)]
    pub fn aes_mode(&mut self) -> AES_MODE_W {
        AES_MODE_W { w: self }
    }
    #[doc = "Bit 0 - Writing a 1 starts AES-128 ciphering/deciphering process. This bit is reset once the process is finished (i.e. ble_crypt_irq interrupt occurs, even masked)"]
    #[inline(always)]
    pub fn aes_start(&mut self) -> AES_START_W {
        AES_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start AES register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_aescntl_reg](index.html) module"]
pub struct BLE_AESCNTL_REG_SPEC;
impl crate::RegisterSpec for BLE_AESCNTL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_aescntl_reg::R](R) reader structure"]
impl crate::Readable for BLE_AESCNTL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_aescntl_reg::W](W) writer structure"]
impl crate::Writable for BLE_AESCNTL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_AESCNTL_REG to value 0"]
impl crate::Resettable for BLE_AESCNTL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
