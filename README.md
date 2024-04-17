# Rust COT Emitter

# Local Development with OpenTAKServer

1. Run RabbitMQ: `docker run -d --hostname rabbit-tak --name rabbit-tak -p 5672:5672 rabbitmq:3`
1. Install pipx
1. Install poetry
1. `poetry shell`
1. `poetry install`
1. Run OpenTAKServer: `TZ="US/Central" python opentakserver/app.py`
