FROM messense/rust-musl-cross:x86_64-musl as builder
# ENV SQLX_OFFLINE=true
WORKDIR /tutorial

#Copy the source code
COPY . .
#Build the application
RUN apt-get update
RUN sudo apt-get install libssl-dev
RUN apt-get install -y pkg-config
RUN cargo build --release --target x86_64-unknown-linux-musl

#Create a new stage with a minimal image
FROM scratch
COPY --from=builder /tutorial/target/x86_64-unknown-linux-musl/release/tutorial /tutorial
ENTRYPOINT [ "/tutorial" ]
EXPOSE 3000