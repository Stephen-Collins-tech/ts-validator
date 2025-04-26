export interface User {
  id: number;
  name: string;
  email: string;
}

export interface Post {
  id: number;
  userId: number;
  title: string;
  content: string;
  createdAt: Date;
}

export interface ApiResponse<T> {
  data: T;
  message?: string;
} 