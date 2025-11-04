/**
 * Vote Service
 * Handles all voting-related API calls
 */

// Use relative path for Next.js API routes
const API_BASE_URL = '';

export interface Vote {
  email: string;
  rating: number;
  comment?: string;
  timestamp: string;
}

export interface VoteStats {
  total: number;
  avgRating: number;
}

/**
 * Get current vote statistics
 */
export const getVoteStats = async (): Promise<VoteStats> => {
  try {
    const response = await fetch(`${API_BASE_URL}/api/votes/stats`);
    if (!response.ok) {
      throw new Error('Failed to fetch vote stats');
    }
    return await response.json();
  } catch (error) {
    console.error('Error fetching vote stats:', error);
    // Return default stats if API fails
    return { total: 0, avgRating: 0 };
  }
};

/**
 * Submit a new vote
 */
export const submitVote = async (vote: Vote): Promise<{ success: boolean; stats: VoteStats }> => {
  try {
    const response = await fetch(`${API_BASE_URL}/api/votes/submit`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(vote),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.error || 'Failed to submit vote');
    }

    return await response.json();
  } catch (error) {
    console.error('Error submitting vote:', error);
    throw error;
  }
};

/**
 * Get all votes (admin only - for review)
 */
export const getAllVotes = async (): Promise<Vote[]> => {
  try {
    const response = await fetch(`${API_BASE_URL}/api/votes/all`, {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('token')}`, // Add auth token
      },
    });

    if (!response.ok) {
      throw new Error('Failed to fetch votes');
    }

    return await response.json();
  } catch (error) {
    console.error('Error fetching all votes:', error);
    throw error;
  }
};
