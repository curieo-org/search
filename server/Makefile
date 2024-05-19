Image = curieo-search-server
ECR = 698471419283.dkr.ecr.eu-central-1.amazonaws.com/curieo-search-server
TAG = 5.0

PLATFORM ?= linux/amd64

build:
	cp -R ../proto proto
	docker buildx build --platform $(PLATFORM) -t $(Image) .
	rm -rf proto

ecr_deploy: build login
	docker tag $(Image) $(ECR):$(TAG)
	docker push $(ECR):$(TAG)

login:
	aws ecr get-login-password --region eu-central-1 | docker login --username AWS --password-stdin 698471419283.dkr.ecr.eu-central-1.amazonaws.com
