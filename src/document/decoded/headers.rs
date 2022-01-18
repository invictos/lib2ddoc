#[derive(PartialEq,Debug)]
pub struct Headers {
    pub marqueur : String,
    pub version : String,
    pub identifiant_de_ac : String,
    pub identifiant_du_certificat : String,
    pub date_emission : String,
    pub date_signature: String,
    pub type_document: String,
    pub perimetre : Option<String>,
    pub pays_emetteur : Option<String>,
}

impl Headers{
    
    pub fn new (str: &str) -> (Headers, usize){
        let mut entete = Headers ::initialiser(str); 
        entete.remplir_cas_particuliers(str);

        let taille=Headers::get_taille(entete.version.as_str());
        return (entete,taille);
    }
    
    fn initialiser(str : &str) -> Headers {
        Headers {
            marqueur : (&str[0..2]).to_string(),
            version :  (&str[2..4]).to_string(),
            identifiant_de_ac : (&str[4..8]).to_string(), 
            identifiant_du_certificat : (&str[8..12]).to_string(),
            date_emission : (&str[12..16]).to_string(),
            date_signature : (&str[16..20]).to_string(),
            type_document : (&str[20..22]).to_string(),
            perimetre :None,
            pays_emetteur :None,
        }
    }

    fn remplir_cas_particuliers (&mut self, str : &str) {
        if self.version == "03" {
            self.perimetre = Some(((&str[22..24])).to_string());
        }
        else if self.version == "04" {
            self.perimetre = Some(((&str[22..24])).to_string());
            self.pays_emetteur = Some(((&str[24..26])).to_string());
        }
    }
    fn get_taille(version : &str)-> usize{
        match version {
            "01" => 22,
            "02" => 22,
            "03" => 24,
            "04" => 26,
            _ =>panic!("Version doesnt exist"),
        }
    }

}

#[cfg(test)]
mod tests{
    use super::*; 
    #[test]

    fn test_entete(){
        let testentete = Headers {
            marqueur:String::from("DC"),
            version :String::from("03"),
            identifiant_de_ac :String::from ("FR0A"), 
            identifiant_du_certificat :String::from ("XT4A"),
            date_emission :String::from ("0E84"),
            date_signature :String::from ("0E8A"),
            type_document : String::from("01"),
            perimetre :Some(String::from("01")),
            pays_emetteur :None,
        };
        let (testentete1,taille) = Headers::new(&String::from("DC03FR0AXT4A0E840E8A0101"));

        assert_eq!(testentete,testentete1);
        assert_eq!(taille,24);
    }

    #[test]
    fn test_entete2(){
        let testentete2 = Headers {
            marqueur:String::from("DC"),
            version :String::from("04"),
            identifiant_de_ac :String::from ("FR0A"), 
            identifiant_du_certificat :String::from ("XT4A"),
            date_emission :String::from ("0E84"),
            date_signature :String::from ("0E8A"),
            type_document : String::from("01"),
            perimetre :Some(String::from("01")),
            pays_emetteur :Some(String::from("FR")),
        };
        let (testentete3,taille) = Headers::new(&String::from("DC04FR0AXT4A0E840E8A0101FR"));

        assert_eq!(testentete2,testentete3);
        assert_eq!(taille,26);
    }
}