export interface BlogPost {
  id: string;
  title: string;
  content: string;
  type: 'experience' | 'documentation' | 'bug_report' | 'general';
  author: {
    walletAddress: string;
    name: string;
    avatar?: string;
  };
  tags: string[];
  createdAt: string;
  updatedAt: string;
  likes: number;
  comments: Comment[];
  isPublic: boolean;
  autoPosted: boolean;
}

export interface Comment {
  id: string;
  postId: string;
  content: string;
  author: {
    walletAddress: string;
    name: string;
    avatar?: string;
  };
  createdAt: string;
  likes: number;
  replies: Comment[];
}

export interface CreatePostRequest {
  title: string;
  content: string;
  type: 'experience' | 'documentation' | 'bug_report' | 'general';
  tags: string[];
  isPublic: boolean;
  autoPost: boolean;
}

export interface CreateCommentRequest {
  postId: string;
  content: string;
  parentCommentId?: string;
}

export interface BlogFilters {
  type?: 'experience' | 'documentation' | 'bug_report' | 'general';
  author?: string;
  tags?: string[];
  dateRange?: {
    start: string;
    end: string;
  };
  sortBy: 'newest' | 'oldest' | 'most_liked' | 'most_commented';
}

export interface Message {
  id: string;
  fromWallet: string;
  toWallet: string;
  content: string;
  createdAt: string;
  read: boolean;
  type: 'direct' | 'post_mention' | 'comment_reply';
  relatedPostId?: string;
  relatedCommentId?: string;
}
