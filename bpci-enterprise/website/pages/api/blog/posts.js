// Blog Posts API - Get all posts or create new post
import { Pool } from 'pg';

const pool = new Pool({
  user: process.env.DB_USER || 'postgres',
  host: process.env.DB_HOST || 'localhost',
  database: process.env.DB_NAME || 'pravyom',
  password: process.env.DB_PASSWORD,
  port: process.env.DB_PORT || 5432,
});

// Initialize database table
const initDB = async () => {
  const createTableQuery = `
    CREATE TABLE IF NOT EXISTS blog_posts (
      id SERIAL PRIMARY KEY,
      title VARCHAR(500) NOT NULL,
      content TEXT NOT NULL,
      author_name VARCHAR(255) NOT NULL,
      author_email VARCHAR(255) NOT NULL,
      wallet_address VARCHAR(255),
      image_url TEXT,
      tags TEXT[],
      likes INTEGER DEFAULT 0,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    CREATE INDEX IF NOT EXISTS idx_blog_created ON blog_posts(created_at DESC);
    CREATE INDEX IF NOT EXISTS idx_blog_author ON blog_posts(author_email);
    CREATE INDEX IF NOT EXISTS idx_blog_wallet ON blog_posts(wallet_address);
  `;

  try {
    await pool.query(createTableQuery);
  } catch (error) {
    console.error('Error initializing blog database:', error);
  }
};

export default async function handler(req, res) {
  await initDB();

  if (req.method === 'GET') {
    // Get all posts
    try {
      const result = await pool.query(`
        SELECT 
          id,
          title,
          content,
          author_name,
          author_email,
          image_url,
          tags,
          likes,
          created_at,
          updated_at
        FROM blog_posts
        ORDER BY created_at DESC
        LIMIT 50
      `);

      res.status(200).json(result.rows);
    } catch (error) {
      console.error('Error fetching posts:', error);
      res.status(500).json({ error: 'Failed to fetch posts' });
    }
  } else if (req.method === 'POST') {
    // Create new post
    const { title, content, author_name, author_email, wallet_address, image_url, tags } = req.body;

    // Validation
    if (!title || !content || !author_name || !author_email) {
      return res.status(400).json({ error: 'Title, content, author name, and email are required' });
    }

    if (!author_email.includes('@')) {
      return res.status(400).json({ error: 'Valid email is required' });
    }

    // Check for authorization header (optional - can add strict auth check here)
    const authHeader = req.headers.authorization;
    if (!authHeader) {
      return res.status(401).json({ error: 'Authentication required. Please login with your Mojo wallet.' });
    }

    try {
      const result = await pool.query(
        `INSERT INTO blog_posts (title, content, author_name, author_email, wallet_address, image_url, tags) 
         VALUES ($1, $2, $3, $4, $5, $6, $7) 
         RETURNING *`,
        [title, content, author_name, author_email, wallet_address || null, image_url || null, tags || []]
      );

      res.status(201).json(result.rows[0]);
    } catch (error) {
      console.error('Error creating post:', error);
      res.status(500).json({ error: 'Failed to create post' });
    }
  } else {
    res.status(405).json({ message: 'Method not allowed' });
  }
}
