// Tests for the Dedicon braille code (Dutch math notation in ASCII): parentheses.
// The same expression is written with and without the mrow around the parentheses and around their content, and
// with mfenced. Every mfenced has open and close, except the tests of the default parentheses and default comma in
// module functie_komma. All variants of an expression should give the same linear notation.
// Rules: Rules/Braille/Dedicon/

// Functiewaarde van -8, met -8 als één mn: haakjes met en zonder mrow, en met mfenced
mod functie_min_acht_mn {
    use crate::common::*;
    use anyhow::Result;

    /// f(-8) met het minteken in de mn, een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><mn>-8</mn></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken in de mn, alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mn>-8</mn><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken in de mn, alleen een mrow om de inhoud, niet om de haakjes.
    #[test]
    fn alleen_mrow_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mrow><mn>-8</mn></mrow><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken in de mn, zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mn>-8</mn><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken in de mn, mfenced met separators="": geen komma's tussen de kinderen.
    #[test]
    fn mfenced_separators_leeg() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")" separators=""><mn>-8</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken in de mn, mfenced met open en close, zonder separators-attribuut en zonder mrow: volgens de MathML-standaard een komma tussen alle kinderen.
    #[test]
    fn mfenced_zonder_separators() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mn>-8</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }
}

// Functiewaarde van -8, met het minteken als aparte mo: haakjes met en zonder mrow, en met mfenced
mod functie_min_acht_mo {
    use crate::common::*;
    use anyhow::Result;

    /// f(-8) met het minteken als mo, een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><mo>-</mo><mn>8</mn></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken als mo, alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mo>-</mo><mn>8</mn><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken als mo, alleen een mrow om de inhoud, niet om de haakjes.
    #[test]
    fn alleen_mrow_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mrow><mo>-</mo><mn>8</mn></mrow><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken als mo, zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mo>-</mo><mn>8</mn><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken als mo, mfenced met separators="": geen komma's tussen de kinderen.
    #[test]
    fn mfenced_separators_leeg() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")" separators=""><mo>-</mo><mn>8</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(-8)"#)?;
        return Ok(());
    }

    /// f(-8) met het minteken als mo, mfenced met open en close, zonder separators-attribuut en zonder mrow: volgens de MathML-standaard een komma tussen alle kinderen.
    #[test]
    fn mfenced_zonder_separators() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mo>-</mo><mn>8</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(-, 8)"#)?;
        return Ok(());
    }
}

// Functiewaarde van een som: haakjes met en zonder mrow, en met mfenced
mod functie_som {
    use crate::common::*;
    use anyhow::Result;

    /// f(a + b), een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(a + b)"#)?;
        return Ok(());
    }

    /// f(a + b), alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(a + b)"#)?;
        return Ok(());
    }

    /// f(a + b), alleen een mrow om de inhoud, niet om de haakjes.
    #[test]
    fn alleen_mrow_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(a + b)"#)?;
        return Ok(());
    }

    /// f(a + b), zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(a + b)"#)?;
        return Ok(());
    }

    /// f(a + b), mfenced met separators="": geen komma's tussen de kinderen.
    #[test]
    fn mfenced_separators_leeg() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")" separators=""><mi>a</mi><mo>+</mo><mi>b</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(a + b)"#)?;
        return Ok(());
    }

    /// f(a + b), mfenced met open en close, zonder separators-attribuut en zonder mrow: volgens de MathML-standaard een komma tussen alle kinderen.
    #[test]
    fn mfenced_zonder_separators() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mi>a</mi><mo>+</mo><mi>b</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(a, + , b)"#)?;
        return Ok(());
    }

    /// f(a - b), mfenced met separators="": geen komma's tussen de kinderen.
    #[test]
    fn mfenced_separators_leeg_min() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")" separators=""><mi>a</mi><mo>-</mo><mi>b</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(a - b)"#)?;
        return Ok(());
    }

    /// f(a - b), mfenced met open en close, zonder separators-attribuut en zonder mrow: volgens de MathML-standaard een komma tussen alle kinderen.
    #[test]
    fn mfenced_zonder_separators_min() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mi>a</mi><mo>-</mo><mi>b</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(a, - , b)"#)?;
        return Ok(());
    }
}

// Functie met twee argumenten, gescheiden door een komma: haakjes met en zonder mrow, en met mfenced
mod functie_komma {
    use crate::common::*;
    use anyhow::Result;

