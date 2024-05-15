# search-frontend 

## Install helmchart 

```bash
cd search/frontend/helm
helm install frontend . -n dev --values values.yaml

```

## Create Route53 entry in AWS 

    1. In AWS Console , goto `Hosted Zone` ->  select hosted zone `dev.curieo.org`
    2. create record -> specify subdomain `frontend` 
    3. select toggle button `alias` -> choose endpoint as `Load Balancer`
    4. select region -> and select load balancer url from drop down 