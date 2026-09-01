FROM node:20-alpine AS builder

WORKDIR /app
COPY package*.json ./
RUN npm ci || true
COPY . .
RUN npm run build || true

FROM node:20-alpine AS runner
WORKDIR /app
ENV NODE_ENV=production
COPY --from=builder /app ./

EXPOSE 3000
CMD ["npm", "run", "start"]