    /// f(x, y) met de komma als mo, een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>,</mo><mi>y</mi></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }

    /// f(x, y) met de komma als mo, alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }

    /// f(x, y) met de komma als mo, alleen een mrow om de inhoud, niet om de haakjes.
    #[test]
    fn alleen_mrow_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mrow><mi>x</mi><mo>,</mo><mi>y</mi></mrow><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }

    /// f(x, y) met de komma als mo, zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }

    /// f(x, y) met de komma als mo, mfenced met separators="": geen komma's tussen de kinderen.
    #[test]
    fn mfenced_separators_leeg() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")" separators=""><mi>x</mi><mo>,</mo><mi>y</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }

    /// f(x, y) met de komma als mo, mfenced met de inhoud, inclusief de komma, in één mrow: geen logische codering, maar er komt geen extra komma omdat mfenced maar één kind heeft.
    #[test]
    fn mfenced_met_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mrow><mi>x</mi><mo>,</mo><mi>y</mi></mrow></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }

    /// f(x, y) met de komma als mo, mfenced met separators="," en de twee argumenten als kinderen.
    #[test]
    fn mfenced_separators_komma() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")" separators=","><mi>x</mi><mi>y</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }

    /// f(x, y) met de komma als mo, mfenced met open en close, zonder separators-attribuut: de standaardkomma tussen de twee argumenten.
    #[test]
    fn mfenced_zonder_separators() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mi>x</mi><mi>y</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }

    /// f(x, y) met de komma als mo, mfenced met separators=",", zonder open en close: de standaardhaakjes ( en ).
    #[test]
    fn mfenced_standaardhaakjes() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced separators=","><mi>x</mi><mi>y</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }

    /// f(x, y) met de komma als mo, mfenced zonder open, close en separators: de standaardhaakjes en de standaardkomma.
    #[test]
    fn mfenced_alle_standaarden() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced><mi>x</mi><mi>y</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(x, y)"#)?;
        return Ok(());
    }
}

// Een som tussen haakjes als teller van een breuk: haakjes met en zonder mrow, en met mfenced
mod breuk_som {
    use crate::common::*;
    use anyhow::Result;

    /// (a + b)/c: een som tussen haakjes als teller, een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><mfrac><mrow><mo>(</mo><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>)</mo></mrow><mi>c</mi></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"(a + b)/c"#)?;
        return Ok(());
    }

    /// (a + b)/c: een som tussen haakjes als teller, alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><mfrac><mrow><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></mrow><mi>c</mi></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"(a + b)/c"#)?;
        return Ok(());
    }

    /// (a + b)/c: een som tussen haakjes als teller, mfenced met separators="": geen komma's tussen de kinderen.
    #[test]
    fn mfenced_separators_leeg() -> Result<()> {
        let expr = r#"<math><mfrac><mfenced open="(" close=")" separators=""><mi>a</mi><mo>+</mo><mi>b</mi></mfenced><mi>c</mi></mfrac></math>"#;
        test_braille("Dedicon", expr, r#"(a + b)/c"#)?;
        return Ok(());
    }
}

// Een som tussen haakjes als grondtal van een macht: haakjes met en zonder mrow, en met mfenced
mod macht_som {
    use crate::common::*;
    use anyhow::Result;

    /// (a + b)^2: een som tussen haakjes als grondtal, een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><msup><mrow><mo>(</mo><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"(a + b)^2"#)?;
        return Ok(());
    }

    /// (a + b)^2: een som tussen haakjes als grondtal, alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><msup><mrow><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"(a + b)^2"#)?;
        return Ok(());
    }

    /// (a + b)^2: een som tussen haakjes als grondtal, mfenced met separators="": geen komma's tussen de kinderen.
    #[test]
    fn mfenced_separators_leeg() -> Result<()> {
        let expr = r#"<math><msup><mfenced open="(" close=")" separators=""><mi>a</mi><mo>+</mo><mi>b</mi></mfenced><mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"(a + b)^2"#)?;
        return Ok(());
    }
}

// Een negatief getal tussen haakjes als grondtal van een macht: haakjes met en zonder mrow, en met mfenced
mod macht_min_acht {
    use crate::common::*;
    use anyhow::Result;

