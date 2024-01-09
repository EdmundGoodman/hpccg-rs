### Variables ###
# https://clarkgrubb.com/makefile-style-guide
MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := install


### Installation ###
.PHONY: install
install:
	poetry install

### Static analysis ###
.PHONY: check
check: .venv/
	poetry run black src
	poetry run isort src
	poetry run ruff check src
	poetry run mypy src

### Execution ###
.PHONY: remote_run_all
remote_run_all: .venv/
	@poetry run python3 src/remote_run_all/main.py

.PHONY: analyse_remote_runs
analyse_remote_runs: .venv/
	@poetry run python3 src/analyse_remote_runs/main.py

### Cleanup ###
.PHONY: clean_cache
clean_cache:
	rm -rf .mypy_cache/ .ruff_cache/

.PHONY: clean_pycache
clean_pycache:
	find . -not -path "./.venv/*" | \
		grep -E "(/__pycache__$$|\.pyc$$|\.pyo$$)" | \
		xargs rm -rf

.PHONY: clean
clean: clean_cache clean_pycache

.PHONY: clobber
clobber: clean
	rm -rf .venv/
