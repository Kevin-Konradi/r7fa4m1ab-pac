#[doc = "Register `CACR1` reader"]
pub type R = crate::R<Cacr1Spec>;
#[doc = "Register `CACR1` writer"]
pub type W = crate::W<Cacr1Spec>;
#[doc = "CACREF Pin Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cacrefe {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<Cacrefe> for bool {
    #[inline(always)]
    fn from(variant: Cacrefe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACREFE` reader - CACREF Pin Input Enable"]
pub type CacrefeR = crate::BitReader<Cacrefe>;
impl CacrefeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cacrefe {
        match self.bits {
            false => Cacrefe::_0,
            true => Cacrefe::_1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cacrefe::_0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cacrefe::_1
    }
}
#[doc = "Field `CACREFE` writer - CACREF Pin Input Enable"]
pub type CacrefeW<'a, REG> = crate::BitWriter<'a, REG, Cacrefe>;
impl<'a, REG> CacrefeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cacrefe::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cacrefe::_1)
    }
}
#[doc = "Measurement Target Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fmcs {
    #[doc = "0: Main clock"]
    _000 = 0,
    #[doc = "1: Sub-clock"]
    _001 = 1,
    #[doc = "2: HOCO clock"]
    _010 = 2,
    #[doc = "3: MOCO clock"]
    _011 = 3,
    #[doc = "4: LOCO clock"]
    _100 = 4,
    #[doc = "5: Peripheral module clock(PCLKB)"]
    _101 = 5,
    #[doc = "6: IWDTCLK clock"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<Fmcs> for u8 {
    #[inline(always)]
    fn from(variant: Fmcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fmcs {
    type Ux = u8;
}
impl crate::IsEnum for Fmcs {}
#[doc = "Field `FMCS` reader - Measurement Target Clock Select"]
pub type FmcsR = crate::FieldReader<Fmcs>;
impl FmcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fmcs {
        match self.bits {
            0 => Fmcs::_000,
            1 => Fmcs::_001,
            2 => Fmcs::_010,
            3 => Fmcs::_011,
            4 => Fmcs::_100,
            5 => Fmcs::_101,
            6 => Fmcs::_110,
            7 => Fmcs::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Fmcs::_000
    }
    #[doc = "Sub-clock"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Fmcs::_001
    }
    #[doc = "HOCO clock"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Fmcs::_010
    }
    #[doc = "MOCO clock"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Fmcs::_011
    }
    #[doc = "LOCO clock"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Fmcs::_100
    }
    #[doc = "Peripheral module clock(PCLKB)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Fmcs::_101
    }
    #[doc = "IWDTCLK clock"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Fmcs::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Fmcs::_111
    }
}
#[doc = "Field `FMCS` writer - Measurement Target Clock Select"]
pub type FmcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fmcs, crate::Safe>;
impl<'a, REG> FmcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_000)
    }
    #[doc = "Sub-clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_001)
    }
    #[doc = "HOCO clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_010)
    }
    #[doc = "MOCO clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_011)
    }
    #[doc = "LOCO clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_100)
    }
    #[doc = "Peripheral module clock(PCLKB)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_101)
    }
    #[doc = "IWDTCLK clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Fmcs::_111)
    }
}
#[doc = "Measurement Target Clock Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcss {
    #[doc = "0: No division"]
    _00 = 0,
    #[doc = "1: x 1/4 clock"]
    _01 = 1,
    #[doc = "2: x 1/8 clock"]
    _10 = 2,
    #[doc = "3: x 1/32 clock"]
    _11 = 3,
}
impl From<Tcss> for u8 {
    #[inline(always)]
    fn from(variant: Tcss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcss {
    type Ux = u8;
}
impl crate::IsEnum for Tcss {}
#[doc = "Field `TCSS` reader - Measurement Target Clock Frequency Division Ratio Select"]
pub type TcssR = crate::FieldReader<Tcss>;
impl TcssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcss {
        match self.bits {
            0 => Tcss::_00,
            1 => Tcss::_01,
            2 => Tcss::_10,
            3 => Tcss::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Tcss::_00
    }
    #[doc = "x 1/4 clock"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Tcss::_01
    }
    #[doc = "x 1/8 clock"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Tcss::_10
    }
    #[doc = "x 1/32 clock"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Tcss::_11
    }
}
#[doc = "Field `TCSS` writer - Measurement Target Clock Frequency Division Ratio Select"]
pub type TcssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tcss, crate::Safe>;
impl<'a, REG> TcssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No division"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_00)
    }
    #[doc = "x 1/4 clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_01)
    }
    #[doc = "x 1/8 clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_10)
    }
    #[doc = "x 1/32 clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Tcss::_11)
    }
}
#[doc = "Valid Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edges {
    #[doc = "0: Rising edge"]
    _00 = 0,
    #[doc = "1: Falling edge"]
    _01 = 1,
    #[doc = "2: Both rising and falling edges"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Edges> for u8 {
    #[inline(always)]
    fn from(variant: Edges) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edges {
    type Ux = u8;
}
impl crate::IsEnum for Edges {}
#[doc = "Field `EDGES` reader - Valid Edge Select"]
pub type EdgesR = crate::FieldReader<Edges>;
impl EdgesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edges {
        match self.bits {
            0 => Edges::_00,
            1 => Edges::_01,
            2 => Edges::_10,
            3 => Edges::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Edges::_00
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Edges::_01
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Edges::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Edges::_11
    }
}
#[doc = "Field `EDGES` writer - Valid Edge Select"]
pub type EdgesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edges, crate::Safe>;
impl<'a, REG> EdgesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Edges::_00)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Edges::_01)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Edges::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Edges::_11)
    }
}
impl R {
    #[doc = "Bit 0 - CACREF Pin Input Enable"]
    #[inline(always)]
    pub fn cacrefe(&self) -> CacrefeR {
        CacrefeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Measurement Target Clock Select"]
    #[inline(always)]
    pub fn fmcs(&self) -> FmcsR {
        FmcsR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:5 - Measurement Target Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn tcss(&self) -> TcssR {
        TcssR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Valid Edge Select"]
    #[inline(always)]
    pub fn edges(&self) -> EdgesR {
        EdgesR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACR1")
            .field("edges", &self.edges())
            .field("tcss", &self.tcss())
            .field("fmcs", &self.fmcs())
            .field("cacrefe", &self.cacrefe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CACREF Pin Input Enable"]
    #[inline(always)]
    pub fn cacrefe(&mut self) -> CacrefeW<Cacr1Spec> {
        CacrefeW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Measurement Target Clock Select"]
    #[inline(always)]
    pub fn fmcs(&mut self) -> FmcsW<Cacr1Spec> {
        FmcsW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Measurement Target Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn tcss(&mut self) -> TcssW<Cacr1Spec> {
        TcssW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Valid Edge Select"]
    #[inline(always)]
    pub fn edges(&mut self) -> EdgesW<Cacr1Spec> {
        EdgesW::new(self, 6)
    }
}
#[doc = "CAC Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cacr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cacr1Spec;
impl crate::RegisterSpec for Cacr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cacr1::R`](R) reader structure"]
impl crate::Readable for Cacr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cacr1::W`](W) writer structure"]
impl crate::Writable for Cacr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACR1 to value 0"]
impl crate::Resettable for Cacr1Spec {}
