#[doc = "Register `CACR2` reader"]
pub type R = crate::R<Cacr2Spec>;
#[doc = "Register `CACR2` writer"]
pub type W = crate::W<Cacr2Spec>;
#[doc = "Reference Signal Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rps {
    #[doc = "0: CACREF pin input"]
    _0 = 0,
    #[doc = "1: Internal clock (internally generated signal)"]
    _1 = 1,
}
impl From<Rps> for bool {
    #[inline(always)]
    fn from(variant: Rps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPS` reader - Reference Signal Select"]
pub type RpsR = crate::BitReader<Rps>;
impl RpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rps {
        match self.bits {
            false => Rps::_0,
            true => Rps::_1,
        }
    }
    #[doc = "CACREF pin input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rps::_0
    }
    #[doc = "Internal clock (internally generated signal)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rps::_1
    }
}
#[doc = "Field `RPS` writer - Reference Signal Select"]
pub type RpsW<'a, REG> = crate::BitWriter<'a, REG, Rps>;
impl<'a, REG> RpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CACREF pin input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rps::_0)
    }
    #[doc = "Internal clock (internally generated signal)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rps::_1)
    }
}
#[doc = "Measurement Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rscs {
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
impl From<Rscs> for u8 {
    #[inline(always)]
    fn from(variant: Rscs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rscs {
    type Ux = u8;
}
impl crate::IsEnum for Rscs {}
#[doc = "Field `RSCS` reader - Measurement Reference Clock Select"]
pub type RscsR = crate::FieldReader<Rscs>;
impl RscsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rscs {
        match self.bits {
            0 => Rscs::_000,
            1 => Rscs::_001,
            2 => Rscs::_010,
            3 => Rscs::_011,
            4 => Rscs::_100,
            5 => Rscs::_101,
            6 => Rscs::_110,
            7 => Rscs::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Rscs::_000
    }
    #[doc = "Sub-clock"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Rscs::_001
    }
    #[doc = "HOCO clock"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Rscs::_010
    }
    #[doc = "MOCO clock"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Rscs::_011
    }
    #[doc = "LOCO clock"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rscs::_100
    }
    #[doc = "Peripheral module clock(PCLKB)"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Rscs::_101
    }
    #[doc = "IWDTCLK clock"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Rscs::_110
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Rscs::_111
    }
}
#[doc = "Field `RSCS` writer - Measurement Reference Clock Select"]
pub type RscsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rscs, crate::Safe>;
impl<'a, REG> RscsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_000)
    }
    #[doc = "Sub-clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_001)
    }
    #[doc = "HOCO clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_010)
    }
    #[doc = "MOCO clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_011)
    }
    #[doc = "LOCO clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_100)
    }
    #[doc = "Peripheral module clock(PCLKB)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_101)
    }
    #[doc = "IWDTCLK clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Rscs::_111)
    }
}
#[doc = "Measurement Reference Clock Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcds {
    #[doc = "0: 1/32 clock"]
    _00 = 0,
    #[doc = "1: 1/128 clock"]
    _01 = 1,
    #[doc = "2: 1/1024 clock"]
    _10 = 2,
    #[doc = "3: 1/8192 clock"]
    _11 = 3,
}
impl From<Rcds> for u8 {
    #[inline(always)]
    fn from(variant: Rcds) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcds {
    type Ux = u8;
}
impl crate::IsEnum for Rcds {}
#[doc = "Field `RCDS` reader - Measurement Reference Clock Frequency Division Ratio Select"]
pub type RcdsR = crate::FieldReader<Rcds>;
impl RcdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcds {
        match self.bits {
            0 => Rcds::_00,
            1 => Rcds::_01,
            2 => Rcds::_10,
            3 => Rcds::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "1/32 clock"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Rcds::_00
    }
    #[doc = "1/128 clock"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Rcds::_01
    }
    #[doc = "1/1024 clock"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rcds::_10
    }
    #[doc = "1/8192 clock"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Rcds::_11
    }
}
#[doc = "Field `RCDS` writer - Measurement Reference Clock Frequency Division Ratio Select"]
pub type RcdsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rcds, crate::Safe>;
impl<'a, REG> RcdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/32 clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Rcds::_00)
    }
    #[doc = "1/128 clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Rcds::_01)
    }
    #[doc = "1/1024 clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rcds::_10)
    }
    #[doc = "1/8192 clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Rcds::_11)
    }
}
#[doc = "Digital Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dfs {
    #[doc = "0: Digital filtering is disabled."]
    _00 = 0,
    #[doc = "1: The sampling clock for the digital filter is the frequency measuring clock."]
    _01 = 1,
    #[doc = "2: The sampling clock for the digital filter is the frequency measuring clock divided by 4."]
    _10 = 2,
    #[doc = "3: The sampling clock for the digital filter is the frequency measuring clock divided by 16."]
    _11 = 3,
}
impl From<Dfs> for u8 {
    #[inline(always)]
    fn from(variant: Dfs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dfs {
    type Ux = u8;
}
impl crate::IsEnum for Dfs {}
#[doc = "Field `DFS` reader - Digital Filter Selection"]
pub type DfsR = crate::FieldReader<Dfs>;
impl DfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfs {
        match self.bits {
            0 => Dfs::_00,
            1 => Dfs::_01,
            2 => Dfs::_10,
            3 => Dfs::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Digital filtering is disabled."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dfs::_00
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Dfs::_01
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 4."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dfs::_10
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 16."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dfs::_11
    }
}
#[doc = "Field `DFS` writer - Digital Filter Selection"]
pub type DfsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dfs, crate::Safe>;
impl<'a, REG> DfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital filtering is disabled."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::_00)
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::_01)
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::_10)
    }
    #[doc = "The sampling clock for the digital filter is the frequency measuring clock divided by 16."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Reference Signal Select"]
    #[inline(always)]
    pub fn rps(&self) -> RpsR {
        RpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Measurement Reference Clock Select"]
    #[inline(always)]
    pub fn rscs(&self) -> RscsR {
        RscsR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:5 - Measurement Reference Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn rcds(&self) -> RcdsR {
        RcdsR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Digital Filter Selection"]
    #[inline(always)]
    pub fn dfs(&self) -> DfsR {
        DfsR::new((self.bits >> 6) & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACR2")
            .field("dfs", &self.dfs())
            .field("rcds", &self.rcds())
            .field("rscs", &self.rscs())
            .field("rps", &self.rps())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reference Signal Select"]
    #[inline(always)]
    pub fn rps(&mut self) -> RpsW<Cacr2Spec> {
        RpsW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Measurement Reference Clock Select"]
    #[inline(always)]
    pub fn rscs(&mut self) -> RscsW<Cacr2Spec> {
        RscsW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Measurement Reference Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn rcds(&mut self) -> RcdsW<Cacr2Spec> {
        RcdsW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Digital Filter Selection"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DfsW<Cacr2Spec> {
        DfsW::new(self, 6)
    }
}
#[doc = "CAC Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cacr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cacr2Spec;
impl crate::RegisterSpec for Cacr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cacr2::R`](R) reader structure"]
impl crate::Readable for Cacr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cacr2::W`](W) writer structure"]
impl crate::Writable for Cacr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACR2 to value 0"]
impl crate::Resettable for Cacr2Spec {}
