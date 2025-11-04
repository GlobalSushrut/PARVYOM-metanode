// Blog Like API - Like a post
import { Pool } from 'pg';

const pool = new Pool({
  user: process.env.DB_USER || 'postgres',
  host: process.env.DB_HOST || 'localhost',
  database: process.env.DB_NAME || 'pravyom',
  password: process.env.DB_PASSWORD,
  port: process.env.DB_PORT || 5432,
});

export default async function handler(req, res) {
  if (req.method !== 'POST') {
    return res.status(405).json({ message: 'Method not allowed' });
  }

  const { postId } = req.body;

  if (!postId) {
    return res.status(400).json({ error: 'Post ID is required' });
  }

  try {
    const result = await pool.query(
      'UPDATE blog_posts SET likes = likes + 1 WHERE id = $1 RETURNING likes',
      [postId]
    );

    if (result.rows.length === 0) {
      return res.status(404).json({ error: 'Post not found' });
    }

    res.status(200).json({ likes: result.rows[0].likes });
  } catch (error) {
    console.error('Error liking post:', error);
    res.status(500).json({ error: 'Failed to like post' });
  }
}
