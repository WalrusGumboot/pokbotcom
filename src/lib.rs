#![allow(dead_code, unused_variables)]

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

use std::{cell::OnceCell, sync::atomic::{AtomicU64, Ordering}};

use crate::kaart::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SpelStatus {
    Wachtend,
    Lopend,
    Gestopt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Positie {
    Dealer,
    SmallBlind,
    BigBlind,
    Overige,
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
    hand: Option<[Kaart; 2]>,
}

const CHIPS_PER_SPELER: u64 = 1000;

impl Speler {
    pub fn new_zonder_id(naam: String) -> Self {
        Speler {
            id: OnceCell::new(),
            naam,
            chips: CHIPS_PER_SPELER,
            hand: None,
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
            status: SpelStatus::Wachtend,
        }
    }
}

enum Actie {
    Fold,
    Call,
    Bet(u64),
    Raise(u64),
}

#[derive(Debug)]
struct Centrale {
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
        let geregistreerde_id = SpelerId(self.volgende_geldige_speler_id.fetch_add(1, Ordering::Relaxed));
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
        let geregistreerde_id = SpelId(self.volgende_geldige_spel_id.fetch_add(1, Ordering::Relaxed));
        let mut leeg_spel = Spel::new(geregistreerde_id);
        leeg_spel.spelers = spelers;
        self.spellen.push(leeg_spel);
        geregistreerde_id
    }

    fn stuur_actie(&mut self, speler_id: SpelerId, spel_id: SpelId, actie: Actie) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_spel() {
        let speler_a = Speler::new_zonder_id(String::from("Jakobus"));
        let speler_b = Speler::new_zonder_id(String::from("Annemarieke"));
        let speler_c = Speler::new_zonder_id(String::from("Maruschka"));

        let mut centrale = Centrale::new();

        let id_a = centrale.registreer_speler(speler_a);
        let id_b = centrale.registreer_speler(speler_b);
        let id_c = centrale.registreer_speler(speler_c);

        dbg!(id_b);

        let spel_id = centrale.maak_spel(vec![id_a, id_b, id_c]);

        assert!(id_a == SpelerId(0));
        assert!(id_b == SpelerId(1));
        assert!(id_c == SpelerId(2));
        assert!(spel_id == SpelId(0));
    }
}