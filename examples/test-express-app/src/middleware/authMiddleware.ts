import { Request, Response, NextFunction } from 'express';
import config from '../config';

// This is a simple auth middleware for demonstration
// In a real app, you'd use JWT tokens or sessions

export const apiKeyAuth = (req: Request, res: Response, next: NextFunction) => {
  const apiKey = req.headers['x-api-key'];
  
  // Use API key from config
  const validApiKey = config.api.key;
  
  if (!apiKey || apiKey !== validApiKey) {
    return res.status(401).json({
      data: null,
      message: 'Unauthorized: Invalid or missing API key'
    });
  }
  
  next();
};

// For routes that need user authentication
export const requireAuth = (req: Request, res: Response, next: NextFunction) => {
  // In a real app, this would validate JWT tokens or session cookies
  const authHeader = req.headers.authorization;
  
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return res.status(401).json({
      data: null,
      message: 'Unauthorized: Authentication required'
    });
  }
  
  const token = authHeader.split(' ')[1];
  
  // Demo token validation (in a real app use JWT verify with config.jwt.secret)
  if (token !== 'demo-token-1234') {
    return res.status(401).json({
      data: null,
      message: 'Unauthorized: Invalid token'
    });
  }
  
  // Attach user to request (in a real app, decode JWT payload)
  (req as any).user = { id: 1, role: 'admin' };
  
  next();
}; 