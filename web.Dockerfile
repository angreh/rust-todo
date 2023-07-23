FROM node:lts
WORKDIR /var/www
COPY web .
RUN npm install -g vite
CMD ["npm", "run", "dev"]
