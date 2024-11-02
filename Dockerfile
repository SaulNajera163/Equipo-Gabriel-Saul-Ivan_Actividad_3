# Usamos la imagen oficial de Rust como base para la construcción
FROM rust:1.70 AS builder

# Establecemos el directorio de trabajo
WORKDIR /usr/src/app

# Copiamos los archivos de configuración de Cargo
COPY Cargo.toml Cargo.lock ./

# Copiamos el código fuente
COPY src ./src

# Compilamos el proyecto en modo release
RUN cargo build --release

# Usamos una imagen más ligera para la etapa final
FROM debian:bullseye-slim

# Instalamos las dependencias necesarias (si Rocket necesita SSL)
RUN apt-get update && apt-get install -y libssl-dev && apt-get clean

# Copiamos el binario desde la etapa de construcción
COPY --from=builder /usr/src/app/target/release/HolaMundo /usr/local/bin/HolaMundo

# Exponemos el nuevo puerto en el que la aplicación escucha
EXPOSE 8081

# Comando para ejecutar la aplicación
CMD ["/usr/local/bin/HolaMundo"]