#![allow(dead_code, unused_variables)]

use itertools::Itertools;
use rand::prelude::*;

pub mod kaart {
    use core::cmp::Ordering::{self, Equal, Greater, Less};

    use itertools::Itertools;
    use Combinatie::*;
    use Kleur::*;
    use Waarde::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Kleur {
        Klaveren,
        Schoppen,
        Harten,
        Ruiten,
    }

    impl std::fmt::Display for Kleur {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Klaveren => '♣',
                    Schoppen => '♠',
                    Harten => '♡',
                    Ruiten => '♢',
                }
            )
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Waarde {
        Tal(u8),
        Boer,
        Koningin,
        Koning,
        Aas,
    }

    impl std::fmt::Display for Waarde {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Tal(a) => a.to_string(),
                    Boer => "J".to_string(),
                    Koningin => "Q".to_string(),
                    Koning => "K".to_string(),
                    Aas => "A".to_string(),
                }
            )
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Kaart {
        waarde: Waarde,
        kleur: Kleur,
    }

    impl Kaart {
        pub fn new(kleur: Kleur, waarde: Waarde) -> Self {
            Self { kleur, waarde }
        }

        pub fn maak_deck() -> [Kaart; 52] {
            let mut deck = [Kaart::new(Harten, Aas); 52];

            for i in 0..52u8 {
                deck[i as usize] = Kaart::new(
                    if i % 4 == 0 {
                        Harten
                    } else if i % 4 == 1 {
                        Klaveren
                    } else if i % 4 == 2 {
                        Schoppen
                    } else {
                        Ruiten
                    },
                    match i % 13 {
                        0 => Aas,
                        1..=9 => Tal(i % 13 + 1),
                        10 => Boer,
                        11 => Koningin,
                        12 => Koning,
                        _ => unreachable!(),
                    },
                )
            }

            deck
        }
    }

    impl std::fmt::Display for Kaart {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}{}", self.kleur, self.waarde)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Combinatie {
        High,
        Paar(Waarde),
        TweePaar(Waarde, Waarde), // hoogste paar eerst
        ThreeOfAKind(Waarde),
        Straight(Waarde, Option<bool>), // waarde van de hoogste kaart, en als aas: high of low
        Flush,
        FullHouse(Waarde, Waarde), // eerst het triplet, dan het dubbel
        FourOfAKind(Waarde),
        StraightFlush(Waarde), // waarde van de hoogste kaart
        RoyalFlush,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Hand {
        combinatie: Combinatie,
        kaarten: [Kaart; 5],
    }

    impl Hand {
        pub fn new(kaarten: [Kaart; 5]) -> Self {
            let mut kaarten = kaarten.clone();
            let slice: &mut [Kaart] = kaarten.as_mut_slice();
            slice.sort_by(|a, b| a.cmp(b).reverse());

            let waarden = (
                slice[0].waarde,
                slice[1].waarde,
                slice[2].waarde,
                slice[3].waarde,
                slice[4].waarde,
            );

            let allen_dezelfde_kleur = slice.iter().map(|s| s.kleur).all_equal();
            let combinatie = match waarden {
                (Aas, Koning, Koningin, Boer, Tal(10)) => {
                    if allen_dezelfde_kleur {
                        RoyalFlush
                    } else {
                        Straight(Aas, Some(true))
                    }
                }
                (a, b, c, d, _) | (_, a, b, c, d) if a == b && a == c && a == d => FourOfAKind(a),
                (a, b, c, d, e) if a == b && a == c && d == e => FullHouse(a, d),
                (a, b, c, d, e) if a == b && c == d && c == e => FullHouse(c, a),
                _ => {
                    if allen_dezelfde_kleur {
                        match waarden {
                            (Aas, Tal(5), Tal(4), Tal(3), Tal(2)) => StraightFlush(Aas),
                            (Koning, Koningin, Boer, Tal(10), Tal(9)) => StraightFlush(Koning),
                            (Koningin, Boer, Tal(10), Tal(9), Tal(8)) => StraightFlush(Koningin),
                            (Boer, Tal(10), Tal(9), Tal(8), Tal(7)) => StraightFlush(Boer),
                            (Tal(a), Tal(b), Tal(c), Tal(d), Tal(e))
                                if a == b + 1 && b == c + 1 && c == d + 1 && d == e + 1 =>
                            {
                                StraightFlush(Tal(a))
                            }
                            _ => Flush,
                        }
                    } else {
                        match waarden {
                            (Aas, Tal(5), Tal(4), Tal(3), Tal(2)) => Straight(Aas, Some(false)),
                            (Koning, Koningin, Boer, Tal(10), Tal(9)) => Straight(Koning, None),
                            (Koningin, Boer, Tal(10), Tal(9), Tal(8)) => Straight(Koningin, None),
                            (Boer, Tal(10), Tal(9), Tal(8), Tal(7)) => Straight(Boer, None),
                            (Tal(a), Tal(b), Tal(c), Tal(d), Tal(e))
                                if a == b + 1 && b == c + 1 && c == d + 1 && d == e + 1 =>
                            {
                                Straight(Tal(a), None)
                            }

                            (a, b, c, _, _) | (_, a, b, c, _) | (_, _, a, b, c)
                                if a == b && a == c =>
                            {
                                ThreeOfAKind(a)
                            }
                            (a, b, c, d, _) | (a, b, _, c, d) | (_, a, b, c, d)
                                if a == b && c == d =>
                            {
                                TweePaar(a, c)
                            }
                            (a, b, _, _, _)
                            | (_, a, b, _, _)
                            | (_, _, a, b, _)
                            | (_, _, _, a, b)
                                if a == b =>
                            {
                                Paar(a)
                            }
                            _ => High,
                        }
                    }
                }
            };

            Hand {
                combinatie,
                kaarten,
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.combinatie.partial_cmp(&other.combinatie) {
                Some(Equal) => match self.combinatie {
                    High => Some(self.kaarten.cmp(&other.kaarten)),
                    Paar(a) => Some(a.cmp(&match other.combinatie {
                        Paar(b) => b,
                        _ => unreachable!(),
                    })),
                    TweePaar(a, b) => {
                        let (c, d) = match other.combinatie {
                            TweePaar(c, d) => (c, d),
                            _ => unreachable!(),
                        };

                        let eerste_paar_vgl = a.cmp(&c);

                        Some(match eerste_paar_vgl {
                            Equal => b.cmp(&d),
                            _ => eerste_paar_vgl,
                        })
                    }
                    ThreeOfAKind(a) => Some(a.cmp(&match other.combinatie {
                        ThreeOfAKind(b) => b,
                        _ => unreachable!(),
                    })),
                    Straight(a, mss_ace_high_a) => {
                        let (b, mss_ace_high_b) = match other.combinatie {
                            Straight(b, mss_ace_high_b) => (b, mss_ace_high_b),
                            _ => unreachable!(),
                        };

                        let eerste_paar_vgl = a.cmp(&b);

                        Some(match eerste_paar_vgl {
                            Equal => {
                                if a == Aas {
                                    let ace_high_a = mss_ace_high_a.unwrap();
                                    let ace_high_b = mss_ace_high_b.unwrap();
                                    if ace_high_a && !ace_high_b {
                                        Ordering::Greater
                                    } else if !ace_high_a && ace_high_b {
                                        Ordering::Less
                                    } else {
                                        Equal
                                    }
                                } else {
                                    Equal
                                }
                            }
                            _ => eerste_paar_vgl,
                        })
                    }
                    Flush => todo!(),
                    FullHouse(a, b) => {
                        let (c, d) = match other.combinatie {
                            TweePaar(c, d) => (c, d),
                            _ => unreachable!(),
                        };

                        let eerste_paar_vgl = a.cmp(&c);

                        Some(match eerste_paar_vgl {
                            Equal => b.cmp(&d),
                            _ => eerste_paar_vgl,
                        })
                    }
                    FourOfAKind(a) => Some(a.cmp(&match other.combinatie {
                        FourOfAKind(b) => b,
                        _ => unreachable!(),
                    })),
                    StraightFlush(a) => {
                        let b = match other.combinatie {
                            StraightFlush(b) => b,
                            _ => unreachable!(),
                        };

                        Some(if a == Aas && b != Aas {
                            Less
                        } else if a != Aas && b == Aas {
                            Greater
                        } else {
                            a.cmp(&b)
                        })
                    }
                    RoyalFlush => Some(Equal),
                },
                ord => return ord,
            }
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn hand_categorisatie(tafel: [Kaart; 5], verwachte_combinatie: Combinatie) {
            let Hand {
                combinatie,
                kaarten,
            } = Hand::new(tafel);
            println!(
                "[{} {} {} {} {}] is {combinatie:?}",
                kaarten[0], kaarten[1], kaarten[2], kaarten[3], kaarten[4]
            );
            assert_eq!(combinatie, verwachte_combinatie);
        }

        #[test]
        fn flushes() {
            hand_categorisatie(
                [
                    Kaart::new(Harten, Waarde::Tal(7)),
                    Kaart::new(Harten, Waarde::Tal(5)),
                    Kaart::new(Harten, Waarde::Tal(6)),
                    Kaart::new(Harten, Waarde::Tal(8)),
                    Kaart::new(Harten, Waarde::Tal(9)),
                ],
                StraightFlush(Tal(9)),
            );

            hand_categorisatie(
                [
                    Kaart::new(Harten, Waarde::Boer),
                    Kaart::new(Harten, Waarde::Tal(10)),
                    Kaart::new(Harten, Waarde::Koningin),
                    Kaart::new(Harten, Waarde::Tal(8)),
                    Kaart::new(Harten, Waarde::Tal(9)),
                ],
                StraightFlush(Koningin),
            );

            hand_categorisatie(
                [
                    Kaart::new(Harten, Waarde::Koning),
                    Kaart::new(Harten, Waarde::Tal(10)),
                    Kaart::new(Harten, Waarde::Koningin),
                    Kaart::new(Harten, Waarde::Tal(8)),
                    Kaart::new(Harten, Waarde::Tal(9)),
                ],
                Flush,
            );

            hand_categorisatie(
                [
                    Kaart::new(Harten, Waarde::Boer),
                    Kaart::new(Harten, Waarde::Koning),
                    Kaart::new(Harten, Waarde::Aas),
                    Kaart::new(Harten, Waarde::Koningin),
                    Kaart::new(Harten, Waarde::Tal(10)),
                ],
                RoyalFlush,
            );

            hand_categorisatie(
                [
                    Kaart::new(Schoppen, Waarde::Boer),
                    Kaart::new(Harten, Waarde::Koning),
                    Kaart::new(Harten, Waarde::Aas),
                    Kaart::new(Harten, Waarde::Koningin),
                    Kaart::new(Harten, Waarde::Tal(10)),
                ],
                Straight(Aas, Some(true)),
            );
        }

        #[test]
        fn vergelijkingen() {
            assert!(
                Hand::new([
                    Kaart::new(Harten, Waarde::Boer),
                    Kaart::new(Harten, Waarde::Koning),
                    Kaart::new(Harten, Waarde::Aas),
                    Kaart::new(Harten, Waarde::Koningin),
                    Kaart::new(Harten, Waarde::Tal(10)),
                ]) > Hand::new([
                    Kaart::new(Harten, Waarde::Boer),
                    Kaart::new(Harten, Waarde::Tal(10)),
                    Kaart::new(Harten, Waarde::Koningin),
                    Kaart::new(Harten, Waarde::Tal(8)),
                    Kaart::new(Harten, Waarde::Tal(9)),
                ],)
            )

            // TODO: uitgebreider testen!
        }
    }
}

use std::{
    cell::OnceCell,
    ops::Rem,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::kaart::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpelStatus {
    Wachtend,
    Lopend,
    Gestopt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpelerId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpelId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Speler {
    pub id: OnceCell<SpelerId>,
    pub naam: String,
    pub chips: u64,
    pub hand: Option<(Kaart, Kaart)>,
    pub inzet: u64,
}

pub const CHIPS_PER_SPELER: u64 = 1000;
pub const SMALL_BLIND: u64 = 10;
pub const BIG_BLIND: u64 = 20;

impl Speler {
    pub fn new_zonder_id(naam: String) -> Self {
        Speler {
            id: OnceCell::new(),
            naam,
            chips: CHIPS_PER_SPELER,
            hand: None,
            inzet: 0,
        }
    }

    pub fn stuur_bericht(&self, bericht: PokbotcomMelding, in_respons: bool) {
        println!(
            "{}[SERV] → [{}]: {bericht:?}",
            if in_respons { "    " } else { "" },
            self.naam
                .chars()
                .chain([' '].into_iter().cycle())
                .take(4)
                .fold(String::new(), |mut acc, c| {
                    acc.push(c);
                    acc
                })
        )
    }

    pub fn zet_chips_in(&mut self, te_vorderen: u64) -> Result<()> {
        if self.chips >= te_vorderen {
            self.chips -= te_vorderen;
            self.inzet += te_vorderen;
            Ok(())
        } else {
            Err(PokbotcomError::NietGenoegChips.into())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Spel {
    pub id: OnceCell<SpelId>,
    pub spelers: Vec<SpelerId>,
    pub pot: u64,
    pub tafel: (Option<(Kaart, Kaart, Kaart)>, Option<Kaart>, Option<Kaart>),
    pub huidige_dealer: usize, // index in vector van spelers
    pub aan_de_beurt: usize,   // index in vector van spelers
    pub deck: Vec<Kaart>,
    pub huidige_inzet: u64,
    pub laatste_actionabele_speler: Option<SpelerId>,
    pub status: SpelStatus,
    rng: StdRng,
}

impl Spel {
    pub fn new(toegekende_id: SpelId, rng_seed: Option<u64>) -> Self {
        Spel {
            id: OnceCell::from(toegekende_id),
            spelers: Vec::new(),
            pot: 0,
            tafel: (None, None, None),
            huidige_dealer: 0,
            aan_de_beurt: 1,
            deck: Kaart::maak_deck().to_vec(),
            huidige_inzet: BIG_BLIND,
            laatste_actionabele_speler: None,
            status: SpelStatus::Wachtend,
            rng: match rng_seed {
                Some(seed) => StdRng::seed_from_u64(seed),
                None => StdRng::from_entropy(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Actie {
    Fold,
    Check,
    Call,
    Bet(u64),
}

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
pub enum PokbotcomError {
    #[error("Deze speler had zich al geregistreerd, met id {0:?}.")]
    SpelerAlGeregistreerd(SpelerId),
    #[error("Je hebt niet genoeg chips ingezet om deze actie te kunnen voltooien.")]
    NietGenoegChips,
    #[error("Het is niet jouw beurt.")]
    NietJouwBeurt,
}

#[derive(Clone, Copy, Debug)]
pub enum PokbotcomMelding {
    Hand(Kaart, Kaart),
    Flop(Kaart, Kaart, Kaart),
    Turn(Kaart),
    River(Kaart),
    SpelerActie(SpelerId, Actie),
    AanDeBeurt,
    RondeOver,
    Gewonnen(Hand, SpelerId),
}

#[derive(Debug)]
pub struct Centrale {
    pub spelers: Vec<Speler>,
    pub spellen: Vec<Spel>,

    pub volgende_geldige_speler_id: AtomicU64,
    pub volgende_geldige_spel_id: AtomicU64,
}

impl Centrale {
    pub fn new() -> Self {
        Centrale {
            spelers: Vec::new(),
            spellen: Vec::new(),
            volgende_geldige_spel_id: 0.into(),
            volgende_geldige_speler_id: 0.into(),
        }
    }

    fn laad_uit_db() -> Self {
        unimplemented!()
    }

    pub fn registreer_speler(&mut self, speler: Speler) -> Result<SpelerId> {
        let geregistreerde_id = SpelerId(
            self.volgende_geldige_speler_id
                .fetch_add(1, Ordering::Relaxed),
        );
        speler
            .id
            .set(geregistreerde_id)
            .map_err(|id| PokbotcomError::SpelerAlGeregistreerd(id))?;

        self.spelers.push(speler);
        Ok(geregistreerde_id)
    }

    fn verwijder_speler(&mut self, speler_id: SpelerId) {
        self.spelers.swap_remove(
            self.spelers
                .iter()
                .position(|s| s.id.get().unwrap() == &speler_id)
                .unwrap(),
        );
    }

    fn get_mut_speler(&mut self, speler_id: SpelerId) -> &mut Speler {
        self.spelers
            .iter_mut()
            .find(|s| s.id.get().unwrap() == &speler_id)
            .unwrap()
    }

    fn get_speler(&self, speler_id: SpelerId) -> &Speler {
        self.spelers
            .iter()
            .find(|s| s.id.get().unwrap() == &speler_id)
            .unwrap()
    }

    fn get_mut_spel(&mut self, spel_id: SpelId) -> &mut Spel {
        self.spellen
            .iter_mut()
            .find(|s| s.id.get().unwrap() == &spel_id)
            .unwrap()
    }

    fn get_spel(&self, spel_id: SpelId) -> &Spel {
        self.spellen
            .iter()
            .find(|s| s.id.get().unwrap() == &spel_id)
            .unwrap()
    }

    pub fn maak_spel(&mut self, spelers: Vec<SpelerId>, rng_seed: Option<u64>) -> SpelId {
        let geregistreerde_id = SpelId(
            self.volgende_geldige_spel_id
                .fetch_add(1, Ordering::Relaxed),
        );
        let mut leeg_spel = Spel::new(geregistreerde_id, rng_seed);
        leeg_spel.spelers = spelers;
        self.spellen.push(leeg_spel);
        geregistreerde_id
    }

    pub fn start_spel(&mut self, spel_id: SpelId) -> Result<()> {
        let spel = self
            .spellen
            .iter_mut()
            .find(|s| s.id.get().unwrap() == &spel_id)
            .unwrap();
        assert!(spel.spelers.len() >= 2);

        spel.deck = Kaart::maak_deck().to_vec();
        spel.deck.shuffle(&mut spel.rng);

        spel.status = SpelStatus::Lopend;

        spel.tafel = (None, None, None);

        spel.huidige_inzet = BIG_BLIND;

        for (lokale_id, speler_id) in spel.spelers.iter().enumerate() {
            let speler = self
                .spelers
                .iter_mut()
                .find(|s| s.id.get().unwrap() == speler_id)
                .unwrap();
            speler.hand = Some((spel.deck.pop().unwrap(), spel.deck.pop().unwrap()));
            speler.stuur_bericht(
                PokbotcomMelding::Hand(speler.hand.unwrap().0, speler.hand.unwrap().1),
                false,
            );

            if lokale_id == (spel.huidige_dealer + 1).rem(spel.spelers.len()) {
                speler.zet_chips_in(SMALL_BLIND)?;
            } else if lokale_id == (spel.huidige_dealer + 2).rem(spel.spelers.len()) {
                speler.zet_chips_in(BIG_BLIND)?;
            } else if lokale_id == (spel.huidige_dealer + 3).rem(spel.spelers.len()) {
                speler.stuur_bericht(PokbotcomMelding::AanDeBeurt, false);
            }
        }

        spel.aan_de_beurt = (spel.huidige_dealer + 3).rem(spel.spelers.len());

        Ok(())
    }

    pub fn stuur_naar_alle_spelers(&self, spel_id: SpelId, melding: PokbotcomMelding) {
        let speler_ids = &self
            .spellen
            .iter()
            .find(|s| s.id.get().unwrap() == &spel_id)
            .unwrap()
            .spelers;
        for speler_id in speler_ids {
            let speler = self
                .spelers
                .iter()
                .find(|s| s.id.get().unwrap() == speler_id)
                .unwrap();
            speler.stuur_bericht(melding, true);
        }
    }

    fn verzamel_pot(&mut self, spel_id: SpelId) {
        let spel = self
            .spellen
            .iter_mut()
            .find(|s| s.id.get().unwrap() == &spel_id)
            .unwrap();
        for speler_id in &spel.spelers {
            let speler = self
                .spelers
                .iter_mut()
                .find(|s| s.id.get().unwrap() == speler_id)
                .unwrap();
            spel.pot += speler.inzet;
            speler.inzet = 0;
        }
    }

    fn naar_volgende_gesamtronde(&mut self, spel_id: SpelId) -> Result<()> {
        let spel = self
            .spellen
            .iter_mut()
            .find(|s| s.id.get().unwrap() == &spel_id)
            .unwrap();

        spel.huidige_dealer = (spel.huidige_dealer + 1) % spel.spelers.len();

        spel.tafel = (None, None, None);

        spel.huidige_inzet = BIG_BLIND;

        for (lokale_id, speler_id) in spel.spelers.iter().enumerate() {
            let speler = self
                .spelers
                .iter_mut()
                .find(|s| s.id.get().unwrap() == speler_id)
                .unwrap();
            speler.hand = Some((spel.deck.pop().unwrap(), spel.deck.pop().unwrap()));
            speler.stuur_bericht(
                PokbotcomMelding::Hand(speler.hand.unwrap().0, speler.hand.unwrap().1),
                false,
            );

            if lokale_id == (spel.huidige_dealer + 1).rem(spel.spelers.len()) {
                speler.zet_chips_in(SMALL_BLIND)?;
            } else if lokale_id == (spel.huidige_dealer + 2).rem(spel.spelers.len()) {
                speler.zet_chips_in(BIG_BLIND)?;
            } else if lokale_id == (spel.huidige_dealer + 3).rem(spel.spelers.len()) {
                speler.stuur_bericht(PokbotcomMelding::AanDeBeurt, false);
            }
        }

        spel.aan_de_beurt = (spel.huidige_dealer + 3).rem(spel.spelers.len());

        Ok(())
    }

    pub fn ronde_klaar(&mut self, spel_id: SpelId) -> Result<()> {
        self.stuur_naar_alle_spelers(spel_id, PokbotcomMelding::RondeOver);
        self.verzamel_pot(spel_id);
        let spel = self
            .spellen
            .iter_mut()
            .by_ref()
            .find(|s| s.id.get().unwrap() == &spel_id)
            .unwrap();

        spel.laatste_actionabele_speler = None;
        spel.huidige_inzet = 0;

        let melding = match spel.tafel {
            (None, None, None) => {
                let flop = (
                    spel.deck.pop().unwrap(),
                    spel.deck.pop().unwrap(),
                    spel.deck.pop().unwrap(),
                );

                spel.tafel.0 = Some(flop);
                PokbotcomMelding::Flop(flop.0, flop.1, flop.2)
            }
            (Some(_), None, None) => {
                let turn = spel.deck.pop().unwrap();

                spel.tafel.1 = Some(turn);
                PokbotcomMelding::Turn(turn)
            }
            (Some(_), Some(_), None) => {
                let river = spel.deck.pop().unwrap();

                spel.tafel.2 = Some(river);
                PokbotcomMelding::River(river)
            }
            (Some(_), Some(_), Some(_)) => {
                // spelletje klaar

                let tafel = vec![
                    spel.tafel.0.unwrap().0,
                    spel.tafel.0.unwrap().1,
                    spel.tafel.0.unwrap().2,
                    spel.tafel.1.unwrap(),
                    spel.tafel.2.unwrap(),
                ];
                let deelnemende_handen = self
                    .spelers
                    .iter()
                    .filter(|s| spel.spelers.contains(s.id.get().unwrap()))
                    .filter(|s| s.hand.is_some())
                    .map(|s| (s.hand.unwrap(), *s.id.get().unwrap()));

                let mut beste_handen = Vec::new();

                for hand_en_speler in deelnemende_handen {
                    let hand_iter = [hand_en_speler.0 .0, hand_en_speler.0 .1].into_iter();
                    for combinatie in tafel
                        .clone()
                        .into_iter()
                        .chain(hand_iter)
                        .tuple_combinations::<(_, _, _, _, _)>()
                    {
                        let array: [Kaart; 5] = [
                            combinatie.0,
                            combinatie.1,
                            combinatie.2,
                            combinatie.3,
                            combinatie.4,
                        ];
                        // TODO: check of de tafel de winnende hand beet heeft
                        let berekende_hand = Hand::new(array);
                        beste_handen.push((berekende_hand, hand_en_speler.1));
                    }
                }

                beste_handen.sort_by(|(h1, _), (h2, _)| h1.cmp(h2).reverse());

                let winnaar = self
                    .spelers
                    .iter_mut()
                    .find(|s| s.id.get().unwrap() == &beste_handen[0].1)
                    .unwrap();
                dbg!(winnaar.chips);
                dbg!(spel.pot);
                winnaar.chips += spel.pot;
                dbg!(winnaar.chips);
                spel.pot = 0;

                PokbotcomMelding::Gewonnen(beste_handen[0].0, beste_handen[0].1)
            }
            _ => unreachable!(),
        };

        spel.aan_de_beurt = spel.huidige_dealer;
        loop {
            spel.aan_de_beurt = (spel.aan_de_beurt + 1) % spel.spelers.len();
            if self
                .spelers
                .iter()
                .find(|s| s.id.get().unwrap() == &spel.spelers[spel.aan_de_beurt])
                .unwrap()
                .hand
                .is_some()
            {
                break;
            }
        }

        self.stuur_naar_alle_spelers(spel_id, melding);
        if matches!(melding, PokbotcomMelding::Gewonnen(..)) {
            self.naar_volgende_gesamtronde(spel_id)?;
        }

        Ok(())
    }

    pub fn ontvang_actie(
        &mut self,
        spel_id: SpelId,
        speler_id: SpelerId,
        actie: Actie,
    ) -> Result<()> {
        let spel = self
            .spellen
            .iter_mut()
            .find(|s| s.id.get().unwrap() == &spel_id)
            .unwrap();

        let mut uninit_speler: Option<&mut Speler> = None;
        let mut andere_spelers: Vec<&Speler> = Vec::new();

        for sp in self.spelers.iter_mut() {
            if sp.id.get().unwrap() == &speler_id {
                uninit_speler = Some(sp);
            } else {
                andere_spelers.push(sp);
            }
        }

        let speler = uninit_speler.expect(&format!(
            "Speler met id {speler_id:?} niet in spel {spel_id:?} gevonden."
        ));

        println!(
            "[{}] → [SERV]: {actie:?}",
            speler
                .naam
                .chars()
                .chain([' '].into_iter().cycle())
                .take(4)
                .fold(String::new(), |mut acc, c| {
                    acc.push(c);
                    acc
                })
        );

        let res = if speler_id != spel.spelers[spel.aan_de_beurt] {
            Err(PokbotcomError::NietJouwBeurt.into())
        } else {
            match actie {
                Actie::Fold => {
                    speler.hand = None;
                    Ok(())
                }
                Actie::Check => {
                    match spel.laatste_actionabele_speler {
                        None => {
                            spel.laatste_actionabele_speler = Some(speler_id);
                        }
                        Some(laatste_speler) if laatste_speler == speler_id => {
                            return self.ronde_klaar(spel_id);
                        }
                        _ => {}
                    }

                    if speler.inzet < spel.huidige_inzet {
                        Err(PokbotcomError::NietGenoegChips.into())
                    } else {
                        Ok(())
                    }
                }
                Actie::Call => speler.zet_chips_in(spel.huidige_inzet - speler.inzet),
                Actie::Bet(extra_chips) => {
                    spel.laatste_actionabele_speler = Some(speler_id);

                    let res = speler.zet_chips_in(spel.huidige_inzet - speler.inzet + extra_chips);
                    if res.is_ok() {
                        spel.huidige_inzet += extra_chips;
                    }

                    res
                }
            }
        };

        if res.is_ok() {
            println!("    [SERV] OK");
            // stuur actie naar alle andere spelers
            for sp in andere_spelers {
                sp.stuur_bericht(PokbotcomMelding::SpelerActie(speler_id, actie), true);
            }

            loop {
                spel.aan_de_beurt = (spel.aan_de_beurt + 1).rem(spel.spelers.len());
                let nu_actieve_speler = self
                    .spelers
                    .iter()
                    .find(|s| s.id.get().unwrap() == &spel.spelers[spel.aan_de_beurt])
                    .unwrap();
                if nu_actieve_speler.hand.is_some() {
                    if spel
                        .laatste_actionabele_speler
                        .is_some_and(|id| &id == nu_actieve_speler.id.get().unwrap())
                    {
                        return self.ronde_klaar(spel_id);
                    }
                    break;
                } else {
                    nu_actieve_speler.stuur_bericht(PokbotcomMelding::AanDeBeurt, true);
                }
            }
        } else {
            println!("    [SERV] {:?}", res);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_spel_happy_flow() {
        let speler_a = Speler::new_zonder_id(String::from("Aart"));
        let speler_b = Speler::new_zonder_id(String::from("Bart"));
        let speler_c = Speler::new_zonder_id(String::from("Cart"));
        let speler_d = Speler::new_zonder_id(String::from("Dart"));

        let mut centrale = Centrale::new();

        let id_a = centrale.registreer_speler(speler_a).unwrap();
        let id_b = centrale.registreer_speler(speler_b).unwrap();
        let id_c = centrale.registreer_speler(speler_c).unwrap();
        let id_d = centrale.registreer_speler(speler_d).unwrap();

        let spel_id = centrale.maak_spel(vec![id_a, id_b, id_c, id_d], Some(0));

        assert!(id_a == SpelerId(0));
        assert!(id_b == SpelerId(1));
        assert!(id_c == SpelerId(2));
        assert!(id_d == SpelerId(3));
        assert!(spel_id == SpelId(0));

        centrale.start_spel(spel_id).unwrap();

        assert_eq!(centrale.spellen[0].aan_de_beurt, 3);

        assert!(centrale.ontvang_actie(spel_id, id_a, Actie::Fold).is_err());
        assert!(centrale.ontvang_actie(spel_id, id_d, Actie::Fold).is_ok());

        assert_eq!(centrale.spellen[0].aan_de_beurt, 0);

        assert!(centrale.ontvang_actie(spel_id, id_a, Actie::Check).is_err());
        assert_eq!(centrale.spellen[0].aan_de_beurt, 0);
        assert!(centrale.ontvang_actie(spel_id, id_a, Actie::Call).is_ok());

        assert_eq!(centrale.spelers[0].chips, CHIPS_PER_SPELER - BIG_BLIND);
        assert_eq!(centrale.spelers[0].inzet, BIG_BLIND);

        assert!(centrale
            .ontvang_actie(spel_id, id_b, Actie::Bet(50))
            .is_ok());
        assert_eq!(centrale.spelers[1].inzet, BIG_BLIND + 50);
        assert_eq!(centrale.spelers[1].chips, CHIPS_PER_SPELER - BIG_BLIND - 50);

        assert!(centrale.ontvang_actie(spel_id, id_c, Actie::Call).is_ok());
        assert_eq!(centrale.spelers[2].inzet, BIG_BLIND + 50);
        assert_eq!(centrale.spelers[2].chips, CHIPS_PER_SPELER - BIG_BLIND - 50);

        assert_eq!(centrale.spellen[0].aan_de_beurt, 0);
        
        // Door te folden, kunnen we de turn te zien krijgen.
        assert!(centrale.ontvang_actie(spel_id, id_a, Actie::Fold).is_ok());
        
        assert!(centrale.ontvang_actie(spel_id, id_b, Actie::Check).is_ok());
        assert!(centrale.ontvang_actie(spel_id, id_c, Actie::Check).is_ok());
        // De flop
        assert!(centrale.ontvang_actie(spel_id, id_b, Actie::Check).is_ok());
        assert!(centrale.ontvang_actie(spel_id, id_c, Actie::Check).is_ok());

        // We checken even of de pot wel degelijk 50 + 50 + drie BIG_BLINDS bevat
        assert_eq!(centrale.spellen[0].pot, 50 + 50 + BIG_BLIND + BIG_BLIND + BIG_BLIND);

        // De river
        assert!(centrale.ontvang_actie(spel_id, id_b, Actie::Check).is_ok());
        assert!(centrale.ontvang_actie(spel_id, id_c, Actie::Check).is_ok());
        
        // Cart heeft een straight, wat de beste hand is. Dus Cart wint de pot.
        // Hij is Big blind en heeft 50 gebet, dus zou op CHIPS_PER_SPELER - 50 - BIG BLIND + (POT) moeten zitten
        // MAAR aangezien de volgende ronde onmiddellijk al begonnen is, en Cart daarin Small Blind is, gaat er nog SMALL_BLIND vanaf gegaan zijn
        assert_eq!(centrale.spelers[2].chips, CHIPS_PER_SPELER - 50 - BIG_BLIND + (50 + 50 + BIG_BLIND + BIG_BLIND + BIG_BLIND) - SMALL_BLIND);

        // Gesamtronde voorbij, Aart is aan de beurt aangezien Bart dealt, Cart SB en Dart BB is.
        assert_eq!(centrale.spellen[0].aan_de_beurt, 0);
    }
}
