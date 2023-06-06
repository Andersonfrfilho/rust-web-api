## create a container docker

- Create Dockerfile
- run command:

  - `docker build -t rust-web-service .`

- create container
  - `docker run -p 8080:8080 --rm --name rust-web-service-api rust-web-service`
