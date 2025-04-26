import { Request, Response, NextFunction } from 'express';
import { ZodError } from 'zod';

// Custom error class
export class AppError extends Error {
  statusCode: number;
  
  constructor(message: string, statusCode: number) {
    super(message);
    this.statusCode = statusCode;
    this.name = this.constructor.name;
    Error.captureStackTrace(this, this.constructor);
  }
}

// Not found middleware
export const notFound = (req: Request, res: Response, next: NextFunction) => {
  const error = new AppError(`Not Found - ${req.originalUrl}`, 404);
  next(error);
};

// Error handler middleware
export const errorHandler = (err: any, req: Request, res: Response, next: NextFunction) => {
  // Log the error for server-side debugging
  console.error('Error:', err);
  
  // Handle Zod validation errors
  if (err instanceof ZodError) {
    return res.status(400).json({
      data: null,
      message: 'Validation error',
      errors: err.errors
    });
  }
  
  // Handle custom application errors
  if (err instanceof AppError) {
    return res.status(err.statusCode).json({
      data: null,
      message: err.message
    });
  }
  
  // Handle other errors
  const statusCode = res.statusCode !== 200 ? res.statusCode : 500;
  res.status(statusCode).json({
    data: null,
    message: err.message || 'Internal Server Error',
    stack: process.env.NODE_ENV === 'production' ? undefined : err.stack
  });
}; 