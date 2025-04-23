import express, { Request, Response } from 'express';
import { z } from 'zod';
import { User, ApiResponse } from './types';

const app = express();
const port = 3000;

app.use(express.json());

let users: User[] = [
  { id: 1, name: 'Alice', email: 'alice@example.com' },
  { id: 2, name: 'Bob', email: 'bob@example.com' },
];
let nextUserId = 3;

// Zod schemas
const userSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  email: z.string().email('Invalid email format'),
});

const userIdSchema = z.object({
  id: z.string().transform((val) => parseInt(val, 10)),
});

// Get all users
app.get('/users', (req: Request, res: Response<ApiResponse<User[]>>) => {
  res.json({ data: users });
});

// Get user by ID
app.get('/users/:id', (req: Request<{ id: string }>, res: Response<ApiResponse<User | null>>) => {
  try {
    const { id } = userIdSchema.parse(req.params);
    const user = users.find(u => u.id === id);
    if (user) {
      res.json({ data: user });
    } else {
      res.status(404).json({ data: null, message: 'User not found' });
    }
  } catch (error) {
    if (error instanceof z.ZodError) {
      res.status(400).json({ data: null, message: error.errors[0].message });
    } else {
      res.status(500).json({ data: null, message: 'Internal server error' });
    }
  }
});

// Create user
app.post('/users', (req: Request<{}, {}, Omit<User, 'id'>>, res: Response<ApiResponse<User | null>>) => {
  try {
    const validatedData = userSchema.parse(req.body);
    const newUser: User = {
      id: nextUserId++,
      ...validatedData,
    };
    users.push(newUser);
    res.status(201).json({ data: newUser, message: 'User created successfully' });
  } catch (error) {
    if (error instanceof z.ZodError) {
      res.status(400).json({ data: null, message: error.errors[0].message });
    } else {
      res.status(500).json({ data: null, message: 'Internal server error' });
    }
  }
});

app.listen(port, () => {
  console.log(`Server listening at http://localhost:${port}`);
});