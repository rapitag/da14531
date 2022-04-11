#[doc = "Register `BLE_BDADDRU_REG` reader"]
pub struct R(crate::R<BLE_BDADDRU_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_BDADDRU_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_BDADDRU_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_BDADDRU_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLE_BDADDRU_REG` writer"]
pub struct W(crate::W<BLE_BDADDRU_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLE_BDADDRU_REG_SPEC>;
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
impl From<crate::W<BLE_BDADDRU_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLE_BDADDRU_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIV_NPUB` reader - Bluetooth Low Energy Device Address privacy indicator 0: Public Bluetooth Device Address 1: Private Bluetooth Device Address"]
pub struct PRIV_NPUB_R(crate::FieldReader<bool, bool>);
impl PRIV_NPUB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRIV_NPUB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV_NPUB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV_NPUB` writer - Bluetooth Low Energy Device Address privacy indicator 0: Public Bluetooth Device Address 1: Private Bluetooth Device Address"]
pub struct PRIV_NPUB_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV_NPUB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `BDADDRU` reader - Bluetooth Low Energy Device Address. MSB part."]
pub struct BDADDRU_R(crate::FieldReader<u16, u16>);
impl BDADDRU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BDADDRU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDADDRU_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDADDRU` writer - Bluetooth Low Energy Device Address. MSB part."]
pub struct BDADDRU_W<'a> {
    w: &'a mut W,
}
impl<'a> BDADDRU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Bluetooth Low Energy Device Address privacy indicator 0: Public Bluetooth Device Address 1: Private Bluetooth Device Address"]
    #[inline(always)]
    pub fn priv_npub(&self) -> PRIV_NPUB_R {
        PRIV_NPUB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 0:15 - Bluetooth Low Energy Device Address. MSB part."]
    #[inline(always)]
    pub fn bdaddru(&self) -> BDADDRU_R {
        BDADDRU_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - Bluetooth Low Energy Device Address privacy indicator 0: Public Bluetooth Device Address 1: Private Bluetooth Device Address"]
    #[inline(always)]
    pub fn priv_npub(&mut self) -> PRIV_NPUB_W {
        PRIV_NPUB_W { w: self }
    }
    #[doc = "Bits 0:15 - Bluetooth Low Energy Device Address. MSB part."]
    #[inline(always)]
    pub fn bdaddru(&mut self) -> BDADDRU_W {
        BDADDRU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLE device address MSB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_bdaddru_reg](index.html) module"]
pub struct BLE_BDADDRU_REG_SPEC;
impl crate::RegisterSpec for BLE_BDADDRU_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_bdaddru_reg::R](R) reader structure"]
impl crate::Readable for BLE_BDADDRU_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ble_bdaddru_reg::W](W) writer structure"]
impl crate::Writable for BLE_BDADDRU_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLE_BDADDRU_REG to value 0"]
impl crate::Resettable for BLE_BDADDRU_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
