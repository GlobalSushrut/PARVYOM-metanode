import type { BlogPost, Comment, CreatePostRequest, CreateCommentRequest, BlogFilters, Message } from '../types/blog';

const API_BASE_URL = 'http://localhost:8080/api';

class BlogService {
  private getAuthHeaders(): HeadersInit {
    const token = localStorage.getItem('bpci_auth_token');
    return {
      'Content-Type': 'application/json',
      'Authorization': token ? `Bearer ${token}` : '',
    };
  }

  // Blog Posts
  async createPost(postData: CreatePostRequest): Promise<BlogPost> {
    const response = await fetch(`${API_BASE_URL}/blog/posts`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(postData),
    });

    if (!response.ok) {
      throw new Error('Failed to create post');
    }

    return response.json();
  }

  async getPosts(filters?: BlogFilters): Promise<BlogPost[]> {
    const queryParams = new URLSearchParams();
    
    if (filters) {
      if (filters.type) queryParams.append('type', filters.type);
      if (filters.author) queryParams.append('author', filters.author);
      if (filters.tags) queryParams.append('tags', filters.tags.join(','));
      if (filters.dateRange) {
        queryParams.append('start_date', filters.dateRange.start);
        queryParams.append('end_date', filters.dateRange.end);
      }
      queryParams.append('sort_by', filters.sortBy);
    }

    const response = await fetch(`${API_BASE_URL}/blog/posts?${queryParams}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch posts');
    }

    return response.json();
  }

  async getPost(postId: string): Promise<BlogPost> {
    const response = await fetch(`${API_BASE_URL}/blog/posts/${postId}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch post');
    }

    return response.json();
  }

  async updatePost(postId: string, updates: Partial<CreatePostRequest>): Promise<BlogPost> {
    const response = await fetch(`${API_BASE_URL}/blog/posts/${postId}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(updates),
    });

    if (!response.ok) {
      throw new Error('Failed to update post');
    }

    return response.json();
  }

  async deletePost(postId: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/blog/posts/${postId}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete post');
    }
  }

  async likePost(postId: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/blog/posts/${postId}/like`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to like post');
    }
  }

  // Comments
  async createComment(commentData: CreateCommentRequest): Promise<Comment> {
    const response = await fetch(`${API_BASE_URL}/blog/comments`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(commentData),
    });

    if (!response.ok) {
      throw new Error('Failed to create comment');
    }

    return response.json();
  }

  async getComments(postId: string): Promise<Comment[]> {
    const response = await fetch(`${API_BASE_URL}/blog/posts/${postId}/comments`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch comments');
    }

    return response.json();
  }

  async likeComment(commentId: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/blog/comments/${commentId}/like`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to like comment');
    }
  }

  async deleteComment(commentId: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/blog/comments/${commentId}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete comment');
    }
  }

  // Messaging
  async sendMessage(toWallet: string, content: string, type: 'direct' | 'post_mention' | 'comment_reply' = 'direct', relatedPostId?: string, relatedCommentId?: string): Promise<Message> {
    const response = await fetch(`${API_BASE_URL}/blog/messages`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify({
        toWallet,
        content,
        type,
        relatedPostId,
        relatedCommentId,
      }),
    });

    if (!response.ok) {
      throw new Error('Failed to send message');
    }

    return response.json();
  }

  async getMessages(): Promise<Message[]> {
    const response = await fetch(`${API_BASE_URL}/blog/messages`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch messages');
    }

    return response.json();
  }

  async markMessageAsRead(messageId: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/blog/messages/${messageId}/read`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to mark message as read');
    }
  }

  // Auto-posting functionality
  async enableAutoPost(postId: string, platforms: string[]): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/blog/posts/${postId}/auto-post`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify({ platforms }),
    });

    if (!response.ok) {
      throw new Error('Failed to enable auto-posting');
    }
  }

  // Search and discovery
  async searchPosts(query: string): Promise<BlogPost[]> {
    const response = await fetch(`${API_BASE_URL}/blog/search?q=${encodeURIComponent(query)}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to search posts');
    }

    return response.json();
  }

  async getTrendingTags(): Promise<string[]> {
    const response = await fetch(`${API_BASE_URL}/blog/trending-tags`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch trending tags');
    }

    return response.json();
  }

  // User profile and activity
  async getUserPosts(walletAddress: string): Promise<BlogPost[]> {
    const response = await fetch(`${API_BASE_URL}/blog/users/${walletAddress}/posts`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch user posts');
    }

    return response.json();
  }

  async getUserActivity(walletAddress: string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/blog/users/${walletAddress}/activity`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch user activity');
    }

    return response.json();
  }
}

export const blogService = new BlogService();
