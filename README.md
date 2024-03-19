# Sistema de Criação de Heróis

Este é um programa simples escrito em Rust que permite criar heróis com nome, idade e classe. O programa solicitará que você insira o nome, a idade e a classe do herói, e em seguida, o herói atacará usando um tipo específico de ataque, dependendo da classe escolhida.

## Como usar

1. Certifique-se de ter o Rust instalado em seu sistema. Se não tiver, pode fazer o download em [rust-lang.org](https://www.rust-lang.org/).
2. Clone ou baixe este repositório em sua máquina.
3. Abra um terminal e navegue até o diretório onde você baixou/clonou o código.
4. Execute o seguinte comando para compilar e executar o programa:

    ```bash
    cargo run
    ```

5. Siga as instruções no terminal para criar seu herói. Você será solicitado a inserir o nome, a idade e a classe do herói. As classes válidas são "guerreiro", "mago", "monge" e "ninja".
6. Após inserir todas as informações, o herói será criado e realizará um ataque usando o tipo de ataque específico da classe escolhida.

## Estrutura do Projeto

O projeto consiste em dois arquivos principais:

- **main.rs**: Este arquivo contém a função principal `main()` que interage com o usuário, solicita informações sobre o herói e cria uma instância da struct `Hero`.
- **classes.rs**: Este arquivo contém a definição da struct `Hero` e suas implementações, incluindo métodos para escolher um ataque e verificar se a classe é válida.

## Contribuindo

Contribuições são bem-vindas! Se você encontrar algum problema, tiver alguma sugestão de melhoria ou quiser adicionar novos recursos, sinta-se à vontade para abrir uma issue ou enviar um pull request.

## Licença

Este projeto está licenciado sob a licença GNU. Consulte o arquivo [LICENSE](LICENSE) para obter mais detalhes.
