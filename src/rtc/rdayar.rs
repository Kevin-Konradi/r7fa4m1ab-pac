#[doc = "Register `RDAYAR` reader"]
pub type R = crate::R<RdayarSpec>;
#[doc = "Register `RDAYAR` writer"]
pub type W = crate::W<RdayarSpec>;
#[doc = "Field `DATE1` reader - 1 Day Value for the ones place of days"]
pub type Date1R = crate::FieldReader;
#[doc = "Field `DATE1` writer - 1 Day Value for the ones place of days"]
pub type Date1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATE10` reader - 10 Days Value for the tens place of days"]
pub type Date10R = crate::FieldReader;
#[doc = "Field `DATE10` writer - 10 Days Value for the tens place of days"]
pub type Date10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb {
    #[doc = "0: The register value is not compared with the RDAYCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RDAYCNT counter value."]
    _1 = 1,
}
impl From<Enb> for bool {
    #[inline(always)]
    fn from(variant: Enb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB` reader - Compare enable"]
pub type EnbR = crate::BitReader<Enb>;
impl EnbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb {
        match self.bits {
            false => Enb::_0,
            true => Enb::_1,
        }
    }
    #[doc = "The register value is not compared with the RDAYCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enb::_0
    }
    #[doc = "The register value is compared with the RDAYCNT counter value."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enb::_1
    }
}
#[doc = "Field `ENB` writer - Compare enable"]
pub type EnbW<'a, REG> = crate::BitWriter<'a, REG, Enb>;
impl<'a, REG> EnbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The register value is not compared with the RDAYCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_0)
    }
    #[doc = "The register value is compared with the RDAYCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1 Day Value for the ones place of days"]
    #[inline(always)]
    pub fn date1(&self) -> Date1R {
        Date1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10 Days Value for the tens place of days"]
    #[inline(always)]
    pub fn date10(&self) -> Date10R {
        Date10R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDAYAR")
            .field("enb", &self.enb())
            .field("date10", &self.date10())
            .field("date1", &self.date1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - 1 Day Value for the ones place of days"]
    #[inline(always)]
    pub fn date1(&mut self) -> Date1W<RdayarSpec> {
        Date1W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 10 Days Value for the tens place of days"]
    #[inline(always)]
    pub fn date10(&mut self) -> Date10W<RdayarSpec> {
        Date10W::new(self, 4)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<RdayarSpec> {
        EnbW::new(self, 7)
    }
}
#[doc = "Date Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdayar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdayar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdayarSpec;
impl crate::RegisterSpec for RdayarSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rdayar::R`](R) reader structure"]
impl crate::Readable for RdayarSpec {}
#[doc = "`write(|w| ..)` method takes [`rdayar::W`](W) writer structure"]
impl crate::Writable for RdayarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDAYAR to value 0"]
impl crate::Resettable for RdayarSpec {}
