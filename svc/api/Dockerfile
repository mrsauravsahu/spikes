FROM node:14.16.1-alpine as build

WORKDIR /app

COPY package*.json ./

RUN npm i

COPY . .

RUN npm run build

FROM node:14.16.1-alpine

WORKDIR /app

COPY --from=build /app/package*.json ./

RUN npm i --only=production

COPY --from=build /app/dist .

EXPOSE 80

CMD node ./main.js