#![allow(dead_code, unused_variables)]

use rand::prelude::*;

pub mod kaart {
    use core::cmp::Ordering::{self, Equal, Greater, Less};

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
                        1..=9 => Tal(i + 1),
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
        fn new(kaarten: [Kaart; 5]) -> Self {
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

            let allen_dezelfde_kleur = slice.iter().all(|k| k.kleur == slice[0].kleur);
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
enum SpelStatus {
    Wachtend,
    Lopend,
    Gestopt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpelerId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpelId(u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Speler {
    id: OnceCell<SpelerId>,
    naam: String,
    chips: u64,
    hand: Option<(Kaart, Kaart)>,
    inzet: u64,
}

const CHIPS_PER_SPELER: u64 = 1000;
const SMALL_BLIND: u64 = 10;
const BIG_BLIND: u64 = 20;

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

    pub fn stuur_bericht(&self, bericht: impl std::fmt::Display) {
        println!(
            "[{}]: {bericht}",
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

    pub fn zet_chips_in(&mut self, te_vorderen: u64) -> Result<(), ()> {
        if self.chips >= te_vorderen {
            self.chips -= te_vorderen;
            self.inzet += te_vorderen;
            Ok(())
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Spel {
    id: OnceCell<SpelId>,
    spelers: Vec<SpelerId>,
    pot: u64,
    tafel: (Option<[Kaart; 3]>, Option<Kaart>, Option<Kaart>),
    huidige_dealer: usize, // index in vector van spelers
    aan_de_beurt: usize,   // index in vector van spelers
    deck: Vec<Kaart>,
    huidige_inzet: u64,
    laatste_actionabele_speler: Option<SpelerId>,
    status: SpelStatus,
}

impl Spel {
    pub fn new(toegekende_id: SpelId) -> Self {
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
        }
    }
}

pub enum Actie {
    Fold,
    Check,
    Call,
    Bet(u64),
}

#[derive(Debug)]
pub struct Centrale {
    spelers: Vec<Speler>,
    spellen: Vec<Spel>,

    volgende_geldige_speler_id: AtomicU64,
    volgende_geldige_spel_id: AtomicU64,
}

impl Centrale {
    fn new() -> Self {
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

    fn registreer_speler(&mut self, speler: Speler) -> SpelerId {
        let geregistreerde_id = SpelerId(
            self.volgende_geldige_speler_id
                .fetch_add(1, Ordering::Relaxed),
        );
        speler
            .id
            .set(geregistreerde_id)
            .expect("Speler had al een ID.");
        self.spelers.push(speler);
        geregistreerde_id
    }

    fn verwijder_speler(&mut self, speler_id: SpelerId) {
        self.spelers.swap_remove(
            self.spelers
                .iter()
                .position(|s| s.id.get().unwrap() == &speler_id)
                .unwrap(),
        );
    }

    fn maak_spel(&mut self, spelers: Vec<SpelerId>) -> SpelId {
        let geregistreerde_id = SpelId(
            self.volgende_geldige_spel_id
                .fetch_add(1, Ordering::Relaxed),
        );
        let mut leeg_spel = Spel::new(geregistreerde_id);
        leeg_spel.spelers = spelers;
        self.spellen.push(leeg_spel);
        geregistreerde_id
    }

    fn start_spel(&mut self, spel_id: SpelId) {
        let spel = self
            .spellen
            .iter_mut()
            .find(|s| s.id.get().unwrap() == &spel_id)
            .unwrap();
        assert!(spel.spelers.len() >= 2);

        spel.deck = Kaart::maak_deck().to_vec();
        let mut rng = thread_rng();
        spel.deck.shuffle(&mut rng);

        spel.status = SpelStatus::Lopend;

        spel.tafel = (None, None, None);

        spel.huidige_inzet = BIG_BLIND;

        for (lokale_id, speler_id) in spel.spelers.iter().enumerate() {
            let speler = self.spelers.iter_mut().find(|s| s.id.get().unwrap() == speler_id).unwrap();
            speler.hand = Some((spel.deck.pop().unwrap(), spel.deck.pop().unwrap()));
            speler.stuur_bericht("hand leeggemaakt");

            if lokale_id == (spel.huidige_dealer + 1).rem(spel.spelers.len()) {
                speler
                    .zet_chips_in(SMALL_BLIND)
                    .expect("Geen chips meer over.");
            } else if lokale_id == (spel.huidige_dealer + 2).rem(spel.spelers.len()) {
                speler
                    .zet_chips_in(BIG_BLIND)
                    .expect("Geen chips meer over.");
            }
        }

        spel.aan_de_beurt = (spel.huidige_dealer + 3).rem(spel.spelers.len());
    }

    pub fn ontvang_actie(
        &mut self,
        spel_id: SpelId,
        speler_id: SpelerId,
        actie: Actie,
    ) -> Result<(), String> {
        let spel = self.spellen.iter_mut().find(|s| s.id.get().unwrap() == &spel_id).unwrap();
        let speler = self.spelers.iter_mut().find(|s| s.id.get().unwrap() == &speler_id).unwrap();

        if speler_id != spel.spelers[spel.aan_de_beurt] {
            return Err("Niet jouw beurt".to_string());
        }

        let res = match actie {
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
                        // ronde klaar
                    }
                    _ => {}
                }

                if speler.inzet < spel.huidige_inzet {
                    Err("Je kunt niet checken, je zit onder de huidige inzet".to_string())
                } else {
                    Ok(())
                }
            }
            Actie::Call => {
                spel.laatste_actionabele_speler = Some(speler_id);

                speler
                    .zet_chips_in(spel.huidige_inzet - speler.inzet)
                    .map_err(|_| "Niet genoeg chips".to_string())
            }
            Actie::Bet(extra_chips) => {
                spel.laatste_actionabele_speler = Some(speler_id);

                let res = speler
                    .zet_chips_in(spel.huidige_inzet - speler.inzet + extra_chips)
                    .map_err(|_| "Niet genoeg chips".to_string());
                if res.is_ok() {
                    spel.huidige_inzet += extra_chips;
                }

                res
            }
        };

        if res.is_ok() {
            // TODO: check gefolde spelers
            spel.aan_de_beurt = (spel.aan_de_beurt + 1).rem(spel.spelers.len());
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_spel_happy_flow() {
        let speler_a = Speler::new_zonder_id(String::from("Jakobus"));
        let speler_b = Speler::new_zonder_id(String::from("Annemarieke"));
        let speler_c = Speler::new_zonder_id(String::from("Maruschka"));
        let speler_d = Speler::new_zonder_id(String::from("Kwezelken"));

        let mut centrale = Centrale::new();

        let id_a = centrale.registreer_speler(speler_a);
        let id_b = centrale.registreer_speler(speler_b);
        let id_c = centrale.registreer_speler(speler_c);
        let id_d = centrale.registreer_speler(speler_d);

        let spel_id = centrale.maak_spel(vec![id_a, id_b, id_c, id_d]);

        assert!(id_a == SpelerId(0));
        assert!(id_b == SpelerId(1));
        assert!(id_c == SpelerId(2));
        assert!(id_d == SpelerId(3));
        assert!(spel_id == SpelId(0));

        centrale.start_spel(spel_id);

        assert_eq!(centrale.spellen[0].aan_de_beurt, 3);

        assert!(centrale.ontvang_actie(spel_id, id_a, Actie::Fold).is_err());
        assert!(centrale.ontvang_actie(spel_id, id_d, Actie::Fold).is_ok());

        assert_eq!(centrale.spellen[0].aan_de_beurt, 0);

        assert!(centrale.ontvang_actie(spel_id, id_a, Actie::Check).is_err());
        assert_eq!(centrale.spellen[0].aan_de_beurt, 0);
        assert!(centrale.ontvang_actie(spel_id, id_a, Actie::Call).is_ok());

        assert_eq!(centrale.spelers[0].chips, CHIPS_PER_SPELER - BIG_BLIND);
        assert_eq!(centrale.spelers[0].inzet, BIG_BLIND);

        assert!(centrale.ontvang_actie(spel_id, id_b, Actie::Bet(50)).is_ok());
    }
}
