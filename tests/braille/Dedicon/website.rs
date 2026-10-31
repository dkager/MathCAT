// Tests for the Dedicon braille code (Dutch math notation in ASCII)
// Most tests are generated from the examples on the Dedicon website: MathML as input, the linear notation
// as expected output. There is one module per rules page; each module starts with the URL of its page.
// Deviations from the website are marked with NOTE (website errors and choices) or DISCUSSION (open questions).
// Rules: Rules/Braille/Dedicon/

// Dedicon examples: breuken
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-breuken/
mod breuken {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: De dubbele punt als deel- of verhoudingsteken.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mn>54</mn> <mo>:</mo> <mn>9</mn> <mo>=</mo> <mn>6</mn></math>"#;
        test_braille("Dedicon", expr, r#"54 : 9 = 6"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Het verhoudingsteken.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mtext>De verhouding van drie getallen is </mtext><mn>7</mn> <mo>:</mo> <mn>5</mn> <mo>:</mo> <mn>3</mn></math>"#;
        test_braille("Dedicon", expr, r#"De verhouding van drie getallen is 7 : 5 : 3"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Deling met de schuine streep.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mn>54</mn> <mo>/</mo> <mn>9</mn> <mo>=</mo> <mn>6</mn></math>"#;
        test_braille("Dedicon", expr, r#"54/9 = 6"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Deling met de horizontale streep.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mfrac><mn>54</mn> <mn>9</mn> </mfrac><mo>=</mo> <mn>6</mn></math>"#;
        test_braille("Dedicon", expr, r#"54/9 = 6"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Deling met het deelteken.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mn>54</mn> <mo>÷</mo> <mn>9</mn> <mo>=</mo> <mn>6</mn></math>"#;
        test_braille("Dedicon", expr, r#"54/9 = 6"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Een eenvoudige breuk.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mfrac><mn>3</mn> <mn>4</mn></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"3/4"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Verschillende schrijfwijzen voor een half.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><mrow><mfrac><mn>1</mn> <mn>2</mn> </mfrac></mrow><mtext>en </mtext><mrow><mn>1</mn> <mo>/</mo> <mn>2</mn> </mrow><mtext>en </mtext><mrow><mn>½</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"1/2 en 1/2 en 1/2"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Breuken in een langere expressie.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mrow><mn>2</mn> <mo>&#x2064;</mo> <mfrac><mn>1</mn> <mn>2</mn> </mfrac></mrow><mo>+</mo> <mfrac><mn>21</mn> <mn>2</mn> </mfrac><mo>=</mo> <mn>13</mn></math>"#;
        test_braille("Dedicon", expr, r#"2 1/2 + 21/2 = 13"#)?;
        return Ok(());
    }

    /// Voorbeeld 9: Een breuk met haken in teller en noemer.
    #[test]
    fn voorbeeld_09() -> Result<()> {
        let expr = r#"<math><mfrac><mrow><mn>4</mn> <mo>+</mo> <mn>2</mn> </mrow><mrow><mn>4</mn> <mo>−</mo> <mn>2</mn> </mrow></mfrac><mo>=</mo> <mn>3</mn></math>"#;
        test_braille("Dedicon", expr, r#"(4 + 2)/(4 - 2) = 3"#)?;
        return Ok(());
    }

    /// Voorbeeld 10: Een breuk met indexen zonder haken.
    #[test]
    fn voorbeeld_10() -> Result<()> {
        let expr = r#"<math><mfrac><msub><mi>u</mi> <mi>n</mi> </msub><msub><mi>v</mi> <mi>n</mi></msub></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"u_n / v_n"#)?;
        return Ok(());
    }

    /// Voorbeeld 11: Een breuk met een wortel in de teller.
    /// NOTE: the website writes (sqrt(5 + 4))/(6 * 3). The fraction rule only puts parentheses around the part that needs
    /// them (e.g. "(-5 + 3)/11", "21/(3sqrt(2))");
    /// a root has its own parentheses, so the expected output is sqrt(5 + 4)/(6 * 3).
    #[test]
    fn voorbeeld_11() -> Result<()> {
        let expr = r#"<math><mfrac><msqrt><mrow><mn>5</mn> <mo>+</mo> <mn>4</mn> </mrow></msqrt><mrow><mn>6</mn> <mo>⋅</mo> <mn>3</mn></mrow></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"sqrt(5 + 4)/(6 * 3)"#)?;
        return Ok(());
    }

    /// Voorbeeld 12: Een breuk met een macht, zonder of met haken.
    /// NOTE: the website lists two alternatives ("a^p / q of (a^p)/(q)") as one linear notation;
    /// the expected output is the first alternative.
    #[test]
    fn voorbeeld_12() -> Result<()> {
        let expr = r#"<math><mfrac><msup><mi>a</mi> <mi>p</mi> </msup><mi>q</mi></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"a^p / q"#)?;
        return Ok(());
    }
}

// Dedicon examples: complexe getallen
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-complexe-getallen/
mod complexe_getallen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: De geconjugeerde van het imaginaire getal 2i.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mrow><mn>2</mn> <mo>&#x2062;</mo> <mi>i</mi> </mrow><mo stretchy="true">–</mo> </mover><mo>=</mo> <mrow><mo>−</mo> <mn>2</mn> <mo>&#x2062;</mo> <mi>i</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"{-2i = -2i"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: De geconjugeerde van -2i.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mrow><mo>−</mo> <mn>2</mn> <mo>&#x2062;</mo> <mi>i</mi> </mrow><mo stretchy="true">–</mo> </mover><mo>=</mo> <mrow><mn>2</mn> <mo>&#x2062;</mo> <mi>i</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"{--2i = 2i"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: De geconjugeerde van een complex getal.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mrow><mo>−</mo> <mn>5</mn> <mo>−</mo> <mn>4</mn> <mo>&#x2062;</mo> <mi>i</mi> </mrow><mo stretchy="true">–</mo> </mover><mo>=</mo> <mrow><mo>−</mo> <mn>5</mn> <mo>+</mo> <mn>4</mn> <mo>&#x2062;</mo> <mi>i</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"{-(-5 - 4i) = -5 + 4i"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: De geconjugeerde van een som is de som van de geconjugeerden.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mover accent="true"><mrow><mi>α</mi> <mo>+</mo> <mi>β</mi> </mrow><mo stretchy="true">–</mo> </mover><mo>=</mo> <mrow><mover accent="true"><mi>α</mi> <mo stretchy="true">–</mo> </mover><mo>+</mo> <mover accent="true"><mi>β</mi> <mo stretchy="true">–</mo></mover></mrow></math>"#;
        test_braille("Dedicon", expr, r#"{-(~a + ~b) = {-~a + {-~b"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: De geconjugeerde van het complexe getal z is a + bi.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mover><mi>z</mi> <mo stretchy="true">–</mo> </mover><mo>=</mo> <mover><mrow><mi>a</mi> <mo>+</mo> <mi>b</mi> <mo>&#x2062;</mo> <mi>i</mi> </mrow><mo stretchy="true">–</mo></mover></math>"#;
        test_braille("Dedicon", expr, r#"{-z = {-(a + bi)"#)?;
        return Ok(());
    }
}

// Dedicon examples: differentieren en integreren
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-differentieren-en-integreren/
mod differentieren_en_integreren {
    use crate::common::*;
    use anyhow::Result;

    /// differentiëren, Voorbeeld 1: Eerste afgeleide.
    #[test]
    fn differentieren_voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mfrac><mi>d</mi> <mi>dq</mi> </mfrac><mfenced><mfrac><mi>dK</mi> <mi>dq</mi> </mfrac></mfenced><mo>=</mo> <mn>0</mn></math>"#;
        test_braille("Dedicon", expr, r#"d/dq (dK/dq) = 0"#)?;
        return Ok(());
    }

    /// differentiëren, Voorbeeld 2: Afgeleide met onderindexen.
    #[test]
    fn differentieren_voorbeeld_02() -> Result<()> {
        let expr = r#"<math><msub><mfenced open="[" close="]"><mfrac><mi>dy</mi> <mi>dx</mi> </mfrac></mfenced><mfenced separators=""><mi>x</mi> <mo>=</mo> <msub><mi>x</mi> <mi>A</mi> </msub></mfenced></msub><mo>=</mo> <mrow><msup><mi>f</mi> <mo>′</mo> </msup><mo>&#x2061;</mo> <mfenced><msub><mi>x</mi> <mi>A</mi></msub></mfenced></mrow></math>"#;
        test_braille("Dedicon", expr, r#"[dy/dx]_(x = x_A) = f'(x_A)"#)?;
        return Ok(());
    }

    /// differentiëren, Voorbeeld 3: Tweede afgeleide, aangegeven met bovenindexen.
    /// NOTE: the website writes (d^2 f)/(dx^2). The fraction rule only puts parentheses around the part that needs them
    /// (e.g. "(-5 + 3)/11", "21/(3sqrt(2))");
    /// a superscript is simple, so the expected output is (d^2 f)/dx^2.
    #[test]
    fn differentieren_voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mfrac><mrow><msup><mi>d</mi> <mn>2</mn> </msup><mi>f</mi> </mrow><msup><mi>dx</mi> <mn>2</mn></msup></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"(d^2 f)/dx^2"#)?;
        return Ok(());
    }

    /// differentiëren, Voorbeeld 4: Een afgeleide.
    #[test]
    fn differentieren_voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mfrac><mi>dy</mi> <mi>dx</mi> </mfrac><mo>=</mo> <mfrac><mi>dy</mi> <mi>du</mi> </mfrac><mo>⋅</mo> <mfrac><mi>du</mi> <mi>dx</mi></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"dy/dx = dy/du * du/dx"#)?;
        return Ok(());
    }

    /// integreren, Voorbeeld 1: De integraal van a naar b van f(x) en zijn primitieve.
    #[test]
    fn integreren_voorbeeld_01() -> Result<()> {
        let expr = r#"<math><munderover><mo>∫</mo> <mi>a</mi> <mi>b</mi> </munderover><mrow><mi>f</mi> <mo>&#x2061;</mo> <mfenced><mi>x</mi> </mfenced></mrow><mi>dx</mi> <mo>=</mo> <msubsup><mfenced open="[" close="]" separators=""><mi>F</mi> <mo>&#x2061;</mo> <mfenced><mi>x</mi> </mfenced></mfenced><mi>a</mi> <mi>b</mi></msubsup></math>"#;
        test_braille("Dedicon", expr, r#"Intg{a..b}f(x)dx = [F(x)]{a..b}"#)?;
        return Ok(());
    }

    /// integreren, Voorbeeld 2: Integraal met een breuk en pi.
    /// DISCUSSION: the Dedicon page writes "sinx" here without a space, but "log x" with a space in the
    /// logarithm examples. For now we chose "sin x" (a space after every function name), so the expected
    /// output differs from the website. See the named-functions rule in Rules/Braille/Dedicon/Dedicon_Rules.yaml.
    /// NOTE: pi is written "~p" as in the symbol table for Greek letters (the example shows "pi").
    #[test]
    fn integreren_voorbeeld_02() -> Result<()> {
        let expr = r#"<math><munderover><mo>∫</mo> <mrow><mfrac><mn>1</mn> <mn>2</mn> </mfrac><mo>&#x2062;</mo> <mi>π</mi> </mrow><mi>π</mi> </munderover><mrow><mo>sin</mo> <mo>&#x2061;</mo> <mi>x</mi> </mrow><mi>dx</mi></math>"#;
        test_braille("Dedicon", expr, r#"Intg{1/2 ~p..~p}sin xdx"#)?;
        return Ok(());
    }

    /// integreren, Voorbeeld 3: Integraal met oneindigheidsteken.
    #[test]
    fn integreren_voorbeeld_03() -> Result<()> {
        let expr = r#"<math><munderover><mo>∫</mo> <mrow><mo>−</mo> <mi>∞</mi> </mrow><mi>∞</mi> </munderover><mrow><mi>f</mi> <mo>&#x2061;</mo> <mfenced><mi>x</mi> </mfenced></mrow><mi>dx</mi></math>"#;
        test_braille("Dedicon", expr, r#"Intg{-inf..inf}f(x)dx"#)?;
        return Ok(());
    }

    /// integreren, Voorbeeld 4: Een integraal met onderindexen in de onder- en bovengrens.
    #[test]
    fn integreren_voorbeeld_04() -> Result<()> {
        let expr = r#"<math><munderover><mo>∫</mo> <msub><mi>x</mi> <mn>1</mn> </msub><msub><mi>x</mi> <mn>2</mn> </msub></munderover><mrow><mi>g</mi> <mo>&#x2061;</mo> <mfenced><mi>x</mi> </mfenced></mrow><mi>dx</mi></math>"#;
        test_braille("Dedicon", expr, r#"Intg{x_1..x_2}g(x)dx"#)?;
        return Ok(());
    }

    /// integreren, Voorbeeld 5: Integraal met een macht en zijn primitieve met een breuk.
    #[test]
    fn integreren_voorbeeld_05() -> Result<()> {
        let expr = r#"<math><munderover><mo>∫</mo> <mn>1</mn> <mn>4</mn> </munderover><msup><mi>x</mi> <mn>2</mn> </msup><mi>dx</mi> <mo>=</mo> <msubsup><mfenced open="[" close="]" separators=""><mfrac><mn>1</mn> <mn>3</mn> </mfrac><mo>&#x2062;</mo> <msup><mi>x</mi> <mn>3</mn> </msup></mfenced><mn>1</mn> <mn>4</mn></msubsup></math>"#;
        test_braille("Dedicon", expr, r#"Intg{1..4}x^2 dx = [1/3 x^3]{1..4}"#)?;
        return Ok(());
    }

    /// integreren, Voorbeeld 6: Een integraal over een volume.
    #[test]
    fn integreren_voorbeeld_06() -> Result<()> {
        let expr = r#"<math><munder><mo>∫</mo> <mi>V</mi> </munder><mrow><mi>f</mi> <mo>&#x2061;</mo> <mfenced><mi>x</mi> <mi>y</mi> <mi>z</mi> </mfenced></mrow><mi>dV</mi></math>"#;
        test_braille("Dedicon", expr, r#"Intg{V}f(x, y, z)dV"#)?;
        return Ok(());
    }
}

// Dedicon examples: limieten
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-limieten/
mod limieten {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Limiet van n naar oneindig.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><munder><mo>lim</mo> <mrow><mi>n</mi> <mo>→</mo> <mi>∞</mi> </mrow></munder><msub><mi>u</mi> <mi>n</mi></msub></math>"#;
        test_braille("Dedicon", expr, r#"lim{n --> inf}u_n"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Linker- of onderlimiet met een breuk.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><munder><mo>lim</mo> <mrow><mi>x</mi> <mo>↑</mo> <mn>0</mn> </mrow></munder><mo>&#x2061;</mo> <mfrac><mn>1</mn> <mi>x</mi> </mfrac><mo>=</mo> <mrow><mo>−</mo> <mi>∞</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"lim{x vanOnder--> 0}1/x = -inf"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Rechter- of bovenlimiet met een breuk.
    /// NOTE: the arrow is written "vanBoven-->" as in this example; the rules text and symbol table on the website
    /// write "vanBoven<--".
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><munder><mo>lim</mo> <mrow><mi>x</mi> <mo>↓</mo> <mn>0</mn> </mrow></munder><mo>&#x2061;</mo> <mfrac><mn>1</mn> <mi>x</mi> </mfrac><mo>=</mo> <mi>∞</mi></math>"#;
        test_braille("Dedicon", expr, r#"lim{x vanBoven--> 0}1/x = inf"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Limiet van een samengestelde breuk.
    /// NOTE: the website writes (1)/(1,01^n). The fraction rule writes a fraction with a sub- or superscript
    /// with spaces around "/" instead (as in breuken 10 and 12: u_n / v_n, a^p / q); the fractions page allows both.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><munder><mo>lim</mo> <mrow><mi>n</mi> <mo>→</mo> <mi>∞</mi> </mrow></munder><mo>&#x2061;</mo> <mfrac><mn>1</mn> <msup><mn>1,01</mn> <mi>n</mi> </msup></mfrac><mo>=</mo> <mn>0</mn></math>"#;
        test_braille("Dedicon", expr, r#"lim{n --> inf}1 / 1,01^n = 0"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Een breuk met limiet in de teller.
    /// NOTE: the website writes the limit with spaces around "/" (lim{...}1 / x) and mentions brackets as the alternative.
    /// The arrow is written "vanBoven-->" as in the examples; the rules text and symbol table on the website
    /// write "vanBoven<--".
    /// Chosen: keep the fraction rule, which only puts parentheses around the part that needs them
    /// (e.g. "(-5 + 3)/11", "21/(3sqrt(2))"),
    /// so the expected output is (lim{x vanBoven--> 0}1)/x, not (lim{x vanBoven--> 0}1)/(x).
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mfrac><mrow><munder><mo>lim</mo> <mrow><mi>x</mi> <mo>↓</mo> <mn>0</mn> </mrow></munder><mn>1</mn> </mrow><mi>x</mi></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"(lim{x vanBoven--> 0}1)/x"#)?;
        return Ok(());
    }
}

// Dedicon examples: logaritmen
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-logaritmen/
mod logaritmen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Europees en Amerikaans logaritme.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><msub><mi>log</mi> <mn>10</mn> </msub><mn>100</mn> <mo>=</mo> <mn>2</mn> <mtext>en </mtext><mmultiscripts><mi>log</mi> <mprescripts></mprescripts><none></none><mn>10</mn> </mmultiscripts><mn>100</mn> <mo>=</mo> <mn>2</mn></math>"#;
        test_braille("Dedicon", expr, r#"log_10 100 = 2 en ^10 log 100 = 2"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Logaritme zonder haken.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mmultiscripts><mi>log</mi> <mprescripts></mprescripts><none></none><mn>10</mn> </mmultiscripts><mn>100</mn> <mo>=</mo> <mn>2</mn></math>"#;
        test_braille("Dedicon", expr, r#"^10 log 100 = 2"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Logaritme met haken.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mmultiscripts><mi>log</mi> <mprescripts></mprescripts><none></none><mn>2</mn> </mmultiscripts><mrow><mo>(</mo> <mn>8</mn> <mo>)</mo> </mrow><mo>=</mo> <mn>3</mn></math>"#;
        test_braille("Dedicon", expr, r#"^2 log(8) = 3"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Logaritme met letters, zonder haken.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mmultiscripts><mi>log</mi> <mprescripts></mprescripts><none></none><mi>g</mi> </mmultiscripts><mi>a</mi></math>"#;
        test_braille("Dedicon", expr, r#"^g log a"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Logaritme met letters en haken.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mmultiscripts><mi>log</mi> <mprescripts></mprescripts><none></none><mi>g</mi> </mmultiscripts><mrow><mo>(</mo> <mi>a</mi> <mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"^g log(a)"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Logaritme van een breuk.
    /// NOTE: the formula on the website has no brackets around a/b, its linear notation adds them: ^g log(a/b).
    /// The expected output follows the formula.
    /// DISCUSSION: "^g log a/b" is ambiguous, it can also be read as (^g log a)/b. Should a fraction as argument of
    /// a function name always get brackets? See the named-functions rule in Rules/Braille/Dedicon/Dedicon_Rules.yaml.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mmultiscripts><mi>log</mi> <mprescripts></mprescripts><none></none><mi>g</mi> </mmultiscripts><mfrac><mi>a</mi> <mi>b</mi> </mfrac><mo>=</mo> <mmultiscripts><mi>log</mi> <mprescripts></mprescripts><none></none><mi>g</mi> </mmultiscripts><mi>a</mi> <mo>–</mo> <mmultiscripts><mi>log</mi> <mprescripts></mprescripts><none></none><mi>g</mi> </mmultiscripts><mi>b</mi></math>"#;
        test_braille("Dedicon", expr, r#"^g log a/b = ^g log a - ^g log b"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Logaritme zonder grondtal als exponent van een macht.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><msup><mn>10</mn> <mrow><mi>log</mi> <mi>x</mi> </mrow></msup><mo>=</mo> <mi>x</mi> <mtext>of </mtext><mi>log</mi> <mrow><mo>(</mo> <mi>x</mi> <mo>)</mo> </mrow><mo>=</mo> <mi>x</mi></math>"#;
        test_braille("Dedicon", expr, r#"10^(log x) = x of log(x) = x"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Logaritme met grondtal als exponent van een macht.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><msup><mn>10</mn> <mrow><mmultiscripts><mi>log</mi> <mprescripts></mprescripts><none></none><mn>10</mn> </mmultiscripts><mi>x</mi></mrow></msup><mo>=</mo> <mi>x</mi></math>"#;
        test_braille("Dedicon", expr, r#"10^(^10 log x) = x"#)?;
        return Ok(());
    }
}

// Dedicon examples: logica
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-logica/
// NOTE: the not sign is expected as "nt" directly followed by the operand ("ntP", "nt(P of Q)").
// This still needs discussion: the Dedicon examples show "¬P", the symbol table says "nt",
// and it is not yet decided whether a space should follow ("nt P").
mod logica {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Distributiviteit.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi>P</mi> <mo>∨</mo> <mrow><mo>(</mo> <mi>Q</mi><mo>∧</mo><mi>R</mi> <mo>)</mo></mrow> <mo>=</mo> <mrow><mo>(</mo> <mi>P</mi><mo>∨</mo><mi>Q</mi> <mo>)</mo></mrow> <mo>∧</mo> <mrow><mo>(</mo> <mi>P</mi><mo>∨</mo><mi>R</mi> <mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"P of (Q en R) = (P of Q) en (P of R)"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Eén van de regels van De Morgan.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mrow><mo>¬</mo><mi>P</mi></mrow> <mo>∧</mo> <mrow><mo>¬</mo><mi>Q</mi></mrow> <mo>=</mo> <mo>¬</mo> <mrow><mo>(</mo> <mi>P</mi><mo>∨</mo><mi>Q</mi> <mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"ntP en ntQ = nt(P of Q)"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Implicatie.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mi>P</mi><mo>→</mo><mi>Q</mi> <mo>=</mo> <mrow><mo>¬</mo><mi>P</mi></mrow> <mo>∨</mo> <mi>Q</mi></math>"#;
        test_braille("Dedicon", expr, r#"P --> Q = ntP of Q"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Equivalentie.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mi>P</mi><mo>↔</mo><mi>Q</mi> <mo>=</mo> <mrow><mo>(</mo> <mrow><mo>¬</mo><mi>P</mi></mrow> <mo>∧</mo> <mrow><mo>¬</mo><mi>Q</mi></mrow> <mo>)</mo></mrow> <mo>∨</mo> <mrow><mo>(</mo> <mi>P</mi><mo>∧</mo><mi>Q</mi> <mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"P ,:, Q = (ntP en ntQ) of (P en Q)"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Existentiële en universele kwantor.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mo>∃</mo><mi>x</mi> <mrow><mo>(</mo> <mrow><mi>A</mi><mi>x</mi></mrow> <mo>∨</mo> <mo>∀</mo><mi>y</mi> <mrow><mi>B</mi><mi>x</mi><mi>y</mi></mrow> <mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"er is een x (Ax of voor alle y Bxy)"#)?;
        return Ok(());
    }
}

// Dedicon examples: machtsverheffen
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-machtsverheffen/
mod machtsverheffen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Een eenvoudige macht.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><msup><mn>5</mn> <mn>2</mn> </msup><mo>=</mo> <mrow><mn>5</mn> <mo>×</mo> <mn>5</mn></mrow></math>"#;
        test_braille("Dedicon", expr, r#"5^2 = 5 * 5"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Vierkante centimeter.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mn>50</mn> <msup><mi>cm</mi> <mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"50cm^2"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Kubieke meter.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mn>10</mn> <msup><mi>m</mi> <mn>3</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"10m^3"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Een eenvoudige exponent beëindigen met een spatie.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><msup><mn>5</mn> <mn>2</mn> </msup><mo>+</mo> <mn>2</mn> <mo>=</mo> <mn>27</mn></math>"#;
        test_braille("Dedicon", expr, r#"5^2 + 2 = 27"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Een samengestelde exponent.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><msup><mn>5</mn> <mrow><mn>2</mn> <mo>+</mo> <mn>2</mn> </mrow></msup><mo>=</mo> <msup><mn>5</mn> <mn>4</mn> </msup><mo>=</mo> <mn>625</mn></math>"#;
        test_braille("Dedicon", expr, r#"5^(2 + 2) = 5^4 = 625"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Een macht met eenvoudige, negatieve exponent.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><msup><mi>x</mi> <mrow><mo>–</mo> <mn>1</mn></mrow></msup></math>"#;
        test_braille("Dedicon", expr, r#"x^-1"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Een breuk als exponent en een wortel.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><msup><mn>9</mn> <mfrac><mn>1</mn> <mn>2</mn> </mfrac></msup><mo>=</mo> <msqrt><mn>9</mn> </msqrt><mo>=</mo> <mn>3</mn></math>"#;
        test_braille("Dedicon", expr, r#"9^(1/2) = sqrt(9) = 3"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Een samengestelde exponent met een breuk.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mi>T</mi> <mo>=</mo> <msup><mn>2</mn> <mrow><mfrac><mn>1</mn> <mn>2</mn> </mfrac><mo>&#x2062;</mo> <mi>M</mi> <mo>+</mo> <mn>1</mn> </mrow></msup><mo>−</mo> <mn>2</mn></math>"#;
        test_braille("Dedicon", expr, r#"T = 2^(1/2 M + 1) - 2"#)?;
        return Ok(());
    }

    /// Voorbeeld 9: Een exponent met een breuk.
    #[test]
    fn voorbeeld_09() -> Result<()> {
        let expr = r#"<math><msup><mi>a</mi> <mfrac><mi>p</mi> <mi>q</mi></mfrac></msup></math>"#;
        test_braille("Dedicon", expr, r#"a^(p/q)"#)?;
        return Ok(());
    }

    /// Voorbeeld 10: Een breuk met in de teller a tot de macht p.
    #[test]
    fn voorbeeld_10() -> Result<()> {
        let expr = r#"<math><mfrac><msup><mi>a</mi> <mi>p</mi> </msup><mi>q</mi></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"a^p / q"#)?;
        return Ok(());
    }

    /// Voorbeeld 11: Een macht tot de macht.
    #[test]
    fn voorbeeld_11() -> Result<()> {
        let expr = r#"<math><msup><mn>5</mn><mrow><mn>2</mn><mo>&#x2062;</mo><mi>x</mi><mo>+</mo><mn>2</mn></mrow></msup><mo>=</mo><msup><msup><mn>5</mn><mn>2</mn></msup><mi>x</mi></msup><mo>⋅</mo><mn>25</mn></math>"#;
        test_braille("Dedicon", expr, r#"5^(2x + 2) = (5^2)^x * 25"#)?;
        return Ok(());
    }
}

// Dedicon examples: matrices
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-matrices/
mod matrices {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Een matrix van drie-bij-drie met vierkante haken.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi>A</mi> <mo>=</mo> <mfenced open="[" close="]" separators=""><mtable><mtr><mtd><mi>a</mi> </mtd><mtd><mi>b</mi> </mtd><mtd><mi>c</mi> </mtd></mtr><mtr><mtd><mi>d</mi> </mtd><mtd><mi>e</mi> </mtd><mtd><mi>f</mi> </mtd></mtr><mtr><mtd><mi>g</mi> </mtd><mtd><mi>h</mi> </mtd><mtd><mi>i</mi></mtd></mtr></mtable></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"A = [a:b:c; d:e:f; g:h:i]"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Een matrix van drie-bij-drie met ronde haken.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mi>A</mi> <mo>=</mo> <mfenced separators=""><mtable><mtr><mtd><mi>a</mi> </mtd><mtd><mi>b</mi> </mtd><mtd><mi>c</mi> </mtd></mtr><mtr><mtd><mi>d</mi> </mtd><mtd><mi>e</mi> </mtd><mtd><mi>f</mi> </mtd></mtr><mtr><mtd><mi>g</mi> </mtd><mtd><mi>h</mi> </mtd><mtd><mi>i</mi></mtd></mtr></mtable></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"A = (a:b:c; d:e:f; g:h:i)"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Een determinant met verticale strepen.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mrow><mi>det</mi> <mo>&#x2061;</mo> <mfenced><mi>A</mi> </mfenced></mrow><mo>=</mo> <mfenced open="|" close="|" separators=""><mtable><mtr><mtd><mi>a</mi> </mtd><mtd><mi>b</mi> </mtd><mtd><mi>c</mi> </mtd></mtr><mtr><mtd><mi>d</mi> </mtd><mtd><mi>e</mi> </mtd><mtd><mi>f</mi> </mtd></mtr><mtr><mtd><mi>g</mi> </mtd><mtd><mi>h</mi> </mtd><mtd><mi>i</mi></mtd></mtr></mtable></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"det(A) = |a:b:c; d:e:f; g:h:i|"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Een matrix met vier rijen en twee kolommen.
    /// NOTE: the formula on the website has square brackets, its linear notation round brackets. The rules page
    /// says the brackets of the formula are taken over, so the expected output follows the formula.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mfenced open="[" close="]" separators=""><mtable><mtr><mtd><mn>1,1</mn> </mtd><mtd><mn>2,2</mn> </mtd></mtr><mtr><mtd><mn>3,3</mn> </mtd><mtd><mn>4,4</mn> </mtd></mtr><mtr><mtd><mn>5,5</mn> </mtd><mtd><mn>6,6</mn> </mtd></mtr><mtr><mtd><mn>7,7</mn> </mtd><mtd><mn>8,8</mn></mtd></mtr></mtable></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"[1,1:2,2; 3,3:4,4; 5,5:6,6; 7,7:8,8]"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Een matrix met breuken.
    /// NOTE: the formula on the website has square brackets, its linear notation round brackets. The rules page
    /// says the brackets of the formula are taken over, so the expected output follows the formula.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mfenced open="[" close="]" separators=""><mtable><mtr><mtd><mn>1</mn> <mfrac><mn>1</mn> <mn>2</mn> </mfrac></mtd><mtd><mn>1</mn> </mtd></mtr><mtr><mtd><mn>2</mn> <mfrac><mn>1</mn> <mn>4</mn> </mfrac></mtd><mtd><mn>2</mn></mtd></mtr></mtable></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"[1 1/2:1; 2 1/4:2]"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Een matrix met niet-ingevulde plaatsen en onderindexen.
    /// NOTE: the formula on the website has square brackets, its linear notation round brackets. The rules page
    /// says the brackets of the formula are taken over, so the expected output follows the formula.
    /// The ellipses are written as "..." (ASCII) instead of the "…" on the website.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mfenced open="[" close="]" separators=""><mtable><mtr><mtd><msub><mi>a</mi> <mrow><mn>1</mn> <mo>&#x2063;</mo> <mn>1</mn> </mrow></msub></mtd><mtd><mi>⋯</mi> </mtd><mtd><msub><mi>a</mi> <mrow><mn>1</mn> <mo>&#x2063;</mo> <mi>n</mi> </mrow></msub></mtd></mtr><mtr><mtd><mi>⋮</mi> </mtd><mtd><mi>⋱</mi> </mtd><mtd><mi>⋮</mi> </mtd></mtr><mtr><mtd><msub><mi>a</mi> <mrow><mi>m</mi> <mo>&#x2063;</mo> <mn>1</mn> </mrow></msub></mtd><mtd><mi>⋯</mi> </mtd><mtd><msub><mi>a</mi> <mrow><mi>m</mi> <mo>&#x2063;</mo> <mi>n</mi></mrow></msub></mtd></mtr></mtable></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"[a_11:...:a_1n; ...:...:...; a_m1:...:a_mn]"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Een matrixvermenigvuldiging.
    /// NOTE: the formula on the website has square brackets, its linear notation round brackets. The rules page
    /// says the brackets of the formula are taken over, so the expected output follows the formula.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mfenced open="[" close="]" separators=""><mtable><mtr><mtd><mi>a</mi> </mtd><mtd><mi>b</mi> </mtd></mtr><mtr><mtd><mi>c</mi> </mtd><mtd><mi>d</mi> </mtd></mtr></mtable></mfenced><mo>⋅</mo> <mfenced open="[" close="]" separators=""><mtable><mtr><mtd><mi>n</mi> </mtd></mtr><mtr><mtd><mi>k</mi></mtd></mtr></mtable></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"[a:b; c:d] * [n; k]"#)?;
        return Ok(());
    }

    // Own tests (not from the Dedicon website)

    /// An empty cell in a matrix is written as "gg".
    #[test]
    fn empty_cell() -> Result<()> {
        let expr = r#"<math><mrow><mo>[</mo><mtable>
            <mtr><mtd><mi>a</mi></mtd><mtd></mtd></mtr>
            <mtr><mtd><mi>c</mi></mtd><mtd><mi>d</mi></mtd></mtr>
        </mtable><mo>]</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"[a:gg; c:d]"#)?;
        return Ok(());
    }

    /// A cell that contains only white space (here a non-breaking space) also counts as empty and is written as "gg".
    #[test]
    fn empty_cell_white_space() -> Result<()> {
        let expr = r#"<math><mrow><mo>(</mo><mtable>
            <mtr><mtd><mtext>&#xA0;</mtext></mtd><mtd><mn>1</mn></mtd></mtr>
            <mtr><mtd><mn>2</mn></mtd><mtd><mn>3</mn></mtd></mtr>
        </mtable><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"(gg:1; 2:3)"#)?;
        return Ok(());
    }
}

// Dedicon examples: onderindex en bovenindex
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-onderindex-en-bovenindex/
mod onderindex_en_bovenindex {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Eenvoudige onderindex.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><msub><mi>x</mi> <mn>1</mn> </msub><mo>=</mo> <mn>7</mn></math>"#;
        test_braille("Dedicon", expr, r#"x_1 = 7"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Spatie sluit de onderindex af.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><msub><mi>u</mi> <mn>1</mn> </msub><mo>+</mo> <msub><mi>u</mi> <mn>2</mn></msub></math>"#;
        test_braille("Dedicon", expr, r#"u_1 + u_2"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Een onderindex met haken.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><msub><mi>x</mi> <mrow><mi>n</mi> <mo>+</mo> <mn>1</mn> </mrow></msub><mo>=</mo> <mn>5</mn></math>"#;
        test_braille("Dedicon", expr, r#"x_(n + 1) = 5"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Een vermenigvuldiging in een onderindex zonder haken.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><msub><mi>a</mi> <mrow><mi>m</mi> <mo>&#x2062;</mo> <mi>n</mi> </mrow></msub><mo>⋅</mo> <msub><mi>b</mi> <mrow><mi>m</mi> <mo>&#x2062;</mo> <mi>n</mi></mrow></msub></math>"#;
        test_braille("Dedicon", expr, r#"a_mn * b_mn"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Onderindex als exponent.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><msup><mi>e</mi> <msub><mi>x</mi> <mn>1</mn> </msub></msup><mo>+</mo> <msup><mi>e</mi> <msub><mi>x</mi> <mn>2</mn></msub></msup></math>"#;
        test_braille("Dedicon", expr, r#"e^(x_1) + e^(x_2)"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Sub- en superscript bij één letter.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><msubsup><mi>A</mi> <mn>1</mn> <mn>2</mn></msubsup></math>"#;
        test_braille("Dedicon", expr, r#"A_1 ^2"#)?;
        return Ok(());
    }
}

// Dedicon examples: som en product
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-som-en-product/
mod som_en_product {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: De som van de termen u_k, voor k is nul tot en met k is n.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><munderover><mo>∑</mo> <mrow><mi>k</mi> <mo>=</mo> <mn>0</mn> </mrow><mi>n</mi> </munderover><mo>&#x2061;</mo> <msub><mi>u</mi> <mi>k</mi></msub></math>"#;
        test_braille("Dedicon", expr, r#"~S{k = 0..n}u_k"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Uitgeschreven som van k is twee tot en met k is vier.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><munderover><mo>∑</mo> <mrow><mi>k</mi> <mo>=</mo> <mn>2</mn> </mrow><mi>4</mi> </munderover><mo>&#x2061;</mo> <msub><mi>b</mi> <mi>k</mi> </msub><mo>=</mo> <mrow><msub><mi>b</mi> <mn>2</mn> </msub><mo>+</mo> <msub><mi>b</mi> <mn>3</mn> </msub><mo>+</mo> <msub><mi>b</mi> <mn>4</mn></msub></mrow></math>"#;
        test_braille("Dedicon", expr, r#"~S{k = 2..4}b_k = b_2 + b_3 + b_4"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Sommatie met alleen een ondergrens.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><munder><mo>∑</mo> <mi>k</mi> </munder><mo>&#x2061;</mo> <msub><mi>u</mi> <mi>k</mi></msub></math>"#;
        test_braille("Dedicon", expr, r#"~S{k}u_k"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Het product van factoren x_i, voor i is m tot en met i is n.
    /// NOTE: the product sign is written "~P" as in the rules text and symbol table (the example shows "Pi").
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><munderover><mo>∏</mo> <mrow><mi>i</mi> <mo>=</mo> <mi>m</mi> </mrow><mi>n</mi> </munderover><mo>&#x2061;</mo> <msub><mi>x</mi> <mi>i</mi></msub></math>"#;
        test_braille("Dedicon", expr, r#"~P{i = m..n}x_i"#)?;
        return Ok(());
    }
}

// Dedicon examples: vectoren
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-vectoren/
// NOTE: the website writes the vector arrow as "{->"; the correct notation is "{-->", like the arrow "-->".
mod vectoren {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Twee vectoren.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mover><mi>a</mi> <mo>→</mo> </mover><mo>=</mo> <mover><mi>OA</mi> <mo>→</mo></mover></math>"#;
        test_braille("Dedicon", expr, r#"{-->a = {-->OA"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: De som van twee vectoren.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mover><mi>t</mi> <mo>→</mo> </mover><mo>=</mo> <mrow><mover><mi>v</mi> <mo>→</mo> </mover><mo>+</mo> <mover><mi>w</mi> <mo>→</mo></mover></mrow></math>"#;
        test_braille("Dedicon", expr, r#"{-->t = {-->v + {-->w"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Samengestelde vector met onderindexen.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mover><mrow><msub><mi>F</mi> <mn>1</mn> </msub><mo>+</mo> <msub><mi>F</mi> <mn>2</mn> </msub></mrow><mo>→</mo></mover></math>"#;
        test_braille("Dedicon", expr, r#"{-->(F_1 + F_2)"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: De norm van een vector.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mfenced open="||" close="||" separators=""><mover><mi>v</mi> <mo>→</mo></mover></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"||{-->v||"#)?;
        return Ok(());
    }
}

// Dedicon examples: vermenigvuldigen
// Source: https://www.dedicon.nl/artikel/regels-en-voorbeelden-vermenigvuldigen/
mod vermenigvuldigen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Vermenigvuldiging met het maalteken.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mn>7</mn> <mo>×</mo> <mn>9</mn> <mo>=</mo> <mn>63</mn></math>"#;
        test_braille("Dedicon", expr, r#"7 * 9 = 63"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Vermenigvuldiging met de vermenigvuldigingspunt.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mn>7</mn> <mo>⋅</mo> <mn>9</mn> <mo>=</mo> <mn>63</mn></math>"#;
        test_braille("Dedicon", expr, r#"7 * 9 = 63"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Vermenigvuldiging met de asterisk.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mn>7</mn> <mo>∗</mo> <mn>9</mn> <mo>=</mo> <mn>63</mn></math>"#;
        test_braille("Dedicon", expr, r#"7 * 9 = 63"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Een expressie zonder vermenigvuldigingsteken.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mfenced separators=""><mn>5</mn> <mo>+</mo> <mn>3</mn> </mfenced><mo>&#x2062;</mo> <mfenced separators=""><mn>4</mn> <mo>+</mo> <mn>6</mn> </mfenced><mo>=</mo> <mn>80</mn></math>"#;
        test_braille("Dedicon", expr, r#"(5 + 3)(4 + 6) = 80"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Cartesisch product van twee verzamelingen.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mi>V</mi> <mo>×</mo> <mi>W</mi></math>"#;
        test_braille("Dedicon", expr, r#"V * W"#)?;
        return Ok(());
    }
}

// Dedicon examples: verzamelingen
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-verzamelingen/
mod verzamelingen {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Distributiviteit.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><mi>A</mi><mo>∪</mo> <mrow><mo>(</mo> <mi>B</mi><mo>∩</mo><mi>C</mi> <mo>)</mo></mrow> <mo>=</mo> <mrow><mo>(</mo> <mi>A</mi><mo>∪</mo><mi>B</mi> <mo>)</mo></mrow> <mo>∩</mo> <mrow><mo>(</mo> <mi>A</mi><mo>∪</mo><mi>C</mi> <mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"A verenigd (B doorsnede C) = (A verenigd B) doorsnede (A verenigd C)"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Eén van de regels van De Morgan.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><msup><mi>A</mi><mi>c</mi> </msup><mo>∩</mo> <msup><mi>B</mi><mi>c</mi> </msup><mo>=</mo> <msup><mrow><mo>(</mo> <mi>A</mi><mo>∪</mo><mi>B</mi> <mo>)</mo></mrow> <mi>c</mi></msup></math>"#;
        test_braille("Dedicon", expr, r#"A^c doorsnede B^c = (A verenigd B)^c"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Doorsnede.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><mfenced open="{" close="}"><mn>1</mn><mn>2</mn> </mfenced><mo>∩</mo> <mfenced open="{" close="}"><mn>2</mn><mn>3</mn> </mfenced><mo>=</mo> <mfenced open="{" close="}"><mn>2</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"{1, 2} doorsnede {2, 3} = {2}"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Vereniging.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><mfenced open="{" close="}"><mn>1</mn><mn>2</mn> </mfenced><mo>∪</mo> <mfenced open="{" close="}"><mn>2</mn><mn>3</mn> </mfenced><mo>=</mo> <mfenced open="{" close="}"><mn>1</mn><mn>2</mn><mn>3</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"{1, 2} verenigd {2, 3} = {1, 2, 3}"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: Lege verzameling.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><mfenced open="{" close="}"><mn>1</mn><mn>2</mn><mn>3</mn> </mfenced><mo>∩</mo> <mfenced open="{" close="}"><mn>4</mn> </mfenced><mo>=</mo> <mi>∅</mi></math>"#;
        test_braille("Dedicon", expr, r#"{1, 2, 3} doorsnede {4} = {}"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: Deelverzameling.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><mfenced open="{" close="}"><mn>2</mn> </mfenced><mo>⊂</mo> <mfenced open="{" close="}"><mn>1</mn><mn>2</mn><mn>3</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"{2} deelverz {1, 2, 3}"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Deelverzameling of gelijk aan.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><mfenced open="{" close="}"><mn>2</mn> </mfenced><mo>⊆</mo> <mfenced open="{" close="}"><mn>1</mn><mn>2</mn><mn>3</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"{2} deelverzOf= {1, 2, 3}"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Geen deelverzameling.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mfenced open="{" close="}"><mn>4</mn> </mfenced><mo>⊄</mo> <mfenced open="{" close="}"><mn>1</mn><mn>2</mn><mn>3</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"{4} geenDeelverz {1, 2, 3}"#)?;
        return Ok(());
    }

    /// Voorbeeld 9: Een element van de gehele getallen.
    #[test]
    fn voorbeeld_09() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>∈</mo><mi>ℤ</mi></math>"#;
        test_braille("Dedicon", expr, r#"2 element Z"#)?;
        return Ok(());
    }

    /// Voorbeeld 10: Geen element van de natuurlijke getallen.
    /// NOTE: the linear notation on the website has "-2", but its MathML has "2" (2 is a natural number,
    /// so -2 is probably meant); the expected output follows the MathML used here.
    #[test]
    fn voorbeeld_10() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>∉</mo><mi>ℕ</mi></math>"#;
        test_braille("Dedicon", expr, r#"2 geenElement N"#)?;
        return Ok(());
    }
}

// Dedicon examples: worteltrekken
// Source: https://www.dedicon.nl/artikel/scholen/regels-en-voorbeelden-worteltrekken/
mod worteltrekken {
    use crate::common::*;
    use anyhow::Result;

    /// Voorbeeld 1: Vierkantswortel (of wortel) van een getal.
    #[test]
    fn voorbeeld_01() -> Result<()> {
        let expr = r#"<math><msqrt><mn>25</mn> </msqrt><mo>=</mo> <mn>5</mn></math>"#;
        test_braille("Dedicon", expr, r#"sqrt(25) = 5"#)?;
        return Ok(());
    }

    /// Voorbeeld 2: Vierkantswortel in een langere expressie.
    #[test]
    fn voorbeeld_02() -> Result<()> {
        let expr = r#"<math><mn>2</mn> <mo>&#x2062;</mo> <msqrt><mn>25</mn> </msqrt><mo>=</mo> <mn>10</mn></math>"#;
        test_braille("Dedicon", expr, r#"2sqrt(25) = 10"#)?;
        return Ok(());
    }

    /// Voorbeeld 3: Vierkantswortel van een som.
    #[test]
    fn voorbeeld_03() -> Result<()> {
        let expr = r#"<math><msqrt><mrow><mn>11</mn> <mo>+</mo> <mn>89</mn> </mrow></msqrt><mo>=</mo> <mn>10</mn></math>"#;
        test_braille("Dedicon", expr, r#"sqrt(11 + 89) = 10"#)?;
        return Ok(());
    }

    /// Voorbeeld 4: Twee vierkantswortels.
    #[test]
    fn voorbeeld_04() -> Result<()> {
        let expr = r#"<math><msqrt><mi>x</mi> </msqrt><mo>⋅</mo> <msqrt><mrow><mi>y</mi> <mo>&#x2062;</mo> <mi>z</mi> </mrow></msqrt><mo>=</mo> <msqrt><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi> <mo>&#x2062;</mo> <mi>z</mi></mrow></msqrt></math>"#;
        test_braille("Dedicon", expr, r#"sqrt(x) * sqrt(yz) = sqrt(xyz)"#)?;
        return Ok(());
    }

    /// Voorbeeld 5: De vierkantswortel van een breuk.
    #[test]
    fn voorbeeld_05() -> Result<()> {
        let expr = r#"<math><msqrt><mfrac><mn>1</mn> <mn>9</mn></mfrac></msqrt></math>"#;
        test_braille("Dedicon", expr, r#"sqrt(1/9)"#)?;
        return Ok(());
    }

    /// Voorbeeld 6: De vierkantswortel van een kwadraat.
    #[test]
    fn voorbeeld_06() -> Result<()> {
        let expr = r#"<math><msqrt><msup><mn>8</mn> <mn>2</mn> </msup></msqrt><mo>=</mo> <mn>8</mn></math>"#;
        test_braille("Dedicon", expr, r#"sqrt(8^2) = 8"#)?;
        return Ok(());
    }

    /// Voorbeeld 7: Een derdemachtswortel.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><mroot><mn>8</mn> <mn>3</mn> </mroot><mo>=</mo> <mn>2</mn></math>"#;
        test_braille("Dedicon", expr, r#"root_3(8) = 2"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: Vierdemachtswortel van een som.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mroot><mrow><mn>1</mn> <mo>+</mo> <mn>80</mn> </mrow><mn>4</mn> </mroot><mo>=</mo> <mn>3</mn></math>"#;
        test_braille("Dedicon", expr, r#"root_4(1 + 80) = 3"#)?;
        return Ok(());
    }

    /// Voorbeeld 9: (1 plus 2)-de-machtswortel.
    #[test]
    fn voorbeeld_09() -> Result<()> {
        let expr = r#"<math><mroot><mn>27</mn> <mrow><mn>1</mn> <mo>+</mo> <mn>2</mn> </mrow></mroot><mo>=</mo> <mn>3</mn></math>"#;
        test_braille("Dedicon", expr, r#"root_(1 + 2)(27) = 3"#)?;
        return Ok(());
    }

    /// Voorbeeld 10: Wortel met letters, een macht en een breuk.
    #[test]
    fn voorbeeld_10() -> Result<()> {
        let expr = r#"<math><mroot><msup><mi>a</mi> <mi>p</mi> </msup><mi>q</mi> </mroot><mo>=</mo> <msup><mi>a</mi> <mfrac><mi>p</mi> <mi>q</mi></mfrac></msup></math>"#;
        test_braille("Dedicon", expr, r#"root_q(a^p) = a^(p/q)"#)?;
        return Ok(());
    }
}
