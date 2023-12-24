FROM node:14.16.1-alpine as build

WORKDIR /app

COPY package*.json ./

RUN npm i

COPY . .

RUN npm run build

FROM node:14.16.1-alpine as runtime

WORKDIR /app

COPY static /app/static

COPY package*.json ./

RUN npm i --prod

COPY --from=build /app/__sapper__ ./__sapper__

ENV PORT=80

EXPOSE 80

ENTRYPOINT node __sapper__/build/index.js