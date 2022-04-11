#[doc = "Register `POR_PIN_REG` reader"]
pub struct R(crate::R<POR_PIN_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POR_PIN_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POR_PIN_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POR_PIN_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POR_PIN_REG` writer"]
pub struct W(crate::W<POR_PIN_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POR_PIN_REG_SPEC>;
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
impl From<crate::W<POR_PIN_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POR_PIN_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR_PIN_POLARITY` reader - 0: Active Low 1: Active High Note: This applies only for the GPIO pin. Reset pad has a fixed polarity"]
pub struct POR_PIN_POLARITY_R(crate::FieldReader<bool, bool>);
impl POR_PIN_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_PIN_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_PIN_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_PIN_POLARITY` writer - 0: Active Low 1: Active High Note: This applies only for the GPIO pin. Reset pad has a fixed polarity"]
pub struct POR_PIN_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_PIN_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `POR_PIN_SELECT` reader - Selects the GPIO which is used for POR generation. 0x0: GPIO pin POReset disabled 0x1: P0_0 0x2: P0_1 ... 0xB: P0_10 0xC: P0_11 0xD - 0xF: reserved"]
pub struct POR_PIN_SELECT_R(crate::FieldReader<u8, u8>);
impl POR_PIN_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        POR_PIN_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_PIN_SELECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_PIN_SELECT` writer - Selects the GPIO which is used for POR generation. 0x0: GPIO pin POReset disabled 0x1: P0_0 0x2: P0_1 ... 0xB: P0_10 0xC: P0_11 0xD - 0xF: reserved"]
pub struct POR_PIN_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_PIN_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - 0: Active Low 1: Active High Note: This applies only for the GPIO pin. Reset pad has a fixed polarity"]
    #[inline(always)]
    pub fn por_pin_polarity(&self) -> POR_PIN_POLARITY_R {
        POR_PIN_POLARITY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Selects the GPIO which is used for POR generation. 0x0: GPIO pin POReset disabled 0x1: P0_0 0x2: P0_1 ... 0xB: P0_10 0xC: P0_11 0xD - 0xF: reserved"]
    #[inline(always)]
    pub fn por_pin_select(&self) -> POR_PIN_SELECT_R {
        POR_PIN_SELECT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - 0: Active Low 1: Active High Note: This applies only for the GPIO pin. Reset pad has a fixed polarity"]
    #[inline(always)]
    pub fn por_pin_polarity(&mut self) -> POR_PIN_POLARITY_W {
        POR_PIN_POLARITY_W { w: self }
    }
    #[doc = "Bits 0:3 - Selects the GPIO which is used for POR generation. 0x0: GPIO pin POReset disabled 0x1: P0_0 0x2: P0_1 ... 0xB: P0_10 0xC: P0_11 0xD - 0xF: reserved"]
    #[inline(always)]
    pub fn por_pin_select(&mut self) -> POR_PIN_SELECT_W {
        POR_PIN_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects a GPIO pin for POR generation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [por_pin_reg](index.html) module"]
pub struct POR_PIN_REG_SPEC;
impl crate::RegisterSpec for POR_PIN_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [por_pin_reg::R](R) reader structure"]
impl crate::Readable for POR_PIN_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [por_pin_reg::W](W) writer structure"]
impl crate::Writable for POR_PIN_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POR_PIN_REG to value 0"]
impl crate::Resettable for POR_PIN_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
