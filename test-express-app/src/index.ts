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

// ✅ Fully validated input
app.post('/users', (req: Request<{}, {}, Omit<User, 'id'>>, res: Response<ApiResponse<User | null>>) => {
  try {
    const validatedData = userSchema.parse(req.body); // ✅ Safe usage
    const newUser: User = {
      id: nextUserId++,
      ...validatedData,
    };
    users.push(newUser);
    res.status(201).json({ data: newUser, message: 'User created successfully' });
  } catch (error) {
    res.status(400).json({ data: null, message: 'Invalid user' });
  }
});

// ✅ Lenient validation (only valid under --rules zod-lenient)
app.post('/users-lenient', (req: Request, res: Response<ApiResponse<User | null>>) => {
  const result = userSchema.safeParse(req.body); // ✅ Lenient usage

  if (!result.success) {
    res.status(400).json({ data: null, message: 'Invalid user (lenient)' });
    return;
  }

  const newUser: User = {
    id: nextUserId++,
    ...result.data,
  };

  users.push(newUser);
  res.status(201).json({ data: newUser, message: 'User created (lenient)' });
});


// ❗️Unvalidated access: direct use of req.query
app.get('/raw-query', (req: Request, res: Response) => {
  console.log('Raw query:', req.query); // ❌ Unvalidated
  res.send('Logged query');
});

// ❗️Unvalidated access: alias
app.get('/alias-body', (req: Request, res: Response) => {
  const data = req.body; // ❌ alias to req.body
  console.log(data); // ❌ unvalidated usage
  res.send('Aliased body used');
});

// ❗️Unvalidated access: destructured
app.get('/destructured-body', (req: Request, res: Response) => {
  const { body } = req; // ❌ destructured alias
  console.log(body); // ❌ unvalidated usage
  res.send('Destructured body used');
});

// ❗️Unvalidated access: renamed destructured alias
app.get('/renamed-alias', (req: Request, res: Response) => {
  const { body: input } = req; // ❌ input → req.body
  console.log(input.email); // ❌ usage
  res.send('Renamed destructured alias used');
});
