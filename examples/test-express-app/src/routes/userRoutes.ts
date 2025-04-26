import express, { Request, Response } from 'express';
import { z } from 'zod';
import { User, ApiResponse } from '../types';

const router = express.Router();

// Zod schemas
const userSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  email: z.string().email('Invalid email format'),
});

const userIdSchema = z.object({
  id: z.string().transform((val) => parseInt(val, 10)),
});

// Mock database
let users: User[] = [
  { id: 1, name: 'Alice', email: 'alice@example.com' },
  { id: 2, name: 'Bob', email: 'bob@example.com' },
];
let nextUserId = 3;

// Get all users
router.get('/', (req: Request, res: Response<ApiResponse<User[]>>) => {
  res.json({ data: users, message: 'Users retrieved successfully' });
});

// Get single user - Using safeParse
router.get('/:id', (req: Request, res: Response<ApiResponse<User | null>>) => {
  // ✅ Using safeParse instead of parse
  const result = userIdSchema.safeParse({ id: req.params.id });
  
  if (!result.success) {
    return res.status(400).json({ data: null, message: 'Invalid user ID' });
  }
  
  const { id } = result.data;
  const user = users.find(u => u.id === id);
  
  if (!user) {
    return res.status(404).json({ data: null, message: 'User not found' });
  }
  
  res.json({ data: user, message: 'User retrieved successfully' });
});

// Create user - Still using parse for strict validation
router.post('/', (req: Request<{}, {}, Omit<User, 'id'>>, res: Response<ApiResponse<User | null>>) => {
  try {
    // ✅ Using parse for strict validation
    const validatedData = userSchema.parse(req.body);
    const newUser: User = {
      id: nextUserId++,
      ...validatedData,
    };
    users.push(newUser);
    res.status(201).json({ data: newUser, message: 'User created successfully' });
  } catch (error) {
    res.status(400).json({ data: null, message: 'Invalid user data' });
  }
});

// Update user - NO VALIDATION AT ALL
router.put('/:id', (req: Request, res: Response<ApiResponse<User | null>>) => {
  // ❌ No validation on ID parameter
  const id = parseInt(req.params.id, 10);
  
  // ❌ No validation on request body
  const { name, email } = req.body;
  
  const userIndex = users.findIndex(u => u.id === id);
  if (userIndex === -1) {
    return res.status(404).json({ data: null, message: 'User not found' });
  }
  
  const updatedUser = {
    id,
    name,
    email
  };
  
  users[userIndex] = updatedUser;
  res.json({ data: updatedUser, message: 'User updated successfully' });
});

// Delete user - Using safeParse for ID only
router.delete('/:id', (req: Request, res: Response<ApiResponse<null>>) => {
  // ✅ Using safeParse for partial validation
  const result = userIdSchema.safeParse({ id: req.params.id });
  
  if (!result.success) {
    return res.status(400).json({ data: null, message: 'Invalid user ID' });
  }
  
  const { id } = result.data;
  const userIndex = users.findIndex(u => u.id === id);
  
  if (userIndex === -1) {
    return res.status(404).json({ data: null, message: 'User not found' });
  }
  
  users.splice(userIndex, 1);
  res.json({ data: null, message: 'User deleted successfully' });
});

export default router; 