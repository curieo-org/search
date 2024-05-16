ruff:
	poetry run ruff check .

ruff-fix:
	poetry run ruff check --fix .

ruff-fmt:
# isort via ruff check --fix
	poetry run ruff check --select I --fix
	poetry run ruff format .

mypy:
	poetry run dmypy run .

lint: ruff mypy

lint-fix: ruff-fix

fmt: ruff-fmt

test:
	poetry run pytest .
