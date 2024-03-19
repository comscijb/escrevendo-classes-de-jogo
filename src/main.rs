mod classes;
use std::io;
use crate::classes::Hero;

fn main() {
    println!("Olá, digite o nome do herói a ser criado: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Falha ao ler o nome do herói");

    println!("Digite a idade do herói: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age)
        .expect("Falha ao ler a idade do herói");

    let age: i32 = age.trim().parse()
        .expect("Falha ao ler a idade do herói");

    let mut class_input: String = String::new();

    loop {
        println!("Digite a classe do herói: [guerreiro, mago, monge, ninja]");
        io::stdin().read_line(&mut class_input)
            .expect("Falha ao ler a classe do herói");

        let class = class_input.trim();

        if Hero::is_class_valid(&class){
            break;
        } else{
            println!("Classe inválida, tente novamente!");
            class_input.clear();
        }
    }

    let hero = Hero {
        name: name,
        age: age,
        class: class_input,
    }; 
    hero.atacar()
}
