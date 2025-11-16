#[doc = "Register `RCR2` reader"]
pub type R = crate::R<Rcr2Spec>;
#[doc = "Register `RCR2` writer"]
pub type W = crate::W<Rcr2Spec>;
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: Prescaler and time counter are stopped."]
    _0 = 0,
    #[doc = "1: Prescaler and time counter operate normally."]
    _1 = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::_0,
            true => Start::_1,
        }
    }
    #[doc = "Prescaler and time counter are stopped."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Start::_0
    }
    #[doc = "Prescaler and time counter operate normally."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Start::_1
    }
}
#[doc = "Field `START` writer - Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaler and time counter are stopped."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_0)
    }
    #[doc = "Prescaler and time counter operate normally."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_1)
    }
}
#[doc = "RTC Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)"]
    _0 = 0,
    #[doc = "1: The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)"]
    _1 = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - RTC Software Reset"]
pub type ResetR = crate::BitReader<Reset>;
impl ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::_0,
            true => Reset::_1,
        }
    }
    #[doc = "Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reset::_0
    }
    #[doc = "The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reset::_1
    }
}
#[doc = "Field `RESET` writer - RTC Software Reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing is invalid.(write) / In normal time operation, or an RTC software reset has completed.(read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::_0)
    }
    #[doc = "The prescaler and the target registers for RTC software reset *1 are initialized.(write) / During an RTC software reset.(read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::_1)
    }
}
#[doc = "30-Second Adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adj30 {
    #[doc = "0: Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)"]
    _0 = 0,
    #[doc = "1: 30-second adjustment is executed.(write) / During 30-second adjustment.(read)"]
    _1 = 1,
}
impl From<Adj30> for bool {
    #[inline(always)]
    fn from(variant: Adj30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADJ30` reader - 30-Second Adjustment"]
pub type Adj30R = crate::BitReader<Adj30>;
impl Adj30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adj30 {
        match self.bits {
            false => Adj30::_0,
            true => Adj30::_1,
        }
    }
    #[doc = "Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adj30::_0
    }
    #[doc = "30-second adjustment is executed.(write) / During 30-second adjustment.(read)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adj30::_1
    }
}
#[doc = "Field `ADJ30` writer - 30-Second Adjustment"]
pub type Adj30W<'a, REG> = crate::BitWriter<'a, REG, Adj30>;
impl<'a, REG> Adj30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing is invalid.(write) / In normal time operation, or 30-second adjustment has completed.(read)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adj30::_0)
    }
    #[doc = "30-second adjustment is executed.(write) / During 30-second adjustment.(read)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adj30::_1)
    }
}
#[doc = "RTCOUT Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcoe {
    #[doc = "0: RTCOUT output disabled."]
    _0 = 0,
    #[doc = "1: RTCOUT output enabled."]
    _1 = 1,
}
impl From<Rtcoe> for bool {
    #[inline(always)]
    fn from(variant: Rtcoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOE` reader - RTCOUT Output Enable"]
pub type RtcoeR = crate::BitReader<Rtcoe>;
impl RtcoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcoe {
        match self.bits {
            false => Rtcoe::_0,
            true => Rtcoe::_1,
        }
    }
    #[doc = "RTCOUT output disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtcoe::_0
    }
    #[doc = "RTCOUT output enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcoe::_1
    }
}
#[doc = "Field `RTCOE` writer - RTCOUT Output Enable"]
pub type RtcoeW<'a, REG> = crate::BitWriter<'a, REG, Rtcoe>;
impl<'a, REG> RtcoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTCOUT output disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcoe::_0)
    }
    #[doc = "RTCOUT output enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcoe::_1)
    }
}
#[doc = "Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aadje {
    #[doc = "0: Automatic adjustment is disabled."]
    _0 = 0,
    #[doc = "1: Automatic adjustment is enabled."]
    _1 = 1,
}
impl From<Aadje> for bool {
    #[inline(always)]
    fn from(variant: Aadje) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AADJE` reader - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
pub type AadjeR = crate::BitReader<Aadje>;
impl AadjeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aadje {
        match self.bits {
            false => Aadje::_0,
            true => Aadje::_1,
        }
    }
    #[doc = "Automatic adjustment is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aadje::_0
    }
    #[doc = "Automatic adjustment is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aadje::_1
    }
}
#[doc = "Field `AADJE` writer - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
pub type AadjeW<'a, REG> = crate::BitWriter<'a, REG, Aadje>;
impl<'a, REG> AadjeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic adjustment is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aadje::_0)
    }
    #[doc = "Automatic adjustment is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aadje::_1)
    }
}
#[doc = "Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aadjp {
    #[doc = "0: The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every minute."]
    _0 = 0,
    #[doc = "1: The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every 10 seconds."]
    _1 = 1,
}
impl From<Aadjp> for bool {
    #[inline(always)]
    fn from(variant: Aadjp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AADJP` reader - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
pub type AadjpR = crate::BitReader<Aadjp>;
impl AadjpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aadjp {
        match self.bits {
            false => Aadjp::_0,
            true => Aadjp::_1,
        }
    }
    #[doc = "The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every minute."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Aadjp::_0
    }
    #[doc = "The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every 10 seconds."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Aadjp::_1
    }
}
#[doc = "Field `AADJP` writer - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
pub type AadjpW<'a, REG> = crate::BitWriter<'a, REG, Aadjp>;
impl<'a, REG> AadjpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every minute."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Aadjp::_0)
    }
    #[doc = "The RADJ.ADJ\\[5:0\\] setting value is adjusted from the count value of the prescaler every 10 seconds."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Aadjp::_1)
    }
}
#[doc = "Hours Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hr24 {
    #[doc = "0: The RTC operates in 12-hour mode."]
    _0 = 0,
    #[doc = "1: The RTC operates in 24-hour mode."]
    _1 = 1,
}
impl From<Hr24> for bool {
    #[inline(always)]
    fn from(variant: Hr24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HR24` reader - Hours Mode"]
pub type Hr24R = crate::BitReader<Hr24>;
impl Hr24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hr24 {
        match self.bits {
            false => Hr24::_0,
            true => Hr24::_1,
        }
    }
    #[doc = "The RTC operates in 12-hour mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hr24::_0
    }
    #[doc = "The RTC operates in 24-hour mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hr24::_1
    }
}
#[doc = "Field `HR24` writer - Hours Mode"]
pub type Hr24W<'a, REG> = crate::BitWriter<'a, REG, Hr24>;
impl<'a, REG> Hr24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RTC operates in 12-hour mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hr24::_0)
    }
    #[doc = "The RTC operates in 24-hour mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hr24::_1)
    }
}
#[doc = "Count Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntmd {
    #[doc = "0: The calendar count mode."]
    _0 = 0,
    #[doc = "1: The binary count mode."]
    _1 = 1,
}
impl From<Cntmd> for bool {
    #[inline(always)]
    fn from(variant: Cntmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTMD` reader - Count Mode Select"]
pub type CntmdR = crate::BitReader<Cntmd>;
impl CntmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntmd {
        match self.bits {
            false => Cntmd::_0,
            true => Cntmd::_1,
        }
    }
    #[doc = "The calendar count mode."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cntmd::_0
    }
    #[doc = "The binary count mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cntmd::_1
    }
}
#[doc = "Field `CNTMD` writer - Count Mode Select"]
pub type CntmdW<'a, REG> = crate::BitWriter<'a, REG, Cntmd>;
impl<'a, REG> CntmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The calendar count mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cntmd::_0)
    }
    #[doc = "The binary count mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cntmd::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Software Reset"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 30-Second Adjustment"]
    #[inline(always)]
    pub fn adj30(&self) -> Adj30R {
        Adj30R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTCOUT Output Enable"]
    #[inline(always)]
    pub fn rtcoe(&self) -> RtcoeR {
        RtcoeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub fn aadje(&self) -> AadjeR {
        AadjeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub fn aadjp(&self) -> AadjpR {
        AadjpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hours Mode"]
    #[inline(always)]
    pub fn hr24(&self) -> Hr24R {
        Hr24R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Count Mode Select"]
    #[inline(always)]
    pub fn cntmd(&self) -> CntmdR {
        CntmdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR2")
            .field("cntmd", &self.cntmd())
            .field("hr24", &self.hr24())
            .field("aadjp", &self.aadjp())
            .field("aadje", &self.aadje())
            .field("rtcoe", &self.rtcoe())
            .field("adj30", &self.adj30())
            .field("reset", &self.reset())
            .field("start", &self.start())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Rcr2Spec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Software Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<Rcr2Spec> {
        ResetW::new(self, 1)
    }
    #[doc = "Bit 2 - 30-Second Adjustment"]
    #[inline(always)]
    pub fn adj30(&mut self) -> Adj30W<Rcr2Spec> {
        Adj30W::new(self, 2)
    }
    #[doc = "Bit 3 - RTCOUT Output Enable"]
    #[inline(always)]
    pub fn rtcoe(&mut self) -> RtcoeW<Rcr2Spec> {
        RtcoeW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic Adjustment Enable (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub fn aadje(&mut self) -> AadjeW<Rcr2Spec> {
        AadjeW::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic Adjustment Period Select (When the LOCO clock is selected, the setting of this bit is disabled.)"]
    #[inline(always)]
    pub fn aadjp(&mut self) -> AadjpW<Rcr2Spec> {
        AadjpW::new(self, 5)
    }
    #[doc = "Bit 6 - Hours Mode"]
    #[inline(always)]
    pub fn hr24(&mut self) -> Hr24W<Rcr2Spec> {
        Hr24W::new(self, 6)
    }
    #[doc = "Bit 7 - Count Mode Select"]
    #[inline(always)]
    pub fn cntmd(&mut self) -> CntmdW<Rcr2Spec> {
        CntmdW::new(self, 7)
    }
}
#[doc = "RTC Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcr2Spec;
impl crate::RegisterSpec for Rcr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rcr2::R`](R) reader structure"]
impl crate::Readable for Rcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rcr2::W`](W) writer structure"]
impl crate::Writable for Rcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCR2 to value 0"]
impl crate::Resettable for Rcr2Spec {}
