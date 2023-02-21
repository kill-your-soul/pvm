FROM rust

COPY . .

RUN cargo build --release
RUN ./target/release/pvm 
ENTRYPOINT [ "bash" ]