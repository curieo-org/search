Image = curieo-search-frontend
ECR = 698471419283.dkr.ecr.eu-central-1.amazonaws.com/curieo-search-frontend
TAG = <TAG>

POSTHOG_KEY = <POSTHOG_API_KEY>
POSTHOG_API_HOST = <FRONTEND_URL>/ingest
POSTHOG_UI_HOST = https://app.posthog.com
API_URL = <BACKEND_URL>
AUTH_URL = <FRONTEND_URL>
AUTH_SECRET = <base64 32 bit secret string>
SENTRY_DSN = <SENTRY_DSN>
SENTRY_AUTH_TOKEN = <SENTRY_AUTH_TOKEN>

ecr_deploy: login
	docker buildx build --platform linux/amd64 --build-arg POSTHOG_KEY=$(POSTHOG_KEY) --build-arg POSTHOG_API_HOST=$(POSTHOG_API_HOST) --build-arg POSTHOG_UI_HOST=$(POSTHOG_UI_HOST) --build-arg API_URL=$(API_URL) --build-arg AUTH_URL=$(AUTH_URL) --build-arg AUTH_SECRET=$(AUTH_SECRET) --build-arg SENTRY_DSN=$(SENTRY_DSN) --build-arg SENTRY_AUTH_TOKEN=$(SENTRY_AUTH_TOKEN) -t $(Image) .
	docker tag $(Image) $(ECR):$(TAG)
	docker push $(ECR):$(TAG)

login:
	aws ecr get-login-password --region eu-central-1 | docker login --username AWS --password-stdin 698471419283.dkr.ecr.eu-central-1.amazonaws.com