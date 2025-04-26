import express, { Request, Response } from 'express';
import { z } from 'zod';
import { Post, ApiResponse } from '../types';

const router = express.Router();

// Zod schemas
const postSchema = z.object({
  userId: z.number().int().positive(),
  title: z.string().min(1, 'Title is required'),
  content: z.string().min(1, 'Content is required'),
});

const postIdSchema = z.object({
  id: z.string().transform((val) => parseInt(val, 10)),
});

// Mock database
let posts: Post[] = [
  { id: 1, userId: 1, title: 'First post', content: 'This is the first post', createdAt: new Date() },
  { id: 2, userId: 2, title: 'Second post', content: 'This is the second post', createdAt: new Date() },
];
let nextPostId = 3;

// Get all posts - No validation for query params
router.get('/', (req: Request, res: Response<ApiResponse<Post[]>>) => {
  // ❌ No validation for userId query parameter
  const userId = req.query.userId ? parseInt(req.query.userId as string, 10) : undefined;
  
  if (userId) {
    const filteredPosts = posts.filter(post => post.userId === userId);
    return res.json({ data: filteredPosts, message: 'Posts retrieved successfully' });
  }
  
  res.json({ data: posts, message: 'Posts retrieved successfully' });
});

// Get single post - Using safeParse
router.get('/:id', (req: Request, res: Response<ApiResponse<Post | null>>) => {
  // ✅ Using safeParse instead of parse
  const result = postIdSchema.safeParse({ id: req.params.id });
  
  if (!result.success) {
    return res.status(400).json({ data: null, message: 'Invalid post ID' });
  }
  
  const { id } = result.data;
  const post = posts.find(p => p.id === id);
  
  if (!post) {
    return res.status(404).json({ data: null, message: 'Post not found' });
  }
  
  res.json({ data: post, message: 'Post retrieved successfully' });
});

// Create post - NO VALIDATION AT ALL
router.post('/', (req: Request, res: Response<ApiResponse<Post | null>>) => {
  // ❌ No validation on request body
  const { userId, title, content } = req.body;
  
  const newPost: Post = {
    id: nextPostId++,
    userId,
    title,
    content,
    createdAt: new Date()
  };
  
  posts.push(newPost);
  res.status(201).json({ data: newPost, message: 'Post created successfully' });
});

// Update post - Still using parse for strict validation
router.put('/:id', (req: Request, res: Response<ApiResponse<Post | null>>) => {
  try {
    // ✅ Using parse for strict validation
    const { id } = postIdSchema.parse({ id: req.params.id });
    const validatedData = postSchema.parse(req.body);
    
    const postIndex = posts.findIndex(p => p.id === id);
    if (postIndex === -1) {
      return res.status(404).json({ data: null, message: 'Post not found' });
    }
    
    const updatedPost = {
      id,
      ...validatedData,
      createdAt: posts[postIndex].createdAt
    };
    
    posts[postIndex] = updatedPost;
    res.json({ data: updatedPost, message: 'Post updated successfully' });
  } catch (error) {
    res.status(400).json({ data: null, message: 'Invalid post data' });
  }
});

// Delete post - NO VALIDATION AT ALL
router.delete('/:id', (req: Request, res: Response<ApiResponse<null>>) => {
  // ❌ No validation on ID parameter
  const id = parseInt(req.params.id, 10);
  const postIndex = posts.findIndex(p => p.id === id);
  
  if (postIndex === -1) {
    return res.status(404).json({ data: null, message: 'Post not found' });
  }
  
  posts.splice(postIndex, 1);
  res.json({ data: null, message: 'Post deleted successfully' });
});

export default router; 