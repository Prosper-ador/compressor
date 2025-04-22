# Use an official Node.js image
FROM node:20-slim

# Set working directory
WORKDIR /app

# Copy package.json and package-lock.json or yarn.lock
COPY package*.json ./

# Install dependencies (including devDependencies for tests & benchmarks)
RUN npm install

# Copy all source code
COPY . .

# Optional: Run tests
# RUN npm test

# Optional: Run benchmarks (comment this out in production)
# RUN npm run bench

# Set default command
CMD ["node", "index.js"]
