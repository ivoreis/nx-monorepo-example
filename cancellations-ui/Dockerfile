# Stage 1: Build the React app
FROM node:18 AS builder

# Declare build arguments
ARG SERVER_NAME
ARG SERVER_PORT
ARG API_URL
ARG API_PROXY_URL

# Set environment variables using the build arguments
ENV SERVER_NAME=${SERVER_NAME}
ENV SERVER_PORT=${SERVER_PORT}
ENV API_URL=${API_URL}
ENV API_PROXY_URL=${API_PROXY_URL}
ENV NX_SOCKET_DIR=/tmp/nx-tmp

WORKDIR /workspace
COPY ../package.json ../pnpm-lock.yaml ../nx.json ../tsconfig.base.json ../eslint.config.js ./
COPY ../cancellations-ui ./cancellations-ui
RUN npm install -g pnpm
RUN pnpm install --frozen-lockfile
RUN SERVER_NAME=${SERVER_NAME} SERVER_PORT=${SERVER_PORT} ENABLE_SSL=false VITE_API_URL=${API_URL} pnpm nx run cancellations-ui:build

# Stage 2: Set up Amazon Linux with Apache to serve the static files
FROM amazonlinux:2

ARG SERVER_NAME
ENV SERVER_NAME=${SERVER_NAME}
ARG API_PROXY_URL
ENV API_PROXY_URL=${API_PROXY_URL}

WORKDIR /var/www/html
RUN yum install -y httpd && yum clean all

COPY --from=builder /workspace/dist/cancellations-ui /var/www/html
RUN echo "ServerName ${SERVER_NAME}" >> /etc/httpd/conf/httpd.conf && \
    echo '<Directory "/var/www/html">' >> /etc/httpd/conf/httpd.conf && \
    echo '    Options FollowSymLinks' >> /etc/httpd/conf/httpd.conf && \
    echo '    AllowOverride All' >> /etc/httpd/conf/httpd.conf && \
    echo '</Directory>' >> /etc/httpd/conf/httpd.conf && \
    echo '<IfModule mod_rewrite.c>' >> /etc/httpd/conf/httpd.conf && \
    echo '    RewriteEngine On' >> /etc/httpd/conf/httpd.conf && \
    echo '    RewriteCond %{REQUEST_URI} !^/api/ [NC]' >> /etc/httpd/conf/httpd.conf && \
    echo '    RewriteCond %{REQUEST_FILENAME} !-f' >> /etc/httpd/conf/httpd.conf && \
    echo '    RewriteCond %{REQUEST_FILENAME} !-d' >> /etc/httpd/conf/httpd.conf && \
    echo '    RewriteCond %{REQUEST_URI} !\.(css|js|png|jpg|jpeg|gif|svg|ico|woff|woff2|ttf|otf|eot|json|txt|pdf)$ [NC]' >> /etc/httpd/conf/httpd.conf && \
    echo '    RewriteRule ^(.*)$ /index.html [L]' >> /etc/httpd/conf/httpd.conf && \
    echo '</IfModule>' >> /etc/httpd/conf/httpd.conf && \ 
    echo '<Location /api>' >> /etc/httpd/conf/httpd.conf && \
    echo "    ProxyPass ${API_PROXY_URL}" >> /etc/httpd/conf/httpd.conf && \
    echo "    ProxyPassReverse ${API_PROXY_URL}" >> /etc/httpd/conf/httpd.conf && \
    echo '</Location>' >> /etc/httpd/conf/httpd.conf
EXPOSE 80

CMD ["httpd", "-D", "FOREGROUND"]
