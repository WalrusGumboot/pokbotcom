#![allow(dead_code)]

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
        fn new(kleur: Kleur, waarde: Waarde) -> Self {
            Self { kleur, waarde }
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

pub mod spel {
    use crate::kaart::*;

    enum Positie {
        Dealer,
        SmallBlind,
        BigBlind,
        Overige
    }

    pub struct Speler {
        naam: String,
        chips: u64,
        hand: Option<[Kaart; 2]>,
        positie: Positie
    }

    pub struct Spel {
        spelers: Vec<Speler>,
        pot: u64,
        tafel: (
            Option<[Kaart; 3]>,
            Option<Kaart>,
            Option<Kaart>
        ),
        huidige_dealer: usize, // index in vector van spelers
        aan_de_beurt: usize, // index in vector van spelers
    }
}