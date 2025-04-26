import { z } from 'zod';

// Define environment variables schema with validation
const envSchema = z.object({
  NODE_ENV: z.enum(['development', 'test', 'production']).default('development'),
  PORT: z.string().default('3000'),
  API_KEY: z.string().default('test-api-key-1234'),
  JWT_SECRET: z.string().default('your-jwt-secret-key-for-development-only'),
  LOG_LEVEL: z.enum(['error', 'warn', 'info', 'http', 'debug']).default('info'),
});

// Try to parse environment variables, with fallback defaults
const envVars = envSchema.parse(process.env);

// Create and export config object
const config = {
  env: envVars.NODE_ENV,
  port: parseInt(envVars.PORT, 10),
  isProduction: envVars.NODE_ENV === 'production',
  isDevelopment: envVars.NODE_ENV === 'development',
  isTest: envVars.NODE_ENV === 'test',
  api: {
    key: envVars.API_KEY,
  },
  jwt: {
    secret: envVars.JWT_SECRET,
    expiresIn: '1d',
  },
  logging: {
    level: envVars.LOG_LEVEL,
  },
};

export default config; 