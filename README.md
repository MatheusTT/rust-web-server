# Rust Web Server

Este projeto é um servidor web simples implementado em Rust, baseado no capítulo 20 do [livro oficial de Rust](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html). Ele foi desenvolvido como parte do aprendizado sobre como construir aplicações robustas e performáticas com Rust.

## Funcionalidades

- **Serviço HTTP básico:** Processa requisições GET e responde com arquivos estáticos.
- **Multithreading:** Usa um pool de threads para lidar com várias conexões simultaneamente.
- **Graceful Shutdown:** Permite desligar o servidor de forma segura ao receber o sinal de encerramento.

## Como executar

1. Certifique-se de ter o [Rust](https://www.rust-lang.org/) instalado (recomenda-se a versão mais recente).
2. Clone este repositório:
```bash
git clone https://github.com/MatheusTT/rust-web-server.git
cd rust-web-server
```

3. Compile e execute o servidor:
```bash
cargo run
```

4. Acesse o servidor em http://localhost:7878 ou outro endereço especificado.
