# Webuniverse

## Einrichtung
### Web
Archiv `wu-app.tar.xz` in `/var/www/html/web/` hochladen

`cd /var/www/html/web/`<br>

`cp config.js ../._restore_config.js && tar xfJ wu-app.tar.xz && rm wu-app.tar.xz config.js && mv ../._restore_config.js config.js`

### API
`wu-api` in `/home/user/` hochladen

`cd /home/user/`

`chmod +x wu-api`

`screen -dmS wu-api ./wu-api --api-key MfyiWrCfCncxBabm2M1eJKWxUzbaSXl6 --mysql-db DATENBANK --mysql-user BENUTZER --mysql-pass PASSWORT`

### Stats
`wu-client` in `/home/user/` hochladen

`cd /home/user/`

`chmod +x wu-client`

`screen -dmS wu-stats ./wu-client send-stats --name "Dedicated Server" --api-key MfyiWrCfCncxBabm2M1eJKWxUzbaSXl6`

### MC Server
*!Beispiel: Lobby!*

`wu-client` in `/home/user/` hochladen

`cd /home/user/`

`chmod +x wu-client`

`(cd Server/lobby && screen -dmS wu-lobby ../../wu-client add-server --name "Lobby" --api-key MfyiWrCfCncxBabm2M1eJKWxUzbaSXl6 bash ./startsrv.sh)`
