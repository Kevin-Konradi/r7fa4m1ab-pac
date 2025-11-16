#[doc = "Register `AGTMR1` reader"]
pub type R = crate::R<Agtmr1Spec>;
#[doc = "Register `AGTMR1` writer"]
pub type W = crate::W<Agtmr1Spec>;
#[doc = "Operating mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmod {
    #[doc = "0: Timer mode"]
    _000 = 0,
    #[doc = "1: Pulse output mode"]
    _001 = 1,
    #[doc = "2: Event counter mode"]
    _010 = 2,
    #[doc = "3: Pulse width measurement mode"]
    _011 = 3,
    #[doc = "4: Pulse period measurement mode."]
    _100 = 4,
    #[doc = "5: settings are prohibited"]
    Others = 5,
}
impl From<Tmod> for u8 {
    #[inline(always)]
    fn from(variant: Tmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmod {
    type Ux = u8;
}
impl crate::IsEnum for Tmod {}
#[doc = "Field `TMOD` reader - Operating mode"]
pub type TmodR = crate::FieldReader<Tmod>;
impl TmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmod {
        match self.bits {
            0 => Tmod::_000,
            1 => Tmod::_001,
            2 => Tmod::_010,
            3 => Tmod::_011,
            4 => Tmod::_100,
            _ => Tmod::Others,
        }
    }
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Tmod::_000
    }
    #[doc = "Pulse output mode"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Tmod::_001
    }
    #[doc = "Event counter mode"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Tmod::_010
    }
    #[doc = "Pulse width measurement mode"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Tmod::_011
    }
    #[doc = "Pulse period measurement mode."]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Tmod::_100
    }
    #[doc = "settings are prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Tmod::Others)
    }
}
#[doc = "Field `TMOD` writer - Operating mode"]
pub type TmodW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tmod, crate::Safe>;
impl<'a, REG> TmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::_000)
    }
    #[doc = "Pulse output mode"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::_001)
    }
    #[doc = "Event counter mode"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::_010)
    }
    #[doc = "Pulse width measurement mode"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::_011)
    }
    #[doc = "Pulse period measurement mode."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::_100)
    }
    #[doc = "settings are prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Tmod::Others)
    }
}
#[doc = "Edge polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tedgpl {
    #[doc = "0: Single-edge"]
    _0 = 0,
    #[doc = "1: Both-edge."]
    _1 = 1,
}
impl From<Tedgpl> for bool {
    #[inline(always)]
    fn from(variant: Tedgpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEDGPL` reader - Edge polarity"]
pub type TedgplR = crate::BitReader<Tedgpl>;
impl TedgplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tedgpl {
        match self.bits {
            false => Tedgpl::_0,
            true => Tedgpl::_1,
        }
    }
    #[doc = "Single-edge"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tedgpl::_0
    }
    #[doc = "Both-edge."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tedgpl::_1
    }
}
#[doc = "Field `TEDGPL` writer - Edge polarity"]
pub type TedgplW<'a, REG> = crate::BitWriter<'a, REG, Tedgpl>;
impl<'a, REG> TedgplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tedgpl::_0)
    }
    #[doc = "Both-edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tedgpl::_1)
    }
}
#[doc = "Count source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tck {
    #[doc = "0: PCLKB"]
    _000 = 0,
    #[doc = "1: PCLKB/8"]
    _001 = 1,
    #[doc = "3: PCLKB/2"]
    _011 = 3,
    #[doc = "4: Divided clock AGTLCLK specified by CKS\\[2:0\\] bits in the AGTMR2 register"]
    _100 = 4,
    #[doc = "5: Underflow event signal from AGT0*6"]
    _101 = 5,
    #[doc = "6: Divided clock AGTSCLK specified by CKS\\[2:0\\] bits in the AGTMR2 register."]
    _110 = 6,
    #[doc = "2: settings are prohibited."]
    Others = 2,
}
impl From<Tck> for u8 {
    #[inline(always)]
    fn from(variant: Tck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tck {
    type Ux = u8;
}
impl crate::IsEnum for Tck {}
#[doc = "Field `TCK` reader - Count source"]
pub type TckR = crate::FieldReader<Tck>;
impl TckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tck {
        match self.bits {
            0 => Tck::_000,
            1 => Tck::_001,
            3 => Tck::_011,
            4 => Tck::_100,
            5 => Tck::_101,
            6 => Tck::_110,
            _ => Tck::Others,
        }
    }
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Tck::_000
    }
    #[doc = "PCLKB/8"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Tck::_001
    }
    #[doc = "PCLKB/2"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Tck::_011
    }
    #[doc = "Divided clock AGTLCLK specified by CKS\\[2:0\\] bits in the AGTMR2 register"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Tck::_100
    }
    #[doc = "Underflow event signal from AGT0*6"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Tck::_101
    }
    #[doc = "Divided clock AGTSCLK specified by CKS\\[2:0\\] bits in the AGTMR2 register."]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Tck::_110
    }
    #[doc = "settings are prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Tck::Others)
    }
}
#[doc = "Field `TCK` writer - Count source"]
pub type TckW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tck, crate::Safe>;
impl<'a, REG> TckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLKB"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Tck::_000)
    }
    #[doc = "PCLKB/8"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Tck::_001)
    }
    #[doc = "PCLKB/2"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Tck::_011)
    }
    #[doc = "Divided clock AGTLCLK specified by CKS\\[2:0\\] bits in the AGTMR2 register"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Tck::_100)
    }
    #[doc = "Underflow event signal from AGT0*6"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Tck::_101)
    }
    #[doc = "Divided clock AGTSCLK specified by CKS\\[2:0\\] bits in the AGTMR2 register."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Tck::_110)
    }
    #[doc = "settings are prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Tck::Others)
    }
}
impl R {
    #[doc = "Bits 0:2 - Operating mode"]
    #[inline(always)]
    pub fn tmod(&self) -> TmodR {
        TmodR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Edge polarity"]
    #[inline(always)]
    pub fn tedgpl(&self) -> TedgplR {
        TedgplR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Count source"]
    #[inline(always)]
    pub fn tck(&self) -> TckR {
        TckR::new((self.bits >> 4) & 7)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGTMR1")
            .field("tck", &self.tck())
            .field("tedgpl", &self.tedgpl())
            .field("tmod", &self.tmod())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Operating mode"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TmodW<Agtmr1Spec> {
        TmodW::new(self, 0)
    }
    #[doc = "Bit 3 - Edge polarity"]
    #[inline(always)]
    pub fn tedgpl(&mut self) -> TedgplW<Agtmr1Spec> {
        TedgplW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Count source"]
    #[inline(always)]
    pub fn tck(&mut self) -> TckW<Agtmr1Spec> {
        TckW::new(self, 4)
    }
}
#[doc = "AGT Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`agtmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agtmr1Spec;
impl crate::RegisterSpec for Agtmr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`agtmr1::R`](R) reader structure"]
impl crate::Readable for Agtmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`agtmr1::W`](W) writer structure"]
impl crate::Writable for Agtmr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGTMR1 to value 0"]
impl crate::Resettable for Agtmr1Spec {}
