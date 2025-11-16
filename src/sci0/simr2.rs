#[doc = "Register `SIMR2` reader"]
pub type R = crate::R<Simr2Spec>;
#[doc = "Register `SIMR2` writer"]
pub type W = crate::W<Simr2Spec>;
#[doc = "I2C Interrupt Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicintm {
    #[doc = "0: Use ACK/NACK interrupts."]
    _0 = 0,
    #[doc = "1: Use reception and transmission interrupts"]
    _1 = 1,
}
impl From<Iicintm> for bool {
    #[inline(always)]
    fn from(variant: Iicintm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICINTM` reader - I2C Interrupt Mode Select"]
pub type IicintmR = crate::BitReader<Iicintm>;
impl IicintmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicintm {
        match self.bits {
            false => Iicintm::_0,
            true => Iicintm::_1,
        }
    }
    #[doc = "Use ACK/NACK interrupts."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicintm::_0
    }
    #[doc = "Use reception and transmission interrupts"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicintm::_1
    }
}
#[doc = "Field `IICINTM` writer - I2C Interrupt Mode Select"]
pub type IicintmW<'a, REG> = crate::BitWriter<'a, REG, Iicintm>;
impl<'a, REG> IicintmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use ACK/NACK interrupts."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicintm::_0)
    }
    #[doc = "Use reception and transmission interrupts"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicintm::_1)
    }
}
#[doc = "Clock Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iiccsc {
    #[doc = "0: No synchronization with the clock signal"]
    _0 = 0,
    #[doc = "1: Synchronization with the clock signal"]
    _1 = 1,
}
impl From<Iiccsc> for bool {
    #[inline(always)]
    fn from(variant: Iiccsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICCSC` reader - Clock Synchronization"]
pub type IiccscR = crate::BitReader<Iiccsc>;
impl IiccscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iiccsc {
        match self.bits {
            false => Iiccsc::_0,
            true => Iiccsc::_1,
        }
    }
    #[doc = "No synchronization with the clock signal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iiccsc::_0
    }
    #[doc = "Synchronization with the clock signal"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iiccsc::_1
    }
}
#[doc = "Field `IICCSC` writer - Clock Synchronization"]
pub type IiccscW<'a, REG> = crate::BitWriter<'a, REG, Iiccsc>;
impl<'a, REG> IiccscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No synchronization with the clock signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iiccsc::_0)
    }
    #[doc = "Synchronization with the clock signal"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iiccsc::_1)
    }
}
#[doc = "ACK Transmission Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicackt {
    #[doc = "0: ACK transmission"]
    _0 = 0,
    #[doc = "1: NACK transmission and reception of ACK/NACK"]
    _1 = 1,
}
impl From<Iicackt> for bool {
    #[inline(always)]
    fn from(variant: Iicackt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICACKT` reader - ACK Transmission Data"]
pub type IicacktR = crate::BitReader<Iicackt>;
impl IicacktR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicackt {
        match self.bits {
            false => Iicackt::_0,
            true => Iicackt::_1,
        }
    }
    #[doc = "ACK transmission"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iicackt::_0
    }
    #[doc = "NACK transmission and reception of ACK/NACK"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iicackt::_1
    }
}
#[doc = "Field `IICACKT` writer - ACK Transmission Data"]
pub type IicacktW<'a, REG> = crate::BitWriter<'a, REG, Iicackt>;
impl<'a, REG> IicacktW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACK transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicackt::_0)
    }
    #[doc = "NACK transmission and reception of ACK/NACK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicackt::_1)
    }
}
impl R {
    #[doc = "Bit 0 - I2C Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(&self) -> IicintmR {
        IicintmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(&self) -> IiccscR {
        IiccscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(&self) -> IicacktR {
        IicacktR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIMR2")
            .field("iicackt", &self.iicackt())
            .field("iiccsc", &self.iiccsc())
            .field("iicintm", &self.iicintm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I2C Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(&mut self) -> IicintmW<Simr2Spec> {
        IicintmW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(&mut self) -> IiccscW<Simr2Spec> {
        IiccscW::new(self, 1)
    }
    #[doc = "Bit 5 - ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(&mut self) -> IicacktW<Simr2Spec> {
        IicacktW::new(self, 5)
    }
}
#[doc = "I2C Mode Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`simr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Simr2Spec;
impl crate::RegisterSpec for Simr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`simr2::R`](R) reader structure"]
impl crate::Readable for Simr2Spec {}
#[doc = "`write(|w| ..)` method takes [`simr2::W`](W) writer structure"]
impl crate::Writable for Simr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIMR2 to value 0"]
impl crate::Resettable for Simr2Spec {}