    /// (-8)^2: een negatief getal tussen haakjes als grondtal, een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><msup><mrow><mo>(</mo><mrow><mo>-</mo><mn>8</mn></mrow><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"(-8)^2"#)?;
        return Ok(());
    }

    /// (-8)^2: een negatief getal tussen haakjes als grondtal, alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><msup><mrow><mo>(</mo><mo>-</mo><mn>8</mn><mo>)</mo></mrow><mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"(-8)^2"#)?;
        return Ok(());
    }

    /// (-8)^2: een negatief getal tussen haakjes als grondtal, mfenced met separators="": geen komma's tussen de kinderen.
    #[test]
    fn mfenced_separators_leeg() -> Result<()> {
        let expr = r#"<math><msup><mfenced open="(" close=")" separators=""><mo>-</mo><mn>8</mn></mfenced><mn>2</mn></msup></math>"#;
        test_braille("Dedicon", expr, r#"(-8)^2"#)?;
        return Ok(());
    }
}

// Een getal voor haakjes (onzichtbare vermenigvuldiging): haakjes met en zonder mrow, en met mfenced
mod onzichtbare_vermenigvuldiging {
    use crate::common::*;
    use anyhow::Result;

    /// 2(x + 3), een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"2(x + 3)"#)?;
        return Ok(());
    }

    /// 2(x + 3), alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mrow><mo>(</mo><mi>x</mi><mo>+</mo><mn>3</mn><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"2(x + 3)"#)?;
        return Ok(());
    }

    /// 2(x + 3), alleen een mrow om de inhoud, niet om de haakjes.
    #[test]
    fn alleen_mrow_binnen() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"2(x + 3)"#)?;
        return Ok(());
    }

    /// 2(x + 3), zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mo>(</mo><mi>x</mi><mo>+</mo><mn>3</mn><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"2(x + 3)"#)?;
        return Ok(());
    }

    /// 2(x + 3), mfenced met separators="": geen komma's tussen de kinderen.
    #[test]
    fn mfenced_separators_leeg() -> Result<()> {
        let expr = r#"<math><mn>2</mn><mfenced open="(" close=")" separators=""><mi>x</mi><mo>+</mo><mn>3</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"2(x + 3)"#)?;
        return Ok(());
    }

    /// 2(x + 3) met een onzichtbaar maalteken (U+2062) tussen 2 en de haakjes.
    #[test]
    fn onzichtbaar_maal_teken() -> Result<()> {
        let expr = r#"<math><mrow><mn>2</mn><mo>&#x2062;</mo><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mo>)</mo></mrow></mrow></math>"#;
        test_braille("Dedicon", expr, r#"2(x + 3)"#)?;
        return Ok(());
    }
}

// Een macht als enige inhoud van de haakjes: haakjes met en zonder mrow, en met mfenced
mod functie_macht {
    use crate::common::*;
    use anyhow::Result;

    /// f(x^2), een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><msup><mi>x</mi><mn>2</mn></msup></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(x^2)"#)?;
        return Ok(());
    }

    /// f(x^2), alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(x^2)"#)?;
        return Ok(());
    }

    /// f(x^2), alleen een mrow om de inhoud, niet om de haakjes.
    #[test]
    fn alleen_mrow_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mrow><msup><mi>x</mi><mn>2</mn></msup></mrow><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(x^2)"#)?;
        return Ok(());
    }

    /// f(x^2), zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><msup><mi>x</mi><mn>2</mn></msup><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(x^2)"#)?;
        return Ok(());
    }

    /// f(x^2), als enig kind van mfenced (met open en close, zonder separators).
    #[test]
    fn mfenced() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><msup><mi>x</mi><mn>2</mn></msup></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(x^2)"#)?;
        return Ok(());
    }

    /// f(x^2), in een mrow als enig kind van mfenced.
    #[test]
    fn mfenced_met_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mrow><msup><mi>x</mi><mn>2</mn></msup></mrow></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(x^2)"#)?;
        return Ok(());
    }
}

// Een breuk als enige inhoud van de haakjes: haakjes met en zonder mrow, en met mfenced
mod functie_breuk {
    use crate::common::*;
    use anyhow::Result;

