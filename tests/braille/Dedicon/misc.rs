// Tests for the Dedicon braille code (Dutch math notation in ASCII): more examples of MathML, with the expected
// linear notation. The notation follows the rules and examples on the Dedicon website (see website.rs for the pages
// per topic). Choices that are not covered by those pages are marked with NOTE.
// Rules: Rules/Braille/Dedicon/

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
}

// Dedicon examples: accenten
// A hat over an expression is written as "{^" before it, like the overbar "{-" and the vector arrow "{->"
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
}

// Dedicon examples: getallen met spaties
// A space (no-break space or thin mspace) between groups of digits is a thousands separator and becomes a point,
// if the grouping is valid: a first group of 1 to 3 digits, then only groups of exactly 3 digits.
// Otherwise the space stays, as it does after the decimal comma.
mod getallen_met_spaties {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Duizendtal met een no-break space.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mn>1&#xA0;000</mn></math>"#;
        test_braille("Dedicon", expr, r#"1.000"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Duizendtal met een smalle mspace tussen de cijfergroepen.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mn>1</mn><mspace width="0.167em"/><mn>000</mn></math>"#;
        test_braille("Dedicon", expr, r#"1.000"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Groot getal met meer groepen.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mn>50&#xA0;000&#xA0;000</mn></math>"#;
        test_braille("Dedicon", expr, r#"50.000.000"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Getal met cijfergroepen en een decimale komma.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mn>1&#xA0;000,5</mn></math>"#;
        test_braille("Dedicon", expr, r#"1.000,5"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Ongeldige groep van 2 cijfers.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mn>1&#xA0;00</mn></math>"#;
        test_braille("Dedicon", expr, r#"1 00"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Ongeldige groep in het midden van het getal.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mn>50&#xA0;00&#xA0;000</mn></math>"#;
        test_braille("Dedicon", expr, r#"50 00 000"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Ongeldige eerste groep van 4 cijfers.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><mn>1234&#xA0;567</mn></math>"#;
        test_braille("Dedicon", expr, r#"1234 567"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Spatie na de decimale komma.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mn>0,123&#xA0;456</mn></math>"#;
        test_braille("Dedicon", expr, r#"0,123 456"#)?;
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
