// Vote Statistics API - Get current vote stats
import { Pool } from 'pg';

// PostgreSQL connection
const pool = new Pool({
  user: process.env.DB_USER || 'postgres',
  host: process.env.DB_HOST || 'localhost',
  database: process.env.DB_NAME || 'pravyom',
  password: process.env.DB_PASSWORD,
  port: process.env.DB_PORT || 5432,
});

export default async function handler(req, res) {
  if (req.method !== 'GET') {
    return res.status(405).json({ message: 'Method not allowed' });
  }

  try {
    const result = await pool.query(`
      SELECT 
        COUNT(*) as total,
        COALESCE(AVG(rating), 0) as avg_rating
      FROM community_votes
    `);

    const stats = {
      total: parseInt(result.rows[0].total),
      avgRating: parseFloat(result.rows[0].avg_rating)
    };

    res.status(200).json(stats);
  } catch (error) {
    console.error('Error fetching vote stats:', error);
    
    // If table doesn't exist yet, return zero stats
    if (error.code === '42P01') {
      return res.status(200).json({ total: 0, avgRating: 0 });
    }
    
    res.status(500).json({ error: 'Failed to fetch vote statistics' });
  }
}
