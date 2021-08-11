FROM fedora:34

WORKDIR /app
COPY udbot /app
RUN chmod -R 777 /app

CMD ["/app/udbot"]
