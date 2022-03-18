FROM gcr.io/distroless/cc:latest

WORKDIR /app
COPY udbot /app
RUN chmod -R 777 /app

CMD ["/app/udbot"]