    /// f(1/2), een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(1/2)"#)?;
        return Ok(());
    }

    /// f(1/2), alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(1/2)"#)?;
        return Ok(());
    }

    /// f(1/2), alleen een mrow om de inhoud, niet om de haakjes.
    #[test]
    fn alleen_mrow_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mrow><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(1/2)"#)?;
        return Ok(());
    }

    /// f(1/2), zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(1/2)"#)?;
        return Ok(());
    }

    /// f(1/2), als enig kind van mfenced (met open en close, zonder separators).
    #[test]
    fn mfenced() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mfrac><mn>1</mn><mn>2</mn></mfrac></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(1/2)"#)?;
        return Ok(());
    }

    /// f(1/2), in een mrow als enig kind van mfenced.
    #[test]
    fn mfenced_met_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mrow><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(1/2)"#)?;
        return Ok(());
    }
}

// Een wortel als enige inhoud van de haakjes: haakjes met en zonder mrow, en met mfenced
mod functie_wortel {
    use crate::common::*;
    use anyhow::Result;

    /// f(sqrt(x)), een mrow om de haakjes en een mrow om de inhoud.
    #[test]
    fn mrow_buiten_en_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><msqrt><mi>x</mi></msqrt></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(sqrt(x))"#)?;
        return Ok(());
    }

    /// f(sqrt(x)), alleen een mrow om de haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrow_buiten() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><msqrt><mi>x</mi></msqrt><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(sqrt(x))"#)?;
        return Ok(());
    }

    /// f(sqrt(x)), alleen een mrow om de inhoud, niet om de haakjes.
    #[test]
    fn alleen_mrow_binnen() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mrow><msqrt><mi>x</mi></msqrt></mrow><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(sqrt(x))"#)?;
        return Ok(());
    }

    /// f(sqrt(x)), zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><msqrt><mi>x</mi></msqrt><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(sqrt(x))"#)?;
        return Ok(());
    }

    /// f(sqrt(x)), als enig kind van mfenced (met open en close, zonder separators).
    #[test]
    fn mfenced() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><msqrt><mi>x</mi></msqrt></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(sqrt(x))"#)?;
        return Ok(());
    }

    /// f(sqrt(x)), in een mrow als enig kind van mfenced.
    #[test]
    fn mfenced_met_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mrow><msqrt><mi>x</mi></msqrt></mrow></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(sqrt(x))"#)?;
        return Ok(());
    }
}

// Haakjes in haakjes (mfenced in mfenced): haakjes met en zonder mrow, en met mfenced
mod genest_haken {
    use crate::common::*;
    use anyhow::Result;

    /// [(a + b) - 1], mfenced met vierkante haken om een mfenced met ronde haken.
    #[test]
    fn mfenced_in_mfenced() -> Result<()> {
        let expr = r#"<math><mfenced open="[" close="]" separators=""><mfenced open="(" close=")" separators=""><mi>a</mi><mo>+</mo><mi>b</mi></mfenced><mo>-</mo><mn>1</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"[(a + b) - 1]"#)?;
        return Ok(());
    }

    /// [(a + b) - 1], mfenced met vierkante haken om ronde haken als mrow en mo.
    #[test]
    fn mfenced_buiten_mrow_binnen() -> Result<()> {
        let expr = r#"<math><mfenced open="[" close="]" separators=""><mrow><mo>(</mo><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>)</mo></mrow><mo>-</mo><mn>1</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"[(a + b) - 1]"#)?;
        return Ok(());
    }

    /// [(a + b) - 1], vierkante haken als mrow en mo om een mfenced met ronde haken.
    #[test]
    fn mrow_buiten_mfenced_binnen() -> Result<()> {
        let expr = r#"<math><mrow><mo>[</mo><mrow><mfenced open="(" close=")" separators=""><mi>a</mi><mo>+</mo><mi>b</mi></mfenced><mo>-</mo><mn>1</mn></mrow><mo>]</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"[(a + b) - 1]"#)?;
        return Ok(());
    }

    /// [(a + b) - 1], alle haakjes als mo, met een mrow om elke groep en om elke inhoud.
    #[test]
    fn alle_mrows() -> Result<()> {
        let expr = r#"<math><mrow><mo>[</mo><mrow><mrow><mo>(</mo><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>)</mo></mrow><mo>-</mo><mn>1</mn></mrow><mo>]</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"[(a + b) - 1]"#)?;
        return Ok(());
    }

