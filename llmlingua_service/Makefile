VERSION = $(shell cat VERSION)

.ONESHELL:

.PHONY: docker_build
docker_build:
	DOCKER_BUILDKIT=1 docker build --platform linux/amd64 \
	--rm \
	-t search-llmlingua:latest \
	-t search-llmlingua:$(VERSION) \
	-f Dockerfile .

.PHONY: docker_push
docker_push:
	docker tag search-llmlingua:latest 698471419283.dkr.ecr.eu-central-1.amazonaws.com/search-llmlingua:latest || exit 1
	docker tag search-llmlingua:$(VERSION) 698471419283.dkr.ecr.eu-central-1.amazonaws.com/search-llmlingua:$(VERSION) || exit 1
	docker push 698471419283.dkr.ecr.eu-central-1.amazonaws.com/search-llmlingua:latest || exit 1
	docker push 698471419283.dkr.ecr.eu-central-1.amazonaws.com/search-llmlingua:$(VERSION) || exit 1