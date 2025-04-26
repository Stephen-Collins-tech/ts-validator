import express from 'express';
import userRoutes from './routes/userRoutes';
import postRoutes from './routes/postRoutes';
import { apiKeyAuth } from './middleware/authMiddleware';
import { notFound, errorHandler } from './middleware/errorMiddleware';
import config from './config';

const app = express();
const { port } = config;

// Middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Basic request logger middleware
app.use((req, res, next) => {
  console.log(`${new Date().toISOString()} - ${req.method} ${req.path}`);
  next();
});

// Welcome route
app.get('/', (req, res) => {
  res.json({
    message: 'Welcome to the API',
    version: '1.0.0',
    environment: config.env,
    endpoints: {
      users: '/api/users',
      posts: '/api/posts'
    }
  });
});

// Routes
app.use('/api/users', userRoutes);

// Apply API key auth middleware only to posts routes
app.use('/api/posts', apiKeyAuth, postRoutes);

// Custom 404 handler
app.use(notFound);

// Global error handler
app.use(errorHandler);

// Start server
app.listen(port, () => {
  console.log(`Server running in ${config.env} mode on port ${port}`);
  console.log(`API Documentation available at http://localhost:${port}`);
});

export default app; // For testing purposes