    /// [(a + b) - 1], een mrow om elke groep met haakjes, niet om de inhoud.
    #[test]
    fn alleen_mrows_om_haakjes() -> Result<()> {
        let expr = r#"<math><mrow><mo>[</mo><mrow><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo></mrow><mo>-</mo><mn>1</mn><mo>]</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"[(a + b) - 1]"#)?;
        return Ok(());
    }

    /// [(a + b) - 1], een mrow om elke inhoud, niet om de haakjes.
    #[test]
    fn alleen_mrows_om_inhoud() -> Result<()> {
        let expr = r#"<math><mo>[</mo><mrow><mo>(</mo><mrow><mi>a</mi><mo>+</mo><mi>b</mi></mrow><mo>)</mo><mo>-</mo><mn>1</mn></mrow><mo>]</mo></math>"#;
        test_braille("Dedicon", expr, r#"[(a + b) - 1]"#)?;
        return Ok(());
    }

    /// [(a + b) - 1], zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mo>[</mo><mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo><mo>-</mo><mn>1</mn><mo>]</mo></math>"#;
        test_braille("Dedicon", expr, r#"[(a + b) - 1]"#)?;
        return Ok(());
    }
}

// Haakjes in haakjes met de standaardkomma (mfenced in mfenced): haakjes met en zonder mrow, en met mfenced
mod genest_komma {
    use crate::common::*;
    use anyhow::Result;

    /// f((a, b), c), mfenced zonder separators in een mfenced zonder separators: de standaardkomma op beide niveaus.
    #[test]
    fn mfenced_in_mfenced() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mfenced open="(" close=")"><mi>a</mi><mi>b</mi></mfenced><mi>c</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f((a, b), c)"#)?;
        return Ok(());
    }

    /// f((a, b), c), haakjes en komma's als mo, met een mrow om elke groep en om elke inhoud.
    #[test]
    fn alle_mrows() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><mrow><mo>(</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>)</mo></mrow><mo>,</mo><mi>c</mi></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f((a, b), c)"#)?;
        return Ok(());
    }

    /// f((a, b), c), haakjes en komma's als mo, zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mo>(</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>)</mo><mo>,</mo><mi>c</mi><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f((a, b), c)"#)?;
        return Ok(());
    }
}

// Een functie in een functie (mfenced in mfenced): haakjes met en zonder mrow, en met mfenced
mod genest_functie {
    use crate::common::*;
    use anyhow::Result;

    /// f(g(x)), g(x) in een mrow als enig kind van de buitenste mfenced.
    #[test]
    fn mfenced_met_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mrow><mi>g</mi><mfenced open="(" close=")"><mi>x</mi></mfenced></mrow></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(g(x))"#)?;
        return Ok(());
    }

    /// f(g(x)), de buitenste mfenced met separators="" en g en zijn mfenced als losse kinderen.
    #[test]
    fn mfenced_separators_leeg() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")" separators=""><mi>g</mi><mfenced open="(" close=")"><mi>x</mi></mfenced></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(g(x))"#)?;
        return Ok(());
    }

    /// f(g(x)), haakjes als mo, met een mrow om elke groep en om elke inhoud.
    #[test]
    fn alle_mrows() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><mi>g</mi><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(g(x))"#)?;
        return Ok(());
    }

    /// f(g(x)), haakjes als mo, zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(g(x))"#)?;
        return Ok(());
    }
}

// Een functie met twee argumenten in een functie (mfenced in mfenced): haakjes met en zonder mrow, en met mfenced
mod genest_functie_komma {
    use crate::common::*;
    use anyhow::Result;

