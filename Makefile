.PHONY: help run test

PORT ?= 3000
CURL_FLAGS ?= -v

help:
	cat Makefile

run:
	cargo run --example simple_server

test:
	@echo ""

	curl $(CURL_FLAGS) http://127.0.0.1:$(PORT)/users
	@echo ""
	@echo "Expected: Returning all users"
	@echo ""

	curl $(CURL_FLAGS) http://127.0.0.1:$(PORT)/users/123
	@echo ""
	@echo "Expected: Fetching user with id: 123"
	@echo ""

	curl $(CURL_FLAGS) -X POST -d '{"name": "test"}' http://127.0.0.1:$(PORT)/users
	@echo ""
	@echo "Expected: User created"
	@echo ""
