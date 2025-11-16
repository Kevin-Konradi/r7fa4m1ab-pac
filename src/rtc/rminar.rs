#[doc = "Register `RMINAR` reader"]
pub type R = crate::R<RminarSpec>;
#[doc = "Register `RMINAR` writer"]
pub type W = crate::W<RminarSpec>;
#[doc = "Field `MIN1` reader - 1-Minute Count Value for the ones place of minutes"]
pub type Min1R = crate::FieldReader;
#[doc = "Field `MIN1` writer - 1-Minute Count Value for the ones place of minutes"]
pub type Min1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIN10` reader - 10-Minute Count Value for the tens place of minutes"]
pub type Min10R = crate::FieldReader;
#[doc = "Field `MIN10` writer - 10-Minute Count Value for the tens place of minutes"]
pub type Min10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb {
    #[doc = "0: The register value is not compared with the RMINCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RMINCNT counter value."]
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
    #[doc = "The register value is not compared with the RMINCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enb::_0
    }
    #[doc = "The register value is compared with the RMINCNT counter value."]
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
    #[doc = "The register value is not compared with the RMINCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_0)
    }
    #[doc = "The register value is compared with the RMINCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Minute Count Value for the ones place of minutes"]
    #[inline(always)]
    pub fn min1(&self) -> Min1R {
        Min1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Minute Count Value for the tens place of minutes"]
    #[inline(always)]
    pub fn min10(&self) -> Min10R {
        Min10R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMINAR")
            .field("enb", &self.enb())
            .field("min10", &self.min10())
            .field("min1", &self.min1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Minute Count Value for the ones place of minutes"]
    #[inline(always)]
    pub fn min1(&mut self) -> Min1W<RminarSpec> {
        Min1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 10-Minute Count Value for the tens place of minutes"]
    #[inline(always)]
    pub fn min10(&mut self) -> Min10W<RminarSpec> {
        Min10W::new(self, 4)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<RminarSpec> {
        EnbW::new(self, 7)
    }
}
#[doc = "Minute Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rminar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rminar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RminarSpec;
impl crate::RegisterSpec for RminarSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rminar::R`](R) reader structure"]
impl crate::Readable for RminarSpec {}
#[doc = "`write(|w| ..)` method takes [`rminar::W`](W) writer structure"]
impl crate::Writable for RminarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RMINAR to value 0"]
impl crate::Resettable for RminarSpec {}
