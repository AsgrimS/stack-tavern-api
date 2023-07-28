dev:
	cargo watch -c -w ./src/ -x run

test:
	cargo test

start_services:
	docker compose up -d

stop_services:
	docker compose stop

	
