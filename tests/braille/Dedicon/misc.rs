// Tests for the Dedicon braille code (Dutch math notation in ASCII): more examples of MathML, with the expected
// linear notation. The notation follows the rules and examples on the Dedicon website (see website.rs for the pages
// per topic). Choices that are not covered by those pages are marked with NOTE.
// Rules: Rules/Braille/Dedicon/

// Dedicon examples: onder en boven
// Constructs under or over an expression that are not accents or large operators (default munder/mover rule).
// NOTE: Dedicon still has to decide on a notation for these; the expected output is provisional.
mod onder_en_boven {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Accolade onder een som: de som krijgt haakjes.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><munder><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>⏟</mo></munder></math>"#;
        test_braille("Dedicon", expr, r#"(a + b)_accoladeOnder"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Accolade boven een som, met een tekst erboven.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mover><mover><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>⏞</mo></mover><mtext>som</mtext></mover></math>"#;
        test_braille("Dedicon", expr, r#"(a + b)^accoladeBoven^som"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Streep onder een variabele.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><munder><mi>x</mi><mo>_</mo></munder></math>"#;
        test_braille("Dedicon", expr, r#"x_-"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Tilde boven een variabele.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mover><mi>x</mi><mo>~</mo></mover><mo>+</mo><mn>1</mn></math>"#;
        test_braille("Dedicon", expr, r#"x^~ + 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Iets onder en boven een variabele.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><munderover><mi>A</mi><mn>1</mn><mn>2</mn></munderover></math>"#;
        test_braille("Dedicon", expr, r#"A_1 ^2"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Tekst boven een isgelijkteken (is per definitie gelijk aan).
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mi>a</mi><mover><mo>=</mo><mtext>def</mtext></mover><mi>b</mi></math>"#;
        test_braille("Dedicon", expr, r#"a = ^def b"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Label boven een pijl.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><mi>A</mi><mover><mo>→</mo><mi>f</mi></mover><mi>B</mi></math>"#;
        test_braille("Dedicon", expr, r#"A-->^f B"#)?;
        return Ok(());
    }
}

// Dedicon examples: samengestelde constructies
// Parentheses and spaces in combinations of fractions, scripts, function names, accents and text.
mod samengestelde_constructies {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Macht van een breuk: de breuk als grondtal krijgt haakjes.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><msup><mfrac><mn>1</mn><mn>2</mn></mfrac><mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"(1/2)^2"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Macht van een onderindex: een spatie sluit de onderindex af voor de "^".
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><msup><msub><mi>x</mi><mi>n</mi></msub><mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"x_n ^2"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Deelstreep als losse operator tussen twee onderindexen: spaties om de "/".
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><msub><mi>u</mi><mi>n</mi></msub><mo>/</mo><msub><mi>v</mi><mi>n</mi></msub></math>"#;
        test_braille("Dedicon", expr, r#"u_n / v_n"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Breuk met een minteken (U+2212) in de teller: geen haakjes.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mfrac><mrow><mo>−</mo><mn>5</mn></mrow><mn>11</mn></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"-5/11"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Breuk met een negatieve variabele in de teller: geen haakjes.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mfrac><mrow><mo>-</mo><mi>x</mi></mrow><mn>2</mn></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"-x/2"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Functienaam direct na een variabele: een spatie ervoor en erna.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mi>a</mi><mi>sin</mi><mi>x</mi></math>"#;
        test_braille("Dedicon", expr, r#"a sin x"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Functienaam als tekst (mtext): een spatie tussen de naam en het argument.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><mtext>sin</mtext><mi>x</mi></math>"#;
        test_braille("Dedicon", expr, r#"sin x"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Streep boven een uitdrukking die al haakjes heeft: geen extra haakjes.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mrow><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></mrow><mo>¯</mo></mover></math>"#;
        test_braille("Dedicon", expr, r#"{-(a + b)"#)?;
        return Ok(());
    }

    /// Voorbeeld 9: Streep boven een variabele met het teken ‾ (overline).
    #[test]
    fn voorbeeld_09() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mi>z</mi><mo>‾</mo></mover></math>"#;
        test_braille("Dedicon", expr, r#"{-z"#)?;
        return Ok(());
    }

    /// Voorbeeld 10: Streep boven twee variabelen: geen haakjes.
    #[test]
    fn voorbeeld_10() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mrow><mi>A</mi><mi>B</mi></mrow><mo>¯</mo></mover></math>"#;
        test_braille("Dedicon", expr, r#"{-AB"#)?;
        return Ok(());
    }

    /// Voorbeeld 11: Tekst als eerste kind van een geneste rij: de spatie ervoor komt van de rij erboven.
    #[test]
    fn voorbeeld_11() -> Result<()> {
        let expr = r#"<math><mi>x</mi><mo>=</mo><mn>1</mn><mrow><mtext>en</mtext><mi>y</mi><mo>=</mo><mn>2</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"x = 1 en y = 2"#)?;
        return Ok(());
    }

    /// Voorbeeld 12: Tekst als laatste kind van een geneste rij: de spatie erna komt van de rij erboven.
    #[test]
    fn voorbeeld_12() -> Result<()> {
        let expr = r#"<math><mrow><mi>x</mi><mo>=</mo><mn>1</mn><mtext>en</mtext></mrow><mi>y</mi><mo>=</mo><mn>2</mn></math>"#;
        test_braille("Dedicon", expr, r#"x = 1 en y = 2"#)?;
        return Ok(());
    }

    /// Voorbeeld 13: Tekst direct tussen haakjes: geen spaties naast de haakjes.
    #[test]
    fn voorbeeld_13() -> Result<()> {
        let expr = r#"<math><mi>P</mi><mo>(</mo><mtext>geen rode knikkers</mtext><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"P(geen rode knikkers)"#)?;
        return Ok(());
    }

    /// Voorbeeld 14: Breuk als eerste kind, twee rijen diep: een spatie ervoor komt van de rij erboven.
    #[test]
    fn voorbeeld_14() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mrow><mrow><mfrac><mn>1</mn><mn>3</mn></mfrac><mi>x</mi></mrow><mo>+</mo><mn>1</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"2 1/3 x + 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 15: Breuk als laatste kind, twee rijen diep: een spatie erna komt van de rij erboven.
    #[test]
    fn voorbeeld_15() -> Result<()> {
        let expr = r#"<math><mrow><mn>1</mn><mo>+</mo><mrow><mi>x</mi><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow></mrow><mi>y</mi></math>"#;
        test_braille("Dedicon", expr, r#"1 + x 1/2 y"#)?;
        return Ok(());
    }

    /// Voorbeeld 16: Breuk in een breuk: de breuk in de teller krijgt haakjes.
    #[test]
    fn voorbeeld_16() -> Result<()> {
        let expr = r#"<math><mfrac><mfrac><mn>1</mn><mn>2</mn></mfrac><mn>3</mn></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"(1/2)/3"#)?;
        return Ok(());
    }

    /// Voorbeeld 17: Functienaam direct na een getal: geen spatie ervoor (zoals "2sqrt(25)").
    #[test]
    fn voorbeeld_17() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mi>sin</mi><mi>x</mi></math>"#;
        test_braille("Dedicon", expr, r#"2sin x"#)?;
        return Ok(());
    }

    /// Voorbeeld 18: Logaritme direct na een getal: geen spatie ervoor.
    #[test]
    fn voorbeeld_18() -> Result<()> {
        let expr = r#"<math><mn>3</mn><mi>log</mi><mi>x</mi></math>"#;
        test_braille("Dedicon", expr, r#"3log x"#)?;
        return Ok(());
    }

    /// Voorbeeld 19: Tekst met een afkorting: de spatie na de punt blijft staan.
    #[test]
    fn voorbeeld_19() -> Result<()> {
        let expr = r#"<math><mrow><mtext>opp. vloer</mtext><mo>=</mo><mn>12</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"opp. vloer = 12"#)?;
        return Ok(());
    }

    /// Voorbeeld 20: Functienaam voor een Griekse letter: de spatie ertussen blijft staan, ook na een getal.
    #[test]
    fn voorbeeld_20() -> Result<()> {
        let expr = r#"<math><mi>sin</mi><mi>α</mi><mo>+</mo><mn>2</mn><mi>sin</mi><mi>β</mi></math>"#;
        test_braille("Dedicon", expr, r#"sin ~a + 2sin ~b"#)?;
        return Ok(());
    }
}

// Dedicon examples: logica
mod logica {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Niet voor alle.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mo>¬</mo><mo>∀</mo><mi>x</mi><mi>P</mi></math>"#;
        test_braille("Dedicon", expr, r#"nt voor alle x P"#)?;
        return Ok(());
    }
}

// Dedicon examples: omcirkelde operatoren
// NOTE: the circled operators need discussion (see unicode.yaml).
mod omcirkelde_operatoren {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Plusteken, minteken en vermenigvuldigingsteken in een cirkel, met spaties.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi>a</mi><mo>⊕</mo><mi>b</mi><mo>⊖</mo><mi>c</mi><mo>⊗</mo><mi>d</mi></math>"#;
        test_braille("Dedicon", expr, r#"a ["+ b ["- c ["* d"#)?;
        return Ok(());
    }
}

// Dedicon examples: pijlen
mod pijlen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Pijl naar boven en beneden, met spaties.
    /// NOTE: ",ô'" contains a non-ASCII letter; this needs discussion (see unicode.yaml).
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi>a</mi><mo>↕</mo><mi>b</mi></math>"#;
        test_braille("Dedicon", expr, r#"a ,ô' b"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Pijl naar rechts boven een pijl naar links (evenwicht), zonder spaties.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mi>A</mi><mo>⇄</mo><mi>B</mi></math>"#;
        test_braille("Dedicon", expr, r#"A'=,B"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Pijl naar links boven een pijl naar rechts, zonder spaties.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mi>A</mi><mo>⇆</mo><mi>B</mi></math>"#;
        test_braille("Dedicon", expr, r#"A,='B"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Pijl naar links met dubbele stok, met spaties.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mi>P</mi><mo>⇐</mo><mi>Q</mi></math>"#;
        test_braille("Dedicon", expr, r#"P ,= Q"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Lange pijl naar links met dubbele stok, geschreven als de gewone.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mi>P</mi><mo>⟸</mo><mi>Q</mi></math>"#;
        test_braille("Dedicon", expr, r#"P ,= Q"#)?;
        return Ok(());
    }
}

// Dedicon examples: valuta
mod valuta {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Euroteken voor een bedrag.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi>€</mi><mn>700</mn></math>"#;
        test_braille("Dedicon", expr, r#"€700"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Pondteken voor een bedrag.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mi>£</mi><mn>5</mn></math>"#;
        test_braille("Dedicon", expr, r#"£5"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Yenteken voor een bedrag.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mi>¥</mi><mn>100</mn></math>"#;
        test_braille("Dedicon", expr, r#"yen100"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Florijnteken voor een bedrag.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mi>ƒ</mi><mn>2,50</mn></math>"#;
        test_braille("Dedicon", expr, r#"f2,50"#)?;
        return Ok(());
    }
}

// Dedicon examples: letters in andere stijlen
// Bold, script and other styles are written as the plain letter; Greek letters get "~" (see unicode-full.yaml).
mod letters_in_andere_stijlen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Vette, schrijf- en vette Griekse letter (mathvariant).
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi mathvariant="bold">v</mi><mo>+</mo><mi mathvariant="script">L</mi><mo>+</mo><mi mathvariant="bold">α</mi></math>"#;
        test_braille("Dedicon", expr, r#"v + L + ~a"#)?;
        return Ok(());
    }
}

// Dedicon examples: griekse letters
// NOTE: the Greek letters still need to be checked with Dedicon (see unicode.yaml).
mod griekse_letters {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Sigma aan het eind van een woord (final sigma), geschreven als sigma.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi>ς</mi><mo>+</mo><mi>σ</mi></math>"#;
        test_braille("Dedicon", expr, r#"~s + ~s"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Alle Griekse hoofdletters.
    /// NOTE: to check with Dedicon, especially eta "~Û", theta "~Ô" and chi "~Ç".
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mi>Α</mi><mo>,</mo><mi>Β</mi><mo>,</mo><mi>Γ</mi><mo>,</mo><mi>Δ</mi><mo>,</mo><mi>Ε</mi><mo>,</mo><mi>Ζ</mi><mo>,</mo><mi>Η</mi><mo>,</mo><mi>Θ</mi><mo>,</mo><mi>Ι</mi><mo>,</mo><mi>Κ</mi><mo>,</mo><mi>Λ</mi><mo>,</mo><mi>Μ</mi><mo>,</mo><mi>Ν</mi><mo>,</mo><mi>Ξ</mi><mo>,</mo><mi>Ο</mi><mo>,</mo><mi>Π</mi><mo>,</mo><mi>Ρ</mi><mo>,</mo><mi>Σ</mi><mo>,</mo><mi>Τ</mi><mo>,</mo><mi>Υ</mi><mo>,</mo><mi>Φ</mi><mo>,</mo><mi>Χ</mi><mo>,</mo><mi>Ψ</mi><mo>,</mo><mi>Ω</mi></math>"#;
        test_braille("Dedicon", expr, r#"~A, ~B, ~G, ~D, ~E, ~Z, ~Û, ~Ô, ~I, ~K, ~L, ~M, ~N, ~X, ~O, ~P, ~R, ~S, ~T, ~U, ~F, ~Ç, ~Y, ~W"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Alle Griekse kleine letters.
    /// NOTE: to check with Dedicon, especially eta "~û", theta "~ô" and chi "~ç".
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mi>α</mi><mo>,</mo><mi>β</mi><mo>,</mo><mi>γ</mi><mo>,</mo><mi>δ</mi><mo>,</mo><mi>ε</mi><mo>,</mo><mi>ζ</mi><mo>,</mo><mi>η</mi><mo>,</mo><mi>θ</mi><mo>,</mo><mi>ι</mi><mo>,</mo><mi>κ</mi><mo>,</mo><mi>λ</mi><mo>,</mo><mi>μ</mi><mo>,</mo><mi>ν</mi><mo>,</mo><mi>ξ</mi><mo>,</mo><mi>ο</mi><mo>,</mo><mi>π</mi><mo>,</mo><mi>ρ</mi><mo>,</mo><mi>σ</mi><mo>,</mo><mi>τ</mi><mo>,</mo><mi>υ</mi><mo>,</mo><mi>φ</mi><mo>,</mo><mi>χ</mi><mo>,</mo><mi>ψ</mi><mo>,</mo><mi>ω</mi></math>"#;
        test_braille("Dedicon", expr, r#"~a, ~b, ~g, ~d, ~e, ~z, ~û, ~ô, ~i, ~k, ~l, ~m, ~n, ~x, ~o, ~p, ~r, ~s, ~t, ~u, ~f, ~ç, ~y, ~w"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Griekse letter direct na een getal en voor een variabele.
    /// NOTE: no space, as for other letters ("3x"); to check with Dedicon whether "2~pr" is clear.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mn>3</mn><mi>Φ</mi><mo>+</mo><mn>2</mn><mi>π</mi><mi>r</mi></math>"#;
        test_braille("Dedicon", expr, r#"3~F + 2~pr"#)?;
        return Ok(());
    }
}

// Dedicon examples: accenten
// A hat over an expression is written as "{^" before it, like the overbar "{-" and the vector arrow "{-->"
// (Dedicon symbol list: "{^" dakje midden boven).
mod accenten {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Dakje boven een variabele (circumflex accent ^).
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mi>x</mi><mo>^</mo></mover></math>"#;
        test_braille("Dedicon", expr, r#"{^x"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Dakje boven een variabele (modifier letter circumflex ˆ).
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mi>x</mi><mo>ˆ</mo></mover></math>"#;
        test_braille("Dedicon", expr, r#"{^x"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Dakje boven een variabele (combining circumflex U+0302).
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mi>x</mi><mo>&#x302;</mo></mover></math>"#;
        test_braille("Dedicon", expr, r#"{^x"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Dakje boven een som.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>^</mo></mover><mo>=</mo><mn>1</mn></math>"#;
        test_braille("Dedicon", expr, r#"{^(a + b) = 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Accent als los operatorteken na een lijnstuk.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mrow><mi>ST</mi><mo>'</mo><mo>=</mo><mn>5</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"ST' = 5"#)?;
        return Ok(());
    }
}

// Dedicon examples: onbekende elementen
// Elements without a Dedicon rule fall back on the default rules: the content is written as it is, without extra text.
mod onbekende_elementen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Een element met inhoud zonder eigen regel (merror).
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>+</mo><merror><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></merror></math>"#;
        test_braille("Dedicon", expr, r#"2 + x + 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Een leeg element zonder eigen regel (mglyph).
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mi>a</mi><mo>+</mo><mglyph src="ster.png" alt="ster"/></math>"#;
        test_braille("Dedicon", expr, r#"a +"#)?;
        return Ok(());
    }
}

// Dedicon examples: menclose
// NOTE: it is not known whether Dedicon uses or writes enclosures (the Dedicon website has no rules for them);
// this still needs to be investigated. For now only the content is written, for every notation.
mod menclose {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Kader (box).
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>+</mo><menclose notation="box"><mi>x</mi><mo>+</mo><mn>1</mn></menclose></math>"#;
        test_braille("Dedicon", expr, r#"2 + x + 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Wortelteken (radical).
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>+</mo><menclose notation="radical"><mi>x</mi><mo>+</mo><mn>1</mn></menclose></math>"#;
        test_braille("Dedicon", expr, r#"2 + x + 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Streep erboven (top).
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>+</mo><menclose notation="top"><mi>x</mi><mo>+</mo><mn>1</mn></menclose></math>"#;
        test_braille("Dedicon", expr, r#"2 + x + 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Doorgestreept (updiagonalstrike).
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>+</mo><menclose notation="updiagonalstrike"><mi>x</mi><mo>+</mo><mn>1</mn></menclose></math>"#;
        test_braille("Dedicon", expr, r#"2 + x + 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Meerdere notations tegelijk (box circle) en zonder notation.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>+</mo><menclose notation="box circle"><menclose><mi>x</mi><mo>+</mo><mn>1</mn></menclose></menclose></math>"#;
        test_braille("Dedicon", expr, r#"2 + x + 1"#)?;
        return Ok(());
    }
}

// Dedicon examples: ms (letterlijke tekst)
mod ms {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Letterlijke tekst met de standaard aanhalingstekens.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi>x</mi><mo>=</mo><ms>abc</ms></math>"#;
        test_braille("Dedicon", expr, r#"x = "abc""#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Letterlijke tekst met eigen aanhalingstekens (lquote en rquote).
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><ms lquote="'" rquote="'">abc</ms><mo>+</mo><mn>1</mn></math>"#;
        test_braille("Dedicon", expr, r#"'abc' + 1"#)?;
        return Ok(());
    }
}

// Dedicon examples: witruimte (mspace)
// An mspace of at least 0.7em becomes one space; a narrower mspace is ignored.
mod witruimte {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Smalle witruimte (0.3em).
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi>a</mi><mspace width="0.3em"/><mi>b</mi></math>"#;
        test_braille("Dedicon", expr, r#"ab"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Witruimte van 1em.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mi>a</mi><mspace width="1em"/><mi>b</mi></math>"#;
        test_braille("Dedicon", expr, r#"a b"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Brede witruimte (2em) tussen twee vergelijkingen.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mi>x</mi><mo>=</mo><mn>1</mn><mspace width="2em"/><mi>y</mi><mo>=</mo><mn>2</mn></math>"#;
        test_braille("Dedicon", expr, r#"x = 1 y = 2"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Witruimte naast tekst (mtext).
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mi>x</mi><mo>=</mo><mn>1</mn><mspace width="1em"/><mtext>en</mtext><mspace width="1em"/><mi>y</mi><mo>=</mo><mn>2</mn></math>"#;
        test_braille("Dedicon", expr, r#"x = 1 en y = 2"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Witruimte naast een isgelijkteken.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mi>x</mi><mspace width="1em"/><mo>=</mo><mspace width="1em"/><mn>1</mn></math>"#;
        test_braille("Dedicon", expr, r#"x = 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Een lege mspace als teller van een breuk.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mfrac><mspace width="1em"/><mn>2</mn></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"()/2"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Een lege mspace onder een wortelteken.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><msqrt><mspace width="1em"/></msqrt></math>"#;
        test_braille("Dedicon", expr, r#"sqrt()"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Een no-break space aan het eind van een variabele blijft staan.
    /// NOTE: putting a space inside an mi is an undesired practice; how to handle it needs further discussion.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mrow><mi>opp.&#xA0;</mi><mi>I</mi><mo>+</mo><mi>opp.&#xA0;</mi><mi>II</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"opp. I + opp. II"#)?;
        return Ok(());
    }
}

// Dedicon examples: getallen met spaties
// A space (no-break space or thin mspace) between groups of digits is a thousands separator and becomes a point,
// if the grouping is valid: a first group of 1 to 3 digits, then only groups of exactly 3 digits.
// Otherwise the space stays, as it does after the decimal comma.
// Each example is also tested as a negative number: the minus sign and the number in one mrow.
mod getallen_met_spaties {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Duizendtal met een no-break space.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mn>1&#xA0;000</mn></math>"#;
        test_braille("Dedicon", expr, r#"1.000"#)?;
        let negative = r#"<math><mrow><mo>-</mo><mn>1&#xA0;000</mn></mrow></math>"#;
        test_braille("Dedicon", negative, r#"-1.000"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Duizendtal met een smalle mspace tussen de cijfergroepen.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mn>1</mn><mspace width="0.167em"/><mn>000</mn></math>"#;
        test_braille("Dedicon", expr, r#"1.000"#)?;
        let negative = r#"<math><mrow><mo>-</mo><mn>1</mn><mspace width="0.167em"/><mn>000</mn></mrow></math>"#;
        test_braille("Dedicon", negative, r#"-1.000"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Groot getal met meer groepen.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mn>50&#xA0;000&#xA0;000</mn></math>"#;
        test_braille("Dedicon", expr, r#"50.000.000"#)?;
        let negative = r#"<math><mrow><mo>-</mo><mn>50&#xA0;000&#xA0;000</mn></mrow></math>"#;
        test_braille("Dedicon", negative, r#"-50.000.000"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Getal met cijfergroepen en een decimale komma.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mn>1&#xA0;000,5</mn></math>"#;
        test_braille("Dedicon", expr, r#"1.000,5"#)?;
        let negative = r#"<math><mrow><mo>-</mo><mn>1&#xA0;000,5</mn></mrow></math>"#;
        test_braille("Dedicon", negative, r#"-1.000,5"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Ongeldige groep van 2 cijfers.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mn>1&#xA0;00</mn></math>"#;
        test_braille("Dedicon", expr, r#"1 00"#)?;
        let negative = r#"<math><mrow><mo>-</mo><mn>1&#xA0;00</mn></mrow></math>"#;
        test_braille("Dedicon", negative, r#"-1 00"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Ongeldige groep in het midden van het getal.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mn>50&#xA0;00&#xA0;000</mn></math>"#;
        test_braille("Dedicon", expr, r#"50 00 000"#)?;
        let negative = r#"<math><mrow><mo>-</mo><mn>50&#xA0;00&#xA0;000</mn></mrow></math>"#;
        test_braille("Dedicon", negative, r#"-50 00 000"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Ongeldige eerste groep van 4 cijfers.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><mn>1234&#xA0;567</mn></math>"#;
        test_braille("Dedicon", expr, r#"1234 567"#)?;
        let negative = r#"<math><mrow><mo>-</mo><mn>1234&#xA0;567</mn></mrow></math>"#;
        test_braille("Dedicon", negative, r#"-1234 567"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Spatie na de decimale komma.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mn>0,123&#xA0;456</mn></math>"#;
        test_braille("Dedicon", expr, r#"0,123 456"#)?;
        let negative = r#"<math><mrow><mo>-</mo><mn>0,123&#xA0;456</mn></mrow></math>"#;
        test_braille("Dedicon", negative, r#"-0,123 456"#)?;
        return Ok(());
    }
}

// Dedicon examples: negatieve getallen
mod negatieve_getallen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Negatief getal aan het begin.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mrow><mo>-</mo><mn>1</mn></mrow><mo>+</mo><mn>4</mn></math>"#;
        test_braille("Dedicon", expr, r#"-1 + 4"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Negatief getal na een plusteken.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mn>3</mn><mo>+</mo><mrow><mo>-</mo><mn>2</mn></mrow><mo>=</mo><mn>1</mn></math>"#;
        test_braille("Dedicon", expr, r#"3 + -2 = 1"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Negatief getal met variabele in het kwadraat.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mrow><mi>y</mi><mo>=</mo><mrow><mo>-</mo><mn>0,5</mn><msup><mi>x</mi><mn>2</mn></msup></mrow><mo>+</mo><mn>4,5</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"y = -0,5x^2 + 4,5"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Negatief decimaal getal met puntjes erachter.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mrow><mi>x</mi><mo>=</mo><mn>-16,899...</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"x = -16,899..."#)?;
        return Ok(());
    }
}

// Dedicon examples: sommen met mingetallen
// In a sum like 4 - -2 the second minus belongs to the number 2. Only the third MathML shows that with an <mrow>;
// the first two are common ways of writing it that do not show it.
mod sommen_met_mingetallen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Fout: twee losse <mo>-</mo>; uit de MathML blijkt niet dat de tweede min bij de 2 hoort.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mn>4</mn><mo>-</mo><mo>-</mo><mn>2</mn><mo>=</mo></math>"#;
        test_braille("Dedicon", expr, r#"4 - -2 ="#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Fout: de min staat in de <mn> (<mn>-2</mn>).
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mn>4</mn><mo>-</mo><mn>-2</mn><mo>=</mo></math>"#;
        test_braille("Dedicon", expr, r#"4 - -2 ="#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Goed: de <mrow> geeft aan dat de min bij de 2 hoort.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mn>4</mn><mo>-</mo><mrow><mo>-</mo><mn>2</mn></mrow><mo>=</mo></math>"#;
        test_braille("Dedicon", expr, r#"4 - -2 ="#)?;
        return Ok(());
    }
}

// Dedicon examples: breuken
mod breuken {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Eenvoudige breuk.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mfrac><mn>1</mn><mn>10</mn></mfrac></mrow></math>"#;
        test_braille("Dedicon", expr, r#"1/10"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Breuk met heel getal.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mo>=</mo><mrow><mn>10</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow><mo>-</mo><mrow><mn>1</mn><mfrac><mn>1</mn><mn>3</mn></mfrac><mi>x</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"= 10 1/2 - 1 1/3 x"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Breuk met variabele erachter.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mi>y</mi><mo>=</mo><mn>1</mn><mfrac><mn>1</mn><mn>2</mn></mfrac><mi>x</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"y = 1 1/2 x"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Breuk met een negatief getal in de teller.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mfrac><mrow><mo>-</mo><mn>5</mn></mrow><mrow><mn>11</mn></mrow></mfrac><mo>+</mo><mfrac><mn>3</mn><mrow><mn>11</mn></mrow></mfrac><mo>=</mo></math>"#;
        test_braille("Dedicon", expr, r#"-5/11 + 3/11 ="#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Negatieve breuk.
    /// NOTE: a minus sign before a fraction is separated from it by a space ("- 16/15").
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mo>-</mo><mfrac><mrow><mn>16</mn></mrow><mrow><mn>15</mn></mrow></mfrac><mo>=</mo><mrow><mo>-</mo><mn>1</mn></mrow><mfrac><mn>1</mn><mrow><mn>15</mn></mrow></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"- 16/15 = -1 1/15"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Breuk met een som in de teller.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mo>=</mo><mfrac><mrow><mrow><mo>-</mo><mn>5</mn></mrow><mo>+</mo><mn>3</mn></mrow><mrow><mn>11</mn></mrow></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"= (-5 + 3)/11"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Haakjes in de noemer: de hele noemer krijgt haakjes eromheen.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><mfrac><mi>xy</mi><mrow><mi>x</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"xy/(x(x + y))"#)?;
        return Ok(());
    }
}

// Dedicon examples: wortels
mod wortels {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Wortels.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mn>8</mn><msqrt><mn>3</mn></msqrt><mo>-</mo><msqrt><mn>27</mn></msqrt></mrow></math>"#;
        test_braille("Dedicon", expr, r#"8sqrt(3) - sqrt(27)"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Breuk onder een wortelteken.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><msqrt><mn>1</mn><mfrac><mn>5</mn><mn>11</mn></mfrac></msqrt></mrow></math>"#;
        test_braille("Dedicon", expr, r#"sqrt(1 5/11)"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Wortelteken in de noemer van een breuk.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mfrac><mn>7</mn><msqrt><mn>6</mn></msqrt></mfrac></mrow></math>"#;
        test_braille("Dedicon", expr, r#"7/sqrt(6)"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Heel getal en wortel in de noemer van een breuk.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mfrac><mn>21</mn><mrow><mn>3</mn><msqrt><mn>2</mn></msqrt></mrow></mfrac></mrow></math>"#;
        test_braille("Dedicon", expr, r#"21/(3sqrt(2))"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Derdemachtswortel in de noemer van een breuk.
    /// NOTE: a root has its own parentheses, so it gets no extra parentheses in a fraction, like a square root.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mfrac><mn>7</mn><mroot><mn>6</mn><mn>3</mn></mroot></mfrac></mrow></math>"#;
        test_braille("Dedicon", expr, r#"7/root_3(6)"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Wortel aan het eind van een uitdrukking tussen haakjes.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mrow><mi>x</mi><mrow><mo>(</mo><mrow><mn>6</mn><mo>-</mo><msqrt><mi>x</mi></msqrt></mrow><mo>)</mo></mrow></mrow></math>"#;
        test_braille("Dedicon", expr, r#"x(6 - sqrt(x))"#)?;
        return Ok(());
    }
}

// Dedicon examples: eenheden
mod eenheden {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Vierkante meter.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><msup><mi>m</mi><mrow><mn>2</mn></mrow></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"m^2"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Getal met kubieke centimeter.
    /// NOTE: no space between a number and a unit, as on the Dedicon website (machtsverheffen: "50cm^2", "10m^3"). This
    /// differs from the degree sign, which does get a space ("-14 grC", "30 gr"). Suggestion: mark units in the MathML,
    /// as <mtext> or with attributes on <mi> (mathvariant="normal", class="MathML-unit" or intent=":unit"), so that the
    /// rules can recognize them.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mn>343</mn><msup><mi>cm</mi><mrow><mn>3</mn></mrow></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"343cm^3"#)?;
        return Ok(());
    }
}

// Dedicon examples: machten
mod machten {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Macht van een getal.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><msup><mn>15</mn><mrow><mn>2</mn></mrow></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"15^2"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Macht met negatieve exponent.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mrow><mn>7</mn><mo>×</mo><mrow><msup><mrow><mn>10</mn></mrow><mrow><mn>-5</mn></mrow></msup></mrow></mrow></math>"#;
        test_braille("Dedicon", expr, r#"7 * 10^-5"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Machten bij variabelen.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mi>B</mi><mo>=</mo><mn>19</mn><msup><mi>x</mi><mn>6</mn></msup><mo>+</mo><mn>15</mn><msup><mi>x</mi><mn>5</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"B = 19x^6 + 15x^5"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Meer variabelen met machten achter elkaar.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mi>C</mi><mo>=</mo><mn>11</mn><msup><mi>p</mi><mn>6</mn></msup><msup><mi>q</mi><mn>3</mn></msup><mo>-</mo><mn>14</mn><msup><mi>p</mi><mn>3</mn></msup><msup><mi>q</mi><mn>5</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"C = 11p^6 q^3 - 14p^3 q^5"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Breuk met machten.
    /// NOTE: the fraction rule only puts parentheses around the part that needs them ("(-5 + 3)/11", "21/(3sqrt(2))");
    /// a superscript is simple, so the denominator b^3 gets none.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mfrac><mrow><mn>3</mn><msup><mi>a</mi><mn>2</mn></msup></mrow><mrow><msup><mi>b</mi><mn>3</mn></msup></mrow></mfrac></mrow></math>"#;
        test_braille("Dedicon", expr, r#"(3a^2)/b^3"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Macht van een negatief getal tussen haakjes, in een langere berekening.
    /// The minus sign before the fraction is separated from it by a space; the brackets of mfenced (without a
    /// separators attribute) get no commas next to the minus sign.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mrow><mi>f</mi><mfenced open="(" close=")"><mo>-</mo><mn>8</mn></mfenced><mo>=</mo><mo>-</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>⋅</mo><msup><mfenced open="(" close=")"><mo>-</mo><mn>8</mn></mfenced><mn>2</mn></msup><mo>+</mo><mn>2</mn><mo>⋅</mo><mn>-8</mn><mo>+</mo><mn>1</mn><mo>=</mo><mn>-47</mn></mrow></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(-8) = - 1/2 * (-8)^2 + 2 * -8 + 1 = -47"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Macht van een product met een wortel tussen haakjes.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><msup><mrow><mo>(</mo><mrow><mn>2</mn><msqrt><mn>2</mn></msqrt></mrow><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"(2sqrt(2))^2"#)?;
        return Ok(());
    }
}

// Dedicon examples: meetkunde
mod meetkunde {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Driehoekteken.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mi mathvariant="normal">∆ABC</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"drieh ABC"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Hoekteken.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mi mathvariant="normal">∠A</mi><mo>=</mo><mo>...</mo><mrow><mo>°</mo></mrow></mrow></math>"#;
        test_braille("Dedicon", expr, r#"hoek A = ...gr"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Hoekteken en gradenteken als losse operatortekens.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mrow><mo>∠</mo><mi>O</mi><mo>=</mo><mn>90</mn><mo>°</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"hoek O = 90 gr"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Driehoekteken als los operatorteken, met een breuk en een afkorting.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mrow><mo>∆</mo><mi>AEZ</mi><mo>=</mo><mfrac><mn>1</mn><mn>6</mn></mfrac><mo>⋅</mo><mtext>opp.</mtext><mo>∆</mo><mi>ABC</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"drieh AEZ = 1/6 * opp. drieh ABC"#)?;
        return Ok(());
    }
}

// Dedicon examples: gradenteken
mod gradenteken {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Negatieve temperatuur in graden Celsius.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mo>-</mo><mn>14</mn><mo>°</mo><mi>C</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"-14 grC"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Temperatuur in graden Celsius.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mn>8</mn><mo>°</mo><mi>C</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"8 grC"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Hoek in graden.
    /// NOTE: a space between the number and the degree sign, as for temperatures ("-14 grC").
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mn>30</mn><mo>°</mo></math>"#;
        test_braille("Dedicon", expr, r#"30 gr"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Hoek in graden, met het gradenteken als bovenindex.
    /// NOTE: the same output as with <mo>°</mo>.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><msup><mn>30</mn><mo>°</mo></msup></math>"#;
        test_braille("Dedicon", expr, r#"30 gr"#)?;
        return Ok(());
    }
}

// Dedicon examples: haakjes en leestekens
mod haakjes_en_leestekens {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Haakjes.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mrow><mo>(</mo><mrow><mn>28</mn><mo>-</mo><mn>25</mn></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"(28 - 25)"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Vergelijking van een lijn.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mi>y</mi><mo>=</mo><mrow><mo>-</mo><mi>x</mi></mrow><mo>+</mo><mn>2</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"y = -x + 2"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Coördinaten met een gemengd getal voor de komma: geen spatie voor de komma.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mrow><mi>C</mi><mrow><mo>(</mo><mrow><mn>1</mn><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>,</mo><mn>3</mn></mrow><mo>)</mo></mrow></mrow></math>"#;
        test_braille("Dedicon", expr, r#"C(1 1/2, 3)"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Coördinaten met een gemengd getal na de komma.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mrow><mi>C</mi><mrow><mo>(</mo><mrow><mn>0</mn><mo>,</mo><mn>3</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow><mo>)</mo></mrow></mrow></math>"#;
        test_braille("Dedicon", expr, r#"C(0, 3 1/2)"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Als voorbeeld 3, met mfenced en een komma als scheidingsteken.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mrow><mi>C</mi><mfenced open="(" close=")" separators=","><mrow><mn>1</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow><mn>3</mn></mfenced></mrow></math>"#;
        test_braille("Dedicon", expr, r#"C(1 1/2, 3)"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Als voorbeeld 4, met mfenced en een komma als scheidingsteken.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mrow><mi>C</mi><mfenced open="(" close=")" separators=","><mn>0</mn><mrow><mn>3</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow></mfenced></mrow></math>"#;
        test_braille("Dedicon", expr, r#"C(0, 3 1/2)"#)?;
        return Ok(());
    }
}

// Dedicon examples: relatietekens
mod relatietekens {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Is ongeveer.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mo>≈</mo></math>"#;
        test_braille("Dedicon", expr, r#"=ong"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Is niet.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mn>0,417</mn><mo>≠</mo><mn>0,464</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"0,417 =niet 0,464"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Komt overeen met.
    /// NOTE: no space between a number and a unit, as on the Dedicon website (machtsverheffen: "50cm^2", "10m^3"). This
    /// differs from the degree sign, which does get a space ("-14 grC", "30 gr"). Suggestion: mark units in the MathML,
    /// as <mtext> or with attributes on <mi> (mathvariant="normal", class="MathML-unit" or intent=":unit"), so that the
    /// rules can recognize them.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mrow><mn>1</mn><mtext>cm</mtext><mo>≙</mo><mn>200</mn><mi>N</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"1 cm =^ 200N"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Dubbele pijl.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mo>⇔</mo></math>"#;
        test_braille("Dedicon", expr, r#"<=>"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Groter of gelijk aan.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mi>x</mi><mo>≥</mo><mn>4</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"x >= 4"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Groter of gelijk aan, met een andere variabele.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mi>y</mi><mo>≥</mo><mn>5</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"y >= 5"#)?;
        return Ok(());
    }
}

// Dedicon examples: spatie tussen getal en eenheid
// The same units as elsewhere ("343cm^3", "200N", "50cm^2", "10m^3"), but with a space in the MathML:
// A: an mspace of 1em between the number and the unit; B: a no-break space (U+00A0) after the number.
// Both put a space between the number and the unit in the output.
mod spatie_tussen_getal_en_eenheid {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1A: Getal met kubieke centimeter, met mspace.
    #[test]
    fn voorbeeld_01a() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mn>343</mn><mspace width="1em"/><msup><mi>cm</mi><mrow><mn>3</mn></mrow></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"343 cm^3"#)?;
        return Ok(());
    }

    /// Voorbeeld 1B: Getal met kubieke centimeter, met een no-break space.
    #[test]
    fn voorbeeld_01b() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mn>343&#xA0;</mn><msup><mi>cm</mi><mrow><mn>3</mn></mrow></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"343 cm^3"#)?;
        return Ok(());
    }

    /// Voorbeeld 2A: Komt overeen met, met mspace voor de eenheid N.
    #[test]
    fn voorbeeld_02a() -> Result<()> {
        let expr = r#"<math><mrow><mn>1</mn><mtext>cm</mtext><mo>≙</mo><mn>200</mn><mspace width="1em"/><mi>N</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"1 cm =^ 200 N"#)?;
        return Ok(());
    }

    /// Voorbeeld 2B: Komt overeen met, met een no-break space voor de eenheid N.
    #[test]
    fn voorbeeld_02b() -> Result<()> {
        let expr = r#"<math><mrow><mn>1</mn><mtext>cm</mtext><mo>≙</mo><mn>200&#xA0;</mn><mi>N</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"1 cm =^ 200 N"#)?;
        return Ok(());
    }

    /// Voorbeeld 3A: Vierkante centimeter, met mspace.
    #[test]
    fn voorbeeld_03a() -> Result<()> {
        let expr = r#"<math display="block"><mn>50</mn><mspace width="1em"/><msup><mi>cm</mi> <mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"50 cm^2"#)?;
        return Ok(());
    }

    /// Voorbeeld 3B: Vierkante centimeter, met een no-break space.
    #[test]
    fn voorbeeld_03b() -> Result<()> {
        let expr = r#"<math display="block"><mn>50&#xA0;</mn> <msup><mi>cm</mi> <mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"50 cm^2"#)?;
        return Ok(());
    }

    /// Voorbeeld 4A: Kubieke meter, met mspace.
    #[test]
    fn voorbeeld_04a() -> Result<()> {
        let expr = r#"<math display="block"><mn>10</mn><mspace width="1em"/><msup><mi>m</mi> <mn>3</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"10 m^3"#)?;
        return Ok(());
    }

    /// Voorbeeld 4B: Kubieke meter, met een no-break space.
    #[test]
    fn voorbeeld_04b() -> Result<()> {
        let expr = r#"<math display="block"><mn>10&#xA0;</mn> <msup><mi>m</mi> <mn>3</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"10 m^3"#)?;
        return Ok(());
    }
}

// Dedicon examples: statistiek
mod statistiek {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Steekproefgemiddelde (X met streepje erboven).
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mrow><mover accent="true"><mi>X</mi><mo stretchy="true">-</mo></mover></mrow></math>"#;
        test_braille("Dedicon", expr, r#"{-X"#)?;
        return Ok(());
    }
}

// Dedicon examples: procenten
mod procenten {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Verschil van twee percentages: spaties rond de min, zoals bij andere getallen.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mrow><mn>100</mn><mo>%</mo><mo>-</mo><mn>7</mn><mo>%</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"100% - 7%"#)?;
        return Ok(());
    }
}

// Dedicon examples: stelsels van vergelijkingen
mod stelsels_vergelijkingen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Stelsel van twee vergelijkingen met een accolade: de vergelijkingen gescheiden door ";".
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mrow><mo>{</mo><mtable><mtr><mtd><mi>x</mi><mo>+</mo><mi>y</mi><mo>=</mo><mn>380</mn></mtd></mtr><mtr><mtd><mn>11,5</mn><mi>x</mi><mo>+</mo><mn>17,25</mn><mi>y</mi><mo>=</mo><mn>5865</mn></mtd></mtr></mtable></mrow></math>"#;
        test_braille("Dedicon", expr, r#"{x + y = 380; 11,5x + 17,25y = 5865"#)?;
        return Ok(());
    }
}
