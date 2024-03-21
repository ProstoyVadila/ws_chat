.PHONY: backend frontend

backend:
	@echo "starting backend"
	@cargo run --bin backend

frontend:
	@echo "starting frontend"
	@cd frontend && trunk serve
