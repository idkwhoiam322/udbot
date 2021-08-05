FROM fedora:34

RUN mkdir -p /app/scripts
WORKDIR /app
COPY udbot /app
COPY getapidata.sh /app/scripts
COPY logger.sh /app/scripts
RUN chmod -R 777 /app

CMD ["/app/udbot"]
