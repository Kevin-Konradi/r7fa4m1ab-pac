#[doc = "Register `SPPCR` reader"]
pub type R = crate::R<SppcrSpec>;
#[doc = "Register `SPPCR` writer"]
pub type W = crate::W<SppcrSpec>;
#[doc = "RSPI Loopback\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Splp {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Loopback mode (data is inverted for transmission)"]
    _1 = 1,
}
impl From<Splp> for bool {
    #[inline(always)]
    fn from(variant: Splp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLP` reader - RSPI Loopback"]
pub type SplpR = crate::BitReader<Splp>;
impl SplpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Splp {
        match self.bits {
            false => Splp::_0,
            true => Splp::_1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Splp::_0
    }
    #[doc = "Loopback mode (data is inverted for transmission)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Splp::_1
    }
}
#[doc = "Field `SPLP` writer - RSPI Loopback"]
pub type SplpW<'a, REG> = crate::BitWriter<'a, REG, Splp>;
impl<'a, REG> SplpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Splp::_0)
    }
    #[doc = "Loopback mode (data is inverted for transmission)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Splp::_1)
    }
}
#[doc = "RSPI Loopback 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Splp2 {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Loopback mode (data is not inverted for transmission)"]
    _1 = 1,
}
impl From<Splp2> for bool {
    #[inline(always)]
    fn from(variant: Splp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPLP2` reader - RSPI Loopback 2"]
pub type Splp2R = crate::BitReader<Splp2>;
impl Splp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Splp2 {
        match self.bits {
            false => Splp2::_0,
            true => Splp2::_1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Splp2::_0
    }
    #[doc = "Loopback mode (data is not inverted for transmission)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Splp2::_1
    }
}
#[doc = "Field `SPLP2` writer - RSPI Loopback 2"]
pub type Splp2W<'a, REG> = crate::BitWriter<'a, REG, Splp2>;
impl<'a, REG> Splp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Splp2::_0)
    }
    #[doc = "Loopback mode (data is not inverted for transmission)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Splp2::_1)
    }
}
#[doc = "MOSI Idle Fixed Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moifv {
    #[doc = "0: The level output on the MOSIn pin during MOSI idling corresponds to low."]
    _0 = 0,
    #[doc = "1: The level output on the MOSIn pin during MOSI idling corresponds to high."]
    _1 = 1,
}
impl From<Moifv> for bool {
    #[inline(always)]
    fn from(variant: Moifv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOIFV` reader - MOSI Idle Fixed Value"]
pub type MoifvR = crate::BitReader<Moifv>;
impl MoifvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Moifv {
        match self.bits {
            false => Moifv::_0,
            true => Moifv::_1,
        }
    }
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Moifv::_0
    }
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Moifv::_1
    }
}
#[doc = "Field `MOIFV` writer - MOSI Idle Fixed Value"]
pub type MoifvW<'a, REG> = crate::BitWriter<'a, REG, Moifv>;
impl<'a, REG> MoifvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Moifv::_0)
    }
    #[doc = "The level output on the MOSIn pin during MOSI idling corresponds to high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Moifv::_1)
    }
}
#[doc = "MOSI Idle Value Fixing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moife {
    #[doc = "0: MOSI output value equals final data from previous transfer"]
    _0 = 0,
    #[doc = "1: MOSI output value equals the value set in the MOIFV bit"]
    _1 = 1,
}
impl From<Moife> for bool {
    #[inline(always)]
    fn from(variant: Moife) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOIFE` reader - MOSI Idle Value Fixing Enable"]
pub type MoifeR = crate::BitReader<Moife>;
impl MoifeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Moife {
        match self.bits {
            false => Moife::_0,
            true => Moife::_1,
        }
    }
    #[doc = "MOSI output value equals final data from previous transfer"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Moife::_0
    }
    #[doc = "MOSI output value equals the value set in the MOIFV bit"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Moife::_1
    }
}
#[doc = "Field `MOIFE` writer - MOSI Idle Value Fixing Enable"]
pub type MoifeW<'a, REG> = crate::BitWriter<'a, REG, Moife>;
impl<'a, REG> MoifeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOSI output value equals final data from previous transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Moife::_0)
    }
    #[doc = "MOSI output value equals the value set in the MOIFV bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Moife::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RSPI Loopback"]
    #[inline(always)]
    pub fn splp(&self) -> SplpR {
        SplpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSPI Loopback 2"]
    #[inline(always)]
    pub fn splp2(&self) -> Splp2R {
        Splp2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - MOSI Idle Fixed Value"]
    #[inline(always)]
    pub fn moifv(&self) -> MoifvR {
        MoifvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MOSI Idle Value Fixing Enable"]
    #[inline(always)]
    pub fn moife(&self) -> MoifeR {
        MoifeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPPCR")
            .field("moife", &self.moife())
            .field("moifv", &self.moifv())
            .field("splp2", &self.splp2())
            .field("splp", &self.splp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RSPI Loopback"]
    #[inline(always)]
    pub fn splp(&mut self) -> SplpW<SppcrSpec> {
        SplpW::new(self, 0)
    }
    #[doc = "Bit 1 - RSPI Loopback 2"]
    #[inline(always)]
    pub fn splp2(&mut self) -> Splp2W<SppcrSpec> {
        Splp2W::new(self, 1)
    }
    #[doc = "Bit 4 - MOSI Idle Fixed Value"]
    #[inline(always)]
    pub fn moifv(&mut self) -> MoifvW<SppcrSpec> {
        MoifvW::new(self, 4)
    }
    #[doc = "Bit 5 - MOSI Idle Value Fixing Enable"]
    #[inline(always)]
    pub fn moife(&mut self) -> MoifeW<SppcrSpec> {
        MoifeW::new(self, 5)
    }
}
#[doc = "SPI Pin Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sppcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sppcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SppcrSpec;
impl crate::RegisterSpec for SppcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sppcr::R`](R) reader structure"]
impl crate::Readable for SppcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sppcr::W`](W) writer structure"]
impl crate::Writable for SppcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPPCR to value 0"]
impl crate::Resettable for SppcrSpec {}
