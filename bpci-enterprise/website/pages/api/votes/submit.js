// Vote Submission API - Submit a new vote
import { Pool } from 'pg';

// PostgreSQL connection
const pool = new Pool({
  user: process.env.DB_USER || 'postgres',
  host: process.env.DB_HOST || 'localhost',
  database: process.env.DB_NAME || 'pravyom',
  password: process.env.DB_PASSWORD,
  port: process.env.DB_PORT || 5432,
});

// Initialize database table if it doesn't exist
const initDB = async () => {
  const createTableQuery = `
    CREATE TABLE IF NOT EXISTS community_votes (
      id SERIAL PRIMARY KEY,
      email VARCHAR(255) NOT NULL,
      rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
      comment TEXT,
      timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      ip_address VARCHAR(45),
      user_agent TEXT
    );

    CREATE INDEX IF NOT EXISTS idx_votes_email ON community_votes(email);
    CREATE INDEX IF NOT EXISTS idx_votes_timestamp ON community_votes(timestamp);
  `;

  try {
    await pool.query(createTableQuery);
  } catch (error) {
    console.error('Error initializing database:', error);
  }
};

export default async function handler(req, res) {
  if (req.method !== 'POST') {
    return res.status(405).json({ message: 'Method not allowed' });
  }

  const { email, rating, comment } = req.body;

  // Validation
  if (!email || !email.includes('@')) {
    return res.status(400).json({ error: 'Valid email is required' });
  }

  if (!rating || rating < 1 || rating > 5) {
    return res.status(400).json({ error: 'Rating must be between 1 and 5' });
  }

  try {
    // Initialize DB table if needed
    await initDB();

    // Check if email already voted (optional - remove if you want multiple votes per email)
    const existingVote = await pool.query(
      'SELECT id FROM community_votes WHERE email = $1',
      [email]
    );

    if (existingVote.rows.length > 0) {
      return res.status(400).json({ error: 'This email has already voted' });
    }

    // Insert vote
    const ipAddress = req.headers['x-forwarded-for'] || req.socket.remoteAddress || 'unknown';
    const userAgent = req.headers['user-agent'] || 'unknown';

    await pool.query(
      'INSERT INTO community_votes (email, rating, comment, ip_address, user_agent) VALUES ($1, $2, $3, $4, $5)',
      [email, rating, comment || null, ipAddress, userAgent]
    );

    // Get updated stats
    const statsResult = await pool.query(`
      SELECT 
        COUNT(*) as total,
        COALESCE(AVG(rating), 0) as avg_rating
      FROM community_votes
    `);

    const stats = {
      total: parseInt(statsResult.rows[0].total),
      avgRating: parseFloat(statsResult.rows[0].avg_rating)
    };

    res.status(200).json({
      success: true,
      message: 'Vote submitted successfully',
      stats
    });

  } catch (error) {
    console.error('Error submitting vote:', error);
    res.status(500).json({ error: 'Failed to submit vote' });
  }
}
