get the pem file

```bash 
ssh -i <PEM_FILE> admin@<IP> 

su -i




```

make sure the DNS entries are pointing to the IP address

```
apt install certbot python3-certbot-nginx

sudo certbot --nginx -d johanyim.com -d www.johanyim.com



```
