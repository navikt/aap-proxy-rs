pub trait SoapService {
    fn nyeste_aktive_sak(ident: &str);
    fn opprett_oppgave(ident: &str, enhet: &str, tittel: &str, titler: Vec<String>);
}
