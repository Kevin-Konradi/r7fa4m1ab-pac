#[doc = "Register `ICCR2` reader"]
pub type R = crate::R<Iccr2Spec>;
#[doc = "Register `ICCR2` writer"]
pub type W = crate::W<Iccr2Spec>;
#[doc = "Start Condition Issuance Request Set the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum St {
    #[doc = "0: Does not request to issue a start condition."]
    _0 = 0,
    #[doc = "1: Requests to issue a start condition."]
    _1 = 1,
}
impl From<St> for bool {
    #[inline(always)]
    fn from(variant: St) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST` reader - Start Condition Issuance Request Set the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
pub type StR = crate::BitReader<St>;
impl StR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> St {
        match self.bits {
            false => St::_0,
            true => St::_1,
        }
    }
    #[doc = "Does not request to issue a start condition."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == St::_0
    }
    #[doc = "Requests to issue a start condition."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == St::_1
    }
}
#[doc = "Field `ST` writer - Start Condition Issuance Request Set the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
pub type StW<'a, REG> = crate::BitWriter<'a, REG, St>;
impl<'a, REG> StW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not request to issue a start condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(St::_0)
    }
    #[doc = "Requests to issue a start condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(St::_1)
    }
}
#[doc = "Restart Condition Issuance Request Note: Do not set the RS bit to 1 while issuing a stop condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rs {
    #[doc = "0: Does not request to issue a restart condition."]
    _0 = 0,
    #[doc = "1: Requests to issue a restart condition."]
    _1 = 1,
}
impl From<Rs> for bool {
    #[inline(always)]
    fn from(variant: Rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS` reader - Restart Condition Issuance Request Note: Do not set the RS bit to 1 while issuing a stop condition."]
pub type RsR = crate::BitReader<Rs>;
impl RsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rs {
        match self.bits {
            false => Rs::_0,
            true => Rs::_1,
        }
    }
    #[doc = "Does not request to issue a restart condition."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rs::_0
    }
    #[doc = "Requests to issue a restart condition."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rs::_1
    }
}
#[doc = "Field `RS` writer - Restart Condition Issuance Request Note: Do not set the RS bit to 1 while issuing a stop condition."]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG, Rs>;
impl<'a, REG> RsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not request to issue a restart condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rs::_0)
    }
    #[doc = "Requests to issue a restart condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rs::_1)
    }
}
#[doc = "Stop Condition Issuance Request Note: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state). Note: Do not set the SP bit to 1 while a restart condition is being issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp {
    #[doc = "0: Does not request to issue a stop condition."]
    _0 = 0,
    #[doc = "1: Requests to issue a stop condition."]
    _1 = 1,
}
impl From<Sp> for bool {
    #[inline(always)]
    fn from(variant: Sp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SP` reader - Stop Condition Issuance Request Note: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state). Note: Do not set the SP bit to 1 while a restart condition is being issued."]
pub type SpR = crate::BitReader<Sp>;
impl SpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sp {
        match self.bits {
            false => Sp::_0,
            true => Sp::_1,
        }
    }
    #[doc = "Does not request to issue a stop condition."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sp::_0
    }
    #[doc = "Requests to issue a stop condition."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sp::_1
    }
}
#[doc = "Field `SP` writer - Stop Condition Issuance Request Note: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state). Note: Do not set the SP bit to 1 while a restart condition is being issued."]
pub type SpW<'a, REG> = crate::BitWriter<'a, REG, Sp>;
impl<'a, REG> SpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not request to issue a stop condition."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sp::_0)
    }
    #[doc = "Requests to issue a stop condition."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sp::_1)
    }
}
#[doc = "Transmit/Receive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trs {
    #[doc = "0: Receive mode"]
    _0 = 0,
    #[doc = "1: Transmit mode"]
    _1 = 1,
}
impl From<Trs> for bool {
    #[inline(always)]
    fn from(variant: Trs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRS` reader - Transmit/Receive Mode"]
pub type TrsR = crate::BitReader<Trs>;
impl TrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trs {
        match self.bits {
            false => Trs::_0,
            true => Trs::_1,
        }
    }
    #[doc = "Receive mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trs::_0
    }
    #[doc = "Transmit mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trs::_1
    }
}
#[doc = "Field `TRS` writer - Transmit/Receive Mode"]
pub type TrsW<'a, REG> = crate::BitWriter<'a, REG, Trs>;
impl<'a, REG> TrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trs::_0)
    }
    #[doc = "Transmit mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trs::_1)
    }
}
#[doc = "Master/Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mst {
    #[doc = "0: Slave mode"]
    _0 = 0,
    #[doc = "1: Master mode"]
    _1 = 1,
}
impl From<Mst> for bool {
    #[inline(always)]
    fn from(variant: Mst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST` reader - Master/Slave Mode"]
pub type MstR = crate::BitReader<Mst>;
impl MstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mst {
        match self.bits {
            false => Mst::_0,
            true => Mst::_1,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mst::_0
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mst::_1
    }
}
#[doc = "Field `MST` writer - Master/Slave Mode"]
pub type MstW<'a, REG> = crate::BitWriter<'a, REG, Mst>;
impl<'a, REG> MstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::_0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::_1)
    }
}
#[doc = "Bus Busy Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bbsy {
    #[doc = "0: The I2C bus is released (bus free state)."]
    _0 = 0,
    #[doc = "1: The I2C bus is occupied (bus busy state)."]
    _1 = 1,
}
impl From<Bbsy> for bool {
    #[inline(always)]
    fn from(variant: Bbsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BBSY` reader - Bus Busy Detection Flag"]
pub type BbsyR = crate::BitReader<Bbsy>;
impl BbsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bbsy {
        match self.bits {
            false => Bbsy::_0,
            true => Bbsy::_1,
        }
    }
    #[doc = "The I2C bus is released (bus free state)."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bbsy::_0
    }
    #[doc = "The I2C bus is occupied (bus busy state)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bbsy::_1
    }
}
impl R {
    #[doc = "Bit 1 - Start Condition Issuance Request Set the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restart Condition Issuance Request Note: Do not set the RS bit to 1 while issuing a stop condition."]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop Condition Issuance Request Note: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state). Note: Do not set the SP bit to 1 while a restart condition is being issued."]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit/Receive Mode"]
    #[inline(always)]
    pub fn trs(&self) -> TrsR {
        TrsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mst(&self) -> MstR {
        MstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Busy Detection Flag"]
    #[inline(always)]
    pub fn bbsy(&self) -> BbsyR {
        BbsyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICCR2")
            .field("bbsy", &self.bbsy())
            .field("mst", &self.mst())
            .field("trs", &self.trs())
            .field("sp", &self.sp())
            .field("rs", &self.rs())
            .field("st", &self.st())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Start Condition Issuance Request Set the ST bit to 1 (start condition issuance request) when the BBSY flag is set to 0 (bus free state)."]
    #[inline(always)]
    pub fn st(&mut self) -> StW<Iccr2Spec> {
        StW::new(self, 1)
    }
    #[doc = "Bit 2 - Restart Condition Issuance Request Note: Do not set the RS bit to 1 while issuing a stop condition."]
    #[inline(always)]
    pub fn rs(&mut self) -> RsW<Iccr2Spec> {
        RsW::new(self, 2)
    }
    #[doc = "Bit 3 - Stop Condition Issuance Request Note: Writing to the SP bit is not possible while the setting of the BBSY flag is 0 (bus free state). Note: Do not set the SP bit to 1 while a restart condition is being issued."]
    #[inline(always)]
    pub fn sp(&mut self) -> SpW<Iccr2Spec> {
        SpW::new(self, 3)
    }
    #[doc = "Bit 5 - Transmit/Receive Mode"]
    #[inline(always)]
    pub fn trs(&mut self) -> TrsW<Iccr2Spec> {
        TrsW::new(self, 5)
    }
    #[doc = "Bit 6 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mst(&mut self) -> MstW<Iccr2Spec> {
        MstW::new(self, 6)
    }
}
#[doc = "I2C Bus Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`iccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iccr2Spec;
impl crate::RegisterSpec for Iccr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iccr2::R`](R) reader structure"]
impl crate::Readable for Iccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`iccr2::W`](W) writer structure"]
impl crate::Writable for Iccr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCR2 to value 0"]
impl crate::Resettable for Iccr2Spec {}
