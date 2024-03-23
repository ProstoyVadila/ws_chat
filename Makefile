.PHONY: backend frontend

backend:
	@echo "starting backend"
	@RUST_LOG=info cargo run --bin backend

frontend:
	@echo "starting frontend"
	@cd frontend && trunk serve