    /// f(g(a, b), c), g en zijn mfenced in een mrow als eerste kind van de buitenste mfenced; beide mfenced met de standaardkomma.
    #[test]
    fn mfenced_met_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mfenced open="(" close=")"><mrow><mi>g</mi><mfenced open="(" close=")"><mi>a</mi><mi>b</mi></mfenced></mrow><mi>c</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"f(g(a, b), c)"#)?;
        return Ok(());
    }

    /// f(g(a, b), c), haakjes en komma's als mo, met een mrow om elke groep en om elke inhoud.
    #[test]
    fn alle_mrows() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mrow><mo>(</mo><mrow><mrow><mi>g</mi><mrow><mo>(</mo><mrow><mi>a</mi><mo>,</mo><mi>b</mi></mrow><mo>)</mo></mrow></mrow><mo>,</mo><mi>c</mi></mrow><mo>)</mo></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(g(a, b), c)"#)?;
        return Ok(());
    }

    /// f(g(a, b), c), haakjes en komma's als mo, zonder enige mrow.
    #[test]
    fn zonder_mrow() -> Result<()> {
        let expr = r#"<math><mi>f</mi><mo>(</mo><mi>g</mi><mo>(</mo><mi>a</mi><mo>,</mo><mi>b</mi><mo>)</mo><mo>,</mo><mi>c</mi><mo>)</mo></math>"#;
        test_braille("Dedicon", expr, r#"f(g(a, b), c)"#)?;
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

    /// Voorbeeld 7: mfenced met separators="": geen komma's tussen de kinderen, ook niet tussen twee variabelen.
    #[test]
    fn voorbeeld_07() -> Result<()> {
        let expr = r#"<math><mfenced open="(" close=")" separators=""><mi>a</mi><mi>b</mi></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"(ab)"#)?;
        return Ok(());
    }

    /// Voorbeeld 8: mfenced met separators="" en een operator: geen komma's naast de operator.
    #[test]
    fn voorbeeld_08() -> Result<()> {
        let expr = r#"<math><mfenced open="(" close=")" separators=""><mi>a</mi><mo>-</mo><mn>9</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"(a - 9)"#)?;
        return Ok(());
    }

    /// Voorbeeld 9: mfenced met separators="" en een operator: ook geen komma tussen het getal en de variabele.
    #[test]
    fn voorbeeld_09() -> Result<()> {
        let expr = r#"<math><mfenced open="(" close=")" separators=""><mn>2</mn><mi>a</mi><mo>+</mo><mn>5</mn></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"(2a + 5)"#)?;
        return Ok(());
    }

    /// Voorbeeld 10: mfenced met separators="" en een komma van de auteur: alleen die komma blijft.
    #[test]
    fn voorbeeld_10() -> Result<()> {
        let expr = r#"<math><mi>S</mi><mfenced open="(" close=")" separators=""><mn>3</mn><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>,</mo><mn>-1</mn><mfrac><mn>1</mn><mn>4</mn></mfrac></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"S(3 1/2, -1 1/4)"#)?;
        return Ok(());
    }

    /// Voorbeeld 11: Functiewaarde van een gemengd getal, met de inhoud van mfenced in een mrow, zoals de MathML-standaard het bedoelt.
    #[test]
    fn voorbeeld_11() -> Result<()> {
        let expr = r#"<math display="inline"><mrow><mi>f</mi><mfenced open="(" close=")"><mrow><mn>3</mn><mfrac><mn>1</mn><mn>2</mn></mfrac></mrow></mfenced><mo>=</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>⋅</mo><mn>3</mn><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>-</mo><mn>3</mn><mo>=</mo><mn>-1</mn><mfrac><mn>1</mn><mn>4</mn></mfrac></mrow></math>"#;
        test_braille("Dedicon", expr, r#"f(3 1/2) = 1/2 * 3 1/2 - 3 = -1 1/4"#)?;
        return Ok(());
    }

    /// Voorbeeld 12: Als voorbeeld 10, zonder separators-attribuut en zonder de komma van de auteur: volgens de
    /// MathML-standaard komt er een komma tussen alle kinderen van mfenced.
    #[test]
    fn voorbeeld_12() -> Result<()> {
        let expr = r#"<math><mi>S</mi><mfenced open="(" close=")"><mn>3</mn><mfrac><mn>1</mn><mn>2</mn></mfrac><mn>-1</mn><mfrac><mn>1</mn><mn>4</mn></mfrac></mfenced></math>"#;
        test_braille("Dedicon", expr, r#"S(3, 1/2, -1, 1/4)"#)?;
        return Ok(());
    }

    /// Voorbeeld 13: Als voorbeeld 10, met een mrow om elk gemengd getal: de komma van mfenced scheidt de twee getallen.
    #[test]
    fn voorbeeld_13() -> Result<()> {
        let expr = r#"<math> <mi>S</mi> <mfenced open="(" close=")"> <mrow> <mn>3</mn> <mfrac> <mn>1</mn> <mn>2</mn> </mfrac> </mrow> <mrow> <mn>-1</mn> <mfrac> <mn>1</mn> <mn>4</mn> </mfrac> </mrow> </mfenced> </math>"#;
        test_braille("Dedicon", expr, r#"S(3 1/2, -1 1/4)"#)?;
        return Ok(());
    }
}
