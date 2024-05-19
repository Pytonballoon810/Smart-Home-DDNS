# Smart-Home-DDNS

A rust DDNS (DynDNS) implementation for cloudflare using Docker compose.

# Setup

## AVM FritzBox
Setup the dynDNS settings under `Internet` -> `Permit Access` -> `DynDNS`  
Your `Update URL` shoudl look something like this:  
`http://192.168.x.x:12080/?username=<username>&pwd=<pass>&domain=<domain>&ip=<ipaddr>`  

Under `Domain name`, `Username` and `Password` enter some stuff. It really doesn't matter what you enter here but at least for the `Domain name` I would recommend using the full name of the A-Record on Cloudflare you want to update. All other parameters are just for authentication.

## Docker Compose
Example:
```yaml
version: "3"
services:
  dyndns:
    image: pytonballoon810/smart-home-ddns
    container_name: dyndns
    environment:
      - PUID=1000
      - PGID=1000
      - USERNAME=username
      - PASSWD=password123
      - DOMAIN=home.example.com
      - ZONE_ID=123445565432454
      - API_KEY=jkfghjklkgfgkj
      # - PROXIED=true \Optional
      # - A_RECORD_NAME=example.com \Optional
      # - PORT=8080 \Optional
    ports:
      - 12080:12080
    restart: unless-stopped
```
## Environment Variables

### Required

| Environment Variable Name | Purpose |
|---|---|
|`PUID`|The PUID (user id) of your system with which you want to run the container with. It is strongly recommended to create a custom user just for this container (as well as for all other containers)|
|`PGID`|The PGID (group id) of your system with which you want to run the container with. It is strongly recommended to create a custom user just for this container (as well as for all other containers)|
|`USERNAME`|Must be the same as in the FritzBox configuration. Does not actually do anything other than authentication|
|`PASSWD`|Must be the same as in the FritzBox configuration. Does not actually do anything other than authentication|
|`DOMAIN`|Must be the same as in the FritzBox configuration. Does not actually do anything other than authentication|
|`ZONE_ID`|Can be viewed in the `Overview` section in the Cloudflare dashboard. Just copy and paste it here.|
|`API_KEY`|Must be a valid API key generated in Cloudflare under <img src="https://raw.githubusercontent.com/FortAwesome/Font-Awesome/6.x/svgs/regular/user.svg" width="20" height="20"> -> `My Profile` -> `API Tokens` -> `Create Token`. You can use the DNS Zone Blueprint or manually enable the `Edit DNS Zone` privileges|


### Optional
| Environment Variable Name | Purpose | Default Value |
|---|---|---|
|`Proxied`|Wether or not the DNS record should be proxied over the Cloudflare servers | `false`|
|`A_RECORD_NAME`|Should be the full name of the A-Name-Record you want to update on Cloudflare i.e.: `home.example.com` or just `example.com`. Use this if the A-Record differs from the domain specified in the FritzBox for whatever reason|The value from `DOMAIN`|
|`PORT`|The port on which the docker container should serve the webpage|`12080`|