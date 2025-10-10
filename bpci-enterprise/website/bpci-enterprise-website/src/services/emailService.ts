// Email Service for Contact Form
// This service handles sending contact form emails

export interface ContactFormData {
  name: string;
  email: string;
  company?: string;
  phone?: string;
  subject: string;
  category: string;
  message: string;
}

export interface EmailResponse {
  success: boolean;
  message: string;
  error?: string;
}

// Email service configuration
const EMAIL_CONFIG = {
  // TODO: Replace with actual email service endpoint
  apiEndpoint: '/api/contact/send',
  // TODO: Add your email service API key
  apiKey: import.meta.env.VITE_EMAIL_API_KEY || '',
  // Default recipient email (can be configured)
  defaultRecipient: 'contact@parvyom.com',
  // Email templates by category
  categoryEmails: {
    enterprise: 'enterprise@parvyom.com',
    research: 'research@parvyom.com',
    community: 'community@parvyom.com',
    pilot: 'pilot@parvyom.com',
    technical: 'support@parvyom.com',
    general: 'contact@parvyom.com'
  }
};

/**
 * Send contact form email
 * @param formData - Contact form data
 * @returns Promise with email response
 */
export const sendContactEmail = async (formData: ContactFormData): Promise<EmailResponse> => {
  try {
    // Get the appropriate recipient email based on category
    const recipientEmail = EMAIL_CONFIG.categoryEmails[formData.category as keyof typeof EMAIL_CONFIG.categoryEmails] 
      || EMAIL_CONFIG.defaultRecipient;

    // Prepare email payload
    const emailPayload = {
      to: recipientEmail,
      from: 'noreply@parvyom.com',
      replyTo: formData.email,
      subject: `[PARVYOM ${formData.category.toUpperCase()}] ${formData.subject}`,
      html: generateEmailHTML(formData),
      text: generateEmailText(formData)
    };

    // TODO: Replace this with actual email service call
    // For now, we'll simulate the email sending process
    console.log('Email payload:', emailPayload);
    
    // Simulate API call delay
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    // For development, always return success
    // In production, replace this with actual email service call:
    /*
    const response = await fetch(EMAIL_CONFIG.apiEndpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${EMAIL_CONFIG.apiKey}`
      },
      body: JSON.stringify(emailPayload)
    });

    if (!response.ok) {
      throw new Error(`Email service error: ${response.statusText}`);
    }

    const result = await response.json();
    */

    return {
      success: true,
      message: 'Email sent successfully'
    };

  } catch (error) {
    console.error('Email sending error:', error);
    return {
      success: false,
      message: 'Failed to send email',
      error: error instanceof Error ? error.message : 'Unknown error'
    };
  }
};

/**
 * Generate HTML email template
 */
function generateEmailHTML(formData: ContactFormData): string {
  return `
    <!DOCTYPE html>
    <html>
    <head>
      <meta charset="utf-8">
      <title>New Contact Form Submission - Parvyom</title>
      <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%); color: white; padding: 20px; border-radius: 8px; text-align: center; }
        .content { background: #f8fafc; padding: 30px; border-radius: 8px; margin: 20px 0; }
        .field { margin-bottom: 15px; }
        .label { font-weight: bold; color: #4a5568; }
        .value { margin-top: 5px; padding: 10px; background: white; border-radius: 4px; border-left: 4px solid #3b82f6; }
        .message { background: white; padding: 20px; border-radius: 4px; border-left: 4px solid #10b981; }
        .footer { text-align: center; color: #718096; font-size: 14px; margin-top: 30px; }
      </style>
    </head>
    <body>
      <div class="container">
        <div class="header">
          <h1>🚀 New Contact Form Submission</h1>
          <p>Parvyom BPI/BPCI Enterprise Infrastructure</p>
        </div>
        
        <div class="content">
          <div class="field">
            <div class="label">📋 Category:</div>
            <div class="value">${formData.category.charAt(0).toUpperCase() + formData.category.slice(1)}</div>
          </div>
          
          <div class="field">
            <div class="label">👤 Name:</div>
            <div class="value">${formData.name}</div>
          </div>
          
          <div class="field">
            <div class="label">📧 Email:</div>
            <div class="value">${formData.email}</div>
          </div>
          
          ${formData.company ? `
          <div class="field">
            <div class="label">🏢 Company:</div>
            <div class="value">${formData.company}</div>
          </div>
          ` : ''}
          
          ${formData.phone ? `
          <div class="field">
            <div class="label">📞 Phone:</div>
            <div class="value">${formData.phone}</div>
          </div>
          ` : ''}
          
          <div class="field">
            <div class="label">📝 Subject:</div>
            <div class="value">${formData.subject}</div>
          </div>
          
          <div class="field">
            <div class="label">💬 Message:</div>
            <div class="message">${formData.message.replace(/\n/g, '<br>')}</div>
          </div>
        </div>
        
        <div class="footer">
          <p>This message was sent via the Parvyom Contact Form</p>
          <p>Timestamp: ${new Date().toLocaleString()}</p>
        </div>
      </div>
    </body>
    </html>
  `;
}

/**
 * Generate plain text email
 */
function generateEmailText(formData: ContactFormData): string {
  return `
NEW CONTACT FORM SUBMISSION - PARVYOM

Category: ${formData.category.charAt(0).toUpperCase() + formData.category.slice(1)}
Name: ${formData.name}
Email: ${formData.email}
${formData.company ? `Company: ${formData.company}` : ''}
${formData.phone ? `Phone: ${formData.phone}` : ''}
Subject: ${formData.subject}

Message:
${formData.message}

---
This message was sent via the Parvyom Contact Form
Timestamp: ${new Date().toLocaleString()}
  `.trim();
}

/**
 * Validate email address format
 */
export const isValidEmail = (email: string): boolean => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
};

/**
 * Sanitize form input to prevent XSS
 */
export const sanitizeInput = (input: string): string => {
  return input
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#x27;')
    .replace(/\//g, '&#x2F;');
};

// Export email configuration for external use
export { EMAIL_CONFIG };
