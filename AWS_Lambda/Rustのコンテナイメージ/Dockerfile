FROM rust:latest as builder
COPY "./lambda-container/" "/root/lambda-container/"
WORKDIR "/root/lambda-container"
RUN cargo build --release

#-------------------------------------------------

FROM public.ecr.aws/lambda/provided:al2
COPY --from=builder /root/lambda-container/target/release/lambda-container ${LAMBDA_RUNTIME_DIR}/bootstrap
CMD ["lambda-handler"]
