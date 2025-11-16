#[doc = "Register `USBBCCTRL0` reader"]
pub type R = crate::R<Usbbcctrl0Spec>;
#[doc = "Register `USBBCCTRL0` writer"]
pub type W = crate::W<Usbbcctrl0Spec>;
#[doc = "D- Pin Pull-Down Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpdme0 {
    #[doc = "0: Pull-down off"]
    _0 = 0,
    #[doc = "1: Pull-down on"]
    _1 = 1,
}
impl From<Rpdme0> for bool {
    #[inline(always)]
    fn from(variant: Rpdme0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPDME0` reader - D- Pin Pull-Down Control"]
pub type Rpdme0R = crate::BitReader<Rpdme0>;
impl Rpdme0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpdme0 {
        match self.bits {
            false => Rpdme0::_0,
            true => Rpdme0::_1,
        }
    }
    #[doc = "Pull-down off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rpdme0::_0
    }
    #[doc = "Pull-down on"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rpdme0::_1
    }
}
#[doc = "Field `RPDME0` writer - D- Pin Pull-Down Control"]
pub type Rpdme0W<'a, REG> = crate::BitWriter<'a, REG, Rpdme0>;
impl<'a, REG> Rpdme0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull-down off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rpdme0::_0)
    }
    #[doc = "Pull-down on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rpdme0::_1)
    }
}
#[doc = "D+ Pin IDPSRC Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idpsrce0 {
    #[doc = "0: Stop"]
    _0 = 0,
    #[doc = "1: 10uA output"]
    _1 = 1,
}
impl From<Idpsrce0> for bool {
    #[inline(always)]
    fn from(variant: Idpsrce0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDPSRCE0` reader - D+ Pin IDPSRC Output Control"]
pub type Idpsrce0R = crate::BitReader<Idpsrce0>;
impl Idpsrce0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idpsrce0 {
        match self.bits {
            false => Idpsrce0::_0,
            true => Idpsrce0::_1,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idpsrce0::_0
    }
    #[doc = "10uA output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idpsrce0::_1
    }
}
#[doc = "Field `IDPSRCE0` writer - D+ Pin IDPSRC Output Control"]
pub type Idpsrce0W<'a, REG> = crate::BitWriter<'a, REG, Idpsrce0>;
impl<'a, REG> Idpsrce0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsrce0::_0)
    }
    #[doc = "10uA output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsrce0::_1)
    }
}
#[doc = "D- Pin 0.6 V Input Detection (Comparator and Sink) Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idmsinke0 {
    #[doc = "0: Detection off"]
    _0 = 0,
    #[doc = "1: Detection on ( Comparator and sink current on )"]
    _1 = 1,
}
impl From<Idmsinke0> for bool {
    #[inline(always)]
    fn from(variant: Idmsinke0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDMSINKE0` reader - D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
pub type Idmsinke0R = crate::BitReader<Idmsinke0>;
impl Idmsinke0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idmsinke0 {
        match self.bits {
            false => Idmsinke0::_0,
            true => Idmsinke0::_1,
        }
    }
    #[doc = "Detection off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idmsinke0::_0
    }
    #[doc = "Detection on ( Comparator and sink current on )"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idmsinke0::_1
    }
}
#[doc = "Field `IDMSINKE0` writer - D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
pub type Idmsinke0W<'a, REG> = crate::BitWriter<'a, REG, Idmsinke0>;
impl<'a, REG> Idmsinke0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idmsinke0::_0)
    }
    #[doc = "Detection on ( Comparator and sink current on )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idmsinke0::_1)
    }
}
#[doc = "D+ Pin VDPSRC (0.6 V) Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdpsrce0 {
    #[doc = "0: Stop"]
    _0 = 0,
    #[doc = "1: 0.6V output"]
    _1 = 1,
}
impl From<Vdpsrce0> for bool {
    #[inline(always)]
    fn from(variant: Vdpsrce0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDPSRCE0` reader - D+ Pin VDPSRC (0.6 V) Output Control"]
pub type Vdpsrce0R = crate::BitReader<Vdpsrce0>;
impl Vdpsrce0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdpsrce0 {
        match self.bits {
            false => Vdpsrce0::_0,
            true => Vdpsrce0::_1,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vdpsrce0::_0
    }
    #[doc = "0.6V output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vdpsrce0::_1
    }
}
#[doc = "Field `VDPSRCE0` writer - D+ Pin VDPSRC (0.6 V) Output Control"]
pub type Vdpsrce0W<'a, REG> = crate::BitWriter<'a, REG, Vdpsrce0>;
impl<'a, REG> Vdpsrce0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdpsrce0::_0)
    }
    #[doc = "0.6V output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdpsrce0::_1)
    }
}
#[doc = "D+ Pin 0.6 V Input Detection (Comparator and Sink) Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idpsinke0 {
    #[doc = "0: Detection off"]
    _0 = 0,
    #[doc = "1: Detection on ( Comparator and sink current on )"]
    _1 = 1,
}
impl From<Idpsinke0> for bool {
    #[inline(always)]
    fn from(variant: Idpsinke0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDPSINKE0` reader - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
pub type Idpsinke0R = crate::BitReader<Idpsinke0>;
impl Idpsinke0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idpsinke0 {
        match self.bits {
            false => Idpsinke0::_0,
            true => Idpsinke0::_1,
        }
    }
    #[doc = "Detection off"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idpsinke0::_0
    }
    #[doc = "Detection on ( Comparator and sink current on )"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idpsinke0::_1
    }
}
#[doc = "Field `IDPSINKE0` writer - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
pub type Idpsinke0W<'a, REG> = crate::BitWriter<'a, REG, Idpsinke0>;
impl<'a, REG> Idpsinke0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsinke0::_0)
    }
    #[doc = "Detection on ( Comparator and sink current on )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idpsinke0::_1)
    }
}
#[doc = "D- Pin VDMSRC (0.6 V) Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vdmsrce0 {
    #[doc = "0: Stop"]
    _0 = 0,
    #[doc = "1: 0.6V output"]
    _1 = 1,
}
impl From<Vdmsrce0> for bool {
    #[inline(always)]
    fn from(variant: Vdmsrce0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDMSRCE0` reader - D- Pin VDMSRC (0.6 V) Output Control"]
pub type Vdmsrce0R = crate::BitReader<Vdmsrce0>;
impl Vdmsrce0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vdmsrce0 {
        match self.bits {
            false => Vdmsrce0::_0,
            true => Vdmsrce0::_1,
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vdmsrce0::_0
    }
    #[doc = "0.6V output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vdmsrce0::_1
    }
}
#[doc = "Field `VDMSRCE0` writer - D- Pin VDMSRC (0.6 V) Output Control"]
pub type Vdmsrce0W<'a, REG> = crate::BitWriter<'a, REG, Vdmsrce0>;
impl<'a, REG> Vdmsrce0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vdmsrce0::_0)
    }
    #[doc = "0.6V output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vdmsrce0::_1)
    }
}
#[doc = "BC (Battery Charger) Function Ch0 General Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Batchge0 {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Batchge0> for bool {
    #[inline(always)]
    fn from(variant: Batchge0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BATCHGE0` reader - BC (Battery Charger) Function Ch0 General Enable Control"]
pub type Batchge0R = crate::BitReader<Batchge0>;
impl Batchge0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Batchge0 {
        match self.bits {
            false => Batchge0::_0,
            true => Batchge0::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Batchge0::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Batchge0::_1
    }
}
#[doc = "Field `BATCHGE0` writer - BC (Battery Charger) Function Ch0 General Enable Control"]
pub type Batchge0W<'a, REG> = crate::BitWriter<'a, REG, Batchge0>;
impl<'a, REG> Batchge0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Batchge0::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Batchge0::_1)
    }
}
#[doc = "D- Pin 0.6 V Input Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chgdetsts0 {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Detected"]
    _1 = 1,
}
impl From<Chgdetsts0> for bool {
    #[inline(always)]
    fn from(variant: Chgdetsts0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHGDETSTS0` reader - D- Pin 0.6 V Input Detection Status"]
pub type Chgdetsts0R = crate::BitReader<Chgdetsts0>;
impl Chgdetsts0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chgdetsts0 {
        match self.bits {
            false => Chgdetsts0::_0,
            true => Chgdetsts0::_1,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Chgdetsts0::_0
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Chgdetsts0::_1
    }
}
#[doc = "D+ Pin 0.6 V Input Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pddetsts0 {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Detected"]
    _1 = 1,
}
impl From<Pddetsts0> for bool {
    #[inline(always)]
    fn from(variant: Pddetsts0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDDETSTS0` reader - D+ Pin 0.6 V Input Detection Status"]
pub type Pddetsts0R = crate::BitReader<Pddetsts0>;
impl Pddetsts0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pddetsts0 {
        match self.bits {
            false => Pddetsts0::_0,
            true => Pddetsts0::_1,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pddetsts0::_0
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pddetsts0::_1
    }
}
impl R {
    #[doc = "Bit 0 - D- Pin Pull-Down Control"]
    #[inline(always)]
    pub fn rpdme0(&self) -> Rpdme0R {
        Rpdme0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - D+ Pin IDPSRC Output Control"]
    #[inline(always)]
    pub fn idpsrce0(&self) -> Idpsrce0R {
        Idpsrce0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub fn idmsinke0(&self) -> Idmsinke0R {
        Idmsinke0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - D+ Pin VDPSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdpsrce0(&self) -> Vdpsrce0R {
        Vdpsrce0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub fn idpsinke0(&self) -> Idpsinke0R {
        Idpsinke0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - D- Pin VDMSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdmsrce0(&self) -> Vdmsrce0R {
        Vdmsrce0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - BC (Battery Charger) Function Ch0 General Enable Control"]
    #[inline(always)]
    pub fn batchge0(&self) -> Batchge0R {
        Batchge0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - D- Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub fn chgdetsts0(&self) -> Chgdetsts0R {
        Chgdetsts0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - D+ Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub fn pddetsts0(&self) -> Pddetsts0R {
        Pddetsts0R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBBCCTRL0")
            .field("pddetsts0", &self.pddetsts0())
            .field("chgdetsts0", &self.chgdetsts0())
            .field("batchge0", &self.batchge0())
            .field("vdmsrce0", &self.vdmsrce0())
            .field("idpsinke0", &self.idpsinke0())
            .field("vdpsrce0", &self.vdpsrce0())
            .field("idmsinke0", &self.idmsinke0())
            .field("idpsrce0", &self.idpsrce0())
            .field("rpdme0", &self.rpdme0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - D- Pin Pull-Down Control"]
    #[inline(always)]
    pub fn rpdme0(&mut self) -> Rpdme0W<Usbbcctrl0Spec> {
        Rpdme0W::new(self, 0)
    }
    #[doc = "Bit 1 - D+ Pin IDPSRC Output Control"]
    #[inline(always)]
    pub fn idpsrce0(&mut self) -> Idpsrce0W<Usbbcctrl0Spec> {
        Idpsrce0W::new(self, 1)
    }
    #[doc = "Bit 2 - D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub fn idmsinke0(&mut self) -> Idmsinke0W<Usbbcctrl0Spec> {
        Idmsinke0W::new(self, 2)
    }
    #[doc = "Bit 3 - D+ Pin VDPSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdpsrce0(&mut self) -> Vdpsrce0W<Usbbcctrl0Spec> {
        Vdpsrce0W::new(self, 3)
    }
    #[doc = "Bit 4 - D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub fn idpsinke0(&mut self) -> Idpsinke0W<Usbbcctrl0Spec> {
        Idpsinke0W::new(self, 4)
    }
    #[doc = "Bit 5 - D- Pin VDMSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdmsrce0(&mut self) -> Vdmsrce0W<Usbbcctrl0Spec> {
        Vdmsrce0W::new(self, 5)
    }
    #[doc = "Bit 7 - BC (Battery Charger) Function Ch0 General Enable Control"]
    #[inline(always)]
    pub fn batchge0(&mut self) -> Batchge0W<Usbbcctrl0Spec> {
        Batchge0W::new(self, 7)
    }
}
#[doc = "BC Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`usbbcctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbbcctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usbbcctrl0Spec;
impl crate::RegisterSpec for Usbbcctrl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usbbcctrl0::R`](R) reader structure"]
impl crate::Readable for Usbbcctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`usbbcctrl0::W`](W) writer structure"]
impl crate::Writable for Usbbcctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBBCCTRL0 to value 0"]
impl crate::Resettable for Usbbcctrl0Spec {}
