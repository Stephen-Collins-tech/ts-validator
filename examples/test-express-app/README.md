# Test Express App

A representative Express.js TypeScript application with proper routing, middleware, and validation.

## Features

- TypeScript support
- Express.js for routing and middlewares
- Zod for request validation
- Proper route organization with Express routers
- Error handling middleware
- Authentication middleware

## Project Structure

```
src/
├── config/            # App configuration
├── controllers/       # Route controllers (if used)
├── middleware/        # Express middlewares
│   ├── authMiddleware.ts   # Authentication middleware
│   └── errorMiddleware.ts  # Error handling middleware
├── routes/            # Route definitions
│   ├── userRoutes.ts  # User routes
│   └── postRoutes.ts  # Post routes
├── types.ts           # TypeScript type definitions
└── index.ts           # App entry point
```

## Environment Variables

Create a `.env` file in the root directory with the following variables:

```
# Server configuration
NODE_ENV=development
PORT=3000

# Security
API_KEY=your-api-key-here
JWT_SECRET=your-jwt-secret-key-here

# Logging
LOG_LEVEL=info
```

## Installation

```bash
npm install
```

## Running the App

Development mode:
```bash
npm run dev
```

Production mode:
```bash
npm run build
npm start
```

## API Endpoints

### Users
- `GET /api/users` - Get all users
- `GET /api/users/:id` - Get a single user
- `POST /api/users` - Create a new user
- `PUT /api/users/:id` - Update a user
- `DELETE /api/users/:id` - Delete a user

### Posts
- `GET /api/posts` - Get all posts (requires API key)
- `GET /api/posts/:id` - Get a single post (requires API key)
- `POST /api/posts` - Create a new post (requires API key)
- `PUT /api/posts/:id` - Update a post (requires API key)
- `DELETE /api/posts/:id` - Delete a post (requires API key)

## Authentication

For post routes, include an API key in your request headers:
```
X-API-Key: your-api-key-here
``` 