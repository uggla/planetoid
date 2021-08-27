FROM nginx
LABEL org.opencontainers.image.authors="Uggla@free.fr"
COPY planetoid.wasm /usr/share/nginx/html
COPY index.html /usr/share/nginx/html
COPY sounds /usr/share/nginx/html/sounds
RUN sed -i '/application\/zip/a \    application/wasm                                 wasm;' /etc/nginx/mime.types

