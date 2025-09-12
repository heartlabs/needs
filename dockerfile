FROM ubuntu-latest

WORKDIR /usr/app

ADD assets assets
ADD config config
ADD needs-cli .
ENTRYPOINT ["/usr/app/needs-cli"]
CMD ["start", "-e", "production"]