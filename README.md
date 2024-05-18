# Smart-Home-DDNS

# Setup

## Docker Compose

## Environment Variables

### Required

| Environment Variable Name | Purpose |
|---|---|
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