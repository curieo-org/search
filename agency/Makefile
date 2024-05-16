ruff:
	poetry run ruff check .

black:
	poetry run black --check .

ruff-fix:
	poetry run ruff check --fix .

black-fix:
	poetry run black .

mypy:
	poetry run dmypy run .

fmt: ruff-fix black-fix

lint: ruff black mypy

test:
	poetry run pytest .
