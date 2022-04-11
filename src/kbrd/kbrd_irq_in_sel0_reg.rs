#[doc = "Register `KBRD_IRQ_IN_SEL0_REG` reader"]
pub struct R(crate::R<KBRD_IRQ_IN_SEL0_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KBRD_IRQ_IN_SEL0_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KBRD_IRQ_IN_SEL0_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KBRD_IRQ_IN_SEL0_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KBRD_IRQ_IN_SEL0_REG` writer"]
pub struct W(crate::W<KBRD_IRQ_IN_SEL0_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KBRD_IRQ_IN_SEL0_REG_SPEC>;
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
impl From<crate::W<KBRD_IRQ_IN_SEL0_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KBRD_IRQ_IN_SEL0_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KBRD_P11_EN` reader - enable P0\\[11\\]
for the keyboard interrupt"]
pub struct KBRD_P11_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P11_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P11_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P11_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P11_EN` writer - enable P0\\[11\\]
for the keyboard interrupt"]
pub struct KBRD_P11_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P11_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u16 & 1) << 11);
        self.w
    }
}
#[doc = "Field `KBRD_P10_EN` reader - enable P0\\[10\\]
for the keyboard interrupt"]
pub struct KBRD_P10_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P10_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P10_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P10_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P10_EN` writer - enable P0\\[10\\]
for the keyboard interrupt"]
pub struct KBRD_P10_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P10_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u16 & 1) << 10);
        self.w
    }
}
#[doc = "Field `KBRD_P09_EN` reader - enable P0\\[9\\]
for the keyboard interrupt"]
pub struct KBRD_P09_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P09_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P09_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P09_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P09_EN` writer - enable P0\\[9\\]
for the keyboard interrupt"]
pub struct KBRD_P09_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P09_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u16 & 1) << 9);
        self.w
    }
}
#[doc = "Field `KBRD_P08_EN` reader - enable P0\\[8\\]
for the keyboard interrupt"]
pub struct KBRD_P08_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P08_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P08_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P08_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P08_EN` writer - enable P0\\[8\\]
for the keyboard interrupt"]
pub struct KBRD_P08_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P08_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u16 & 1) << 8);
        self.w
    }
}
#[doc = "Field `KBRD_P07_EN` reader - enable P0\\[7\\]
for the keyboard interrupt"]
pub struct KBRD_P07_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P07_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P07_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P07_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P07_EN` writer - enable P0\\[7\\]
for the keyboard interrupt"]
pub struct KBRD_P07_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P07_EN_W<'a> {
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
#[doc = "Field `KBRD_P06_EN` reader - enable P0\\[6\\]
for the keyboard interrupt"]
pub struct KBRD_P06_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P06_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P06_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P06_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P06_EN` writer - enable P0\\[6\\]
for the keyboard interrupt"]
pub struct KBRD_P06_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P06_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `KBRD_P05_EN` reader - enable P0\\[5\\]
for the keyboard interrupt"]
pub struct KBRD_P05_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P05_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P05_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P05_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P05_EN` writer - enable P0\\[5\\]
for the keyboard interrupt"]
pub struct KBRD_P05_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P05_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `KBRD_P04_EN` reader - enable P0\\[4\\]
for the keyboard interrupt"]
pub struct KBRD_P04_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P04_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P04_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P04_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P04_EN` writer - enable P0\\[4\\]
for the keyboard interrupt"]
pub struct KBRD_P04_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P04_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
#[doc = "Field `KBRD_P03_EN` reader - enable P0\\[3\\]
for the keyboard interrupt"]
pub struct KBRD_P03_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P03_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P03_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P03_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P03_EN` writer - enable P0\\[3\\]
for the keyboard interrupt"]
pub struct KBRD_P03_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P03_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `KBRD_P02_EN` reader - enable P0\\[2\\]
for the keyboard interrupt"]
pub struct KBRD_P02_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P02_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P02_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P02_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P02_EN` writer - enable P0\\[2\\]
for the keyboard interrupt"]
pub struct KBRD_P02_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P02_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `KBRD_P01_EN` reader - enable P0\\[1\\]
for the keyboard interrupt"]
pub struct KBRD_P01_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P01_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P01_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P01_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P01_EN` writer - enable P0\\[1\\]
for the keyboard interrupt"]
pub struct KBRD_P01_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P01_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `KBRD_P00_EN` reader - enable P0\\[0\\]
for the keyboard interrupt"]
pub struct KBRD_P00_EN_R(crate::FieldReader<bool, bool>);
impl KBRD_P00_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_P00_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_P00_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_P00_EN` writer - enable P0\\[0\\]
for the keyboard interrupt"]
pub struct KBRD_P00_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_P00_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - enable P0\\[11\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p11_en(&self) -> KBRD_P11_EN_R {
        KBRD_P11_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - enable P0\\[10\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p10_en(&self) -> KBRD_P10_EN_R {
        KBRD_P10_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - enable P0\\[9\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p09_en(&self) -> KBRD_P09_EN_R {
        KBRD_P09_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - enable P0\\[8\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p08_en(&self) -> KBRD_P08_EN_R {
        KBRD_P08_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - enable P0\\[7\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p07_en(&self) -> KBRD_P07_EN_R {
        KBRD_P07_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - enable P0\\[6\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p06_en(&self) -> KBRD_P06_EN_R {
        KBRD_P06_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - enable P0\\[5\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p05_en(&self) -> KBRD_P05_EN_R {
        KBRD_P05_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - enable P0\\[4\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p04_en(&self) -> KBRD_P04_EN_R {
        KBRD_P04_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - enable P0\\[3\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p03_en(&self) -> KBRD_P03_EN_R {
        KBRD_P03_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - enable P0\\[2\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p02_en(&self) -> KBRD_P02_EN_R {
        KBRD_P02_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - enable P0\\[1\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p01_en(&self) -> KBRD_P01_EN_R {
        KBRD_P01_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - enable P0\\[0\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p00_en(&self) -> KBRD_P00_EN_R {
        KBRD_P00_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - enable P0\\[11\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p11_en(&mut self) -> KBRD_P11_EN_W {
        KBRD_P11_EN_W { w: self }
    }
    #[doc = "Bit 10 - enable P0\\[10\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p10_en(&mut self) -> KBRD_P10_EN_W {
        KBRD_P10_EN_W { w: self }
    }
    #[doc = "Bit 9 - enable P0\\[9\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p09_en(&mut self) -> KBRD_P09_EN_W {
        KBRD_P09_EN_W { w: self }
    }
    #[doc = "Bit 8 - enable P0\\[8\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p08_en(&mut self) -> KBRD_P08_EN_W {
        KBRD_P08_EN_W { w: self }
    }
    #[doc = "Bit 7 - enable P0\\[7\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p07_en(&mut self) -> KBRD_P07_EN_W {
        KBRD_P07_EN_W { w: self }
    }
    #[doc = "Bit 6 - enable P0\\[6\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p06_en(&mut self) -> KBRD_P06_EN_W {
        KBRD_P06_EN_W { w: self }
    }
    #[doc = "Bit 5 - enable P0\\[5\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p05_en(&mut self) -> KBRD_P05_EN_W {
        KBRD_P05_EN_W { w: self }
    }
    #[doc = "Bit 4 - enable P0\\[4\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p04_en(&mut self) -> KBRD_P04_EN_W {
        KBRD_P04_EN_W { w: self }
    }
    #[doc = "Bit 3 - enable P0\\[3\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p03_en(&mut self) -> KBRD_P03_EN_W {
        KBRD_P03_EN_W { w: self }
    }
    #[doc = "Bit 2 - enable P0\\[2\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p02_en(&mut self) -> KBRD_P02_EN_W {
        KBRD_P02_EN_W { w: self }
    }
    #[doc = "Bit 1 - enable P0\\[1\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p01_en(&mut self) -> KBRD_P01_EN_W {
        KBRD_P01_EN_W { w: self }
    }
    #[doc = "Bit 0 - enable P0\\[0\\]
for the keyboard interrupt"]
    #[inline(always)]
    pub fn kbrd_p00_en(&mut self) -> KBRD_P00_EN_W {
        KBRD_P00_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt selection for KBRD_IRQ for P0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kbrd_irq_in_sel0_reg](index.html) module"]
pub struct KBRD_IRQ_IN_SEL0_REG_SPEC;
impl crate::RegisterSpec for KBRD_IRQ_IN_SEL0_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [kbrd_irq_in_sel0_reg::R](R) reader structure"]
impl crate::Readable for KBRD_IRQ_IN_SEL0_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kbrd_irq_in_sel0_reg::W](W) writer structure"]
impl crate::Writable for KBRD_IRQ_IN_SEL0_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KBRD_IRQ_IN_SEL0_REG to value 0"]
impl crate::Resettable for KBRD_IRQ_IN_SEL0_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
