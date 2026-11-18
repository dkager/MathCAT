// Spaties tussen een getal en een eenheid (rapport § 2.3 en § 2.4).
// Een eenheid staat in een mi of mtext; de eventuele spatie staat in de eenheid of aan het eind van de mn.

/// Spatie aan het begin van een mi met de eenheid.
mod eenheid_in_mi {
    use crate::common::*;
    use anyhow::Result;

    /// Getal met eenheid in een mi cm, zonder spatie. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn zonder_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><mi>cm</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mi cm, met een vaste spatie (U+00A0) aan het begin van de mi. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn vaste_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><mi>&#xA0;cm</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mi cm, met een gewone spatie aan het begin van de mi. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn gewone_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><mi> cm</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mi cm tot de macht 3, zonder spatie. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn macht_zonder_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><msup><mi>cm</mi><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3cm^3"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mi cm tot de macht 3, met een vaste spatie (U+00A0) aan het begin van de mi. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn macht_vaste_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><msup><mi>&#xA0;cm</mi><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm^3"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mi cm tot de macht 3, met een gewone spatie aan het begin van de mi. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn macht_gewone_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><msup><mi> cm</mi><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3cm^3"#)?;
        return Ok(());
    }
}

/// Spatie aan het eind van de mn voor een eenheid in een mi.
mod eenheid_in_mi_spatie_in_mn {
    use crate::common::*;
    use anyhow::Result;

    /// Getal met eenheid in een mi cm, met een vaste spatie (U+00A0) aan het eind van de mn. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn vaste_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3&#xA0;</mn><mi>cm</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mi cm, met een gewone spatie aan het eind van de mn. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn gewone_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3 </mn><mi>cm</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mi cm tot de macht 3, met een vaste spatie (U+00A0) aan het eind van de mn. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn macht_vaste_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3&#xA0;</mn><msup><mi>cm</mi><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm^3"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mi cm tot de macht 3, met een gewone spatie aan het eind van de mn. Een vaste spatie blijft staan, een gewone spatie verdwijnt.
    #[test]
    fn macht_gewone_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3 </mn><msup><mi>cm</mi><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3cm^3"#)?;
        return Ok(());
    }
}

/// Spatie aan het begin van een mtext met de eenheid.
mod eenheid_in_mtext {
    use crate::common::*;
    use anyhow::Result;

    /// Getal met eenheid in een mtext cm, zonder spatie. Voor een mtext komt altijd een spatie.
    #[test]
    fn zonder_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><mtext>cm</mtext></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mtext cm, met een vaste spatie (U+00A0) aan het begin van de mtext. Voor een mtext komt altijd een spatie.
    #[test]
    fn vaste_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><mtext>&#xA0;cm</mtext></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mtext cm, met een gewone spatie aan het begin van de mtext. Voor een mtext komt altijd een spatie.
    #[test]
    fn gewone_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><mtext> cm</mtext></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mtext cm tot de macht 3, zonder spatie. Voor een mtext komt altijd een spatie.
    #[test]
    fn macht_zonder_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><msup><mtext>cm</mtext><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm^3"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mtext cm tot de macht 3, met een vaste spatie (U+00A0) aan het begin van de mtext. Voor een mtext komt altijd een spatie.
    #[test]
    fn macht_vaste_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><msup><mtext>&#xA0;cm</mtext><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm^3"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mtext cm tot de macht 3, met een gewone spatie aan het begin van de mtext. Voor een mtext komt altijd een spatie.
    #[test]
    fn macht_gewone_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3</mn><msup><mtext> cm</mtext><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm^3"#)?;
        return Ok(());
    }
}

/// Spatie aan het eind van de mn voor een eenheid in een mtext.
mod eenheid_in_mtext_spatie_in_mn {
    use crate::common::*;
    use anyhow::Result;

    /// Getal met eenheid in een mtext cm, met een vaste spatie (U+00A0) aan het eind van de mn. Voor een mtext komt altijd een spatie.
    #[test]
    fn vaste_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3&#xA0;</mn><mtext>cm</mtext></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mtext cm, met een gewone spatie aan het eind van de mn. Voor een mtext komt altijd een spatie.
    #[test]
    fn gewone_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3 </mn><mtext>cm</mtext></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mtext cm tot de macht 3, met een vaste spatie (U+00A0) aan het eind van de mn. Voor een mtext komt altijd een spatie.
    #[test]
    fn macht_vaste_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3&#xA0;</mn><msup><mtext>cm</mtext><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm^3"#)?;
        return Ok(());
    }

    /// Getal met eenheid in een mtext cm tot de macht 3, met een gewone spatie aan het eind van de mn. Voor een mtext komt altijd een spatie.
    #[test]
    fn macht_gewone_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mn>3 </mn><msup><mtext>cm</mtext><mn>3</mn></msup></mrow></math>"#;
        test_braille("Dedicon", expr, r#"3 cm^3"#)?;
        return Ok(());
    }
}

/// Spatie aan het eind van een mi (rapport § 2.4): een vaste spatie blijft staan, een gewone spatie verdwijnt.
mod spatie_aan_eind_van_mi {
    use crate::common::*;
    use anyhow::Result;

    /// Afkorting in een mi met een vaste spatie (U+00A0) aan het eind: de spatie blijft staan.
    #[test]
    fn vaste_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mi>opp.&#xA0;</mi><mi>I</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"opp. I"#)?;
        return Ok(());
    }

    /// Afkorting in een mi met een gewone spatie aan het eind: de spatie verdwijnt.
    #[test]
    fn gewone_spatie() -> Result<()> {
        let expr = r#"<math><mrow><mi>opp. </mi><mi>I</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"opp.I"#)?;
        return Ok(());
    }

    /// Afkorting met nummer samen in een mi, met een gewone spatie ertussen (zo noteren jullie het voortaan): de spatie blijft staan.
    #[test]
    fn spatie_binnen_mi() -> Result<()> {
        let expr = r#"<math><mrow><mi>opp. I</mi><mo>+</mo><mi>opp. II</mi></mrow></math>"#;
        test_braille("Dedicon", expr, r#"opp. I + opp. II"#)?;
        return Ok(());
    }
}
