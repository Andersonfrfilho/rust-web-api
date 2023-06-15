## create a container docker

- Create Dockerfile
- run command:

  - `docker build -t rust-web-service .`

- create container

  - `docker run -p 8080:8080 --rm --name rust-web-service-api rust-web-service`

  Requisitos:
  [] Logger
  [] Logger Obfucator
  [] Middleware Oauth
  [] Estado global
  [] Sonar
  [] Github Actions (lint, commit-lint, test-unitario, teste de integracao, deploy stg, sonar, fortify, deploy, producao )
  [] Docker
  [] Docker Development
  [] Teste Unitarios
  [] Middleware de validacao de schemas
  [] swagger docs
  [] conexao com banco de dados SQL
  [] conexao com banco de dados noSQL
  [] conexao com bando de dados de cache
  [] conexao com rabbitmq producer
