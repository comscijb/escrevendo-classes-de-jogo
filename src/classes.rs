pub struct Hero {
    pub name: String,
    pub age: i32,
    pub class: String,
}

impl Hero {
    pub fn choose_attack(class: &String) -> String {
        match class.trim() {
            "guerreiro" => "espada".to_string(),
            "mago" => "magia".to_string(),
            "monge" => "artes marciais".to_string(),
            "ninja" => "shuriken".to_string(),
            _ => "classe inválida".to_string(),
        }
    }
    pub fn atacar(&self) {
        let attack = Self::choose_attack(&self.class);
        print!("O {} atacou usando {}", self.class, attack);
    }
    pub fn is_class_valid(class: &str) -> bool {
        let class = class.trim();
        matches!(class, "guerreiro" | "mago" | "monge" | "ninja")
    }
}
