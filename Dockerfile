FROM rust:1.67

WORKDIR /Volumes/kdk/router
COPY . . 

RUN cargo build --release

EXPOSE 80
CMD [ "./target/release/router" ]
