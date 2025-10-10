# Email Service Integration Guide

## Overview
The `emailService.ts` provides a complete email sending solution for the Contact Us form. Currently configured for development with simulation, but ready for production email service integration.

## Current Status
- ✅ Contact form data validation and sanitization
- ✅ HTML and plain text email templates
- ✅ Category-based email routing
- ✅ Professional email formatting with Parvyom branding
- 🔄 **Simulated email sending** (development mode)
- ⏳ **Production email backend integration** (pending)

## Email Categories & Recipients
The service automatically routes emails to appropriate recipients based on inquiry category:

- **Enterprise**: `enterprise@parvyom.com` - Business partnerships, enterprise pilots
- **Research**: `research@parvyom.com` - Academic collaborations, research inquiries  
- **Community**: `community@parvyom.com` - Developer community, open source contributions
- **Pilot**: `pilot@parvyom.com` - Pilot program applications, testing partnerships
- **Technical**: `support@parvyom.com` - Technical support, implementation questions
- **General**: `contact@parvyom.com` - All other inquiries

## Production Integration Options

### Option 1: EmailJS (Recommended for Frontend-Only)
```bash
npm install @emailjs/browser
```

Update `emailService.ts`:
```typescript
import emailjs from '@emailjs/browser';

export const sendContactEmail = async (formData: ContactFormData): Promise<EmailResponse> => {
  try {
    const result = await emailjs.send(
      'YOUR_SERVICE_ID',
      'YOUR_TEMPLATE_ID',
      {
        to_email: EMAIL_CONFIG.categoryEmails[formData.category],
        from_name: formData.name,
        from_email: formData.email,
        subject: formData.subject,
        message: formData.message,
        // ... other fields
      },
      'YOUR_PUBLIC_KEY'
    );
    
    return { success: true, message: 'Email sent successfully' };
  } catch (error) {
    return { success: false, message: 'Failed to send email', error: error.text };
  }
};
```

### Option 2: Backend API Integration
Create a backend endpoint and update the service:

```typescript
export const sendContactEmail = async (formData: ContactFormData): Promise<EmailResponse> => {
  const response = await fetch('/api/contact/send', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${process.env.REACT_APP_API_KEY}`
    },
    body: JSON.stringify(formData)
  });

  if (!response.ok) {
    throw new Error(`API error: ${response.statusText}`);
  }

  return await response.json();
};
```

### Option 3: Serverless Functions (Netlify/Vercel)
Deploy serverless functions for email sending:

**Netlify Functions** (`netlify/functions/send-email.js`):
```javascript
const nodemailer = require('nodemailer');

exports.handler = async (event, context) => {
  if (event.httpMethod !== 'POST') {
    return { statusCode: 405, body: 'Method Not Allowed' };
  }

  const formData = JSON.parse(event.body);
  
  // Configure your email service (Gmail, SendGrid, etc.)
  const transporter = nodemailer.createTransporter({
    // Your email service configuration
  });

  try {
    await transporter.sendMail({
      from: 'noreply@parvyom.com',
      to: getRecipientEmail(formData.category),
      subject: `[PARVYOM] ${formData.subject}`,
      html: generateEmailHTML(formData)
    });

    return {
      statusCode: 200,
      body: JSON.stringify({ success: true, message: 'Email sent' })
    };
  } catch (error) {
    return {
      statusCode: 500,
      body: JSON.stringify({ success: false, error: error.message })
    };
  }
};
```

## Environment Variables
Add these to your `.env` file:

```bash
# For EmailJS
REACT_APP_EMAILJS_SERVICE_ID=your_service_id
REACT_APP_EMAILJS_TEMPLATE_ID=your_template_id  
REACT_APP_EMAILJS_PUBLIC_KEY=your_public_key

# For API integration
REACT_APP_EMAIL_API_KEY=your_api_key
REACT_APP_EMAIL_API_ENDPOINT=https://your-api.com/send

# For backend services
REACT_APP_BACKEND_URL=https://your-backend.com
```

## Email Templates
The service includes professional HTML and plain text email templates with:

- 🎨 Parvyom branding and colors
- 📱 Responsive design
- 🔒 XSS protection and input sanitization
- 📧 Proper email headers and formatting
- ⚡ Category-based routing

## Security Features
- Input sanitization to prevent XSS attacks
- Email validation
- Rate limiting ready (implement in backend)
- Secure environment variable handling

## Testing
The current implementation includes console logging for development testing. Check browser console to see email payload structure.

## Next Steps
1. Choose an email service provider (EmailJS, SendGrid, AWS SES, etc.)
2. Set up email templates in your chosen service
3. Configure environment variables
4. Update the `sendContactEmail` function
5. Test with real email delivery
6. Implement rate limiting and spam protection

## Support
For questions about email service integration, contact the development team or refer to the chosen email service provider's documentation.
